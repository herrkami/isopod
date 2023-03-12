use crate::util::units::mHz;

// Constant multiplier for precision divisions
const NORM: u32 = 1 << 12;
pub const Q_MAX: u32 = NORM;

// MF = 1/(2*pi*dt) for mHz
// sample rate dt = 60 u
// const MF: i32 = 2652582;

pub struct StateVariableFilter {
    lp: i16,
    bp: i16,
    hp: i16,
    no: i16,

    ft: u32,
    q_inv: u32,

    msample_rate: mHz,
}

/// Simple and efficient state variable filter (Chamberlin version)
/// The code is based on
/// https://www.musicdsp.org/en/latest/Filters/142-state-variable-filter-chamberlin-version.html
/// which, in turn, seems to be based on
/// Hal Chamberlin, “Musical Applications of Microprocessors,” 2nd Ed,
/// Hayden Book Company 1985. pp 490-492.
impl StateVariableFilter {
    pub fn new() -> Self {
        Self {
            lp: 0,
            bp: 0,
            hp: 0,
            no: 0,

            ft: 0,
            q_inv: NORM - 0,

            msample_rate: mHz(44_100_000),
        }
    }

    pub fn feed(&mut self, signal: i16) {
        // TODO To gain performance we might try to store the sign, perform u32
        // divisions, then restore the sign.
        self.lp += ((self.ft as i32 * self.bp as i32) / NORM as i32) as i16;
        // dbg!(signal, self.lp, self.bp, self.hp, NORM, self.q_inv);
        self.hp = signal - self.lp - ((self.q_inv as i32 * self.bp as i32) / NORM as i32) as i16;
        self.bp += ((self.ft as i32 * self.hp as i32) / NORM as i32) as i16;
        self.no = self.hp + self.lp;
    }

    pub fn get_lp(&self) -> i16 {
        self.lp
    }

    pub fn get_bp(&self) -> i16 {
        self.bp
    }

    pub fn get_hp(&self) -> i16 {
        self.hp
    }

    pub fn get_no(&self) -> i16 {
        self.no
    }

    pub fn set_mfreq(&mut self, mfreq: mHz) {
        // We use a first order Tailor approximation here. -> Deviations close
        // to Nyquist frequency.
        self.ft = ((NORM as i64 * mfreq.0 as i64) / self.msample_rate.0 as i64) as u32;
    }

    pub fn set_q(&mut self, q: u32) {
        if q > NORM {
            self.q_inv = 0;
        } else {
            self.q_inv = NORM - q;
        }
    }

    pub fn set_msample_rate(&mut self, msample_rate: mHz) {
        self.msample_rate = msample_rate;
    }
}
