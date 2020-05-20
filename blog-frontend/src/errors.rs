use std::fmt::{Error, Formatter};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    pub err: JsValue,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl std::error::Error for FetchError {}
