use std::fmt;
use bytes::{BufMut, BytesMut};
use tokio_util::codec::Encoder;
use vector_core::{config::DataType, event::{Event, LogEvent}, schema};
use chrono::{DateTime, SecondsFormat, Local};
use vrl::{event_path, value::Value};
use vector_config::configurable_component;
use lookup::lookup_v2::parse_target_path;
use std::fmt::Write;
use std::marker::PhantomData;
use serde::de::{self, Deserializer, Visitor};

const NILVALUE: &'static str = "-";

/// Syslog RFC
#[configurable_component]
#[derive(Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyslogRFC {
    /// RFC 3164
    Rfc3164,

    /// RFC 5424
    Rfc5424
}

impl Default for SyslogRFC {
    fn default() -> Self {
        SyslogRFC::Rfc5424
    }
}

/// Syslog facility
#[configurable_component]
#[derive(Clone, Debug, Eq, PartialEq)]
enum Facility {
    /// Syslog facility ordinal number
    Fixed(u8),

    /// Syslog facility name
    Field(String)
}

impl Default for Facility {
    fn default() -> Self {
        Facility::Fixed(1)
    }
}

/// Syslog severity
#[configurable_component]
#[derive(Clone, Debug, Eq, PartialEq)]
enum Severity {
    /// Syslog severity ordinal number
    Fixed(u8),

    /// Syslog severity name
    Field(String)
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Fixed(6)
    }
}

trait SyslogCode: Sized {
    fn from_fixed(num: u8) -> Self;
    fn from_field(field: String) -> Self;
    fn as_fixed(&self) -> Option<u8>;
    fn as_field(&self) -> Option<&str>;
    fn try_parse_str(s: &str) -> Option<u8>;
    fn max_value() -> u8;
    fn default_value() -> u8;
}

impl SyslogCode for Facility {
    fn from_fixed(num: u8) -> Self {
        Facility::Fixed(num)
    }

    fn from_field(field: String) -> Self {
        Facility::Field(field)
    }

    fn as_fixed(&self) -> Option<u8> {
        match self {
            Facility::Fixed(n) => Some(*n),
            _ => None,
        }
    }

    fn as_field(&self) -> Option<&str> {
        match self {
            Facility::Field(f) => Some(f),
            _ => None,
        }
    }

    fn try_parse_str(s: &str) -> Option<u8> {

        if let Ok(num) = s.parse::<u8>() {
            if num <= Self::max_value() {
                return Some(num);
            } else {
                return None;
            }
        }

        let s = s.to_uppercase();
        match s.as_str() {
            "KERN" => Some(0),
            "USER" => Some(1),
            "MAIL" => Some(2),
            "DAEMON" => Some(3),
            "AUTH" => Some(4),
            "SYSLOG" => Some(5),
            "LPR" => Some(6),
            "NEWS" => Some(7),
            "UUCP" => Some(8),
            "CRON" => Some(9),
            "AUTHPRIV" => Some(10),
            "FTP" => Some(11),
            "NTP" => Some(12),
            "SECURITY" => Some(13),
            "CONSOLE" => Some(14),
            "SOLARIS-CRON" => Some(15),
            "LOCAL0" => Some(16),
            "LOCAL1" => Some(17),
            "LOCAL2" => Some(18),
            "LOCAL3" => Some(19),
            "LOCAL4" => Some(20),
            "LOCAL5" => Some(21),
            "LOCAL6" => Some(22),
            "LOCAL7" => Some(23),
            _ => None,
        }
    }

    fn max_value() -> u8 {
        23
    }

    fn default_value() -> u8 {
        1
    }
}

impl SyslogCode for Severity {
    fn from_fixed(num: u8) -> Self {
        Severity::Fixed(num)
    }

    fn from_field(field: String) -> Self {
        Severity::Field(field)
    }

    fn as_fixed(&self) -> Option<u8> {
        match self {
            Severity::Fixed(n) => Some(*n),
            _ => None,
        }
    }

    fn as_field(&self) -> Option<&str> {
        match self {
            Severity::Field(f) => Some(f),
            _ => None,
        }
    }

