// TODO:
// - Write tests.
// - Write examples.
// - Write docs.

/// A bit-vector.
pub struct BitVector {
    contents: Vec<u8>,
    len: usize,
}

impl BitVector {
    /// Constructs a new, empty BitVector.
    pub fn new() -> Self {
        BitVector {
            contents: Vec::new(),
            len: 0,
        }
    }

    /// Returns the number of bits in the bit-vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the bit at `index`.
    pub fn get(&self, index: usize) -> Option<bool> {
        if index / 8 < self.len() {
            Some((&self.contents[index / 8] >> (index % 8)) == 1u8)
        } else {
            None
        }
    }

    /// Sets the bit at `index` to `bit`.
    pub fn set(&mut self, index: usize, bit: bool) -> Result<(), &str> {
        if index / 8 < self.len() {
            if bit {
                self.contents[index / 8] |= 1u8 << (index % 8);
            } else {
                self.contents[index / 8] &= !1u8 << (index % 8);
            }

            Ok(())
        } else {
            Err("index out of bounds")
        }
    }

    /// Pushes a bit to the end of the bit-vector.
    pub fn push(&mut self, bit: bool) -> Result<(), &str> {
        if self.len() % 8 == 0 {
            self.contents.push(0);
        }

        self.len += 1;
        self.set(self.len - 1, bit)?;
        return Ok(());
    }

    /// Pops a bit from the end of the bit-vector.
    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        let popped = self.get(self.len)?;

        if self.len() % 8 == 0 {
            // The final byte in the Vec is now empty.
            self.contents.pop();
        }

        return Some(popped);
    }

    /// Returns `true` if the bit-vector is empty, else `false`.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the bit-vector as a Vec<u8>.
    pub fn to_vec(&self) -> Vec<u8> {
        return self.contents.to_vec();
    }

    /// Creates a bit-vector from a Vec<u8>.
    pub fn from_vec(vec: Vec<u8>) -> BitVector {
        return BitVector {
            contents: vec.clone(),
            len: vec.len() * 8,
        };
    }
}

impl std::fmt::Debug for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut s = String::new();
        s.push('[');

        if !self.is_empty() {
            s.push_str(format!("{}", self.get(0).unwrap()).as_str());
        }

        for i in 1..self.len {
            if self.get(i).unwrap() {
                s.push('1');
            } else {
                s.push('0');
            }

            s.push_str(", ");
        }

        s.push(']');
        write!(f, "{}", s)
    }
}
