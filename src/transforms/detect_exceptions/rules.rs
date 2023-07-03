use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ExceptionState {
    /// StartState for all languages
    StartState,

    /// Java states
    JavaStartException,
    JavaAfterException,
    Java,

    /// Python states
    Python,
    PythonCode,

    /// Php states
    PhpStackBegin,
    PhpStackFrames,

    /// Golang states
    GoAfterPanic,
    GoGoRoutine,
    GoAfterSignal,
    GoFrame1,
    GoFrame2,

    /// Ruby states
    RubyBeforeRailsTrace,
    Ruby,

    /// Dart states
    DartExc,
    DartStack,
    DartTypeErr1,
    DartTypeErr2,
    DartTypeErr3,
    DartTypeErr4,
    DartFormatErr1,
    DartFormatErr2,
    DartFormatErr3,
    DartMethodErr1,
    DartMethodErr2,
    DartMethodErr3,
}

#[derive(Debug, Clone)]
pub struct Rule<'a> {
    pub from_states: Vec<ExceptionState>,
    pub pattern: &'a str,
    pub to_state: ExceptionState,
}

fn rule(from_states: Vec<ExceptionState>, pattern_str: &str, to_state: ExceptionState) -> Rule {
    Rule {
        from_states,
        pattern: pattern_str,
        to_state,
    }
}

fn java_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(
            vec![StartState, JavaStartException],
            r"(?:(Exception|Error|Throwable|V8 errors stack trace)[:\r\n]|java[x]?\..*(Exception|Error))",
            JavaAfterException,
        ),
        rule(
            vec![StartState, JavaStartException],
            r"Error\s*$|V8 errors stack trace\s*$",
            JavaAfterException,
        ),
        rule(
            vec![JavaAfterException],
            r"^[\t ]*nested exception is:[\t ]*",
            JavaStartException,
        ),
        rule(vec![JavaAfterException], r"^[\r\n]*$", JavaAfterException),
        rule(vec![JavaAfterException, Java], "^[\t ]+(?:eval )?at ", Java),
        rule(
            vec![JavaAfterException, Java],
            // C# nested exception.
            r"^[\t ]+--- End of inner exception stack trace ---$",
            Java,
        ),
        rule(
            vec![JavaAfterException, Java],
            // C# exception from async code.
            r"^--- End of stack trace from previous (?x:
           )location where exception was thrown ---$",
            Java,
        ),
        rule(
            vec![JavaAfterException, Java],
            r"^[\t ]*(?:Caused by|Suppressed):",
            JavaAfterException,
        ),
        rule(
            vec![JavaAfterException, Java],
            r"^[\t ]*... \d+ (?:more|common frames omitted)",
            Java,
        ),
    ]
}

fn python_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(
            vec![StartState],
            r"^Traceback \(most recent call last\):$",
            Python,
        ),
        rule(vec![Python], r"^[\t ]+File ", PythonCode),
        rule(vec![PythonCode], r"[^\t ]", Python),
        rule(vec![Python], r"^(?:[^\s.():]+\.)*[^\s.():]+:", StartState),
    ]
}

fn php_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(
            vec![StartState],
            r"(?:PHP\s(?:Notice|Parse\serror|Fatal\serror|Warning):)|(?:exception\s'[^']+'\swith\smessage\s')",
            PhpStackBegin,
        ),
        rule(vec![PhpStackBegin], r"^Stack trace:", PhpStackFrames),
        rule(vec![PhpStackFrames], r"^#\d", PhpStackFrames),
        rule(vec![PhpStackFrames], r"^\s+thrown in ", StartState),
    ]
}

fn go_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(vec![StartState], r"\bpanic: ", GoAfterPanic),
        rule(vec![StartState], r"http: panic serving", GoGoRoutine),
        rule(vec![GoAfterPanic], r"^$", GoGoRoutine),
        rule(
            vec![GoAfterPanic, GoAfterSignal, GoFrame1],
            r"^$",
            GoGoRoutine,
        ),
        rule(vec![GoAfterPanic], r"^\[signal ", GoAfterSignal),
        rule(vec![GoGoRoutine], r"^goroutine \d+ \[[^\]]+\]:$", GoFrame1),
        rule(
            vec![GoFrame1],
            r"^(?:[^\s.:]+\.)*[^\s.():]+\(|^created by ",
            GoFrame2,
        ),
        rule(vec![GoFrame2], r"^\s", GoFrame1),
    ]
}

