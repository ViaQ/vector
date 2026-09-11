mod exception_detector;
mod rules;
use chrono::Utc;
pub use exception_detector::*;

use serde_with::serde_as;

use crate::{
    config::{
        DataType, Input, OutputId, TransformConfig, TransformContext, TransformDescription,
        TransformOutput,
    },
    event::{Event, discriminant::Discriminant},
    schema,
    transforms::{TaskTransform, Transform},
};
use async_stream::stream;
use futures::{Stream, StreamExt, stream};
use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};
use vector_lib::{
    config::{clone_input_definitions, log_schema},
    configurable::configurable_component,
};

use vector_lib::lookup::path::OwnedTargetPath;
use vector_lib::lookup::path::parse_target_path;

/// ProgrammingLanguages
#[configurable_component]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ProgrammingLanguages {
    /// Java
    Java,

    /// Javascript
    Javascript,
    /// Javascript
    Js,

    /// Csharp
    Csharp,

    /// Python
    Python,
    /// Python
    Py,

    /// Php
    Php,

    /// Go
    Go,

    /// Ruby
    Ruby,
    /// Ruby
    Rb,

    /// Dart
    Dart,

    /// All languages
    All,
}

/// Configuration for the `detect_exceptions` transform.
#[serde_as]
#[configurable_component(transform("detect_exceptions"))]
#[derive(Debug, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct DetectExceptionsConfig {
    /// Programming Languages for which to detect Exceptions
    ///
    /// Supported languages are
    ///   - Java
    ///   - Python
    ///   - Go
    ///   - Ruby
    ///   - Php
    ///   - Dart
    ///   - All (includes all above)
    #[serde(default = "default_programming_languages")]
    pub languages: Vec<ProgrammingLanguages>,

    /// The maximum period of time to wait after the last event is received, in milliseconds, before
    /// a combined event should be considered complete.
    #[serde(default = "default_expire_after_ms")]
    #[serde_as(as = "serde_with::DurationMilliSeconds<u64>")]
    pub expire_after_ms: Duration,

    /// The interval to check for and flush any expired events, in milliseconds.
    #[serde(default = "default_flush_period_ms")]
    #[serde_as(as = "serde_with::DurationMilliSeconds<u64>")]
    pub flush_period_ms: Duration,

    /// An ordered list of fields by which to group events.
    ///
    /// Each group with matching values for the specified keys is reduced independently, allowing
    /// you to keep independent event streams separate. When no fields are specified, all events
    /// will be combined in a single group.
    ///
    /// For example, if `group_by = ["host", "region"]`, then all incoming events that have the same
    /// host and region will be grouped together before being reduced.
    #[serde(default)]
    pub group_by: Vec<String>,

    /// The key path to use to find the message of a log event
    pub message_key: String,

    /// The interval of flushing the buffer for multiline exceptions.
    #[serde(default = "default_multiline_flush_interval_ms")]
    #[serde_as(as = "serde_with::DurationMilliSeconds<u64>")]
    pub multiline_flush_interval_ms: Duration,

    /// Maximum number of bytes to flush (0 means no limit). Default: 0.
    #[serde(default = "default_max_bytes_size")]
    pub max_bytes: usize,

    /// Maximum number of lines to flush (0 means no limit). Default: 1000.
    #[serde(default = "default_max_lines_num")]
    pub max_lines: usize,
}

impl Default for DetectExceptionsConfig {
    fn default() -> Self {
        Self {
            languages: default_programming_languages(),
            expire_after_ms: default_expire_after_ms(),
            flush_period_ms: default_flush_period_ms(),
            multiline_flush_interval_ms: default_multiline_flush_interval_ms(),
            max_bytes: default_max_bytes_size(),
            max_lines: default_max_lines_num(),
            message_key: log_schema().owned_message_path().to_string(),
            group_by: vec![],
        }
    }
}

fn default_programming_languages() -> Vec<ProgrammingLanguages> {
    vec![ProgrammingLanguages::All]
}

const fn default_expire_after_ms() -> Duration {
    Duration::from_millis(30000)
}

const fn default_flush_period_ms() -> Duration {
    Duration::from_millis(1000)
}

const fn default_multiline_flush_interval_ms() -> Duration {
    Duration::from_millis(1000)
}

const fn default_max_bytes_size() -> usize {
    0
}

