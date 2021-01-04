use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct General {
    blog_name: String,
    address: String,
    database_url: String,
    port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secret {
    token_secret: String,
}

impl Secret {
    pub fn token_secret(&self) -> String {
        self.token_secret.clone()
    }
}

impl General {
    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database_url(&self) -> String {
        self.database_url.clone()
    }

    pub fn blog_name(&self) -> String {
        self.blog_name.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    general: General,
    secret: Secret,
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

    pub fn secret(&self) -> &Secret {
        &self.secret
    }
}
