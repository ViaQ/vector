use metrics::{Counter, Key, Label, Metadata};
use tracing::trace;

use super::{ByteSize, Protocol, SharedString};

crate::registered_event!(
    BytesSent {
        protocol: SharedString,
        extra_labels: Vec<(SharedString, SharedString)>,
    } => {
        bytes_sent: Counter = {
            let mut labels = vec![Label::new("protocol", self.protocol.clone())];
            for (k, v) in &self.extra_labels {
                labels.push(Label::new(k.clone(), v.clone()));
            }
            let key = Key::from_parts("component_sent_bytes_total", labels);
            metrics::with_recorder(|rec| rec.register_counter(&key, &Metadata::new(module_path!(), metrics::Level::INFO, None)))
        },
        protocol: SharedString = self.protocol,
    }

    fn emit(&self, byte_size: ByteSize) {
        trace!(message = "Bytes sent.", byte_size = %byte_size.0, protocol = %self.protocol);
        self.bytes_sent.increment(byte_size.0 as u64);
    }
);

impl From<Protocol> for BytesSent {
    fn from(protocol: Protocol) -> Self {
        Self {
            protocol: protocol.0,
            extra_labels: vec![],
        }
    }
}
