use crate::Atom;

use super::{find::FindEx, DefaultEx};
use crate::error::Result;

pub struct StrictIter<'a> {
    atom: Option<Atom<'a>>,
    value_type: crate::ValueType,
    needle: String,
}

impl<'a> StrictIter<'a> {
    pub fn new(
        atom: impl Into<Option<Atom<'a>>>,
        needle: impl Into<String>,
        value_type: crate::ValueType,
    ) -> Self {
        let atom = atom.into();
        let needle = needle.into();
        Self {
            atom,
            value_type,
            needle,
        }
    }
}

impl<'a> Iterator for StrictIter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        let mut atom = self.atom.find(&self.needle, self.value_type)?;
        let data = atom.as_bytes().ok()?;

        // shift current value
        atom.pointer = atom.current + data.len();
        self.atom = Some(atom);
        Some(data)
    }
}
pub trait StrictIterEx<'a> {
    fn strict_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = &'a [u8]>;
}
impl<'a> StrictIterEx<'a> for Atom<'a> {
    fn strict_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone(), needle.as_ref(), value_type)
    }
}
impl<'a> StrictIterEx<'a> for Result<Atom<'a>> {
    fn strict_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone().ok(), needle.as_ref(), value_type)
    }
}
impl<'a> StrictIterEx<'a> for Option<Atom<'a>> {
    fn strict_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone(), needle.as_ref(), value_type)
    }
}
