use std::marker::PhantomData;

use num::{Integer, PrimInt};

use crate::util::units::{mHz, Frequency, Hz};

/// Linear feedback shift register in Galois configuration
pub struct LFSR<T> {
    lfsr: T,
    mask: T,
}

impl Default for LFSR<u32> {
    fn default() -> Self {
        Self {
            lfsr: 0xCAFEBABE,
            mask: 0xA3000000,
        }
    }
}

/// 32-bit linear feedback shift register
impl LFSR<u32> {
    /// Returns the next value
    pub fn next(&mut self) -> u32 {
        let lsb: bool = (self.lfsr & 0x01) != 0;
        self.lfsr >>= 1;
        if lsb {
            self.lfsr ^= self.mask;
        }
        self.lfsr
    }
}

impl Default for LFSR<u16> {
    fn default() -> Self {
        Self {
            lfsr: 0xBABE,
            mask: 0xB400,
        }
    }
}

/// 32-bit linear feedback shift register
impl LFSR<u16> {
    /// Returns the next value
    #[inline]
    pub fn next(&mut self) -> u16 {
        let lsb: bool = (self.lfsr & 0x01) != 0;
        self.lfsr >>= 1;
        if lsb {
            self.lfsr ^= self.mask;
        }
        self.lfsr
    }
}

pub struct WhiteNoise<T> {
    lfsr: LFSR<u32>,
    seed: T,
    // phantom: PhantomData<T>,
}

impl<T> WhiteNoise<T> {
    fn set_seed(&mut self, seed: T) {
        self.seed = seed;
    }
}

// impl<T: Integer> WhiteNoise<T>
// // const generic
// where
//     T: Integer,
// {
//     fn new() -> Self {
//         let _lfsr = LFSR::<u32>::default();
//         let _seed: T = 42;
//         let s = Self {
//             seed: _seed,
//             lfsr: _lfsr,
//         };
//         s
//     }
// }

impl WhiteNoise<i16> {
    fn new() -> Self {
        let _lfsr = LFSR::<u32>::default();
        let _seed = 0x0abe;
        let s = Self {
            seed: _seed,
            lfsr: _lfsr,
        };
        s
    }
}

impl WhiteNoise<i32> {
    fn new() -> Self {
        let _lfsr = LFSR::<u32>::default();
        let _seed = 0x0abe;
        let s = Self {
            seed: _seed,
            lfsr: _lfsr,
        };
        s
    }
}

impl Iterator for WhiteNoise<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            i16::MAX
                .overflowing_sub_unsigned((self.lfsr.next() & 0xFFFF) as u16)
                .0,
        )
    }
}

impl Iterator for WhiteNoise<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(i32::MAX.overflowing_sub_unsigned(self.lfsr.next()).0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lfsr16() {
        let mut lfsr16 = LFSR::<u16>::default();
        let start = lfsr16.next();
        let mut period: u32 = 0;
        while lfsr16.next() != start {
            period += 1;
        }
        assert_eq!(period, 65534);
    }

    #[test]
    #[ignore = "takes very long."]
    fn test_lfsr32() {
        let mut lfsr32 = LFSR::<u32>::default();
        let start = lfsr32.next();
        let mut period: u32 = 0;
        while lfsr32.next() != start {
            period += 1;
        }
        assert_eq!(period, 4294967294);
    }

    #[test]
    fn test_white_noise16() {
        const N: i32 = 1_000_000;
        let mut white_noise = WhiteNoise::<i16>::new();
        let mut avg = 0_i32;
        let mut min = 0_i32;
        let mut max = 0_i32;
        let mut sym = 0_i32;
        for _ in 0..N {
            let x = white_noise.next();
            match x {
                Some(x) if x > 0 => sym += 1,
                Some(x) if x < 0 => sym -= 1,
                _ => {}
            }
            if (x.unwrap() as i32) < min {
                min = x.unwrap() as i32;
            }
            if (x.unwrap() as i32) > max {
                max = x.unwrap() as i32;
            }
            avg += x.unwrap() as i32;
            // println!("{:?}", x.unwrap());
        }
        avg /= N;
        // avg_n /= N / 2;
        println!(
            "avg: {:?}, sym: {:?} of {:?}, min: {:?}, max: {:?}",
            avg, sym, N, min, max
        );
    }

    #[test]
    fn test_white_noise32() {
        const N: i32 = 1_000_000;
        let mut white_noise = WhiteNoise::<i32>::new();
        let mut avg = 0_f32;
        let mut min = 0_i64;
        let mut max = 0_i64;
        let mut sym = 0_f32;
        for _ in 0..N {
            let x = white_noise.next();
            match x {
                Some(x) if x > 0 => sym += 1.0,
                Some(x) if x < 0 => sym -= 1.0,
                _ => {}
            }
            if (x.unwrap() as i64) < min {
                min = x.unwrap() as i64;
            }
            if (x.unwrap() as i64) > max {
                max = x.unwrap() as i64;
            }
            avg += x.unwrap() as f32;
            // println!("{:?}", x.unwrap());
        }
        avg /= (i32::MAX as i64 * N as i64) as f32;
        sym /= N as f32;
        // avg_n /= N / 2;
        println!(
            "avg: {:?}, sym: {:?} of {:?}, min: {:?}, max: {:?}",
            avg, sym, N, min, max
        );
    }
}

pub struct PinkNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}

pub struct BitFlipNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}

pub struct CrackleNoise<T> {
    lfsr: LFSR<T>,
    msample_rate: mHz,
}
