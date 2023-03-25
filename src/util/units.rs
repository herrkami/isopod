/// Unit mHz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(mHz(1).to_mHz(), mHz(1));
/// assert_eq!(mHz(1_000).to_Hz(), Hz(1));
/// assert_eq!(mHz(1_000_000).to_kHz(), kHz(1));
///
/// assert_eq!(mHz(1_000).to_us(), us(1_000_000));
/// assert_eq!(mHz(1_000).to_ms(), ms(1_000));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct mHz(pub u32);

/// Unit Hz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(Hz(1).to_mHz(), mHz(1_000));
/// assert_eq!(Hz(1).to_Hz(), Hz(1));
/// assert_eq!(Hz(1_000).to_kHz(), kHz(1));
///
/// assert_eq!(Hz(1).to_us(), us(1_000_000));
/// assert_eq!(Hz(1).to_ms(), ms(1_000));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hz(pub u32);

/// Unit kHz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(kHz(1).to_mHz(), mHz(1_000_000));
/// assert_eq!(kHz(1).to_Hz(), Hz(1_000));
/// assert_eq!(kHz(1).to_kHz(), kHz(1));
///
/// assert_eq!(kHz(1).to_us(), us(1_000));
/// assert_eq!(kHz(1).to_ms(), ms(1));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct kHz(pub u32);

/// Unit ms (milliseconds)
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(ms(1_000_000).to_mHz(), mHz(1));
/// assert_eq!(ms(1_000).to_Hz(), Hz(1));
/// assert_eq!(ms(1).to_kHz(), kHz(1));
///
/// assert_eq!(ms(1).to_us(), us(1_000));
/// assert_eq!(ms(1).to_ms(), ms(1));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ms(pub u32);

/// Unit us (microseconds)
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(us(1_000_000).to_mHz(), mHz(1_000));
/// assert_eq!(us(1_000).to_Hz(), Hz(1_000));
/// assert_eq!(us(1).to_kHz(), kHz(1_000));
///
/// assert_eq!(us(1).to_us(), us(1));
/// assert_eq!(us(1_000).to_ms(), ms(1));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct us(pub u32);

// Conversions
#[allow(non_snake_case)]
pub trait Frequency {
    fn to_mHz(&self) -> mHz;
    fn to_Hz(&self) -> Hz;
    fn to_kHz(&self) -> kHz;
}

pub trait Period {
    fn to_us(&self) -> us;
    fn to_ms(&self) -> ms;
}

impl Frequency for mHz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0 / 1_000)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0 / 1_000_000)
    }
}
impl Period for mHz {
    fn to_us(&self) -> us {
        us(1_000_000_000 / self.0)
    }
    fn to_ms(&self) -> ms {
        ms(1_000_000 / self.0)
    }
}

impl Frequency for Hz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0 * 1_000)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0 / 1_000)
    }
}
impl Period for Hz {
    fn to_us(&self) -> us {
        us(1_000_000 / self.0)
    }
    fn to_ms(&self) -> ms {
        ms(1_000 / self.0)
    }
}

impl Frequency for kHz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0 * 1_000_000)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0 * 1_000)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0)
    }
}
impl Period for kHz {
    fn to_us(&self) -> us {
        us(1_000 / self.0)
    }
    fn to_ms(&self) -> ms {
        ms(1 / self.0)
    }
}

impl Frequency for ms {
    fn to_mHz(&self) -> mHz {
        mHz(1_000_000 / self.0)
    }
    fn to_Hz(&self) -> Hz {
        Hz(1_000 / self.0)
    }
    fn to_kHz(&self) -> kHz {
        kHz(1 / self.0)
    }
}
impl Period for ms {
    fn to_us(&self) -> us {
        us(1_000 * self.0)
    }
    fn to_ms(&self) -> ms {
        ms(self.0)
    }
}

impl Frequency for us {
    fn to_mHz(&self) -> mHz {
        mHz(1_000_000_000 / self.0)
    }
    fn to_Hz(&self) -> Hz {
        Hz(1_000_000 / self.0)
    }
    fn to_kHz(&self) -> kHz {
        kHz(1_000 / self.0)
    }
}
impl Period for us {
    fn to_us(&self) -> us {
        us(1 * self.0)
    }
    fn to_ms(&self) -> ms {
        ms(self.0 / 1_000)
    }
}

/// Normalization constant for Sample. Sample is represented as `i16` which goes
/// from -32768 to 32767. Maximum amplitude and the normalization constant are
/// therefore 32768. As a power of two `SAMPLE_NORM` can also be used for
/// efficient divisions.
///
/// While it would be possible to normalize by 32767 to achieve better symmetry and avoid artifacts (see [Sample::multiply_normed]), it would reduce the normalization efficiency (because divisions by powers of two are cheap).
pub const SAMPLE_NORM: i32 = i16::MAX as i32 + 1_i32;

const SAMPLE_MAX: i16 = i16::MAX;
const SAMPLE_MIN: i16 = i16::MIN;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample(pub i16);

impl Sample {
    /// Multiplies the sample and normalizes it by the maximum amplitude (i.e.
    /// 32768 = 0x8000).
    ///
    /// This amplitude is stored in [SAMPLE_NORM]. While [SAMPLE_NORM] is
    /// equivalent to the absolute minimum value of [Sample], it is larger than
    /// the maximum by one LSB. This effectively results in a linear error.
    /// However, for most practical purposes this error is negligible.
    /// ```
    /// # use isopod::util::units::*;
    /// assert_eq!(Sample(100).multiply_normed(Sample((SAMPLE_NORM / 2) as i16)), Sample(50));
    /// assert_eq!(Sample(100).multiply_normed(Sample(i16::MAX)), Sample(99));
    /// ```
    pub fn multiply_normed(&self, x: Sample) -> Sample {
        Sample(((x.0 as i32 * self.0 as i32) / SAMPLE_NORM as i32) as i16)
    }

    /// Saturating addition
    /// ```
    /// # use isopod::util::units::*;
    /// assert_eq!(Sample(i16::MAX - 13).saturating_add(Sample(200)), Sample(i16::MAX));
    /// assert_eq!(Sample(i16::MIN + 13).saturating_add(Sample(-200)), Sample(i16::MIN));
    /// assert_eq!(Sample(14).saturating_add(Sample(42)), Sample(56));
    /// ```
    pub fn saturating_add(&self, x: Sample) -> Sample {
        Sample(self.0.saturating_add(x.0))
    }

    /// Saturating multiplication
    /// ```
    /// # use isopod::util::units::*;
    /// assert_eq!(Sample(i16::MAX).saturating_mul(Sample(3)), Sample(i16::MAX));
    /// assert_eq!(Sample(i16::MIN).saturating_mul(Sample(4)), Sample(i16::MIN));
    /// assert_eq!(Sample(4).saturating_mul(Sample(-5)), Sample(-20));
    /// ```
    pub fn saturating_mul(&self, x: Sample) -> Sample {
        Sample(self.0.saturating_mul(x.0))
    }

    /// Clipping indicator
    ///
    /// Returns `true` if the signal is clipping
    /// ```
    /// # use isopod::util::units::*;
    /// assert_eq!(Sample(i16::MAX).is_clipping(), true);
    /// assert_eq!(Sample(i16::MIN).is_clipping(), true);
    /// assert_eq!(Sample(i16::MAX - 1).is_clipping(), false);
    /// ```
    pub fn is_clipping(&self) -> bool {
        if (self.0 == SAMPLE_MAX) | (self.0 == SAMPLE_MIN) {
            true
        } else {
            false
        }
    }
}
