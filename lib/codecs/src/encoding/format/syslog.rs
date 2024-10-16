use bytes::{BufMut, BytesMut};
use tokio_util::codec::Encoder;
use vector_core::{config::DataType, event::{Event, LogEvent}, schema};
use chrono::{DateTime, SecondsFormat, Local};
use vrl::{event_path, value::Value};
use serde::{de, Deserialize};
use vector_config::configurable_component;


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
}

impl Encoder<Event> for SyslogSerializer {
    type Error = vector_common::Error;

    fn encode(&mut self, event: Event, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        match event {
            Event::Log(log) => {
                let mut buf = String::from("<");
                let pri = get_num_facility(&self.config.facility, &log) * 8 + get_num_severity(&self.config.severity, &log);
                buf.push_str(&pri.to_string());
                buf.push_str(">");
                match self.config.rfc {
                    SyslogRFC::Rfc3164 => {
                        let timestamp = get_timestamp(&log);
                        let formatted_timestamp = format!("{} ", timestamp.format("%b %e %H:%M:%S"));
                        buf.push_str(&formatted_timestamp);
                        buf.push_str(&get_field("hostname", &log));
                        buf.push(' ');
                        buf.push_str(&get_field_or_config(&self.config.tag, &log));
                        buf.push_str(": ");
                        if self.config.add_log_source {
                            add_log_source(&log, &mut buf);
                        }
                    },
                    SyslogRFC::Rfc5424 => {
                        buf.push_str("1 ");
                        let timestamp = get_timestamp(&log);
                        buf.push_str(&timestamp.to_rfc3339_opts(SecondsFormat::Millis, true));
                        buf.push(' ');
                        buf.push_str(&get_field("hostname", &log));
                        buf.push(' ');
                        buf.push_str(&get_field_or_config(&&self.config.app_name, &log));
                        buf.push(' ');
                        buf.push_str(&get_field_or_config(&&self.config.proc_id, &log));
                        buf.push(' ');
                        buf.push_str(&get_field_or_config(&&self.config.msg_id, &log));
                        buf.push_str(" - "); // no structured data
                        if self.config.add_log_source {
                            add_log_source(&log, &mut buf);
                        }
                    }
                }
                let mut payload = if self.config.payload_key.is_empty() {
                    serde_json::to_vec(&log).unwrap_or_default()
                } else {
                    get_field(&&self.config.payload_key, &log).as_bytes().to_vec()
                };
                let mut vec = buf.as_bytes().to_vec();
                vec.append(&mut payload);
                buffer.put_slice(&vec);
            },
            _ => {}
        }
        Ok(())
    }
}

fn deserialize_facility<'de, D>(d: D) -> Result<Facility, D::Error>
    where D: de::Deserializer<'de>
{
    let value: String = String::deserialize(d)?;
    let num_value = value.parse::<u8>();
    match num_value {
        Ok(num) => {
            if num > 23 {
                return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"facility number too large"));
            } else {
                return Ok(Facility::Fixed(num));
            }
        }
        Err(_) => {
            if let Some(field_name) = value.strip_prefix("$.message.") {
                return Ok(Facility::Field(field_name.to_string()));
            } else {
                let num = match value.to_uppercase().as_str() {
                    "KERN" => 0,
                    "USER" => 1,
                    "MAIL" => 2,
                    "DAEMON" => 3,
                    "AUTH" => 4,
                    "SYSLOG" => 5,
                    "LPR" => 6,
                    "NEWS" => 7,
                    "UUCP" => 8,
                    "CRON" => 9,
                    "AUTHPRIV" => 10,
                    "FTP" => 11,
                    "NTP" => 12,
                    "SECURITY" => 13,
                    "CONSOLE" => 14,
                    "SOLARIS-CRON" => 15,
                    "LOCAL0" => 16,
                    "LOCAL1" => 17,
                    "LOCAL2" => 18,
                    "LOCAL3" => 19,
                    "LOCAL4" => 20,
                    "LOCAL5" => 21,
                    "LOCAL6" => 22,
                    "LOCAL7" => 23,
                    _ => 24,
                };
                if num > 23 {
                    return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"unknown facility"));
                } else {
                    return Ok(Facility::Fixed(num))
                }
            }
        }
    }
}

