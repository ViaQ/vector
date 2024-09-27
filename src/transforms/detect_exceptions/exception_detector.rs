use std::collections::HashMap;
use std::usize;
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::{
    event::LogEvent, event::Value,
    internal_events::detect_exceptions::DetectExceptionsStaleEventFlushed,
    transforms::detect_exceptions::*,
};
use vector_lib::lookup::path::OwnedTargetPath;

#[derive(Debug, Clone)]
pub struct RuleTarget {
    regex: Regex,
    to_state: ExceptionState,
}
type StateMachine = HashMap<ExceptionState, Vec<RuleTarget>>;

use rules::*;

pub fn get_state_machines(
    mut langs: Vec<ProgrammingLanguages>,
) -> HashMap<ExceptionState, Vec<RuleTarget>> {
    let mut rules: HashMap<ExceptionState, Vec<RuleTarget>> = HashMap::new();
    let rules_by_lang = rules_by_lang();
    if langs.is_empty() {
        langs = vec![ProgrammingLanguages::All];
    }
    for lang in langs {
        let rule_config = rules_by_lang.get(&lang).unwrap();
        for rc in rule_config {
            let t = RuleTarget {
                regex: Regex::new(rc.pattern).unwrap(),
                to_state: rc.to_state,
            };
            for s in &rc.from_states {
                let entry = rules.entry(*s).or_insert(vec![]);
                entry.append(&mut vec![t.clone()]);
            }
        }
    }
    rules
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DetectionStatus {
    NoTrace,
    StartTrace,
    InsideTrace,
    EndTrace,
}

pub struct TraceAccumulator {
    max_bytes: usize,
    max_lines: usize,
    message_key: OwnedTargetPath,
    multiline_flush_interval: Duration,
    first_event: LogEvent,
    buffer_size: usize,
    detector: ExceptionDetector,
    pub buffer_start_time: DateTime<Utc>,
    pub accumulated_messages: Vec<String>,
}

impl TraceAccumulator {
    pub fn new(
        languages: Vec<ProgrammingLanguages>,
        multiline_flush_interval: Duration,
        max_bytes: usize,
        max_lines: usize,
        message_key: OwnedTargetPath,
    ) -> TraceAccumulator {
        TraceAccumulator {
            buffer_size: 0,
            max_bytes,
            max_lines,
            message_key,
            multiline_flush_interval,
            first_event: LogEvent::default(),
            buffer_start_time: Utc::now(),
            accumulated_messages: vec![],
            detector: ExceptionDetector {
                state_machine: get_state_machines(languages),
                current_state: ExceptionState::StartState,
            },
        }
    }

    pub fn push(&mut self, le: &LogEvent, output: &mut Vec<Event>) {
        let mut detection_status = DetectionStatus::NoTrace;
        let message = le.get(&self.message_key);
        let message_copy = message.clone();

        match message {
            None => self.detector.reset(),
            Some(v) => {
                let s = v.to_string_lossy();
                if self.max_bytes > 0 && self.buffer_size + s.len() > self.max_bytes {
                    self.force_flush(output);
                }
                detection_status = self.detector.update(&s.to_string());
            }
        }

        self.update_buffer(detection_status, message_copy, le, output);

        if self.max_lines > 0 && self.accumulated_messages.len() == self.max_lines {
            self.force_flush(output);
        }
    }

    pub fn update_buffer(
        &mut self,
        detection_status: DetectionStatus,
        message: Option<&Value>,
        le: &LogEvent,
        output: &mut Vec<Event>,
    ) {
        let trigger_emit = match detection_status {
            DetectionStatus::NoTrace => true,
            DetectionStatus::EndTrace => true,
            _ => false,
        };
        if self.accumulated_messages.is_empty() && trigger_emit {
            output.push(vector_lib::event::Event::Log(le.to_owned()));
            return;
        }

        match detection_status {
            DetectionStatus::InsideTrace => self.add(le, message),
            DetectionStatus::EndTrace => {
                self.add(le, message);
                self.flush(output);
            }
            DetectionStatus::NoTrace => {
                self.flush(output);
                self.add(le, message);
                self.flush(output);
            }
            DetectionStatus::StartTrace => {
                self.flush(output);
                self.add(le, message);
            }
        }
    }

    pub fn add(&mut self, le: &LogEvent, message: Option<&Value>) {
        if self.accumulated_messages.is_empty() {
            self.first_event = le.to_owned();
            self.buffer_start_time = Utc::now();
        }
        if let Some(line) = message {
            let line = line.to_string_lossy();
            let line_len = line.len();
            self.accumulated_messages.push(line.to_string());
            self.buffer_size += line_len;
        }
    }

    pub fn flush(&mut self, output: &mut Vec<Event>) {
        match self.accumulated_messages.len() {
            0 => return,
            1 => {
                output.push(Event::Log(self.first_event.to_owned()));
            }
            _ => {
                self.first_event.insert(
                    &self.message_key,
                    self.accumulated_messages.join("\n"),
                );
                output.push(Event::Log(self.first_event.clone()));
            }
        }
        self.accumulated_messages = vec![];
        self.first_event = LogEvent::default();
        self.buffer_size = 0;
    }

    pub fn force_flush(&mut self, output: &mut Vec<Event>) {
        self.flush(output);
        self.detector.reset();
    }

    pub fn flush_stale_into(&mut self, now: DateTime<Utc>, output: &mut Vec<Event>) {
        if now.timestamp_millis() - self.buffer_start_time.timestamp_millis()
            > self
                .multiline_flush_interval
                .as_millis()
                .try_into()
                .unwrap()
        {
            emit!(DetectExceptionsStaleEventFlushed);
            self.force_flush(output);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionDetectorConfig {}

pub struct ExceptionDetector {
    pub state_machine: StateMachine,
    pub current_state: ExceptionState,
}

impl ExceptionDetector {
    pub fn update(&mut self, line: &String) -> DetectionStatus {
        let trace_seen_before = self.transition(line);
        if !trace_seen_before {
            self.transition(line);
        }
        let trace_seen_after = self.current_state != ExceptionState::StartState;
        match (trace_seen_before, trace_seen_after) {
            (true, true) => DetectionStatus::InsideTrace,
            (true, false) => DetectionStatus::EndTrace,
            (false, true) => DetectionStatus::StartTrace,
            (false, false) => DetectionStatus::NoTrace,
        }
    }

    pub fn transition(&mut self, message: &String) -> bool {
        let transitions = self.state_machine.get(&(self.current_state)).unwrap();
        for transition in transitions {
            if transition.regex.is_match(message.as_ref()) {
                self.current_state = transition.to_state.clone();
                return true;
            }
        }
        self.current_state = ExceptionState::StartState;
        false
    }

    pub fn reset(&mut self) {
        self.current_state = ExceptionState::StartState;
    }
}

#[cfg(test)]
mod exception_detector_tests {

    use super::*;
    use DetectionStatus::*;

    fn check_multiline(
        detector: &mut ExceptionDetector,
        expected_first: DetectionStatus,
        expected_last: DetectionStatus,
        multiline: Vec<&str>
    ) {
        let last_index = multiline.len() - 1;
        let mut index = 0;
        for line in multiline {
            let action = detector.update(&line.to_string());
            if index == 0 {
                assert_eq!(expected_first, action);
            } else if index == last_index {
                assert_eq!(expected_last, action);
            } else {
                assert_eq!(InsideTrace, action);
            }
            index += 1;
        }
    }

    fn check_exception(line: &str, detects_end: bool) {
        let lines = split(line);
        let mut detector = ExceptionDetector {
            state_machine: get_state_machines(default_programming_languages()),
            current_state: ExceptionState::StartState,
        };
        let after_exc = if detects_end { EndTrace } else { InsideTrace };
        let before_second_exc = if detects_end { InsideTrace } else { StartTrace };
        check_multiline(
            &mut detector,
            NoTrace,
            NoTrace,
            vec!["This is not an exception."],
        );
        check_multiline(&mut detector, InsideTrace, after_exc.clone(), lines.clone());
        check_multiline(
            &mut detector,
            NoTrace,
            NoTrace,
            vec!["This is not an exception."],
        );
        check_multiline(&mut detector, InsideTrace, after_exc.clone(), lines.clone());
        check_multiline(
            &mut detector,
            before_second_exc,
            after_exc.clone(),
            lines.clone(),
        );
    }

    #[test]
    fn test_java() {
        check_exception(&java_simple_exception(), false);
        check_exception(&java_complex_exception(), false);
        check_exception(&java_nested_exception(), false);
    }

    fn java_simple_exception() -> &'static str {
        "
Jul 09, 2015 3:23:29 PM com.google.devtools.search.cloud.feeder.MakeLog: RuntimeException: Run from this message!
    at com.my.app.Object.do$a1(MakeLog.java:50)
    at java.lang.Thing.call(Thing.java:10)
    at com.my.app.Object.help(MakeLog.java:40)
    at sun.javax.API.method(API.java:100)
    at com.jetty.Framework.main(MakeLog.java:30)
            "
    }

    fn java_complex_exception() -> &'static str {
        "
javax.servlet.ServletException: Something bad happened
    at com.example.myproject.OpenSessionInViewFilter.doFilter(OpenSessionInViewFilter.java:60)
    at org.mortbay.jetty.servlet.ServletHandler$CachedChain.doFilter(ServletHandler.java:1157)
    at com.example.myproject.ExceptionHandlerFilter.doFilter(ExceptionHandlerFilter.java:28)
    at org.mortbay.jetty.servlet.ServletHandler$CachedChain.doFilter(ServletHandler.java:1157)
    at com.example.myproject.OutputBufferFilter.doFilter(OutputBufferFilter.java:33)
    at org.mortbay.jetty.servlet.ServletHandler$CachedChain.doFilter(ServletHandler.java:1157)
    at org.mortbay.jetty.servlet.ServletHandler.handle(ServletHandler.java:388)
    at org.mortbay.jetty.security.SecurityHandler.handle(SecurityHandler.java:216)
    at org.mortbay.jetty.servlet.SessionHandler.handle(SessionHandler.java:182)
    at org.mortbay.jetty.handler.ContextHandler.handle(ContextHandler.java:765)
    at org.mortbay.jetty.webapp.WebAppContext.handle(WebAppContext.java:418)
    at org.mortbay.jetty.handler.HandlerWrapper.handle(HandlerWrapper.java:152)
    at org.mortbay.jetty.Server.handle(Server.java:326)
    at org.mortbay.jetty.HttpConnection.handleRequest(HttpConnection.java:542)
    at org.mortbay.jetty.HttpConnection$RequestHandler.content(HttpConnection.java:943)
    at org.mortbay.jetty.HttpParser.parseNext(HttpParser.java:756)
    at org.mortbay.jetty.HttpParser.parseAvailable(HttpParser.java:218)
    at org.mortbay.jetty.HttpConnection.handle(HttpConnection.java:404)
    at org.mortbay.jetty.bio.SocketConnector$Connection.run(SocketConnector.java:228)
    at org.mortbay.thread.QueuedThreadPool$PoolThread.run(QueuedThreadPool.java:582)
Caused by: com.example.myproject.MyProjectServletException
    at com.example.myproject.MyServlet.doPost(MyServlet.java:169)
    at javax.servlet.http.HttpServlet.service(HttpServlet.java:727)
    at javax.servlet.http.HttpServlet.service(HttpServlet.java:820)
    at org.mortbay.jetty.servlet.ServletHolder.handle(ServletHolder.java:511)
    at org.mortbay.jetty.servlet.ServletHandler$CachedChain.doFilter(ServletHandler.java:1166)
    at com.example.myproject.OpenSessionInViewFilter.doFilter(OpenSessionInViewFilter.java:30)
    ... 27 common frames omitted
            "
    }

    fn java_nested_exception() -> &'static str {
        "
java.lang.RuntimeException: javax.mail.SendFailedException: Invalid Addresses;
  nested exception is:
com.sun.mail.smtp.SMTPAddressFailedException: 550 5.7.1 <[REDACTED_EMAIL_ADDRESS]>... Relaying denied

    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.sendWithSmtp(AutomaticEmailFacade.java:236)
    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.sendSingleEmail(AutomaticEmailFacade.java:285)
    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.lambda$sendSingleEmail$3(AutomaticEmailFacade.java:254)
    at java.util.Optional.ifPresent(Optional.java:159)
    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.sendSingleEmail(AutomaticEmailFacade.java:253)
    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.sendSingleEmail(AutomaticEmailFacade.java:249)
    at com.nethunt.crm.api.email.EmailSender.lambda$notifyPerson$0(EmailSender.java:80)
    at com.nethunt.crm.api.util.ManagedExecutor.lambda$execute$0(ManagedExecutor.java:36)
    at com.nethunt.crm.api.util.RequestContextActivator.lambda$withRequestContext$0(RequestContextActivator.java:36)
    at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1149)
    at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:624)
    at java.base/java.lang.Thread.run(Thread.java:748)
Caused by: javax.mail.SendFailedException: Invalid Addresses;
  nested exception is:
com.sun.mail.smtp.SMTPAddressFailedException: 550 5.7.1 <[REDACTED_EMAIL_ADDRESS]>... Relaying denied

    at com.sun.mail.smtp.SMTPTransport.rcptTo(SMTPTransport.java:2064)
    at com.sun.mail.smtp.SMTPTransport.sendMessage(SMTPTransport.java:1286)
    at com.nethunt.crm.api.server.adminsync.AutomaticEmailFacade.sendWithSmtp(AutomaticEmailFacade.java:229)
    ... 12 more
Caused by: com.sun.mail.smtp.SMTPAddressFailedException: 550 5.7.1 <[REDACTED_EMAIL_ADDRESS]>... Relaying denied
            "
    }

    #[test]
    fn test_js() {
        check_exception(&node_js_exception(), false);
        check_exception(&client_js_exception(), false);
        check_exception(&v8_js_exception(), false);
    }

    fn node_js_exception() -> &'static str {
        "