const fn default_max_lines_num() -> usize {
    1000
}

impl_generate_config_from_default!(DetectExceptionsConfig);
inventory::submit! {
    TransformDescription::new::<DetectExceptionsConfig>("detect_exceptions", "detect_exceptions", "detect_exceptions", "detect_exceptions")
}

#[async_trait::async_trait]
#[typetag::serde(name = "detect_exceptions")]
impl TransformConfig for DetectExceptionsConfig {
    async fn build(&self, _context: &TransformContext) -> crate::Result<Transform> {
        DetectExceptions::new(self).map(Transform::event_task)
    }

    fn input(&self) -> Input {
        Input::log()
    }

    fn outputs(
        &self,
        _globals: &TransformContext,
        input_definitions: &[(OutputId, schema::Definition)],
    ) -> Vec<TransformOutput> {
        vec![TransformOutput::new(
            DataType::Log,
            clone_input_definitions(input_definitions),
        )]
    }
}

pub struct DetectExceptions {
    accumulators: HashMap<Discriminant, TraceAccumulator>,
    state_machine: Arc<exception_detector::StateMachine>,
    expire_after: Duration,
    flush_period: Duration,
    multiline_flush_interval: Duration,
    max_bytes: usize,
    max_lines: usize,
    message_key: OwnedTargetPath,
    group_by: Vec<String>,
}

impl DetectExceptions {
    pub fn new(config: &DetectExceptionsConfig) -> crate::Result<Self> {
        if config.languages.is_empty() {
            return Err("languages cannot be empty".into());
        }

        let owned_target_path: OwnedTargetPath = parse_target_path(config.message_key.as_str())?;

        // Create the state machine once and share it across all accumulators
        let state_machine = Arc::new(exception_detector::get_state_machines(
            config.languages.clone(),
        ));

        Ok(DetectExceptions {
            accumulators: HashMap::new(),
            state_machine,
            group_by: config.group_by.clone(),
            expire_after: config.expire_after_ms,
            multiline_flush_interval: config.multiline_flush_interval_ms,
            max_bytes: config.max_bytes,
            max_lines: config.max_lines,
            message_key: owned_target_path,
            flush_period: config.flush_period_ms,
        })
    }

    fn consume_one(&mut self, output: &mut Vec<Event>, e: Event) {
        let log_event = e.into_log();
        let discriminant = Discriminant::from_log_event(&log_event, &self.group_by);

        if !self.accumulators.contains_key(&discriminant) {
            self.accumulators.insert(
                discriminant.clone(),
                TraceAccumulator::new(
                    Arc::clone(&self.state_machine),
                    self.multiline_flush_interval,
                    self.max_bytes,
                    self.max_lines,
                    self.message_key.clone(),
                ),
            );
        }
        let accumulator = self.accumulators.get_mut(&discriminant).unwrap();
        accumulator.push(&log_event, output);
    }

    fn flush_all_into(&mut self, output: &mut Vec<Event>) {
        for (k, v) in &mut self.accumulators {
            debug!("flushing {:?}, size: {}", k, v.accumulated_messages.len());
            v.flush(output);
        }
    }

    fn flush_stale_into(&mut self, output: &mut Vec<Event>) {
        let now = Utc::now();
        let mut for_removal: Vec<Discriminant> = vec![];
        for (k, v) in &mut self.accumulators {
            v.flush_stale_into(now, output);
            if v.accumulated_messages.is_empty()
                && now.timestamp_millis() - v.buffer_start_time.timestamp_millis()
                    > self.expire_after.as_millis().try_into().unwrap()
            {
                for_removal.push(k.to_owned());
            }
        }
        for d in for_removal {
            debug!("removing {:?}", d);
            self.accumulators.remove(&d);
        }
    }
}

