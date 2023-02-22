//! Main log4rs configuration functions.

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use serde::{Deserialize, Serialize};

const PATTERN_ENCODER: &str = "{d(%Y-%m-%d %H:%M:%S)} {l} [{M}:{L}] {m}{n}";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum LogsLevel {
    #[default]
    Info,
    Debug,
    Error,
}

impl std::fmt::Display for LogsLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Error => write!(f, "error"),
        }
    }
}

fn create_log(name: &str) -> RollingFileAppender {
    let mut zipped_path = String::from("logs/");
    zipped_path.push_str(name);
    zipped_path.push_str(".{}.log.gz");

    let size_trigger = SizeTrigger::new(250 * 1024 * 1024); // 250M file
    let window_roller = FixedWindowRoller::builder()
        .build(&zipped_path, 50)
        .unwrap();

    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));

    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN_ENCODER)))
        .build(format!("logs/{}.log", name), Box::new(policy))
        .unwrap()
}

fn create_config(logs_level: &LogsLevel) -> Config {
    let log4rs_level = match logs_level {
        LogsLevel::Error => LevelFilter::Error,
        LogsLevel::Info => LevelFilter::Info,
        LogsLevel::Debug => LevelFilter::Debug,
    };

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN_ENCODER)))
        .target(Target::Stdout)
        .build();

    Config::builder()
        .appender(Appender::builder().build("mainlog", Box::new(create_log("main"))))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log4rs_level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("mainlog")
                .appender("stdout")
                .build(log4rs_level),
        )
        .unwrap()
}

pub struct Handle(log4rs::Handle);

pub fn build() -> Handle {
    let config = create_config(&LogsLevel::Info);
    let handle = log4rs::init_config(config).unwrap();
    Handle(handle)
}

impl Handle {
    pub fn set_logger(&mut self, src: &LogsLevel) {
        let config = create_config(src);
        self.0.set_config(config);
    }
}