ReferenceError: myArray is not defined
  at next (/app/node_modules/express/lib/router/index.js:256:14)
  at /app/node_modules/express/lib/router/index.js:615:15
  at next (/app/node_modules/express/lib/router/index.js:271:10)
  at Function.process_params (/app/node_modules/express/lib/router/index.js:330:12)
  at /app/node_modules/express/lib/router/index.js:277:22
  at Layer.handle [as handle_request] (/app/node_modules/express/lib/router/layer.js:95:5)
  at Route.dispatch (/app/node_modules/express/lib/router/route.js:112:3)
  at next (/app/node_modules/express/lib/router/route.js:131:13)
  at Layer.handle [as handle_request] (/app/node_modules/express/lib/router/layer.js:95:5)
  at /app/app.js:52:3
            "
    }

    fn client_js_exception() -> &'static str {
        "
Error
    at bls (<anonymous>:3:9)
    at <anonymous>:6:4
    at a_function_name        
    at Object.InjectedScript._evaluateOn (http://<anonymous>/file.js?foo=bar:875:140)
    at Object.InjectedScript.evaluate (<anonymous>)
            "
    }

    fn v8_js_exception() -> &'static str {
        "
V8 errors stack trace   
  eval at Foo.a (eval at Bar.z (myscript.js:10:3))
  at new Contructor.Name (native)
  at new FunctionName (unknown location)
  at Type.functionName [as methodName] (file(copy).js?query='yes':12:9)
  at functionName [as methodName] (native)
  at Type.main(sample(copy).js:6:4)
            "
    }

    #[test]
    fn test_golang() {
        check_exception(&golang_exception(), false);
        check_exception(&golang_on_gae_exception(), false);
        check_exception(&golang_signal_exception(), false);
        check_exception(&golang_http_exception(), false);
    }

    fn golang_exception() -> &'static str {
        "
