use crate::{ANY, ARRAY, BOOLEAN, NULL, NUMBER, OBJECT, STRING};

#[derive(Debug, Clone, Copy)]
pub struct Needle<'n> {
    pub key: &'n str,
    pub value_type: crate::ValueType,
}
impl<'n> Needle<'n> {
    pub fn new<T>(key: &'n T, value_type: crate::ValueType) -> Self
    where
        T: AsRef<str> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            value_type,
        }
    }
}

impl<'n, T> From<&'n T> for Needle<'n>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'n T) -> Self {
        Self {
            key: value.as_ref(),
            value_type: ANY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedAny<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedAny<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: ANY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedString<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedString<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: STRING,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedArray<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedArray<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: ARRAY,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedObject<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedObject<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: OBJECT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedBoolean<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedBoolean<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: BOOLEAN,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedNumber<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedNumber<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: NUMBER,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NeedNull<'n>(pub &'n str);
impl<'n> Into<Needle<'n>> for NeedNull<'n> {
    fn into(self) -> Needle<'n> {
        Needle {
            key: self.0,
            value_type: NULL,
        }
    }
}
