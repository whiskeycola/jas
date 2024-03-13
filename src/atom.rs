#[derive(Clone, Debug)]
pub struct Atom<'a> {
    pub data: &'a [u8],
    pub pointer: usize,
    pub current: usize,
}

impl<'a> Atom<'a> {
    pub fn new(data: &'a [u8]) -> Atom<'a> {
        Atom {
            data,
            pointer: 0,
            current: 0,
        }
    }
}

impl<'a, T> From<&'a T> for Atom<'a>
where
    T: AsRef<[u8]>,
{
    fn from(value: &'a T) -> Self {
        let data = value.as_ref();
        Self::new(data)
    }
}