panic: my panic

goroutine 4 [running]:
panic(0x45cb40, 0x47ad70)
	/usr/local/go/src/runtime/panic.go:542 +0x46c fp=0xc42003f7b8 sp=0xc42003f710 pc=0x422f7c
main.main.func1(0xc420024120)
	foo.go:6 +0x39 fp=0xc42003f7d8 sp=0xc42003f7b8 pc=0x451339
runtime.goexit()
	/usr/local/go/src/runtime/asm_amd64.s:2337 +0x1 fp=0xc42003f7e0 sp=0xc42003f7d8 pc=0x44b4d1
created by main.main
	foo.go:5 +0x58

goroutine 1 [chan receive]:
runtime.gopark(0x4739b8, 0xc420024178, 0x46fcd7, 0xc, 0xc420028e17, 0x3)
	/usr/local/go/src/runtime/proc.go:280 +0x12c fp=0xc420053e30 sp=0xc420053e00 pc=0x42503c
runtime.goparkunlock(0xc420024178, 0x46fcd7, 0xc, 0x1000f010040c217, 0x3)
	/usr/local/go/src/runtime/proc.go:286 +0x5e fp=0xc420053e70 sp=0xc420053e30 pc=0x42512e
runtime.chanrecv(0xc420024120, 0x0, 0xc420053f01, 0x4512d8)
	/usr/local/go/src/runtime/chan.go:506 +0x304 fp=0xc420053f20 sp=0xc420053e70 pc=0x4046b4
