use derive_deref_rs::Deref;

use crate::osc::luts::SINE_I16;
use crate::osc::luts::{EXP_I16, EXP_I16_NORM, EXP_I16_TAU};
use crate::util::units::{mHz, ms, Frequency, Hz};

/// Maximum value of the phase accumulator
const PHI_MAX: u32 = 1 << 20;
/// Magic normalization constant for efficient divisions in the performance
/// critical functions
const NORM: u32 = 1 << 26;

/// Stateful wavetable signal generator
pub struct Engine<T: 'static> {
    // If false, the oscillator will stop after one period
    repeat: bool,
    // Indicates if the oscillator is running
    running: bool,

    // Signal frequency in mHz
    mfreq: mHz,
    // Sample rate in mHz
    msample_rate: mHz,

    // Wavetable should have a power of two
    wavetable: &'static [T],

    // Phase accumulator
    phi: u32,
    // Frequency dependent phase increment
    delta_phi: u32,
    // Constant for frequency to phase translation
    alpha: u32,

    // Current wavetable index
    idx: usize,
    // Maximum wavetable index
    idx_max: usize,
}

impl<T> Engine<T> {
    fn update_idx(&mut self) {
        self.idx = (((self.idx_max as u32) * self.phi) / PHI_MAX) as usize;
    }

    fn update_alpha(&mut self) {
        self.alpha =
            (((PHI_MAX as u64) * (NORM as u64) as u64) / (self.msample_rate.0 as u64)) as u32;
        // println!("------------------alpha");
        // println!("PHI_MAX: {:?}", PHI_MAX);
        // println!("NORM: {:?}", NORM);
        // println!("PHI_MAX*NORM: {:?}", (PHI_MAX as i64)*(NORM as i64));
        // println!("alpha: {:?}", self.alpha);
        // println!("alpha (casted): {:?}", ((PHI_MAX as i64)*(NORM as i64) as i64) / (self.msample_rate.0 as i64));
    }

    fn update_delta_phi(&mut self) {
        // Normally this function would look like this:
        // self.delta_phi = (((self.mfreq.0 as i64) * (PHI_MAX as i64))
        //     / (self.msample_rate.0 as i64)) as i32;

        // But since msample_rate is not constrained to be a power of
        // two, the resulting 64-bit division could be very expensive.
        // To avoid this, we perform a coefficient exchange between
        // PHI_MAX and msample_rate such that the denominator NORM
        // becomes a power of two and alpha absorbs the exact value. See
        // also [update_alpha].
        self.delta_phi = (((self.mfreq.0 as u64) * (self.alpha as u64)) / (NORM as u64)) as u32;
        // println!("------------------delta_phi");
        // println!("delta_phi: {:?}", self.delta_phi);
        // println!("alpha: {:?}", self.alpha);
        // println!("mfreq: {:?}", self.mfreq.0);
        // println!("NORM: {:?}", NORM);
    }

    /// Increments the phase accumulator and returns the next sample. If
    /// the generator is not running, it returns `None`.
    #[inline]
    pub fn _next(&mut self) -> Option<T>
    where
        T: Copy,
    {
        // TODO Replace if-clause by masked addition
        self.phi += self.delta_phi;
        if self.phi > PHI_MAX {
            self.phi -= PHI_MAX;
            if !self.repeat {
                self.stop_and_reset();
            }
        };
        if self.is_running() {
            self.update_idx();
            let out = self.wavetable[self.idx];
            Some(out)
        } else {
            None
        }
    }

    /// Sets the wavetable.
    pub fn set_wavetable(&mut self, wavetable: &'static [T]) {
        self.wavetable = wavetable;
        self.idx_max = self.wavetable.len();
    }

    /// Sets repeat to true or false. If false, the oscillator will stop
    /// after one period.
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    /// Sets the generator into "running" mode.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stops the generator (disable "running" mode). Holds the last
    /// phase value.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Resets the phase accumulator to 0.
    pub fn reset(&mut self) {
        self.phi = 0;
    }

    /// Resets the phase accumulator 0 and sets the generator into
    /// "running" mode
    pub fn reset_and_start(&mut self) {
        self.reset();
        self.running = true;
    }

    /// Stops the generator and reset the phase accumulator to 0.
    pub fn stop_and_reset(&mut self) {
        self.running = false;
        self.reset();
    }

    /// True if the generator is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Sets the frequency in mHz.
    pub fn set_mfreq(&mut self, mfreq: mHz) {
        self.mfreq = mfreq;
        self.update_delta_phi();
    }

