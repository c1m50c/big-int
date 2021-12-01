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
    fn from(string: &str) -> Self {
        return Self::from(String::from(string));
    }
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
}
