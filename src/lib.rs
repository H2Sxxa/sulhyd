pub mod fancylog;

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};

    use crate::fancylog::{self, LOGGER};

    #[test]
    fn test_logger() {
        fancylog::init().unwrap();
        LOGGER.set_level(log::LevelFilter::Trace);
        let i = "go on a test";
        info!("info {}", i);
        warn!("warn {}", i);
        error!("error {}", i);
        debug!("debug {}", i);
        trace!("trace {}", i)
    }
}
