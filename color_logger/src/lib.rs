extern crate log;
extern crate ansi_term;
extern crate time;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};
use ansi_term::Colour::{Red, Yellow, Cyan, White};
use std::io::Write;

struct ColorLogger;

impl log::Log for ColorLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        let string = format!("{} [{}] {}.{}: {}",
            time::now().rfc3339(),
            record.level(),
            record.location().file(),
            record.location().line(),
            record.args());
        let colored_string = match record.level() {
            LogLevel::Error => Red.paint(string),
            LogLevel::Warn => Yellow.paint(string),
            LogLevel::Info => Cyan.paint(string),
            _ => White.paint(string)
        };
        writeln!(&mut std::io::stderr(), "{}", colored_string);
    }
}

pub fn init(level: LogLevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(level);
        Box::new(ColorLogger)
    })
}
