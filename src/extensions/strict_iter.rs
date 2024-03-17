use crate::{needle::Needle, Atom};

use super::{find::FindEx, DefaultEx};
use crate::error::Result;

pub struct StrictIter<'a, 'n> {
    atom: Option<Atom<'a>>,
    needle: Needle<'n>,
}

impl<'a, 'n> StrictIter<'a, 'n> {
    pub fn new(atom: impl Into<Option<Atom<'a>>>, needle: impl Into<Needle<'n>>) -> Self {
        let atom = atom.into();
        let needle = needle.into();
        Self { atom, needle }
    }
}

impl<'a, 'n> Iterator for StrictIter<'a, 'n> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        let mut atom = self.atom.find(self.needle)?;
        let data = atom.as_bytes().ok()?;

        // shift current value
        atom.pointer = atom.current + data.len();
        self.atom = Some(atom);
        Some(data)
    }
}
pub trait StrictIterEx<'a, 'n> {
    fn strict_iter(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = &'a [u8]>;
}
impl<'a, 'n> StrictIterEx<'a, 'n> for Atom<'a> {
    fn strict_iter(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone(), needle)
    }
}
impl<'a, 'n> StrictIterEx<'a, 'n> for Result<Atom<'a>> {
    fn strict_iter(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone().ok(), needle)
    }
}

impl<'a, 'n> StrictIterEx<'a, 'n> for Option<Atom<'a>> {
    fn strict_iter(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = &'a [u8]> {
        StrictIter::new(self.clone(), needle)
    }
}