fn deserialize_severity<'de, D>(d: D) -> Result<Severity, D::Error>
    where D: de::Deserializer<'de>
{
    let value: String = String::deserialize(d)?;
    let num_value = value.parse::<u8>();
    match num_value {
        Ok(num) => {
            if num > 7 {
                return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"severity number too large"))
            } else {
                return Ok(Severity::Fixed(num))
            }
        }
        Err(_) => {
            if let Some(field_name) = value.strip_prefix("$.message.") {
                return Ok(Severity::Field(field_name.to_string()));
            } else {
                let num = match value.to_uppercase().as_str() {
                    "EMERGENCY" => 0,
                    "ALERT" => 1,
                    "CRITICAL" => 2,
                    "ERROR" => 3,
                    "WARNING" => 4,
                    "NOTICE" => 5,
                    "INFORMATIONAL" => 6,
                    "DEBUG" => 7,
                    _ => 8,
                };
                if num > 7 {
                    return Err(de::Error::invalid_value(de::Unexpected::Unsigned(num as u64), &"unknown severity"))
                } else {
                    return Ok(Severity::Fixed(num))
                }
            }
        }
    }
}

fn default_app_name() -> String {
    String::from("vector")
}

fn default_nilvalue() -> String {
    String::from(NILVALUE)
}

fn add_log_source(log: &LogEvent, buf: &mut String) {
    let namespace_name = log.get(event_path!("kubernetes", "namespace_name"));
    let container_name = log.get(event_path!("kubernetes", "container_name"));
    let pod_name = log.get(event_path!("kubernetes", "pod_name"));

    if namespace_name.is_none() && container_name.is_none() && pod_name.is_none() {
        return
    }


    buf.push_str("namespace_name=");
    buf.push_str(&String::from_utf8(
        namespace_name
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()).unwrap());


    buf.push_str(", container_name=");
    buf.push_str(&String::from_utf8(
        container_name
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()).unwrap());


    buf.push_str(", pod_name=");    
    buf.push_str(&String::from_utf8(
        pod_name
        .map(|h| h.coerce_to_bytes())
        .unwrap_or_default().to_vec()).unwrap());

    buf.push_str(", message=");
}

fn get_num_facility(config_facility: &Facility, log: &LogEvent) -> u8 {
    match config_facility {
        Facility::Fixed(num) => return *num,
        Facility::Field(field_name) => {
            if let Some(field_value) = log.get(event_path!(field_name.as_str())) {
                let field_value_string = String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
                let num_value = field_value_string.parse::<u8>();
                match num_value {
                    Ok(num) => {
                        if num > 23 {
                            return 1 // USER
                        } else {
                            return num
                        }
                    }
                    Err(_) => {
                            let num = match field_value_string.to_uppercase().as_str() {
                                "KERN" => 0,
                                "USER" => 1,
                                "MAIL" => 2,
                                "DAEMON" => 3,
                                "AUTH" => 4,
                                "SYSLOG" => 5,
                                "LPR" => 6,
                                "NEWS" => 7,
                                "UUCP" => 8,
                                "CRON" => 9,
                                "AUTHPRIV" => 10,
                                "FTP" => 11,
                                "NTP" => 12,
                                "SECURITY" => 13,
                                "CONSOLE" => 14,
                                "SOLARIS-CRON" => 15,
                                "LOCAL0" => 16,
                                "LOCAL1" => 17,
                                "LOCAL2" => 18,
                                "LOCAL3" => 19,
                                "LOCAL4" => 20,
                                "LOCAL5" => 21,
                                "LOCAL6" => 22,
                                "LOCAL7" => 23,
                                _ => 24,
                            };
                            if num > 23 {
                                return 1 // USER
                            } else {
                                return num
                            }
                        }
                    }
            } else {
                return 1 // USER
            }
        }
    }
}