runtime.chanrecv1(0xc420024120, 0x0)
	/usr/local/go/src/runtime/chan.go:388 +0x2b fp=0xc420053f50 sp=0xc420053f20 pc=0x40439b
main.main()
	foo.go:9 +0x6f fp=0xc420053f80 sp=0xc420053f50 pc=0x4512ef
runtime.main()
	/usr/local/go/src/runtime/proc.go:185 +0x20d fp=0xc420053fe0 sp=0xc420053f80 pc=0x424bad
runtime.goexit()
	/usr/local/go/src/runtime/asm_amd64.s:2337 +0x1 fp=0xc420053fe8 sp=0xc420053fe0 pc=0x44b4d1

goroutine 2 [force gc (idle)]:
runtime.gopark(0x4739b8, 0x4ad720, 0x47001e, 0xf, 0x14, 0x1)
	/usr/local/go/src/runtime/proc.go:280 +0x12c fp=0xc42003e768 sp=0xc42003e738 pc=0x42503c
runtime.goparkunlock(0x4ad720, 0x47001e, 0xf, 0xc420000114, 0x1)
	/usr/local/go/src/runtime/proc.go:286 +0x5e fp=0xc42003e7a8 sp=0xc42003e768 pc=0x42512e
runtime.forcegchelper()
	/usr/local/go/src/runtime/proc.go:238 +0xcc fp=0xc42003e7e0 sp=0xc42003e7a8 pc=0x424e5c
