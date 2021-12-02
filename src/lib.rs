use std::vec::Vec;


#[derive(Debug)]
pub struct BigInt {
    digits: Vec<u8>,
    positive: bool,
}


impl BigInt {
    #[inline]
    pub const fn new() -> Self {
        return Self {
            digits: Vec::new(),
            positive: true,
        }
    }

    #[inline]
    pub fn digit_count(&self) -> usize {
        return self.digits.len();
    }
}


#[cfg(test)]
mod tests {
    use super::BigInt;
}