impl TaskTransform<Event> for DetectExceptions {
    fn transform(
        self: Box<Self>,
        mut input_rx: Pin<Box<dyn Stream<Item = Event> + Send>>,
    ) -> Pin<Box<dyn Stream<Item = Event> + Send>>
    where
        Self: 'static,
    {
        let mut me = self;

        let poll_period = me.flush_period;

        let mut flush_stream = tokio::time::interval(poll_period);

        Box::pin(
            stream! {
              loop {
                let mut output = Vec::new();
                let done = tokio::select! {
                    _ = flush_stream.tick() => {
                      me.flush_stale_into(&mut output);
                      false
                    }
                    maybe_event = input_rx.next() => {
                      match maybe_event {
                        None => {
                          me.flush_all_into(&mut output);
                          true
                        }
                        Some(event) => {
                          me.consume_one(&mut output, event);
                          false
                        }
                      }
                    }
                };
                yield stream::iter(output.into_iter());
                if done { break }
              }
            }
            .flatten(),
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        config::TransformConfig,
        event::{LogEvent, Value},
    };

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<DetectExceptionsConfig>();
    }

    #[test]
    fn test_generate_config() {
        toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["All"]
group_by = ["kubernetes.namespace_name","kubernetes.pod_name","kubernetes.container_name", "kubernetes.pod_id"]
expire_after_ms = 2000
multiline_flush_interval_ms = 1000
"#,
        )
        .unwrap();
    }

    #[tokio::test]
    async fn test_exception_detector() {
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["Java"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let java_simple_exception = "
Jul 09, 2015 3:23:29 PM com.google.devtools.search.cloud.feeder.MakeLog: RuntimeException: Run from this message!
    at com.my.app.Object.do$a1(MakeLog.java:50)
    at java.lang.Thing.call(Thing.java:10)
    at com.my.app.Object.help(MakeLog.java:40)
    at sun.javax.API.method(API.java:100)
    at com.jetty.Framework.main(MakeLog.java:30)
 ";
        let java_simple_log = "Jul 09, 2015 3:23:39 PM new log message";

        let lines = format!("{}\n{}", java_simple_exception.trim(), java_simple_log);

        let mut counter = 0;
        let input_events: Vec<Event> = lines
            .trim()
            .split("\n")
            .map(|line| {
                let mut le = LogEvent::from(line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_1["message"], java_simple_exception.trim().into());
        assert_eq!(output_1["counter"], Value::from(0));

        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], java_simple_log.trim().into());
        assert_eq!(output_2["counter"], Value::from(6));
    }

    #[tokio::test]
    async fn test_exception_with_non_indented_continuation_lines() {
        // LOG-9995: Oracle ORA errors have non-indented lines between the
        // exception header and the stack trace. Targeted continuation patterns
        // must group these correctly.
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["Java"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let exception_with_continuation = "\
java.sql.SQLException: Listener refused the connection with the following error:
ORA-12521, TNS:listener does not currently know of instance requested in connect descriptor
  (CONNECTION_ID=r6n2ZPL0TqS/BLDhIydj+A==)
    at oracle.jdbc.driver.T4CConnection.handleLogonNetException(T4CConnection.java:893)
    at oracle.jdbc.driver.T4CConnection.logon(T4CConnection.java:698)";
        let regular_log = "2026-06-02 13:55:59.506 INFO normal log message";

        let lines = format!("{}\n{}", exception_with_continuation, regular_log);

        let mut counter = 0;
        let input_events: Vec<Event> = lines
            .split("\n")
            .map(|line| {
                let mut le = LogEvent::from(line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(
            output_1["message"],
            exception_with_continuation.into(),
            "All exception lines including non-indented continuations must be merged"
        );
        assert_eq!(output_1["counter"], Value::from(0));

        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], regular_log.into());
        assert_eq!(output_2["counter"], Value::from(5));
    }

    #[tokio::test]
    async fn test_json_messages_with_exception_keywords_not_merged() {
        // LOG-9963: When parse_json runs before detect_exceptions, the message
        // field contains parsed string values. If a message contains an
        // exception keyword, subsequent independent messages must NOT be merged.
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["All"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let msg1 = "SecretConfigException: Failed to load configuration from vault";
        let msg2 = "Processing request for user 12345";
        let msg3 = "Response sent in 42ms";

        let mut counter = 0;
        let input_events: Vec<Event> = [msg1, msg2, msg3]
            .iter()
            .map(|line| {
                let mut le = LogEvent::from(*line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_1["message"], msg1.into());
        assert_eq!(output_1["counter"], Value::from(0));

        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], msg2.into());
        assert_eq!(output_2["counter"], Value::from(1));

        let output_3 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_3["message"], msg3.into());
        assert_eq!(output_3["counter"], Value::from(2));
    }

    #[tokio::test]
    async fn test_raw_json_log_lines_not_merged() {
        // LOG-9963: When detect_exceptions runs WITHOUT parse_json, the message
        // field contains the entire raw JSON line. Exception keywords embedded
        // inside JSON field values must not trigger merging of subsequent
        // independent JSON records.
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["All"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let json1 = r#"{"timestamp":"2026-08-28T10:00:00Z","level":"ERROR","message":"SecretConfigException: Failed to load config","stackTrace":"java.lang.Throwable\n\tat com.example.Config.load(Config.java:42)"}"#;
        let json2 = r#"{"timestamp":"2026-08-28T10:00:01Z","level":"INFO","message":"Processing request for user 12345"}"#;
        let json3 = r#"{"timestamp":"2026-08-28T10:00:02Z","level":"INFO","message":"Response sent in 42ms"}"#;

        let mut counter = 0;
        let input_events: Vec<Event> = [json1, json2, json3]
            .iter()
            .map(|line| {
                let mut le = LogEvent::from(*line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_1["message"], json1.into());
        assert_eq!(output_1["counter"], Value::from(0));

        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], json2.into());
        assert_eq!(output_2["counter"], Value::from(1));

        let output_3 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_3["message"], json3.into());
        assert_eq!(output_3["counter"], Value::from(2));
    }

    #[tokio::test]
    async fn test_consecutive_exception_keyword_messages_not_merged() {
        // LOG-9963: Multiple independent messages each containing exception
        // keywords must be emitted as separate events, not merged.
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["All"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let msg1 = "ConnectTimeoutException: connection to host timed out";
        let msg2 = "RestException: upstream returned 503";
        let msg3 = "IllegalArgumentException: parameter must not be null";
        let msg4 = "Normal log message without exception keywords";

        let mut counter = 0;
        let input_events: Vec<Event> = [msg1, msg2, msg3, msg4]
            .iter()
            .map(|line| {
                let mut le = LogEvent::from(*line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_1["message"], msg1.into());
        assert_eq!(output_1["counter"], Value::from(0));

        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], msg2.into());
        assert_eq!(output_2["counter"], Value::from(1));

        let output_3 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_3["message"], msg3.into());
        assert_eq!(output_3["counter"], Value::from(2));

        let output_4 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_4["message"], msg4.into());
        assert_eq!(output_4["counter"], Value::from(3));
    }

    #[tokio::test]
    async fn test_real_exception_still_grouped_after_json_fix() {
        // Verify that removing the catch-all does not break grouping of
        // real multi-line exceptions (standard indented stack traces).
        let detect_exceptions = toml::from_str::<DetectExceptionsConfig>(
            r#"
languages = ["Java"]
"#,
        )
        .unwrap()
        .build(&TransformContext::default())
        .await
        .unwrap();

        let detect_exceptions = detect_exceptions.into_task();

        let exception = "\
java.lang.NullPointerException: null
    at com.example.MyClass.myMethod(MyClass.java:42)
    at com.example.Main.main(Main.java:10)";
        let normal_log_1 = "Application started successfully";
        let exception_keyword_log = "JobException: scheduled task failed";
        let normal_log_2 = "Shutting down gracefully";

        let lines = format!(
            "{}\n{}\n{}\n{}",
            exception, normal_log_1, exception_keyword_log, normal_log_2
        );

        let mut counter = 0;
        let input_events: Vec<Event> = lines
            .split("\n")
            .map(|line| {
                let mut le = LogEvent::from(line);
                le.insert("counter", counter);
                counter += 1;
                Event::Log(le)
            })
            .collect();

        let in_stream = Box::pin(stream::iter(input_events));
        let mut out_stream = detect_exceptions.transform_events(in_stream);

        // The real exception (3 lines) must still be grouped.
        let output_1 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_1["message"], exception.into());
        assert_eq!(output_1["counter"], Value::from(0));

        // Each subsequent line is its own event.
        let output_2 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_2["message"], normal_log_1.into());
        assert_eq!(output_2["counter"], Value::from(3));

        let output_3 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_3["message"], exception_keyword_log.into());
        assert_eq!(output_3["counter"], Value::from(4));

        let output_4 = out_stream.next().await.unwrap().into_log();
        assert_eq!(output_4["message"], normal_log_2.into());
        assert_eq!(output_4["counter"], Value::from(5));
    }
}