runtime.goexit()
	/usr/local/go/src/runtime/asm_amd64.s:2337 +0x1 fp=0xc42003e7e8 sp=0xc42003e7e0 pc=0x44b4d1
created by runtime.init.4
	/usr/local/go/src/runtime/proc.go:227 +0x35

goroutine 3 [GC sweep wait]:
runtime.gopark(0x4739b8, 0x4ad7e0, 0x46fdd2, 0xd, 0x419914, 0x1)
	/usr/local/go/src/runtime/proc.go:280 +0x12c fp=0xc42003ef60 sp=0xc42003ef30 pc=0x42503c
runtime.goparkunlock(0x4ad7e0, 0x46fdd2, 0xd, 0x14, 0x1)
	/usr/local/go/src/runtime/proc.go:286 +0x5e fp=0xc42003efa0 sp=0xc42003ef60 pc=0x42512e
runtime.bgsweep(0xc42001e150)
	/usr/local/go/src/runtime/mgcsweep.go:52 +0xa3 fp=0xc42003efd8 sp=0xc42003efa0 pc=0x419973
runtime.goexit()
	/usr/local/go/src/runtime/asm_amd64.s:2337 +0x1 fp=0xc42003efe0 sp=0xc42003efd8 pc=0x44b4d1
created by runtime.gcenable
	/usr/local/go/src/runtime/mgc.go:216 +0x58
            "
    }

    fn golang_on_gae_exception() -> &'static str {
        "
