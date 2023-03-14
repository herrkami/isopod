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
