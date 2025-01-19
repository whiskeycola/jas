use super::{find::FindEx as _, rfind::RFindEx as _, DefaultEx as _};
use crate::error::Result;
use crate::needle::Needle;
use crate::Atom;

pub struct FindIter<'a, 'n> {
    atom_next: Option<Atom<'a>>,
    atom_back: Option<Atom<'a>>,
    needle: Needle<'n>,
}
impl<'a, 'n> FindIter<'a, 'n> {
    pub fn new(atom: impl Into<Option<Atom<'a>>>, needle: impl Into<Needle<'n>>) -> Self {
        let atom_next = atom.into();
        let atom_back = atom_next.end();
        Self {
            atom_next,
            atom_back,
            needle: needle.into(),
        }
    }
    pub fn none(&mut self) {
        self.atom_next = None;
        self.atom_back = None;
    }
}

impl<'a, 'n> Iterator for FindIter<'a, 'n> {
    type Item = Atom<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let back_pointer = self.atom_back.as_ref().map(|a| a.pointer)?;

        let atom = self.atom_next.find(self.needle)?;
        if atom.pointer >= back_pointer {
            self.none();
            return None; // end
        }
        self.atom_next = Some(atom.clone());
        Some(atom)
    }
}

impl<'a, 'n> DoubleEndedIterator for FindIter<'a, 'n> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next_pointer = self.atom_next.as_ref().map(|a| a.pointer)?;
        let atom = self.atom_back.rfind(self.needle)?;
        if atom.pointer <= next_pointer {
            self.none();
            return None;
        }
        self.atom_back = Some(atom.clone());
        Some(atom)
    }
}

pub trait FindIterEx<'a> {
    fn find_iter<'n>(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = Atom<'a>>;
}

impl<'a> FindIterEx<'a> for Atom<'a> {
    fn find_iter<'n>(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone(), needle)
    }
}

impl<'a> FindIterEx<'a> for Result<Atom<'a>> {
    fn find_iter<'n>(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone().ok(), needle)
    }
}

impl<'a> FindIterEx<'a> for Option<Atom<'a>> {
    fn find_iter<'n>(&self, needle: impl Into<Needle<'n>>) -> impl Iterator<Item = Atom<'a>> {
        FindIter::new(self.clone(), needle)
    }
}
