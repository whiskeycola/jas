use crate::error::Result;
use crate::Atom;

pub trait ParentEx<'a> {
    fn parent(&self) -> Option<Atom<'a>>;
}

impl<'a> ParentEx<'a> for Atom<'a> {
    fn parent(&self) -> Option<Atom<'a>> {
        let mut i = self.current;
        let mut ostack = 0usize;
        let mut astack = 0usize;
        while i > 0 {
            i -= 1;
            match self.data[i] {
                b'{' => {
                    if i > 0 && self.data[i - 1] == b'\\' {
                        continue;
                    }
                    if ostack == 0 {
                        return Some(Self {
                            data: &self.data,
                            current: i,
                            pointer: i,
                        });
                    }
                    ostack -= 1;
                }
                b'}' => {
                    if i > 0 && self.data[i - 1] == b'\\' {
                        continue;
                    }
                    ostack += 1;
                    continue;
                }
                b'[' => {
                    if i > 0 && self.data[i - 1] == b'\\' {
                        continue;
                    }
                    if astack == 0 {
                        return Some(Self {
                            data: &self.data,
                            current: i,
                            pointer: i,
                        });
                    }
                    astack -= 1;
                }
                b']' => {
                    if i > 0 && self.data[i - 1] == b'\\' {
                        continue;
                    }
                    astack += 1;
                }
                _ => (),
            }
        }
        None
    }
}
impl<'a> ParentEx<'a> for Result<Atom<'a>> {
    fn parent(&self) -> Option<Atom<'a>> {
        self.as_ref().ok()?.parent()
    }
}
impl<'a> ParentEx<'a> for Option<Atom<'a>> {
    fn parent(&self) -> Option<Atom<'a>> {
        self.as_ref()?.parent()
    }
}
