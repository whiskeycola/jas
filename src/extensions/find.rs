use crate::{
    error::Result,
    needle::Needle,
    util::{is_type, pass_to_item},
    Atom,
};

pub trait FindEx<'a> {
    fn find<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>>;
}
impl<'a> FindEx<'a> for Atom<'a> {
    fn find<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        let needle = needle.into();
        let sep = format!("\"{}\"", needle.key());

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

            if needle.value_type() == crate::ANY || is_type(self.data[current], needle.value_type())
            {
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
    fn find<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        self.as_ref().ok()?.find(needle)
    }
}

impl<'a> FindEx<'a> for Option<Atom<'a>> {
    fn find<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        self.as_ref()?.find(needle)
    }
}
