use crate::osc::luts::SINE_I16;
use crate::util::units::{mHz, Frequency, Hz};

/// Maximum value of the phase accumulator
const PHI_MAX: i32 = 1 << 20;
/// Magic divider constant for efficient divisions in the performance critical
/// functions
const DIVIDER: i32 = 1 << 28;

macro_rules! wavetable_oscillator {
    ($($name: ident),+ $(,)?) => ($(

        /// Stateful wavetable signal generator
        pub struct $name<T: 'static> {
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
            phi: i32,
            // Frequency dependent phase increment
            delta_phi: i32,
            // Constant for frequency to phase translation
            alpha: i32,

            // Current wavetable index
            idx: usize,
            // Maximum wavetable index
            idx_max: usize,
        }

        impl<T> $name<T> {
            fn update_idx(&mut self) {
                self.idx = (((self.idx_max as i32) * self.phi) / PHI_MAX) as usize;
            }

            fn update_alpha(&mut self) {
                self.alpha = ((PHI_MAX as i64)*(DIVIDER as i64) / (self.msample_rate.0 as i64)) as i32;
            }

            fn update_delta_phi(&mut self) {
                // self.delta_phi =
                //     (((self.mfreq.0 as i64) * (PHI_MAX as i64)) / (self.msample_rate.0 as i64)) as i32;
                self.delta_phi =
                    (((self.mfreq.0 as i64) * (self.alpha as i64)) / (DIVIDER as i64)) as i32;
            }

            /// Increments phase accumulator and returns either the next sample or None
            /// if the generator is not running
            #[inline]
            pub fn _next(&mut self) -> Option<T>
            where
                T: Copy,
            {
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

            /// Set the wavetable
            pub fn set_wavetable(&mut self, wavetable: &'static [T]) {
                self.wavetable = wavetable;
                self.idx_max = self.wavetable.len();
            }

            /// Set repeat to true or false
            pub fn set_repeat(&mut self, repeat: bool) {
                self.repeat = repeat;
            }

            /// Set the generator into "running" mode
            pub fn start(&mut self) {
                self.running = true;
            }

            /// Stop the generator (disable "running" mode)
            pub fn stop(&mut self) {
                self.running = false;
            }

            /// Resets the phase accumulator
            pub fn reset(&mut self) {
                self.phi = 0;
            }

            /// Resets the phase accumulator and set the generator into "running" mode
            pub fn reset_and_start(&mut self) {
                self.reset();
                self.running = true;
            }

            /// Stops the generator and reset the phase accumulator
            pub fn stop_and_reset(&mut self) {
                self.running = false;
                self.reset();
            }

            /// Returns whether the generator is running
            pub fn is_running(&self) -> bool {
                self.running
            }

            /// Sets the frequency
            pub fn set_mfreq(&mut self, mfreq: mHz) {
                self.mfreq = mfreq;
                self.update_delta_phi();
            }

            pub fn set_freq(&mut self, freq: Hz) {
                self.mfreq = freq.to_mHz();
                self.update_delta_phi();
            }

            pub fn set_msample_rate(&mut self, msample_rate: mHz) {
                self.msample_rate = msample_rate;
                self.update_alpha();
                self.update_delta_phi();
            }

            pub fn set_sample_rate(&mut self, sample_rate: Hz) {
                self.msample_rate = sample_rate.to_mHz();
                self.update_alpha();
                self.update_delta_phi();
            }
        }

        impl<T> Iterator for $name<T>
        where
            T: Copy,
        {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                self._next()
            }
        }
    )+)
}

// Generic i16
wavetable_oscillator!(WavetableOscillator);
impl WavetableOscillator<i16> {
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
            idx_max: SINE_I16.len(),
        };
        s.set_sample_rate(Hz(44100));
        s.set_freq(Hz(440));
        s
    }
}

// Sine i16
wavetable_oscillator!(SineOscillator);
impl SineOscillator<i16> {
    pub fn new() -> Self {
        let mut s = Self {
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
        };
        s.set_sample_rate(Hz(44100));
        s.set_freq(Hz(440));
        s
    }
}

use core::time::Duration;
use rodio::source::Source;
impl Source for WavetableOscillator<i16> {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.msample_rate.to_Hz().0;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::osc::luts::SINE_I16;

    #[test]
    fn test_wavetable_oscillator() {
        let mut osc = WavetableOscillator::<i16>::new();
        osc.set_wavetable(&SINE_I16);
        osc.set_freq(Hz(2));
        osc.set_sample_rate(Hz(100));
        osc.set_repeat(false);
        osc.start();
        for x in 0..110 {
            let _y = osc._next();
            match _y {
                Some(y) => println!("{}: {}\n", x, (y as f64) / (i32::MAX as f64)),
                None => {
                    println!("Generator stopped at {}", x);
                    break;
                }
            }
        }
    }
}
