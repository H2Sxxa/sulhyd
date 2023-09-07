use std::{
    fs::File,
    fs::{read_to_string, OpenOptions},
    io::Write,
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

    pub fn write_str(&mut self, data: &str) -> &Self {
        write!(self.filea, "{}", data).unwrap();
        self
    }

    pub fn append_str(&mut self, data: &str) -> &Self {
        write!(self.filea, "{}{}", self.read_str().unwrap(), data).unwrap();
        self
    }

    pub fn read_str(&self) -> Result<String, std::io::Error> {
        read_to_string(self.path.clone())
    }

    pub fn newline(&mut self) -> &Self {
        self.append_str("\n");
        self
    }

    pub fn append_strnl(&mut self, data: &str) -> &Self {
        self.append_str(format!("{}\n", data).as_str());
        self
    }

    pub fn append_nlstr(&mut self, data: &str) -> &Self {
        self.append_str(format!("\n{}", data).as_str());
        self
    }

    pub fn create_or_clear(&self) -> &Self {
        File::create(self.path.clone()).unwrap();
        self
    }
}
