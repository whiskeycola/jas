pub mod children_iter;
pub mod deserialize;
pub mod find;
pub mod find_iter;
pub mod parent;
pub mod rfind;

pub use crate::error::{Error, Result};
use crate::{util::pass_all, Atom};

pub trait DefaultEx<'a> {
    fn as_bytes(&self) -> Result<&'a [u8]>;
    fn start(&self) -> Option<Atom<'a>>;
    fn end(&self) -> Option<Atom<'a>>;
    fn enter(&self) -> Result<Atom<'a>>;
}

impl<'a> DefaultEx<'a> for Atom<'a> {
    fn as_bytes(&self) -> Result<&'a [u8]> {
        let end = pass_all(&self.data[self.current..])?;
        Ok(&self.data[self.current..end])
    }

    #[inline]
    fn start(&self) -> Option<Atom<'a>> {
        Some(Atom {
            data: &self.data,
            pointer: 0,
            current: self.current,
        })
    }

    #[inline]
    fn end(&self) -> Option<Atom<'a>> {
        Some(Atom {
            data: &self.data,
            pointer: self.data.len(),
            current: self.current,
        })
    }

    fn enter(&self) -> Result<Atom<'a>> {
        self.as_bytes().map(Atom::new)
    }
}

impl<'a> DefaultEx<'a> for Result<Atom<'a>> {
    #[inline]
    fn as_bytes(&self) -> Result<&'a [u8]> {
        self.as_ref()?.as_bytes()
    }

    #[inline]
    fn start(&self) -> Option<Atom<'a>> {
        self.as_ref().ok()?.start()
    }

    #[inline]
    fn end(&self) -> Option<Atom<'a>> {
        self.as_ref().ok()?.end()
    }

    #[inline]
    fn enter(&self) -> Result<Atom<'a>> {
        self.as_ref()?.enter()
    }
}

impl<'a> DefaultEx<'a> for Option<Atom<'a>> {
    #[inline]
    fn as_bytes(&self) -> Result<&'a [u8]> {
        self.as_ref().ok_or(Error::None)?.as_bytes()
    }

    #[inline]
    fn start(&self) -> Option<Atom<'a>> {
        self.as_ref()?.start()
    }

    #[inline]
    fn end(&self) -> Option<Atom<'a>> {
        self.as_ref()?.end()
    }

    #[inline]
    fn enter(&self) -> Result<Atom<'a>> {
        self.as_ref().ok_or(Error::None)?.enter()
    }
}
