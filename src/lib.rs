pub mod conf;
pub mod fancylog;
pub mod res;
pub mod util;

#[cfg(test)]
mod tests {

    use log::{debug, error, info, trace, warn};
    use serde_derive::{Deserialize, Serialize};

    use crate::{
        conf::{Config, JsonConfig, YamlConfig},
        fancylog::{self, LogConfig, LOGGER},
        res::RFile,
        util::sbr::StringBuilder,
    };

    #[test]
    fn test_entry() {
        fancylog::init_with_conf(LogConfig::default().append_output("latest.log")).unwrap();
        LOGGER.config.get().unwrap().clear_logfile();
        LOGGER.set_level(log::LevelFilter::Trace);
        let i = "test field";
        info!("info -> {}", i);
        warn!("warn -> {}", i);
        error!("error ->  {}", i);
        debug!("debug -> {}", i);
        trace!("trace -> {}", i);
    }

    #[test]
    fn test_res() {
        let f = RFile::new("test.ignore")
            .create_or_clear()
            .writeln_str("data1")
            .newline()
            .write_str("data2")
            .appendln_str("data3")
            .append_str("data4");
        info!("test res -> {}", f.read_str().unwrap());
    }

    #[test]
    fn test_sbr() {
        let sbr = StringBuilder::new()
            .add_strln("string 1")
            .add_str("string 2");

        info!("test sbr -> {}", sbr)
    }

    #[test]
    fn test_conf() {
        #[derive(Clone, Deserialize, Serialize)]
        struct Test {
            user: i32,
            pwd: i32,
        }

        let te = Test {
            user: 100,
            pwd: 100,
        };

        let jcfg = JsonConfig::from_path(te.clone(), "conf.json.ignore");
        jcfg.write_to();

        let jcfg = JsonConfig::from_path(te.clone(), "conf.json.ignore");
        let s = jcfg.read_from();
        info!("{} {}", s.user, s.pwd);

        let ycfg = YamlConfig::from_path(te.clone(), "conf.yaml.ignore");
        ycfg.write_to();

        let ycfg = YamlConfig::from_path(te.clone(), "conf.yaml.ignore");
        let s = ycfg.read_from();
        info!("{} {}", s.user, s.pwd);
    }
}
