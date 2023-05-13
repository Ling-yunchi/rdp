use std::time;
use log::{Record, Level, Metadata};

pub(crate) struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_millis();
            println!("{} {} - {}", record.level(), time, record.args());
        }
    }

    fn flush(&self) {}
}