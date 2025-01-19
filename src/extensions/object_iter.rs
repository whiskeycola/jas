use std::borrow::Cow;

use crate::{
    error::Result,
    util::{is_type, pass_space, pass_string, pass_to_item},
    Atom, STRING,
};

use super::DefaultEx as _;
type IterItem<'a> = (Cow<'a, str>, Atom<'a>);
pub struct ObjectIter<'a> {
    atom: Option<Atom<'a>>,
    cursor: usize,
}
impl<'a> ObjectIter<'a> {
    pub fn new(atom: impl Into<Option<Atom<'a>>>) -> Self {
        let atom = atom.into();
        let cursor = atom.as_ref().map(|c| c.current + 1).unwrap_or(0);
        let children_iter = ObjectIter { atom, cursor };
        children_iter
    }
}
impl<'a> Iterator for ObjectIter<'a> {
    type Item = IterItem<'a>;

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
            if atom.data[*i] == b'}' {
                break;
            }
            if !is_type(atom.data[*i], STRING) {
                // error!("expected key(string) found {}", self.data[i] as char);
                break;
            }

            let Ok(name) = pass_string(&atom.data[*i..]).map(|end| &atom.data[*i..*i + end]) else {
                // error!("bad syntax");
                break;
            };

            if name.len() <= 2 {
                // error!("bad json syntax empty object key")
                break;
            }

            *i += name.len();
            if *i >= atom.data.len() {
                // error!("unexpected end data");
                break;
            }

            *i += match pass_to_item(&atom.data[*i..]) {
                Ok(i) => i,
                Err(_) => {
                    // error!("bad json syntax expected \":\"");
                    break;
                }
            };
            if *i >= atom.data.len() {
                // error!("unexpected end data");
                break;
            }
            let res_atom = match Atom::new(&atom.data[*i..]).value() {
                Ok(o) => o,
                Err(_e) => {
                    // error!("parse as_bytes(): {_e}")
                    break;
                }
            };

            *i += res_atom.inner().len();
            if *i >= atom.data.len() {
                // error!("unexpected end data");
                break;
            }

            let key = String::from_utf8_lossy(&name[1..name.len() - 1]);
            // result.insert(key, obj);
            //
            *i += pass_space(&atom.data[*i..]);

            if *i < atom.data.len() && atom.data[*i] == b',' {
                *i += 1;
            }
            return Some((key, res_atom));
        }
        None
    }
}

pub trait ObjectIterEx<'a> {
    fn object_iter(&self) -> impl Iterator<Item = IterItem<'a>>;
}
impl<'a> ObjectIterEx<'a> for Atom<'a> {
    fn object_iter(&self) -> impl Iterator<Item = IterItem<'a>> {
        ObjectIter::new(self.clone())
    }
}
impl<'a> ObjectIterEx<'a> for Result<Atom<'a>> {
    fn object_iter(&self) -> impl Iterator<Item = IterItem<'a>> {
        ObjectIter::new(self.clone().ok())
    }
}

impl<'a> ObjectIterEx<'a> for Option<Atom<'a>> {
    fn object_iter(&self) -> impl Iterator<Item = IterItem<'a>> {
        ObjectIter::new(self.clone())
    }
}
#[test]
fn test_object_iter() {
    let data = br#"{
        "hello1": "world 1",
        "hello2": "world 2",
        "hello3": "world 3",
    }"#;
    fn map_item((key, body): (Cow<'_, str>, Atom<'_>)) -> (String, String) {
        (
            key.to_string(),
            String::from_utf8_lossy(body.inner()).to_string(),
        )
    }
    let map = Atom::from(&data)
        .object_iter()
        .map(map_item)
        .collect::<std::collections::HashMap<_, _>>();

    assert_eq!(map.get("hello1"), Some("\"world 1\"".to_string()).as_ref());
    assert_eq!(map.get("hello2"), Some("\"world 2\"".to_string()).as_ref());
    assert_eq!(map.get("hello3"), Some("\"world 3\"".to_string()).as_ref());
    assert_eq!(map.len(), 3);
}

#[test]
fn test_other_type() {
    let data = br#"[
        "hello 1",
        "hello 2"
    ]"#;
    let atom = Atom::from(data);
    let mut iter = atom.object_iter();
    assert!(iter.next().is_none());
}
