use crate::error::{Error, Result};
use crate::Atom;

use super::DefaultEx;
pub trait DeserializeEx<'a> {
    fn deserialize<Z: serde::Deserialize<'a>>(&self) -> Result<Z>;
}

impl<'a> DeserializeEx<'a> for Atom<'a> {
    fn deserialize<Z: serde::Deserialize<'a>>(&self) -> Result<Z> {
        let data = self.value()?;
        serde_json::from_slice(data.inner()).map_err(|e| Error::Serde(format!("{e}")))
    }
}

impl<'a> DeserializeEx<'a> for Result<Atom<'a>> {
    fn deserialize<Z: serde::Deserialize<'a>>(&self) -> Result<Z> {
        self.as_ref()?.deserialize()
    }
}

impl<'a> DeserializeEx<'a> for Option<Atom<'a>> {
    fn deserialize<Z: serde::Deserialize<'a>>(&self) -> Result<Z> {
        self.as_ref().ok_or(Error::None)?.deserialize()
    }
}
