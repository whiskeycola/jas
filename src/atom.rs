#[derive(Clone, Debug, Copy)]
pub struct Atom<'a> {
    pub data: &'a [u8],
    pub pointer: usize,
    pub current: usize,
}
impl<'a> Atom<'a> {
    pub fn new<T>(data: &'a T) -> Self
    where
        T: AsRef<[u8]> + ?Sized,
    {
        Self::from_slice(data.as_ref())
    }
    pub fn from_slice(data: &'a [u8]) -> Atom<'a> {
        Atom {
            data,
            pointer: 0,
            current: 0,
        }
    }
    pub fn inner(self) -> &'a [u8] {
        self.data
    }
}

impl<'a, T> From<&'a T> for Atom<'a>
where
    T: AsRef<[u8]> + ?Sized,
{
    fn from(value: &'a T) -> Self {
        Self::new(value)
    }
}
