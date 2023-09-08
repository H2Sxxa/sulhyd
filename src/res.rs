use std::{
    fs::File,
    fs::{read_to_string, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

pub struct RFile {
    filea: File,
    path: String,
}

impl RFile {
    pub fn new(path: &str) -> Self {
        let filea = OpenOptions::new()
            .write(true)
            .open(path)
            .unwrap_or_else(|_| File::create(path).unwrap());
        Self {
            filea: filea,
            path: path.to_string(),
        }
    }

    pub fn write_str(&mut self, data: &str) -> Self {
        write!(self.filea, "{}", data).unwrap();
        self.clone()
    }

    pub fn writeln_str(&mut self, data: &str) -> Self {
        writeln!(self.filea, "{}", data).unwrap();
        self.clone()
    }

    pub fn write_buf(mut self, buf: &[u8]) -> Self {
        self.filea.write_all(buf).unwrap();
        self
    }

    pub fn append_str(&mut self, data: &str) -> Self {
        write!(self.filea, "{}{}", self.read_str().unwrap(), data).unwrap();
        self.clone()
    }

    pub fn appendln_str(&mut self, data: &str) -> Self {
        writeln!(self.filea, "{}{}", self.read_str().unwrap(), data).unwrap();
        self.clone()
    }

    pub fn read_str(&self) -> Result<String, std::io::Error> {
        read_to_string(self.path.clone())
    }

    pub fn read_buf(&mut self, buf: &mut Vec<u8>) -> Result<usize, std::io::Error> {
        self.filea.read_to_end(buf)
    }

    pub fn newline(&mut self) -> Self {
        self.writeln_str("");
        self.clone()
    }

    pub fn create_or_clear(&self) -> &Self {
        File::create(self.path.clone()).unwrap();
        self
    }

    pub fn get_pathbuf(&self) -> PathBuf {
        PathBuf::from(self.path.as_str())
    }
}

impl Clone for RFile {
    fn clone(&self) -> Self {
        Self::new(self.path.as_str())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
