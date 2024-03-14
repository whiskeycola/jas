pub mod atom;
pub mod error;
pub mod extensions;
pub mod util;
pub use atom::Atom;

pub type ValueType = u32;
pub const ANY: ValueType = 0;
pub const STRING: ValueType = 1;
pub const ARRAY: ValueType = 2;
pub const OBJECT: ValueType = 4;
pub const BOOLEAN: ValueType = 8;
pub const NUMBER: ValueType = 16;
pub const NULL: ValueType = 32;
