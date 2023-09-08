pub mod fancylog;
pub mod res;
pub mod util;

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};

    use crate::{
        fancylog::{self, LogConfig, LOGGER},
        res::RFile,
        util::sbr::StringBuilder,
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
        let i = "-> go on a test";
        info!("info {}", i);
        warn!("warn {}", i);
        error!("error {}", i);
        debug!("debug {}", i);
        trace!("trace {}", i)
    }

    #[test]
    fn test_path() {
        RFile::new("test.ignore")
            .writeln_str("data1")
            .write_str("data2")
            .appendln_str("data3")
            .append_str("data4");
    }

    #[test]
    fn test_sbr() {
        let sbr = StringBuilder::new()
            .add_strln("string 1")
            .add_str("string 2");

        info!("{}", sbr)
    }
}
