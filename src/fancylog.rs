use std::ops::Deref;

use super::res::RFile;
use chrono::Local;
use colored::{Color, Colorize};
use log::{set_logger, Level, LevelFilter, Log, SetLoggerError};
use once_cell::sync::{Lazy, OnceCell};

pub struct Logger {
    pub config: OnceCell<LogConfig>,
}

#[derive(Clone, Copy, Debug)]
pub enum LogBlock {
    TIME,
    LEVEL,
    TARGET,
    STR(&'static str),
    MSG,
}

impl LogBlock {
    pub fn default() -> Vec<Self> {
        vec![
            LogBlock::TIME,
            LogBlock::LEVEL,
            LogBlock::TARGET,
            LogBlock::STR(" "),
            LogBlock::MSG,
        ]
    }

    pub fn str_block(s: &'static str) -> LogBlock {
        LogBlock::STR(s)
    }
}

impl Logger {
    pub fn default() -> Self {
        Self {
            config: OnceCell::new(),
        }
    }

    pub fn set_level(&self, level: LevelFilter) {
        log::set_max_level(level)
    }

    pub fn get_config(&self) -> &LogConfig {
        self.config.get_or_init(|| LogConfig::default())
    }

    pub fn get_config_mut(&mut self) -> &mut LogConfig {
        self.config.get_mut().unwrap()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        let color = (self.get_config().fg_color)(record.level());
        let mut msg = String::default();
        self.get_config().log_format.iter().for_each(|b| match b {
            LogBlock::TIME => msg.push_str(
                format!("[{}]", Local::now().format(&self.get_config().time_format)).as_str(),
            ),
            LogBlock::LEVEL => msg.push_str(format!("[{}]", record.level()).as_str()),
            LogBlock::TARGET => msg.push_str(format!("[{}]", record.target(),).as_str()),
            LogBlock::MSG => msg.push_str(record.args().to_string().as_str()),
            LogBlock::STR(v) => msg.push_str(v),
        });
        let s = msg.color(color);
        println!("{}", s);
        self.get_config().outputfile.iter().for_each(|pth| {
            RFile::new(pth).appendln_str(msg.as_str());
        });
    }

    fn flush(&self) {}
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub log_format: Vec<LogBlock>,
    pub time_format: &'static str,
    pub fg_color: fn(Level) -> Color,
    outputfile: Vec<&'static str>,
}

impl LogConfig {
    pub fn default() -> Self {
        Self {
            log_format: LogBlock::default(),
            time_format: "%Y-%m-%d %T",
            fg_color: |level| match level {
                Level::Info => Color::BrightGreen,
                Level::Warn => Color::BrightYellow,
                Level::Error => Color::BrightRed,
                Level::Debug => Color::BrightCyan,
                Level::Trace => Color::BrightMagenta,
            },
            outputfile: vec![],
        }
    }

    pub fn set_timeformat(&mut self, time_fomat: &'static str) -> &Self {
        self.time_format = time_fomat;
        self
    }

    pub fn set_fg_color(mut self, fg_color: fn(Level) -> Color) -> Self {
        self.fg_color = fg_color;
        self
    }

    pub fn set_log_format(mut self, log_format: Vec<LogBlock>) -> Self {
        self.log_format = log_format;
        self
    }

    pub fn append_output(&mut self, path: &'static str) {
        self.outputfile.push(path);
    }

    pub fn clear_logfile(&self) -> &Self {
        self.outputfile.iter().for_each(|pth| {
            RFile::new(pth).create_or_clear();
        });
        self
    }
}

pub static LOGGER: Lazy<Logger> = Lazy::new(Logger::default);
pub static mut UNSAFE_LOGGER: Lazy<Logger> = Lazy::new(Logger::default);

/// if you want to change the config, pls before call the log method
/// or you should use the UNSAFE_LOGGER for dynamic config
///
/// ```
/// use sulhyd::fancylog;
/// use sulhyd::fancylog::{LOGGER,LogConfig};
/// use log::info;
/// fn main(){
///     fancylog::init().unwrap(); //can use init_with_conf too
///     LOGGER.set_level(log::LevelFilter::Trace);
///     LOGGER
///         .config
///         .set(LogConfig::default().append_output("latest.log").clone())
///         .unwrap();
///     info!("hello world")
/// }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Info))
}

pub fn init_with_conf(conf: LogConfig) -> Result<(), SetLoggerError> {
    LOGGER.config.set(conf).unwrap();
    set_logger(LOGGER.deref()).map(|()| log::set_max_level(LevelFilter::Info))
}

/// prefer using the safe logger
pub fn init_unsafe() -> Result<(), SetLoggerError> {
    set_logger(unsafe { UNSAFE_LOGGER.deref() }).map(|()| log::set_max_level(LevelFilter::Info))
}

pub fn init_with_conf_unsafe(conf: LogConfig) -> Result<(), SetLoggerError> {
    unsafe { UNSAFE_LOGGER.deref() }.config.set(conf).unwrap();
    set_logger(unsafe { UNSAFE_LOGGER.deref() }).map(|()| log::set_max_level(LevelFilter::Info))
}
