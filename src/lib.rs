pub mod fancylog;
pub mod res;

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};

    use crate::{
        fancylog::{self, LogConfig, LOGGER},
        res::RFile,
    };

    #[test]
    fn test_logger() {
        fancylog::init().unwrap();
        let mut cf = LogConfig::default();
        cf.append_output("latest.log");
        LOGGER.config.set(cf).unwrap();
        LOGGER.set_level(log::LevelFilter::Trace);
        let i = "go on a test";
        info!("info {}", i);
        warn!("warn {}", i);
        error!("error {}", i);
        debug!("debug {}", i);
        trace!("trace {}", i)
    }

    #[test]
    fn test_path() {
        RFile::new("test.txt").write_str("datsssxxxxxxxa");
    }
}