fn get_num_severity(config_severity: &Severity, log: &LogEvent) -> u8 {
    match config_severity {
        Severity::Fixed(num) => return *num,
        Severity::Field(field_name) => {
            if let Some(field_value) = log.get(event_path!(field_name.as_str())) {
                let field_value_string = String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
                let num_value = field_value_string.parse::<u8>();
                match num_value {
                    Ok(num) => {
                        if num > 7 {
                            return 6 // INFORMATIONAL
                        } else {
                            return num
                        }
                    }
                    Err(_) => {
                            let num = match field_value_string.to_uppercase().as_str() {
                                "EMERGENCY" => 0,
                                "ALERT" => 1,
                                "CRITICAL" => 2,
                                "ERROR" => 3,
                                "WARNING" => 4,
                                "NOTICE" => 5,
                                "INFORMATIONAL" => 6,
                                "DEBUG" => 7,
                                _ => 8,
                            };
                            if num > 7 {
                                return 6 // INFORMATIONAL
                            } else {
                                return num
                            }
                        }
                    }
            } else {
                return 6 // INFORMATIONAL
            }
        }
    }
}

fn get_field_or_config(config_name: &String, log: &LogEvent) -> String {
    if let Some(field_name) = config_name.strip_prefix("$.message.") {
        return get_field(field_name, log)
    } else {
        return config_name.clone()
    }
}

fn get_field(field_name: &str, log: &LogEvent) -> String {
    if let Some(field_value) = log.get(event_path!(field_name)) {
        return String::from_utf8(field_value.coerce_to_bytes().to_vec()).unwrap_or_default();
    } else {
        return NILVALUE.to_string()
    }
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

    #[test]
    fn serialize_to_rfc3164() {
        let preamble = "<14>Jan  1 00:00:00 - :";
        let serialized = serialize_to_syslog(SyslogRFC::Rfc3164, false, false);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn serialize_to_rfc5424() {
        let preamble = "<14>1 1970-01-01T00:00:00.000Z -    -";
        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, false, false);
        assert!(
            serialized.starts_with(preamble),
            "syslog message: '{}' did not start with expected preamble '{}'", serialized, preamble
        );
    }

    #[test]
    fn add_log_source_true() {
        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, true, true);

        // Check for presence of log source namespace_name, container_name, pod_name
        let namespace_regex = Regex::new(r"namespace_name=[^\s,]+").unwrap();
        let container_regex = Regex::new(r"container_name=[^\s,]+").unwrap();
        let pod_regex = Regex::new(r"pod_name=[^\s,]+").unwrap();

        assert!(namespace_regex.is_match(serialized.as_str()), "namespace_name field not found");
        assert!(container_regex.is_match(serialized.as_str()), "container_name field not found");
        assert!(pod_regex.is_match(serialized.as_str()), "pod_name field not found");
    }

    #[test]
    fn add_log_source_true_with_no_log_source_event_data() {
        let serialized = serialize_to_syslog(SyslogRFC::Rfc5424, true, false);

        // Check for absence of log source namespace_name=, container_name=, pod_name=
        let namespace_regex = Regex::new(r"namespace_name=").unwrap();
        let container_regex = Regex::new(r"container_name=").unwrap();
        let pod_regex = Regex::new(r"pod_name=").unwrap();

        assert_eq!(namespace_regex.is_match(serialized.as_str()), false, "namespace_name= field was found");
        assert_eq!(container_regex.is_match(serialized.as_str()), false,"container_name= field was found");
        assert_eq!(pod_regex.is_match(serialized.as_str()), false,"pod_name= field was found");
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

    fn serialize_to_syslog(rfc: SyslogRFC, add_log_source: bool, add_event_data: bool) -> String {
        let _tz_scope = TZScope::new();
        let config = SyslogSerializerConfig{
            add_log_source: add_log_source,
            rfc: rfc,
            ..Default::default()
        };
        let mut serializer = config.build();
        let mut log_event = LogEvent::from_str_legacy("barbaz");
        log_event.insert(event_path!("@timestamp"), Value::Timestamp(DateTime::from_timestamp(0, 0).unwrap()));
        if add_event_data {
            log_event.insert(event_path!("kubernetes", "namespace_name"), "foo_namespace");
            log_event.insert(event_path!("kubernetes", "container_name"), "bar_container");
            log_event.insert(event_path!("kubernetes", "pod_name"), "baz_pod");
        }
        let event = Event::Log(log_event);
        let mut buffer = BytesMut::new();
        let res = serializer.encode(event, &mut buffer);
        assert!(res.is_ok());

        return String::from_utf8((&buffer.freeze()[..]).to_vec()).unwrap();
    }
}
