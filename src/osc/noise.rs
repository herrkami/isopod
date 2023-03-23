use crate::util::units::mHz;

/// Linear feedback shift register in Galois configuration
pub struct LFSR<T> {
    lfsr: T,
    mask: T,
}

/// 32-bit linear feedback shift register
impl LFSR<u32> {
    /// Returns the next value
    #[inline]
    pub fn next(&mut self) -> u32 {
        let lsb: bool = (self.lfsr & 0x01) != 0;
        self.lfsr >>= 1;
        if lsb {
            self.lfsr ^= self.mask;
        }
        self.lfsr
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for LFSR<u32> {
    fn default() -> Self {
        Self {
            lfsr: 0xCAFEBABE,
            mask: 0xA3000000,
        }
    }
}

/// 16-bit linear feedback shift register
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

    pub fn new() -> Self {
        Self::default()
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

/// White noise generator
pub struct WhiteNoise {
    lfsr: LFSR<u32>,
}

impl WhiteNoise {
    pub fn new() -> Self {
        let _lfsr = LFSR::<u32>::default();
        let s = Self { lfsr: _lfsr };
        s
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.lfsr.lfsr = seed;
    }
}

impl Iterator for WhiteNoise {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            i16::MAX
                .overflowing_sub_unsigned((self.lfsr.next() & 0xFFFF) as u16)
                .0,
        )
    }
}

/// Pink noise generator
pub struct PinkNoise {
    lfsr: LFSR<u32>,
    msample_rate: mHz,
}

/// Bit flip noise generator
pub struct BitFlipNoise {
    lfsr: LFSR<u32>,
}

/// Crackle noise generator
pub struct CrackleNoise {
    lfsr: LFSR<u32>,
    msample_rate: mHz,
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
    #[ignore = "takes very long (4_294_967_294 iterations)."]
    fn test_lfsr32() {
        let mut lfsr32 = LFSR::<u32>::default();
        let start = lfsr32.next();
        let mut period: u32 = 0;
        while lfsr32.next() != start {
            period += 1;
        }
        assert_eq!(period, 4_294_967_294);
    }

    #[test]
    fn test_white_noise16() {
        const N: i32 = 1_000_000;
        let mut white_noise = WhiteNoise::new();
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
}
