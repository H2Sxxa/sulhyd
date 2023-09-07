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
        LOGGER
            .config
            .set(LogConfig::default().append_output("latest.log").clone())
            .unwrap();
        LOGGER.config.get().unwrap().clear_logfile();
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
