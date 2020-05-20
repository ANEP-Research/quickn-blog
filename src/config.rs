use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct General {
    blog_name: String,
    address: String,
    port: u16,
}

impl General {
    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn blog_name(&self) -> String {
        self.blog_name.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    general: General,
}

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

impl Config {
    pub fn open(path: &str) -> io::Result<Self> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf).unwrap())
    }

    pub fn general(&self) -> &General {
        &self.general
    }
}
