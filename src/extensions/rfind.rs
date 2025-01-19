use crate::{
    error::Result,
    needle::Needle,
    util::{is_type, pass_to_item},
    Atom,
};

pub trait RFindEx<'a> {
    fn rfind<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>>;
}
impl<'a> RFindEx<'a> for Atom<'a> {
    fn rfind<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        let needle = needle.into();
        let sep = format!("\"{}\"", needle.key());

        let mut end = self.pointer;

        while end >= sep.len() && end <= self.data.len() {
            let pointer = memchr::memmem::rfind(&self.data[..end], sep.as_bytes())?;
            if pointer != 0 && self.data[pointer - 1] == b'\\' {
                end = pointer;
                continue;
            }
            let current = match pass_to_item(&self.data[pointer + sep.len()..]) {
                Ok(c) => c + pointer + sep.len(),
                _ => {
                    // Found string value equals needle
                    end = pointer;
                    continue;
                }
            };

            // is reverse atom.find("name", ANY).rfind("name", ANY) skip current
            if current == self.current {
                end = pointer;
                continue;
            }

            if needle.value_type() == crate::ANY || is_type(self.data[current], needle.value_type())
            {
                return Some(Self {
                    data: &self.data[..],
                    pointer,
                    current,
                });
            }
            end = pointer // continue
        }
        None
    }
}
impl<'a> RFindEx<'a> for Result<Atom<'a>> {
    fn rfind<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        self.as_ref().ok()?.rfind(needle)
    }
}
impl<'a> RFindEx<'a> for Option<Atom<'a>> {
    fn rfind<'n>(&self, needle: impl Into<Needle<'n>>) -> Option<Atom<'a>> {
        self.as_ref()?.rfind(needle)
    }
}