fn ruby_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(vec![StartState], r"Error \(.*\):$", RubyBeforeRailsTrace),
        rule(vec![RubyBeforeRailsTrace], r"^  $", Ruby),
        rule(vec![RubyBeforeRailsTrace], r"^[\t ]+.*?\.rb:\d+:in `", Ruby),
        rule(vec![Ruby], r"^[\t ]+.*?\.rb:\d+:in `", Ruby),
    ]
}

fn dart_rules() -> Vec<Rule<'static>> {
    use ExceptionState::*;
    vec![
        rule(vec![StartState], r"^Unhandled exception:$", DartExc),
        rule(vec![DartExc], r"^Instance of", DartStack),
        rule(vec![DartExc], r"^Exception", DartStack),
        rule(vec![DartExc], r"^Bad state", DartStack),
        rule(vec![DartExc], r"^IntegerDivisionByZeroException", DartStack),
        rule(vec![DartExc], r"^Invalid argument", DartStack),
        rule(vec![DartExc], r"^RangeError", DartStack),
        rule(vec![DartExc], r"^Assertion failed", DartStack),
        rule(vec![DartExc], r"^Cannot instantiate", DartStack),
        rule(vec![DartExc], r"^Reading static variable", DartStack),
        rule(vec![DartExc], r"^UnimplementedError", DartStack),
        rule(vec![DartExc], r"^Unsupported operation", DartStack),
        rule(vec![DartExc], r"^Concurrent modification", DartStack),
        rule(vec![DartExc], r"^Out of Memory", DartStack),
        rule(vec![DartExc], r"^Stack Overflow", DartStack),
        rule(vec![DartExc], r"^'.+?':.+?$", DartTypeErr1),
        rule(vec![DartTypeErr1], r"^#\d+\s+.+?\(.+?\)$", DartStack),
        rule(vec![DartTypeErr1], r"^.+?$", DartTypeErr2),
        rule(vec![DartTypeErr2], r"^.*?\^.*?$", DartTypeErr3),
        rule(vec![DartTypeErr3], r"^$", DartTypeErr4),
        rule(vec![DartTypeErr4], r"^$", DartStack),
        rule(vec![DartExc], r"^FormatException", DartFormatErr1),
        rule(vec![DartFormatErr1], r"^#\d+\s+.+?\(.+?\)$", DartStack),
        rule(vec![DartFormatErr1], r"^.", DartFormatErr2),
        rule(vec![DartFormatErr2], r"^.*?\^", DartFormatErr3),
        rule(vec![DartFormatErr3], r"^$", DartStack),
        rule(vec![DartExc], r"^NoSuchMethodError:", DartMethodErr1),
        rule(vec![DartMethodErr1], r"^Receiver:", DartMethodErr2),
        rule(vec![DartMethodErr2], r"^Tried calling:", DartMethodErr3),
        rule(vec![DartMethodErr3], r"^Found:", DartStack),
        rule(vec![DartMethodErr3], r"^#\d+\s+.+?\(.+?\)$", DartStack),
        rule(vec![DartStack], r"^#\d+\s+.+?\(.+?\)$", DartStack),
        rule(vec![DartStack], r"^<asynchronous suspension>$", DartStack),
    ]
}

fn all_rules() -> Vec<Rule<'static>> {
    [
        java_rules().as_slice(),
        python_rules().as_slice(),
        php_rules().as_slice(),
        go_rules().as_slice(),
        ruby_rules().as_slice(),
        dart_rules().as_slice(),
    ]
    .concat()
}

use super::ProgrammingLanguages;

pub fn rules_by_lang() -> HashMap<ProgrammingLanguages, Vec<Rule<'static>>> {
    HashMap::from([
        (ProgrammingLanguages::Java, java_rules()),
        (ProgrammingLanguages::Javascript, java_rules()),
        (ProgrammingLanguages::Js, java_rules()),
        (ProgrammingLanguages::Csharp, java_rules()),
        (ProgrammingLanguages::Python, python_rules()),
        (ProgrammingLanguages::Py, python_rules()),
        (ProgrammingLanguages::Php, php_rules()),
        (ProgrammingLanguages::Go, go_rules()),
        (ProgrammingLanguages::Ruby, ruby_rules()),
        (ProgrammingLanguages::Rb, ruby_rules()),
        (ProgrammingLanguages::Dart, dart_rules()),
        (ProgrammingLanguages::All, all_rules()),
    ])
}
