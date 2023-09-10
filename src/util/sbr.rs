use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone)]
pub struct StringBuilder {
    s: String,
}

impl StringBuilder {
    pub fn new() -> Self {
        Self { s: String::new() }
    }

    pub fn new_with_str(s: &str) -> Self {
        Self { s: s.to_string() }
    }

    pub fn newline(&mut self) -> Self {
        self.s.push_str("\n");
        self.clone()
    }

    pub fn add_str(&mut self, s: &str) -> Self {
        self.s.push_str(s);
        self.clone()
    }

    pub fn add_strln(&mut self, s: &str) -> Self {
        self.s.push_str(s);
        self.newline()
    }

    pub fn add_string(&mut self, s: String) -> Self {
        self.add_str(s.as_str())
    }

    pub fn add_stringln(&mut self, s: String) -> Self {
        self.add_str(s.as_str()).newline()
    }

    pub fn get_str(&self) -> &str {
        self.s.as_str()
    }

    pub fn get_string(&self) -> String {
        self.s.clone()
    }
}

impl Add for StringBuilder {
    type Output = StringBuilder;

    fn add(self, rhs: Self) -> Self::Output {
        StringBuilder::new_with_str((self.s.clone() + rhs.s.as_str()).as_str())
    }
}

impl Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("StringBuilder[{}]", self.s.replace("\n", "\\n")))
    }
}