    /// Sets the frequency in Hz.
    pub fn set_freq(&mut self, freq: Hz) {
        self.mfreq = freq.to_mHz();
        self.update_delta_phi();
    }

    /// Sets the sample rate in mHz.
    pub fn set_msample_rate(&mut self, msample_rate: mHz) {
        self.msample_rate = msample_rate;
        self.update_alpha();
        self.update_delta_phi();
    }

    /// Sets the sample rate in Hz.
    pub fn set_sample_rate(&mut self, sample_rate: Hz) {
        self.msample_rate = sample_rate.to_mHz();
        self.update_alpha();
        self.update_delta_phi();
    }

    pub fn new() -> Self {
        let mut s = Self {
            repeat: true,
            running: false,

            mfreq: mHz(0),
            msample_rate: mHz(0),

            wavetable: &[],

            phi: 0,
            delta_phi: 0,
            alpha: 0,

            idx: 0,
            idx_max: 0,
        };
        s.set_sample_rate(Hz(44100));
        s.set_freq(Hz(440));
        s
    }
}

// Generic i16
#[derive(Deref)]
pub struct WavetableOscillator {
    _engine: Engine<i16>,
}
impl WavetableOscillator {
    pub fn new() -> Self {
        let mut s = Self {
            _engine: Engine::<i16> {
                repeat: true,
                running: false,

                mfreq: mHz(0),
                msample_rate: mHz(0),

                wavetable: &[],

                phi: 0,
                delta_phi: 0,
                alpha: 0,

                idx: 0,
                idx_max: 0,
            },
        };
        s.set_sample_rate(Hz(44100));
        s.set_freq(Hz(440));
        s
    }
}
impl Iterator for WavetableOscillator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

// Sine i16
#[derive(Deref)]
pub struct SineOscillator {
    _engine: Engine<i16>,
}
impl SineOscillator {
    pub fn new() -> Self {
        let mut s = Self {
            _engine: Engine::<i16> {
                repeat: true,
                running: false,

                mfreq: mHz(0),
                msample_rate: mHz(0),

                wavetable: &SINE_I16,

                phi: 0,
                delta_phi: 0,
                alpha: 0,

                idx: 0,
                idx_max: SINE_I16.len(),
            },
        };
        s.set_sample_rate(Hz(44100));
        s.set_freq(Hz(440));
        s
    }
}
impl Iterator for SineOscillator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

// Exp i16
#[derive(Deref)]
pub struct ExpDecay {
    _engine: Engine<i16>,
}
impl ExpDecay {
    pub fn new() -> Self {
        let mut s = Self {
            _engine: Engine::<i16> {
                repeat: false,
                running: false,

                mfreq: mHz(0),
                msample_rate: mHz(0),

                wavetable: &EXP_I16,

                phi: 0,
                delta_phi: 0,
                alpha: 0,

                idx: 0,
                idx_max: EXP_I16.len(),
            },
        };
        s.set_sample_rate(Hz(44100));
        s.set_decay_ms(ms(1000));
        s
    }

    pub fn set_period_ms(&mut self, dur: ms) {
        self.set_mfreq(dur.to_mHz());
    }

    pub fn set_decay_ms(&mut self, decay: ms) {
        let dur = ms((decay.0 * EXP_I16.len() as u32) / EXP_I16_TAU as u32);
        self.set_mfreq(dur.to_mHz());
    }
}
impl Iterator for ExpDecay {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

use core::time::Duration;
use rodio::source::Source;
impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.msample_rate.to_Hz().0
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::osc::luts::SINE_I16;

    #[test]
    fn test_wavetable_oscillator() {
        let mut osc = WavetableOscillator::new();
        osc.set_wavetable(&SINE_I16);
        osc.set_freq(Hz(2));
        osc.set_sample_rate(Hz(100));
        osc.set_repeat(false);
        osc.start();
        for x in 0..110 {
            let _y = osc._next();
            match _y {
                Some(y) => println!("{}: {}\n", x, (y as f64) / (i16::MAX as f64)),
                None => {
                    println!("Generator stopped at {}", x);
                    break;
                }
            }
        }
    }

    #[test]
    fn test_exp_decay() {
        let mut decay = ExpDecay::new();
        decay.set_sample_rate(Hz(1000));
        decay.set_decay_ms(ms(10));
        decay.set_repeat(false);
        decay.start();
        for x in 0..110 {
            let _y = decay._next();
            match _y {
                Some(y) => println!("{}: {}\n", x, (y as f64) / (i16::MAX as f64)),
                None => {
                    println!("Generator stopped at {}", x);
                    break;
                }
            }
        }
    }
}
