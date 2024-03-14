use std::usize;

use crate::{
    error::{Error, Result},
    Type, ANY, ARRAY, BOOLEAN, NULL, NUMBER, OBJECT, STRING,
};

/// check value type with first char
pub fn is_type(c: u8, t: Type) -> bool {
    match () {
        _ if t & STRING != 0 && c == b'"' => true,
        _ if t & OBJECT != 0 && c == b'{' => true,
        _ if t & ARRAY != 0 && c == b'[' => true,
        _ if t & NUMBER != 0 && (c == b'-' || (c >= b'0' && c <= b'9')) => true,
        _ if t & BOOLEAN != 0 && (c == b'f' || c == b't') => true,
        _ if t & NULL != 0 && c == b'n' => true,
        _ if t == ANY => !is_type(c, OBJECT | ARRAY | BOOLEAN | NUMBER | STRING),
        _ => false,
    }
}

/// detect value type from first char
pub fn get_type(c: u8) -> Result<Type> {
    match () {
        _ if is_type(c, STRING) => Ok(STRING),
        _ if is_type(c, OBJECT) => Ok(OBJECT),
        _ if is_type(c, ARRAY) => Ok(ARRAY),
        _ if is_type(c, NUMBER) => Ok(NUMBER),
        _ if is_type(c, BOOLEAN) => Ok(BOOLEAN),
        _ if is_type(c, NULL) => Ok(NULL),
        _ => Err(Error::UndefinedType(c as char)),
    }
}

/// shift to position end space
pub fn pass_space(data: &[u8]) -> usize {
    for i in 0..data.len() {
        match data[i] {
            32 | 9 | 10 | 13 => continue, // space | tab | \n | \r
            _ => return i,
        }
    }
    data.len()
}

/// get position to start value
pub fn pass_to_item(data: &[u8]) -> Result<usize> {
    let mut i = pass_space(data) + 1;
    if i < data.len() && data[i - 1] == b':' {
        i += pass_space(&data[i..]);
        if i < data.len() {
            return Ok(i);
        }
    }
    Err(Error::UnexpectedEOF)
}

#[test]
fn test_pass_to_item() {
    let data = br#"{"name": "item"}"#;
    assert_eq!(pass_to_item(&data[7..]), Ok(2));
}

/// pass string
pub fn pass_string(data: &[u8]) -> Result<usize> {
    let mut s = 1;
    while s < data.len() {
        s += memchr::memchr(b'"', &data[s..]).ok_or(Error::UnexpectedEOF)?;
        if data[s - 1] == b'\\' {
            if data.len() > 3 && data[s - 2] == b'\\' {
                return Ok(s + 1);
            }
            s += 1;
            continue;
        }
        return Ok(s + 1);
    }
    Err(Error::UnexpectedEOF)
}

#[test]
fn test_pass_string() {
    let data = br#"{"hello":"world"}"#;
    assert_eq!(pass_string(&data[1..]), Ok(7));
    assert_eq!(pass_string(&data[9..]), Ok(7));
}

pub fn pass_number(data: &[u8]) -> usize {
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            b'-' | b'+' | b'e' | b'E' | b'.' | b'0'..=b'9' => {}
            _ => break,
        }
        i += 1;
    }
    i
}

#[test]
fn test_pass_number() {
    let data = br#"{"number":-1.0E+2}"#;
    assert_eq!(pass_number(&data[10..]), 7);
}

pub fn pass_object(data: &[u8]) -> Result<usize> {
    if data.len() == 0 {
        return Err(Error::EmptyData);
    }
    let (cs, cf) = match get_type(data[0]) {
        Ok(OBJECT) => (b'{', b'}'),
        Ok(ARRAY) => (b'[', b']'),
        _ => return Err(Error::ExpectOtherValueType),
    };

    let mut stack = 1;
    let mut i = 1;

    while i < data.len() {
        match data[i] {
            // is string (not need check shielding)
            b'"' => {
                i += pass_string(&data[i..])?;
                continue;
            }
            c if cs == c => {
                stack += 1;
            }
            c if cf == c => {
                if stack == 0 {
                    return Err(Error::BadSyntax("do not expected \"}\"")); // invalid syntax
                }
                stack -= 1;
                if stack == 0 {
                    //TODO: return Some(data[..i+1])
                    i += 1;
                    break;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if i > data.len() {
        Err(Error::UnexpectedEOF)
    } else {
        Ok(i)
    }
}

#[test]
fn test_pass_object() {
    let data = br#"{"name":{"a":{"b":"c"}"b":{"z":"z"}}}"#;
    assert_eq!(pass_object(&data[8..]), Ok(28));
    assert_eq!(pass_object(data), Ok(data.len()));
}

pub fn pass_bool(data: &[u8]) -> Result<usize> {
    let sh = match data[0] {
        b't' => 4,
        b'f' => 5,
        _ => return Err(Error::ExpectOtherValueType),
    };
    if sh > data.len() {
        Err(Error::UnexpectedEOF)
    } else {
        Ok(sh)
    }
}

pub fn pass_null(data: &[u8]) -> Result<usize> {
    if 4 > data.len() {
        Err(Error::UnexpectedEOF)
    } else {
        match data[0] {
            b'n' => Ok(4),
            _ => Err(Error::ExpectOtherValueType),
        }
    }
}
