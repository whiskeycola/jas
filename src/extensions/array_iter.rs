use crate::error::Result;
use crate::{extensions::DefaultEx as _, util::pass_space, Atom};

pub struct ArrayIter<'a> {
    atom: Option<Atom<'a>>,
    cursor: usize,
}
impl<'a> ArrayIter<'a> {
    pub fn new(atom: impl Into<Option<Atom<'a>>>) -> Self {
        let atom = atom.into();
        let cursor = atom.as_ref().map(|a| a.current + 1).unwrap_or(0);
        let atom = atom.clone();
        ArrayIter { atom, cursor }
    }
}
impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let atom = self.atom.as_ref()?;
        let i = &mut self.cursor;
        while *i < atom.data.len() {
            *i += pass_space(&atom.data[*i..]);
            if *i >= atom.data.len() {
                // error!("unexpected end data");
                break;
            }

            // if close tag
            if atom.data[*i] == b']' {
                break;
            }

            let obj = Atom::new(&atom.data[*i..]).as_bytes().ok()?;

            *i += obj.len();
            if *i >= atom.data.len() {
                // error!("unexpected end data");
                break;
            }

            *i += pass_space(&atom.data[*i..]);
            if *i < atom.data.len() && atom.data[*i] == b',' {
                *i += 1;
            }
            return Some(obj);
        }
        None
    }
}

pub trait ArrayIterEx<'a> {
    fn array_iter(&self) -> impl Iterator<Item = &'a [u8]>;
}
impl<'a> ArrayIterEx<'a> for Atom<'a> {
    fn array_iter(&self) -> impl Iterator<Item = &'a [u8]> {
        ArrayIter::new(self.clone())
    }
}
impl<'a> ArrayIterEx<'a> for Result<Atom<'a>> {
    fn array_iter(&self) -> impl Iterator<Item = &'a [u8]> {
        ArrayIter::new(self.clone().ok())
    }
}
impl<'a> ArrayIterEx<'a> for Option<Atom<'a>> {
    fn array_iter(&self) -> impl Iterator<Item = &'a [u8]> {
        ArrayIter::new(self.clone())
    }
}
#[test]
fn test_array_iter() {
    let data = br#"[
        "v1",
        "v2",
    ]"#;
    let arr = Atom::from(data)
        .array_iter()
        .map(|a| String::from_utf8_lossy(a).to_string())
        .collect::<Vec<_>>();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0], String::from("\"v1\""));
    assert_eq!(arr[1], String::from("\"v2\""));
}
