use std::fmt::Write as _;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    /// A buffer for recording internal events emitted by a single test.
    static EVENTS_RECORDED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Returns Ok(()) if the event name pattern is matched only once.
///
/// # Errors
///
/// Will return `Err` if `pattern` is not found in the event record, or is found multiple times.
pub fn contains_name_once(pattern: &str) -> Result<(), String> {
    EVENTS_RECORDED.with(|events| {
        let events = events.borrow();
        let matches: Vec<_> = events.iter().filter(|e| e.ends_with(pattern)).collect();

        match matches.len() {
            0 => Err(format!("Missing event `{pattern}`")),
            1 => Ok(()),
            n => {
                let mut names = String::new();
                for (i, event) in matches.iter().enumerate() {
                    if i > 0 {
                        names.push_str(", ");
                    }
                    _ = write!(names, "`{event}`");
                }
                Err(format!(
                    "Multiple ({n}) events matching `{pattern}`: ({names}). Hint! Don't use the `assert_x_` \
                     test helpers on round-trip tests (tests that run more than a single component)."
                ))
            }
        }
    })
}

pub fn clear_recorded_events() {
    EVENTS_RECORDED.with(|er| er.borrow_mut().clear());
}

#[allow(clippy::print_stdout)]
pub fn debug_print_events() {
    EVENTS_RECORDED.with(|events| {
        for event in &*events.borrow() {
            println!("{event}");
        }
    });
}

/// Record an emitted internal event. This is somewhat dumb at this
/// point, just recording the pure string value of the `emit!` call
/// parameter. At some point, making all internal events implement
/// `Debug` or `Serialize` might allow for more sophistication here, but
/// this is good enough for these tests. This should only be used by the
/// test `emit!` macro. The `check-events` script will test that emitted
/// events contain the right fields, etc.
pub fn record_internal_event(event: &str) {
    // Remove leading '&'
    let event = event.strip_prefix('&').unwrap_or(event);
    // Remove trailing '{fields…}'
    let event = event.find('{').map_or(event, |par| &event[..par]);
    // Remove trailing '::from…'
    let event = event.find(':').map_or(event, |colon| &event[..colon]);

    EVENTS_RECORDED.with(|er| er.borrow_mut().insert(event.trim().into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_event() {
        clear_recorded_events();
        assert!(contains_name_once("BytesSent").is_err());
    }
}
