pub mod atom;
pub mod error;
pub mod util;
pub use atom::Atom;

pub type Type = u32;
pub const ANY: Type = 0;
pub const STRING: Type = 1;
pub const ARRAY: Type = 2;
pub const OBJECT: Type = 4;
pub const BOOLEAN: Type = 8;
pub const NUMBER: Type = 16;
pub const NULL: Type = 32;
