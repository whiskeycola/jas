use crate::{
    error::Result,
    util::{is_type, pass_to_item},
    Atom,
};

pub trait FindEx<'a> {
    fn find(&self, name: impl AsRef<str>, tp: crate::ValueType) -> Option<Atom<'a>>;
}
impl<'a> FindEx<'a> for Atom<'a> {
    fn find(&self, name: impl AsRef<str>, tp: crate::ValueType) -> Option<Atom<'a>> {
        let sep = format!("\"{}\"", name.as_ref());

        let mut start = self.pointer + 1;

        while start < self.data.len() {
            let pointer = start + memchr::memmem::find(&self.data[start..], sep.as_bytes())?;

            if self.data[pointer - 1] == b'\\' {
                start = pointer + sep.len();
                continue;
            };

            let current = {
                let s = pointer + sep.len();
                match pass_to_item(&self.data[s..]) {
                    Ok(c) => c + s,
                    _ => {
                        start = s;
                        continue;
                    }
                }
            };

            if current >= self.data.len() {
                break; // NOT FOUND
            }

            if tp == crate::ANY || is_type(self.data[current], tp) {
                return Some(Self {
                    data: &self.data[..],
                    pointer,
                    current,
                });
            }
            start = current;
        }
        None
    }
}

impl<'a> FindEx<'a> for Result<Atom<'a>> {
    fn find(&self, name: impl AsRef<str>, tp: crate::ValueType) -> Option<Atom<'a>> {
        self.as_ref().ok()?.find(name, tp)
    }
}

impl<'a> FindEx<'a> for Option<Atom<'a>> {
    fn find(&self, name: impl AsRef<str>, tp: crate::ValueType) -> Option<Atom<'a>> {
        self.as_ref()?.find(name, tp)
    }
}
