use syslog_ng_sys::LogMessage;
use syslog_ng_sys::LogParser;

pub use proxies::parser::{
    OptionError,
    RustParser,
    RustParserBuilder
};

#[repr(C)]
#[derive(Clone)]
pub struct RustParserProxy<P> where P: RustParser {
    pub parser: Option<P>,
    pub builder: Option<P::Builder>
}

impl<P> RustParserProxy<P> where P: RustParser {
    pub fn new() -> RustParserProxy<P> {
        RustParserProxy {
            parser: None,
            builder: Some(P::Builder::new())
        }
    }

    pub fn init(&mut self) -> bool {
        let builder = self.builder.take().expect("Called init when builder was not set");
        match builder.build() {
            Ok(parser) => {
                self.parser = Some(parser);
                self.parser.as_mut().expect("Called init on a non-existing Rust Parser").init()
            },
            Err(error) => {
                error!("Error: {:?}", error);
                false
            }
        }
    }

    pub fn set_option(&mut self, name: String, value: String) {
        if self.builder.is_none() {
            self.builder = Some(P::Builder::new());
        }

        let builder = self.builder.as_mut().expect("Failed to get builder on a RustParserProxy");
        builder.option(name, value);
    }

    pub fn process(&mut self, msg: &mut LogMessage, input: &str) -> bool {
        self.parser.as_mut().expect("Called process on a non-existing Rust parser").process(msg, input)
    }

    pub fn parent(&mut self, parent: *mut LogParser) {
        let builder = self.builder.as_mut().expect("Failed to get a builder on a new parser proxy instance");
        builder.parent(parent);
    }
}
