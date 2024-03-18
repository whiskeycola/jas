pub mod array_iter;
pub mod deserialize;
pub mod find;
pub mod find_iter;
pub mod object_iter;
pub mod parent;
pub mod rfind;
pub mod strict_iter;
pub use crate::error::{Error, Result};
use crate::{util::pass_all, Atom};

pub trait DefaultEx<'a> {
    fn start(&self) -> Option<Atom<'a>>;
    fn end(&self) -> Option<Atom<'a>>;
    fn value(&self) -> Result<Atom<'a>>;
}

impl<'a> DefaultEx<'a> for Atom<'a> {
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

    fn value(&self) -> Result<Atom<'a>> {
        if self.current >= self.data.len() {
            return Err(Error::UnexpectedEOF);
        }
        let end = self.current + pass_all(&self.data[self.current..])?;
        let new_atom_data = &self.data[self.current..end];
        Ok(Atom::new(new_atom_data))
    }
}

impl<'a> DefaultEx<'a> for Result<Atom<'a>> {
    #[inline]
    fn start(&self) -> Option<Atom<'a>> {
        self.as_ref().ok()?.start()
    }

    #[inline]
    fn end(&self) -> Option<Atom<'a>> {
        self.as_ref().ok()?.end()
    }

    #[inline]
    fn value(&self) -> Result<Atom<'a>> {
        self.as_ref()?.value()
    }
}

impl<'a> DefaultEx<'a> for Option<Atom<'a>> {
    #[inline]
    fn start(&self) -> Option<Atom<'a>> {
        self.as_ref()?.start()
    }

    #[inline]
    fn end(&self) -> Option<Atom<'a>> {
        self.as_ref()?.end()
    }

    #[inline]
    fn value(&self) -> Result<Atom<'a>> {
        self.as_ref().ok_or(Error::None)?.value()
    }
}

#[test]
fn test_as_bytes() {
    let data = br#"{"a":"result a","b":"result b"}"#;
    let r = Atom::from(data)
        .value()
        // shift cursor to "result a"
        .map(|mut a| {
            a.current = 5;
            a
        })
        .value()
        .map(Aom::inner);
    let needle = br#""result a""#;
    assert_eq!(Ok(&needle[..]), r);
}
