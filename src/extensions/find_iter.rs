use crate::Atom;

use super::{find::FindEx as _, rfind::RFindEx as _, DefaultEx as _};

#[derive(Default)]
pub struct FindIter<'a> {
    atom_next: Option<Atom<'a>>,
    atom_back: Option<Atom<'a>>,
    value_type: crate::Type,
    needle: String,
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
        needle: impl Into<String>,
        value_type: crate::Type,
    ) -> impl Iterator<Item = Atom<'a>>;
}
impl<'a> FindIterEx<'a> for Atom<'a> {
    fn find_iter(
        &self,
        needle: impl Into<String>,
        value_type: crate::Type,
    ) -> impl Iterator<Item = Atom<'a>> {
        FindIter {
            atom_back: self.end(),
            atom_next: Some(self.clone()),
            value_type,
            needle: needle.into(),
        }
    }
}
