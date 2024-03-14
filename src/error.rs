pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    BadSyntax(&'static str),
    UnexpectedEOF,
    UndefinedType(char),
    EmptyData,
    ExpectOtherValueType,
}
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error {}
