use std::fmt::Display;

use rand::{RngExt, SeedableRng, rng, rngs::StdRng};

/// The number of bytes to generate for secure random strings (api keys, admin tokens, etc)
const SECURE_RANDOM_SIZE: usize = 32;

/// Gets a random string of 64 hex digits (32 bytes) generated from a cryptographically secure random number generator ([StdRng])
pub fn get_random_string_s() -> String {
    let mut buff: [u8; SECURE_RANDOM_SIZE] = [0u8; _];
    StdRng::from_rng(&mut rng()).fill(&mut buff);

    format!("{}", LowerCaseHexSlice(&buff))
}

#[allow(unused)]
struct LowerCaseHexSlice<'a>(&'a [u8]);
impl Display for LowerCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

#[allow(unused)]
struct UpperCaseHexSlice<'a>(&'a [u8]);
impl Display for UpperCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}