    fn try_parse_str(s: &str) -> Option<u8> {
        if let Ok(num) = s.parse::<u8>() {
            if num <= Self::max_value() {
                return Some(num);
            } else {
                return None;
            }
        }

        match s.to_uppercase().as_str() {
            "EMERGENCY" => Some(0),
            "ALERT" => Some(1),
            "CRITICAL" => Some(2),
            "ERROR" => Some(3),
            "WARNING" => Some(4),
            "NOTICE" => Some(5),
            "INFORMATIONAL" => Some(6),
            "DEBUG" => Some(7),
            _ => None,
        }
    }

    fn max_value() -> u8 {
        7
    }

    fn default_value() -> u8 {
        6
    }
}

struct SyslogCodeVisitor<T: SyslogCode>(PhantomData<T>);

impl<'de, T: SyslogCode> Visitor<'de> for SyslogCodeVisitor<T> {
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an integer, a numeric string, a named string, or a field reference like $.name")
    }

    fn visit_u64<E>(self, value: u64) -> Result<T, E>
    where
        E: de::Error,
    {
        if value <= T::max_value() as u64 {
            Ok(T::from_fixed(value as u8))
        } else {
            Err(E::custom("numeric value too large"))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: de::Error,
    {
        if let Ok(num) = value.parse::<u8>() {
            if num <= T::max_value() {
                return Ok(T::from_fixed(num));
            } else {
                return Err(E::custom("numeric string too large"));
            }
        }

        if value.starts_with("$.") {
            return Ok(T::from_field(value.to_string()));
        }

        match T::try_parse_str(value) {
            Some(num) => Ok(T::from_fixed(num)),
            None => Err(E::invalid_value(de::Unexpected::Str(value), &"unknown named value")),
        }
    }
}

/// Config used to build a `SyslogSerializer`.
#[configurable_component]
#[derive(Debug, Clone, Default)]
pub struct SyslogSerializerConfig {
    /// RFC
    #[serde(default)]
    rfc: SyslogRFC,

    /// Facility
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_facility")]
    facility: Facility,

    /// Severity
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_severity")]
    severity: Severity,

    /// Tag
    #[serde(default)]
    tag: String,

    /// Trim prefix
    trim_prefix: Option<String>,

    /// Payload key
    #[serde(default)]
    payload_key: String,

    /// Add log source
    #[serde(default)]
    add_log_source: bool,

    /// Namespace_name key
    #[serde(default = "default_namespace_name_key")]
    namespace_name_key: String,

    /// Container_name key
    #[serde(default = "default_container_name_key")]
    container_name_key: String,

    /// Pod_name key
    #[serde(default = "default_pod_name_key")]
    pod_name_key: String,

    /// App Name, RFC 5424 only
    #[serde(default = "default_app_name")]
    app_name: String,

    /// Proc ID, RFC 5424 only
    #[serde(default = "default_nilvalue")]
    proc_id: String,

    /// Msg ID, RFC 5424 only
    #[serde(default = "default_nilvalue")]
    msg_id: String
}

impl SyslogSerializerConfig {
    /// Build the `SyslogSerializer` from this configuration.
    pub fn build(&self) -> SyslogSerializer {
        SyslogSerializer::new(&self)
    }

    /// The data type of events that are accepted by `SyslogSerializer`.
    pub fn input_type(&self) -> DataType {
        DataType::Log
    }

    /// The schema required by the serializer.
    pub fn schema_requirement(&self) -> schema::Requirement {
        schema::Requirement::empty()
    }
}

/// Serializer that converts an `Event` to bytes using the Syslog format.
#[derive(Debug, Clone)]
pub struct SyslogSerializer {
    config: SyslogSerializerConfig
}

impl SyslogSerializer {
    /// Creates a new `SyslogSerializer`.
    pub fn new(conf: &SyslogSerializerConfig) -> Self {
        Self { config: conf.clone() }
    }

    fn build_rfc3164(&self, pri: u8, log: &LogEvent) -> String {
        let mut buf = String::new();
        let ts = get_timestamp(log).format("%b %e %H:%M:%S");
        let hostname = get_field("hostname", log);
        let tag = get_field_or_config(&self.config.tag, log);
        write!(buf, "<{}>{} {} {}: ", pri, ts, hostname, tag).unwrap();

        if self.config.add_log_source {
            add_log_source(
                log,
                &mut buf,
                &self.config.namespace_name_key,
                &self.config.container_name_key,
                &self.config.pod_name_key,
            );
        }
        buf
    }

    fn build_rfc5424(&self, pri: u8, log: &LogEvent) -> String {
        let timestamp = get_timestamp(log).to_rfc3339_opts(SecondsFormat::Millis, true);
        let hostname = get_field("hostname", log);
        let app_name = get_field_or_config(&self.config.app_name, log);
        let proc_id = get_field_or_config(&self.config.proc_id, log);
        let msg_id = get_field_or_config(&self.config.msg_id, log);
        let mut buf = String::new();
        write!(buf, "<{}>1 {} {} {} {} {} - ", pri, timestamp, hostname, app_name, proc_id, msg_id).unwrap();

        if self.config.add_log_source {
            add_log_source(
                log,
                &mut buf,
                &self.config.namespace_name_key,
                &self.config.container_name_key,
                &self.config.pod_name_key,
            );
        }
        buf
    }

    fn build_payload(&self, log: &LogEvent) -> Vec<u8> {
        if self.config.payload_key.is_empty() {
            serde_json::to_vec(log).unwrap_or_default()
        } else {
            get_field(&self.config.payload_key, log).as_bytes().to_vec()
        }
    }

    fn remove_internal_data(log: &mut LogEvent) {
        let parsed_path = parse_target_path(&"_syslog").unwrap();
        log.remove_prune(&parsed_path, false);
    }
}

impl Encoder<Event> for SyslogSerializer {
    type Error = vector_common::Error;

    fn encode(&mut self, event: Event, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        match event {
            Event::Log(mut log) => {
                let facility = get_num_facility(&self.config.facility, &log);
                let severity = get_num_severity(&self.config.severity, &log);
                let pri = facility * 8 + severity;

                let header = match self.config.rfc {
                    SyslogRFC::Rfc3164 => self.build_rfc3164(pri, &log),
                    SyslogRFC::Rfc5424 => self.build_rfc5424(pri, &log),
                };

                Self::remove_internal_data(&mut log);

                let mut payload = self.build_payload(&log);
                let mut vec = header.into_bytes();
                vec.append(&mut payload);
                buffer.put_slice(&vec);
            },
            _ => {}
        }
        Ok(())
    }
}


fn deserialize_syslog_code<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: SyslogCode,
{
    deserializer.deserialize_any(SyslogCodeVisitor::<T>(PhantomData))
}

fn deserialize_facility<'de, D>(d: D) -> Result<Facility, D::Error>
where D: Deserializer<'de>,
{
    deserialize_syslog_code(d)
}

fn deserialize_severity<'de, D>(d: D) -> Result<Severity, D::Error>
where D: Deserializer<'de>,
{
    deserialize_syslog_code(d)
}

fn default_app_name() -> String {
    String::from("vector")
}

fn default_namespace_name_key() -> String {
    String::from(".kubernetes.namespace_name")
}

fn default_container_name_key() -> String {
    String::from(".kubernetes.container_name")
}

fn default_pod_name_key() -> String {
    String::from(".kubernetes.pod_name")
}

fn default_nilvalue() -> String {
    String::from(NILVALUE)
}

fn resolve_syslog_code<T: SyslogCode>(code: &T, log: &LogEvent, get_field: impl Fn(&str, &LogEvent) -> String) -> u8 {
    if let Some(num) = code.as_fixed() {
        return num;
    }

    if let Some(field_name) = code.as_field() {
        if field_name.starts_with("$.") {
            let field_name = field_name.strip_prefix("$.").unwrap_or(field_name);
            let raw_value = get_field(field_name, log);
            if let Ok(num) = raw_value.parse::<u8>() {
                if num <= T::max_value() {
                    return num;
                }
            }
            return T::try_parse_str(&raw_value).unwrap_or(T::default_value());
        } else {
            return T::try_parse_str(field_name).unwrap_or(T::default_value());
        }
    }
    T::default_value()
}
fn get_value_from_path(log: &LogEvent, path: &String, default: &String) -> String {
    let parsed_path = parse_target_path(&path).unwrap_or_else(|_| parse_target_path(&default).unwrap());
    if let Some(field_value) = log.get(&parsed_path) {
        String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default()
    } else {
        NILVALUE.to_string()
    }
}

fn add_log_source(log: &LogEvent, buf: &mut String, namespace_name_path: &String, container_name_path: &String, pod_name_path: &String) {
    let namespace = get_value_from_path(log, namespace_name_path, &default_namespace_name_key());
    let container = get_value_from_path(log, container_name_path, &default_container_name_key());
    let pod = get_value_from_path(log, pod_name_path, &default_pod_name_key());

    if namespace == NILVALUE && container == NILVALUE && pod == NILVALUE {
        return
    }

    buf.push_str("namespace_name=");
    buf.push_str(&namespace);

    buf.push_str(", container_name=");
    buf.push_str(&container);

    buf.push_str(", pod_name=");
    buf.push_str(&pod);

    buf.push_str(", message=");
}

fn get_num_facility(config_facility: &Facility, log: &LogEvent) -> u8 {
    resolve_syslog_code(config_facility, log, get_field)
}

fn get_num_severity(config_severity: &Severity, log: &LogEvent) -> u8 {
    resolve_syslog_code(config_severity, log, get_field)
}

fn get_field_or_config(config_name: &String, log: &LogEvent) -> String {
    config_name
        .strip_prefix("$.")
        .map(|field| get_field(field, log))
        .unwrap_or_else(|| config_name.clone())
}

fn get_field(field_name: &str, log: &LogEvent) -> String {
    log.parse_path_and_get_value(field_name)
        .ok()
        .flatten()
        .map(|v| String::from_utf8(v.coerce_to_bytes().to_vec()).unwrap_or_default())
        .unwrap_or_else(|| NILVALUE.to_string())
}

fn get_timestamp(log: &LogEvent) -> DateTime::<Local> {
    match log.get(event_path!("@timestamp")) {
        Some(value) => {
            if let Value::Timestamp(timestamp) = value {
                DateTime::<Local>::from(*timestamp)
            } else {
                Local::now()
            }
        },
        _ => Local::now()
    }
}

#[cfg(test)]
mod tests {

    use std::env;
    use std::ffi::OsString;
    use super::*;
    use regex::Regex;
    use serde::Deserialize;

    #[test]
    fn serialize_to_rfc3164() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let preamble = "<14>Jan  1 00:00:00 - :";
        let serialized = serialize_to_syslog(SyslogRFC::Rfc3164, false, log_event, NILVALUE.to_string(), NILVALUE.to_string(), NILVALUE.to_string());
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_to_rfc5424() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let preamble = "<14>1 1970-01-01T00:00:00.000Z -    -";
        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, false, log_event, NILVALUE.to_string(), NILVALUE.to_string(), NILVALUE.to_string());
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_to_rfc5424_with_fields() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let preamble = "<111>1 1970-01-01T00:00:00.000Z - foo bar xyz - {";
        let config = SyslogSerializerConfig{
            rfc: SyslogRFC::Rfc5424,
            app_name: "foo".to_string(),
            facility: Facility::Field("SECURITY".to_string()),
            severity: Severity::Fixed(7),
            proc_id: "bar".to_string(),
            msg_id: "xyz".to_string(),
            ..Default::default()
        };
        let serialized = serialize_to_syslog_with_config(log_event, config);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_to_rfc5424_with_num_fields() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let preamble = "<134>1 1970-01-01T00:00:00.000Z - foo bar xyz - {";
        let config = SyslogSerializerConfig{
            rfc: SyslogRFC::Rfc5424,
            app_name: "foo".to_string(),
            facility: Facility::Field("16".to_string()),
            severity: Severity::Fixed(6),
            proc_id: "bar".to_string(),
            msg_id: "xyz".to_string(),
            ..Default::default()
        };
        let serialized = serialize_to_syslog_with_config(log_event, config);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_to_rfc3164_with_fields() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let preamble = "<37>Jan  1 00:00:00 - xyz[bar]: {";
        let config = SyslogSerializerConfig{
            rfc: SyslogRFC::Rfc3164,
            facility: Facility::Field("AUTH".to_string()),
            severity: Severity::Fixed(5),
            tag: "xyz[bar]".to_string(),
            ..Default::default()
        };
        let serialized = serialize_to_syslog_with_config(log_event, config);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_with_fields_reference() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        let parsed_path = parse_target_path(&"_syslog.facility").unwrap();
        log_event.insert(&parsed_path, Value::Integer(7));
        let preamble = "<61>";
        let config = SyslogSerializerConfig{
            rfc: SyslogRFC::Rfc3164,
            facility: Facility::Field("$._syslog.facility".to_string()),
            severity: Severity::Fixed(5),
            ..Default::default()
        };
        let serialized = serialize_to_syslog_with_config(log_event, config);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
        assert!(!serialized.contains("_syslog)"));
    }

    #[test]
    fn serialize_with_fields_str_value() {
        let mut log_event = LogEvent::default();
        log_event.insert(&parse_target_path(&"_syslog.facility").unwrap(), "local0");
        log_event.insert(&parse_target_path(&"_syslog.severity").unwrap(), "Informational");
        let preamble = "<134>1";
        let config = SyslogSerializerConfig{
            rfc: SyslogRFC::Rfc5424,
            facility: Facility::Field("$._syslog.facility".to_string()),
            severity: Severity::Field("$._syslog.severity".to_string()),
            ..Default::default()
        };
        let serialized = serialize_to_syslog_with_config(log_event, config);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
        assert!(!serialized.contains("_syslog)"));
    }

    fn dummy_log_event_with_field() -> LogEvent {
        let json_str = r#"{
  "level": "default",
  "log_type": "application",
  "facility_num": 7,
  "facility_invalid": "invalid",
  "severity_invalid": "bad_severity",
  "severity_num": 4,
  "message": {
    "appname_key": "rec_appname",
    "msgcontent": "My life is my message",
    "msgid_key": "rec_msgid",
    "procid_key": "rec_procid",
    "timestamp": "2021-02-16 18:55:01",
    "facility_key": "syslog",
    "severity_key": "critical"
  },
  "_syslog": {
    "facility": "syslog",
    "severity": "critical"
  }
}"#;
        let value: Value = serde_json::from_str(json_str).unwrap();
        let log = LogEvent::from(value);
        log
    }

    #[test]
    fn get_field_common()  {
        let log = dummy_log_event_with_field();
        let str = get_field(&"message.appname_key".to_string(), &log);
        assert_eq!(str, "rec_appname");
        let str = get_field(&"message.facility_key".to_string(), &log);
        assert_eq!(str, "syslog");
    }

    #[test]
    fn get_field_or_config_prefixed() {
        let log = dummy_log_event_with_field();
        let config_name = "$.level".to_string();
        let result = get_field_or_config(&config_name, &log);
        assert_eq!(result, "default");
    }

    #[test]
    fn get_field_or_config_no_prefix() {
        let log = LogEvent::default();
        let config_name = "log_type".to_string();
        let result = get_field_or_config(&config_name, &log);
        assert_eq!(result, "log_type");
    }

    #[test]
    fn get_field_or_config_fallback() {
        let log = dummy_log_event_with_field();
        let config_name = "$.missing_key".to_string();
        let result = get_field_or_config(&config_name, &log);
        assert_eq!(result, "-");
    }


    #[test]
    fn fixed_facility() {
        let log = LogEvent::default();
        let facility = Facility::Fixed(5);
        assert_eq!(get_num_facility(&facility, &log), 5);
    }

    #[test]
    fn field_facility_num_field() {
        let mut log = LogEvent::default();
        log.insert(event_path!("facility_num"), 16);
        let facility = Facility::Field("$.facility_num".to_string());
        assert_eq!(get_num_facility(&facility, &log), 16);
    }

    #[test]
    fn field_facility() {
        let log = dummy_log_event_with_field();
        let facility = Facility::Field("$.message.facility_key".to_string());
        assert_eq!(get_num_facility(&facility, &log), 5); // SYSLOG = 5
    }

    #[test]
    fn field_facility_invalid() {
        let log = dummy_log_event_with_field();
        let facility = Facility::Field("facility_invalid".to_string());
        assert_eq!(get_num_facility(&facility, &log), 1); // falls back to default USER = 1
    }

    #[test]
    fn field_facility_string_value() {
        let log = LogEvent::default();
        let facility = Facility::Field("SECURITY".to_string());
        assert_eq!(get_num_facility(&facility, &log), 13);
    }

    #[test]
    fn field_facility_num_value() {
        let log = LogEvent::default();
        let facility = Facility::Field("13".to_string());
        assert_eq!(get_num_facility(&facility, &log), 13);
    }

    #[test]
    fn fixed_severity() {
        let log = LogEvent::default();
        let severity = Severity::Fixed(3);
        assert_eq!(get_num_severity(&severity, &log), 3);
    }

    #[test]
    fn severity_num_field() {
        let log = dummy_log_event_with_field();
        let severity = Severity::Field("$.severity_num".to_string());
        assert_eq!(get_num_severity(&severity, &log), 4);
    }

    #[test]
    fn field_severity() {
        let log = dummy_log_event_with_field();
        let severity = Severity::Field("$.message.severity_key".to_string());
        assert_eq!(get_num_severity(&severity, &log), 2); // CRITICAL = 2
    }

    #[test]
    fn field_severity_invalid() {
        let log = dummy_log_event_with_field();
        let severity = Severity::Field("severity_invalid".to_string());
        assert_eq!(get_num_severity(&severity, &log), 6); // falls back to default INFORMATIONAL = 6
    }


    #[derive(Deserialize)]
    struct TestSyslogConfig {
        #[serde(deserialize_with = "deserialize_facility")]
        facility: Facility,
        #[serde(deserialize_with = "deserialize_severity")]
        severity: Severity,
    }

    #[test]
    fn deserialize_syslog_field_numeric() {
        let json = r#"{ "facility": 3, "severity": 4 }"#;
        let cfg: TestSyslogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.facility, Facility::Fixed(3));
        assert_eq!(cfg.severity, Severity::Fixed(4));
    }

    #[test]
    fn deserialize_syslog_field_various() {
        let json = r#"{ "facility": "3", "severity": 4 }"#;
        let cfg: TestSyslogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.facility, Facility::Fixed(3));
        assert_eq!(cfg.severity, Severity::Fixed(4));
    }

    #[test]
    fn deserialize_syslog_named_field() {
        let json = r#"{ "facility": "AUTH", "severity": "WARNING" }"#;
        let cfg: TestSyslogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.facility, Facility::Fixed(4)); // AUTH = 4
        assert_eq!(cfg.severity, Severity::Fixed(4)); // WARNING = 4
    }

    #[test]
    fn deserialize_syslog_field() {
        let json = r#"{ "facility": "$.source_fac", "severity": "$.source_sev" }"#;
        let cfg: TestSyslogConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.facility, Facility::Field("$.source_fac".to_string()));
        assert_eq!(cfg.severity, Severity::Field("$.source_sev".to_string()));
    }

    #[test]
    fn deserialize_syslog_invalid_named() {
        let json = r#"{ "facility": "FOOBAR", "severity": "BAZ" }"#;
        let result = serde_json::from_str::<TestSyslogConfig>(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_syslog_code_too_large() {
        let json = r#"{ "facility": 99, "severity": 42 }"#;
        let result = serde_json::from_str::<TestSyslogConfig>(json);
        assert!(result.is_err());
    }


    #[test]
    fn add_log_source_true() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));
        log_event.insert(event_path!("kubernetes", "namespace_name"), "foo_namespace");
        log_event.insert(event_path!("kubernetes", "container_name"), "bar_container");
        log_event.insert(event_path!("kubernetes", "pod_name"), "baz_pod");

        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, true, log_event, default_namespace_name_key(), default_container_name_key(), default_pod_name_key());

        // Check for presence of log source namespace_name, container_name, pod_name
        let namespace_regex = Regex::new(r"namespace_name=foo_namespace").unwrap();
        let container_regex = Regex::new(r"container_name=bar_container").unwrap();
        let pod_regex = Regex::new(r"pod_name=baz_pod").unwrap();

        assert!(namespace_regex.is_match(serialized.as_str()), "namespace_name field not found");
        assert!(container_regex.is_match(serialized.as_str()), "container_name field not found");
        assert!(pod_regex.is_match(serialized.as_str()), "pod_name field not found");
    }

    #[test]
    fn add_log_source_true_with_no_log_source_event_data() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));

        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, true, log_event, NILVALUE.to_string(), NILVALUE.to_string(), NILVALUE.to_string());

        // Check for absence of log source namespace_name=, container_name=, pod_name=
        let namespace_regex = Regex::new(r"namespace_name=").unwrap();
        let container_regex = Regex::new(r"container_name=").unwrap();
        let pod_regex = Regex::new(r"pod_name=").unwrap();

        assert_eq!(namespace_regex.is_match(serialized.as_str()), false, "namespace_name= field was found");
        assert_eq!(container_regex.is_match(serialized.as_str()), false,"container_name= field was found");
        assert_eq!(pod_regex.is_match(serialized.as_str()), false,"pod_name= field was found");
    }

    #[test]
    fn add_log_source_true_custom_enrichment_paths() {
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));
        log_event.insert(event_path!("k8s", "namespace"), "k8s_name");
        log_event.insert(event_path!("k8s", "container"), "k8s_cont");
        log_event.insert(event_path!("k8s","pod"), "k8s_pod");

        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, true, log_event, String::from(".k8s.namespace"), String::from(".k8s.container"), String::from("k8s.pod"));

        // Check for presence of log source namespace_name, container_name, pod_name
        let namespace_regex = Regex::new(r"namespace_name=k8s_name").unwrap();
        let container_regex = Regex::new(r"container_name=k8s_cont").unwrap();
        let pod_regex = Regex::new(r"pod_name=k8s_pod").unwrap();

        assert!(namespace_regex.is_match(serialized.as_str()), "namespace_name field not found");
        assert!(container_regex.is_match(serialized.as_str()), "container_name field not found");
        assert!(pod_regex.is_match(serialized.as_str()), "pod_name field not found");
    }

    // set the local timezone to UTC for the duration of a scope
    // in order to get predictable event timestamp
    // from get_timestamp()
    //
    struct TZScope{
        tz: Option<OsString>,
    }

    impl TZScope {
        pub fn new() -> TZScope {
            let ret = Self{tz: env::var_os("TZ")};
            env::set_var("TZ", "UTC");
            ret
        }
    }

    impl Drop for TZScope {
        fn drop(&mut self) {
            match &self.tz {
                Some(val) => env::set_var("TZ", val),
                None => env::remove_var("TZ"),
            }
        }
    }

    fn serialize_to_syslog(rfc: SyslogRFC, add_log_source: bool, log_event: LogEvent, namespace_key: String, container_key: String, pod_key: String) -> String {
        let _tz_scope = TZScope::new();
        let config = SyslogSerializerConfig{
            add_log_source,
            rfc,
            namespace_name_key: namespace_key,
            container_name_key: container_key,
            pod_name_key: pod_key,
            ..Default::default()
        };

        let mut serializer = config.build();
        let event = Event::Log(log_event);
        let mut buffer = BytesMut::new();
        let res = serializer.encode(event, &mut buffer);
        assert!(res.is_ok());

        String::from_utf8((&buffer.freeze()[..]).to_vec()).unwrap()
    }

    fn serialize_to_syslog_with_config(log_event: LogEvent, config: SyslogSerializerConfig) -> String {
        let _tz_scope = TZScope::new();
        let mut serializer = config.build();
        let event = Event::Log(log_event);
        let mut buffer = BytesMut::new();
        let res = serializer.encode(event, &mut buffer);
        assert!(res.is_ok());

        String::from_utf8((&buffer.freeze()[..]).to_vec()).unwrap()
    }

}