panic: runtime error: index out of range

goroutine 12 [running]:
main88989.memoryAccessException()
	crash_example_go.go:58 +0x12a
main88989.handler(0x2afb7042a408, 0xc01042f880, 0xc0104d3450)
	crash_example_go.go:36 +0x7ec
net/http.HandlerFunc.ServeHTTP(0x13e5128, 0x2afb7042a408, 0xc01042f880, 0xc0104d3450)
	go/src/net/http/server.go:1265 +0x56
net/http.(*ServeMux).ServeHTTP(0xc01045cab0, 0x2afb7042a408, 0xc01042f880, 0xc0104d3450)
	go/src/net/http/server.go:1541 +0x1b4
appengine_internal.executeRequestSafely(0xc01042f880, 0xc0104d3450)
	go/src/appengine_internal/api_prod.go:288 +0xb7
appengine_internal.(*server).HandleRequest(0x15819b0, 0xc010401560, 0xc0104c8180, 0xc010431380, 0x0, 0x0)
	go/src/appengine_internal/api_prod.go:222 +0x102b
reflect.Value.call(0x1243fe0, 0x15819b0, 0x113, 0x12c8a20, 0x4, 0xc010485f78, 0x3, 0x3, 0x0, 0x0, ...)
	/tmp/appengine/go/src/reflect/value.go:419 +0x10fd
reflect.Value.Call(0x1243fe0, 0x15819b0, 0x113, 0xc010485f78, 0x3, 0x3, 0x0, 0x0, 0x0)
	/tmp/ap
            "
    }

    fn golang_signal_exception() -> &'static str {
        "
panic: runtime error: invalid memory address or nil pointer dereference
[signal SIGSEGV: segmentation violation code=0x1 addr=0x0 pc=0x7fd34f]

goroutine 5 [running]:
panics.nilPtrDereference()
	panics/panics.go:33 +0x1f
panics.Wait()
	panics/panics.go:16 +0x3b
created by main.main
	server.go:20 +0x91
            "
    }

    fn golang_http_exception() -> &'static str {
        "
2019/01/15 07:48:05 http: panic serving [::1]:54143: test panic
goroutine 24 [running]:
net/http.(*conn).serve.func1(0xc00007eaa0)
	/usr/local/go/src/net/http/server.go:1746 +0xd0
panic(0x12472a0, 0x12ece10)
	/usr/local/go/src/runtime/panic.go:513 +0x1b9
main.doPanic(0x12f0ea0, 0xc00010e1c0, 0xc000104400)
	/Users/ingvar/src/go/src/httppanic.go:8 +0x39
net/http.HandlerFunc.ServeHTTP(0x12be2e8, 0x12f0ea0, 0xc00010e1c0, 0xc000104400)
	/usr/local/go/src/net/http/server.go:1964 +0x44
net/http.(*ServeMux).ServeHTTP(0x14a17a0, 0x12f0ea0, 0xc00010e1c0, 0xc000104400)
	/usr/local/go/src/net/http/server.go:2361 +0x127
net/http.serverHandler.ServeHTTP(0xc000085040, 0x12f0ea0, 0xc00010e1c0, 0xc000104400)
	/usr/local/go/src/net/http/server.go:2741 +0xab
net/http.(*conn).serve(0xc00007eaa0, 0x12f10a0, 0xc00008a780)
	/usr/local/go/src/net/http/server.go:1847 +0x646
