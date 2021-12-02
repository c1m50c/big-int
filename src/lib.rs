#[allow(unused_imports)]
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[allow(unused_imports)]
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

use std::convert::From;
use std::vec::Vec;


#[derive(Debug)]
pub struct BigInt {
    number: String,
}


impl BigInt {
    #[inline]
    pub fn new() -> Self {
        return Self {
            number: String::from("0"),
        };
    }

    pub fn len(&self) -> usize {
        return self.number.len();
    }
}


impl Eq for BigInt {  }
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }

        let other_chars = other.number.chars().collect::<Vec<char>>();
        for (i, c) in self.number.char_indices() {
            if c != other_chars[i] { return false; }
        }
        
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        if self.len() != other.len() { return true; }

        let other_chars = other.number.chars().collect::<Vec<char>>();
        for (i, c) in self.number.char_indices() {
            if c != other_chars[i] { return true; }
        }
        
        return false;
    }
}


// impl Ord for BigInt {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         return Ordering::Equal;
//     }

//     fn max(self, other: Self) -> Self {
//         return Self::new();
//     }

//     fn min(self, other: Self) -> Self {
//         return Self::new();
//     }

//     fn clamp(self, min: Self, max: Self) -> Self {
//         return Self::new();
//     }
// }


// impl PartialOrd for BigInt {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         return None;
//     }
    
//     fn lt(&self, other: &Self) -> bool {
//         return false;
//     }
    
//     fn le(&self, other: &Self) -> bool {
//         return false;
//     }
    
//     fn gt(&self, other: &Self) -> bool {
//         return false;
//     }
    
//     fn ge(&self, other: &Self) -> bool {
//         return false;
//     }
// }


impl From<String> for BigInt {
    fn from(string: String) -> Self {
        let mut result: BigInt = BigInt::new();
        let temp = string.strip_prefix("-");

        match temp {
            Some(s) => {
                if s.chars().all(char::is_numeric) {
                    result.number = String::from(s);
                    result.number.insert(0, '-');
                } else {
                    panic!("Cannot create a BigInt from String '{}'.", string);
                }
            }

            None => {
                if string.chars().all(char::is_numeric) { result.number = string; }
                else { panic!("Cannot create a BigInt from String '{}'.", string); }
            }
        }

        return result;
    }
}


impl From<&str> for BigInt {
    fn from(string: &str) -> Self { Self::from(String::from(string)) }
}


impl From<i8> for BigInt {
    fn from(int: i8) -> Self { Self::from(int.to_string()) }
}


impl From<i16> for BigInt {
    fn from(int: i16) -> Self { Self::from(int.to_string()) }
}


impl From<i32> for BigInt {
    fn from(int: i32) -> Self { Self::from(int.to_string()) }
}


impl From<i64> for BigInt {
    fn from(int: i64) -> Self { Self::from(int.to_string()) }
}


impl From<i128> for BigInt {
    fn from(int: i128) -> Self { Self::from(int.to_string()) }
}


impl From<u8> for BigInt {
    fn from(int: u8) -> Self { Self::from(int.to_string()) }
}


impl From<u16> for BigInt {
    fn from(int: u16) -> Self { Self::from(int.to_string()) }
}


impl From<u32> for BigInt {
    fn from(int: u32) -> Self { Self::from(int.to_string()) }
}


impl From<u64> for BigInt {
    fn from(int: u64) -> Self { Self::from(int.to_string()) }
}


impl From<u128> for BigInt {
    fn from(int: u128) -> Self { Self::from(int.to_string()) }
}


#[cfg(test)]
mod tests {
    use super::BigInt;

    #[test]
    fn eq() {
        assert_eq!(BigInt::from("-4239"), BigInt::from("-4239"));
        assert_eq!(BigInt::from("63"), BigInt::from("63"));
    }

    #[test]
    fn ne() {
        assert_ne!(BigInt::from("777"), BigInt::from("89"));
        assert_ne!(BigInt::from("49520"), BigInt::from("60439"));
        assert_ne!(BigInt::from("-9074"), BigInt::from("-9075"));
    }

    #[test]
    fn from_string() {
        assert_eq!(BigInt::from("1337").number, String::from("1337"));
        assert_eq!(BigInt::from("-100").number, String::from("-100"));
    }

    #[test]
    fn from_integer() {
        assert_eq!(BigInt::from(i8::MAX as i8).number, (i8::MAX as i8).to_string());
        assert_eq!(BigInt::from(i8::MIN as i8).number, (i8::MIN as i8).to_string());
        assert_eq!(BigInt::from(i16::MAX as i16).number, (i16::MAX as i16).to_string());
        assert_eq!(BigInt::from(i16::MIN as i16).number, (i16::MIN as i16).to_string());
        assert_eq!(BigInt::from(i32::MAX as i32).number, (i32::MAX as i32).to_string());
        assert_eq!(BigInt::from(i32::MIN as i32).number, (i32::MIN as i32).to_string());
        assert_eq!(BigInt::from(i64::MAX as i64).number, (i64::MAX as i64).to_string());
        assert_eq!(BigInt::from(i64::MIN as i64).number, (i64::MIN as i64).to_string());
        assert_eq!(BigInt::from(i128::MAX as i128).number, (i128::MAX as i128).to_string());
        assert_eq!(BigInt::from(i128::MIN as i128).number, (i128::MIN as i128).to_string());
    }

    #[test]
    fn from_unsigned_integer() {
        assert_eq!(BigInt::from(u8::MAX as u8).number, (u8::MAX as u8).to_string());
        assert_eq!(BigInt::from(u8::MIN as u8).number, (u8::MIN as u8).to_string());
        assert_eq!(BigInt::from(u16::MAX as u16).number, (u16::MAX as u16).to_string());
        assert_eq!(BigInt::from(u16::MIN as u16).number, (u16::MIN as u16).to_string());
        assert_eq!(BigInt::from(u32::MAX as u32).number, (u32::MAX as u32).to_string());
        assert_eq!(BigInt::from(u32::MIN as u32).number, (u32::MIN as u32).to_string());
        assert_eq!(BigInt::from(u64::MAX as u64).number, (u64::MAX as u64).to_string());
        assert_eq!(BigInt::from(u64::MIN as u64).number, (u64::MIN as u64).to_string());
        assert_eq!(BigInt::from(u128::MAX as u128).number, (u128::MAX as u128).to_string());
        assert_eq!(BigInt::from(u128::MIN as u128).number, (u128::MIN as u128).to_string());
    }
}
