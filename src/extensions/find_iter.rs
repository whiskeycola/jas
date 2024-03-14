use super::{find::FindEx as _, rfind::RFindEx as _, DefaultEx as _};
use crate::error::Result;
use crate::Atom;

#[derive(Default)]
pub struct FindIter<'a> {
    atom_next: Option<Atom<'a>>,
    atom_back: Option<Atom<'a>>,
    value_type: crate::ValueType,
    needle: String,
}
impl<'a> FindIter<'a> {
    pub fn new(
        atom: impl Into<Option<Atom<'a>>>,
        needle: impl Into<String>,
        value_type: crate::ValueType,
    ) -> Self {
        let atom_next = atom.into();
        let atom_back = atom_next.end();
        Self {
            atom_next,
            atom_back,
            value_type,
            needle: needle.into(),
        }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Atom<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let back_pointer = self.atom_back.as_ref().map(|a| a.pointer)?;

        let atom = self.atom_next.find(&self.needle, self.value_type)?;
        if atom.pointer >= back_pointer {
            *self = Self::default();
            return None; // end
        }
        self.atom_next = Some(atom.clone());
        Some(atom)
    }
}

impl<'a> DoubleEndedIterator for FindIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next_pointer = self.atom_next.as_ref().map(|a| a.pointer)?;
        let atom = self.atom_back.rfind(&self.needle, self.value_type)?;
        if atom.pointer <= next_pointer {
            *self = Self::default();
            return None;
        }
        self.atom_back = Some(atom.clone());
        Some(atom)
    }
}

pub trait FindIterEx<'a> {
    fn find_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = Atom<'a>>;
}
impl<'a> FindIterEx<'a> for Atom<'a> {
    fn find_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone(), needle.as_ref(), value_type)
    }
}
impl<'a> FindIterEx<'a> for Result<Atom<'a>> {
    fn find_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone().ok(), needle.as_ref(), value_type)
    }
}
impl<'a> FindIterEx<'a> for Option<Atom<'a>> {
    fn find_iter(
        &self,
        needle: impl AsRef<str>,
        value_type: crate::ValueType,
    ) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone(), needle.as_ref(), value_type)
    }
}
