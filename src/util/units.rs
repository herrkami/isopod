use std::ops::Deref;

/// Unit mHz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(mHz(1).to_mHz(), mHz(1));
/// assert_eq!(mHz(1000).to_Hz(), Hz(1));
/// assert_eq!(mHz(1000000).to_kHz(), kHz(1));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct mHz(pub u32);

/// Unit Hz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(Hz(1).to_mHz(), mHz(1000));
/// assert_eq!(Hz(1).to_Hz(), Hz(1));
/// assert_eq!(Hz(1000).to_kHz(), kHz(1));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hz(pub u32);

/// Unit kHz
/// ```
/// # use isopod::util::units::*;
/// assert_eq!(kHz(1).to_mHz(), mHz(1000000));
/// assert_eq!(kHz(1).to_Hz(), Hz(1000));
/// assert_eq!(kHz(1).to_kHz(), kHz(1));
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct kHz(pub u32);

/// Conversions
pub trait Frequency {
    fn to_mHz(&self) -> mHz;
    fn to_Hz(&self) -> Hz;
    fn to_kHz(&self) -> kHz;
}

impl Frequency for mHz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0 / 1000)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0 / 1000000)
    }
}

impl Frequency for Hz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0 * 1000)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0 / 1000)
    }
}

impl Frequency for kHz {
    fn to_mHz(&self) -> mHz {
        mHz(self.0 * 1000000)
    }
    fn to_Hz(&self) -> Hz {
        Hz(self.0 * 1000)
    }
    fn to_kHz(&self) -> kHz {
        kHz(self.0)
    }
}

// impl Deref for mHz {
//     type Target = u32;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