created by net/http.(*Server).Serve
	/usr/local/go/src/net/http/server.go:2851 +0x2f5
            "
    }

    #[test]
    fn test_ruby() {
        check_exception(&ruby_exception(), false);
        check_exception(&rails_exception(), false);
    }

    fn ruby_exception() -> &'static str {
        "
 NoMethodError (undefined method `resursivewordload' for #<BooksController:0x007f8dd9a0c738>):
  app/controllers/books_controller.rb:69:in `recursivewordload'
  app/controllers/books_controller.rb:75:in `loadword'
  app/controllers/books_controller.rb:79:in `loadline'
  app/controllers/books_controller.rb:83:in `loadparagraph'
  app/controllers/books_controller.rb:87:in `loadpage'
  app/controllers/books_controller.rb:91:in `onload'
  app/controllers/books_controller.rb:95:in `loadrecursive'
  app/controllers/books_controller.rb:99:in `requestload'
  app/controllers/books_controller.rb:118:in `generror'
  config/error_reporting_logger.rb:62:in `tagged'
            "
    }

    fn rails_exception() -> &'static str {
        r#"
 ActionController::RoutingError (No route matches [GET] "/settings"):
  
  actionpack (5.1.4) lib/action_dispatch/middleware/debug_exceptions.rb:63:in `call'
  actionpack (5.1.4) lib/action_dispatch/middleware/show_exceptions.rb:31:in `call'
  railties (5.1.4) lib/rails/rack/logger.rb:36:in `call_app'
  railties (5.1.4) lib/rails/rack/logger.rb:24:in `block in call'
  activesupport (5.1.4) lib/active_support/tagged_logging.rb:69:in `block in tagged'
  activesupport (5.1.4) lib/active_support/tagged_logging.rb:26:in `tagged'
  activesupport (5.1.4) lib/active_support/tagged_logging.rb:69:in `tagged'
  railties (5.1.4) lib/rails/rack/logger.rb:24:in `call'
  actionpack (5.1.4) lib/action_dispatch/middleware/remote_ip.rb:79:in `call'
  actionpack (5.1.4) lib/action_dispatch/middleware/request_id.rb:25:in `call'
  rack (2.0.3) lib/rack/method_override.rb:22:in `call'
  rack (2.0.3) lib/rack/runtime.rb:22:in `call'
  activesupport (5.1.4) lib/active_support/cache/strategy/local_cache_middleware.rb:27:in `call'
  actionpack (5.1.4) lib/action_dispatch/middleware/executor.rb:12:in `call'
  rack (2.0.3) lib/rack/sendfile.rb:111:in `call'
  railties (5.1.4) lib/rails/engine.rb:522:in `call'
  puma (3.10.0) lib/puma/configuration.rb:225:in `call'
  puma (3.10.0) lib/puma/server.rb:605:in `handle_request'
  puma (3.10.0) lib/puma/server.rb:437:in `process_client'
  puma (3.10.0) lib/puma/server.rb:301:in `block in run'
  puma (3.10.0) lib/puma/thread_pool.rb:120:in `block in spawn_thread'
            "#
    }

    #[test]
    fn test_python() {
        check_exception(&python_exception(), true);
    }

    fn python_exception() -> &'static str {
        r#"
Traceback (most recent call last):
  File "/base/data/home/runtimes/python27/python27_lib/versions/third_party/webapp2-2.5.2/webapp2.py", line 1535, in __call__
    rv = self.handle_exception(request, response, e)
  File "/base/data/home/apps/s~nearfieldspy/1.378705245900539993/nearfieldspy.py", line 17, in start
    return get()
  File "/base/data/home/apps/s~nearfieldspy/1.378705245900539993/nearfieldspy.py", line 5, in get
    raise Exception('spam', 'eggs')
Exception: ('spam', 'eggs')
            "#
    }

    fn split(line: &str) -> Vec<&str> {
        line.trim().split("\n").collect::<Vec<&str>>()
    }
}
