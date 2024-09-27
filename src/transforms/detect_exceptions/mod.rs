mod exception_detector;
mod rules;
use chrono::Utc;
pub use exception_detector::*;

use serde_with::serde_as;

use crate::{
    config::{DataType, Input, OutputId, TransformOutput, TransformConfig, TransformContext, TransformDescription},
    event::{discriminant::Discriminant, Event},
    schema,
    transforms::{TaskTransform, Transform}
};
use async_stream::stream;
use futures::{stream, Stream, StreamExt};
use std::{collections::HashMap, pin::Pin, time::Duration};
use vector_lib::{
    configurable::configurable_component,
    config::{log_schema,LogNamespace, clone_input_definitions},
    enrichment,
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
pub struct DetectExceptionsConfig{
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
        _: enrichment::TableRegistry,
        input_definitions: &[(OutputId, schema::Definition)],
        _: LogNamespace,
    ) -> Vec<TransformOutput> {
        vec![TransformOutput::new(
            DataType::Log,
            clone_input_definitions(input_definitions),
        )]
    }
}

pub struct DetectExceptions {
    accumulators: HashMap<Discriminant, TraceAccumulator>,
    languages: Vec<ProgrammingLanguages>,
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
        let owned_target_path: OwnedTargetPath;
        match parse_target_path(config.message_key.as_str()){
            Err(e) => return Err(e.into()),
            Ok(value) =>{
                owned_target_path = value
            },
        };

        Ok(DetectExceptions {
            accumulators: HashMap::new(),
            languages: config.languages.clone(),
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
                    self.languages.clone(),
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
            if v.accumulated_messages.len() == 0 {
                if now.timestamp_millis() - v.buffer_start_time.timestamp_millis()
                    > self.expire_after.as_millis().try_into().unwrap()
                {
                    for_removal.push(k.to_owned());
                }
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
}
