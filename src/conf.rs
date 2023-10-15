use serde::{Deserialize, Serialize};

use crate::res::RFile;

pub trait Config {
    type Stur;
    fn write_to(&self);
    fn read_from(&self) -> Self::Stur;
    fn from_path(content: Self::Stur, path: &str) -> Self;
}

#[derive(Clone)]
pub struct JsonConfig<S>
where
    for<'a> S: Clone + Serialize + Deserialize<'a>,
{
    pub content: S,
    pub path: String,
}

impl<S> Config for JsonConfig<S>
where
    for<'a> S: Clone + Serialize + Deserialize<'a>,
{
    type Stur = S;

    fn write_to(&self) {
        RFile::new(&self.path).write_str(
            serde_json::to_string(&self.content.clone())
                .unwrap()
                .as_str(),
        );
    }

    fn read_from(&self) -> Self::Stur {
        serde_json::from_str::<S>(RFile::new(&self.path).read_str().unwrap().as_str()).unwrap()
    }

    fn from_path(content: S, path: &str) -> Self {
        Self {
            content: content.clone(),
            path: path.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct YamlConfig<S>
where
    for<'a> S: Clone + Serialize + Deserialize<'a>,
{
    pub content: S,
    pub path: String,
}

impl<S> Config for YamlConfig<S>
where
    for<'a> S: Clone + Serialize + Deserialize<'a>,
{
    type Stur = S;

    fn write_to(&self) {
        RFile::new(&self.path).write_str(
            serde_yaml::to_string(&self.content.clone())
                .unwrap()
                .as_str(),
        );
    }

    fn read_from(&self) -> Self::Stur {
        serde_yaml::from_str::<S>(RFile::new(&self.path).read_str().unwrap().as_str()).unwrap()
    }

    fn from_path(content: S, path: &str) -> Self {
        Self {
            content: content.clone(),
            path: path.to_string(),
        }
    }
}
