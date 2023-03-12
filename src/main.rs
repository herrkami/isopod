use isopod::fx::filter::{StateVariableFilter, Q_MAX};
use isopod::osc::luts::SINE_I16;
use isopod::osc::noise::{WhiteNoise, LFSR};
use isopod::osc::wavetable::WavetableOscillator;
use isopod::synth::Synth;
use isopod::util::units::{mHz, Frequency, Hz};
use rodio::{OutputStream, Source};

fn main() {
    struct ProtoSynth {
        msample_rate: mHz,

        // Primitives
        noise: WhiteNoise<i16>,
        filter: StateVariableFilter,
    }

    impl Synth for ProtoSynth {
        fn new() -> Self {
            let mut s = Self {
                msample_rate: mHz(44_100_000),
                noise: WhiteNoise::<i16>::new(),
                filter: StateVariableFilter::new(),
            };

            s.filter.set_msample_rate(s.msample_rate);
            s.filter.set_mfreq(Hz(200).to_mHz());
            s.filter.set_q(Q_MAX / 8);
            s
        }

        fn _next(&mut self) -> Option<i16> {
            self.filter.feed(self.noise.next().unwrap() / 4);
            Some(self.filter.get_lp())
        }

        fn get_sample_rate(&self) -> Hz {
            self.msample_rate.to_Hz()
        }

        fn set_sample_rate(&mut self, sample_rate: Hz) {
            self.msample_rate = sample_rate.to_mHz();
        }
    }

    impl Iterator for ProtoSynth {
        type Item = i16;
        fn next(&mut self) -> Option<Self::Item> {
            self._next()
        }
    }

    impl Source for ProtoSynth {
        fn channels(&self) -> u16 {
            1
        }

        fn sample_rate(&self) -> u32 {
            self.get_sample_rate().0
        }

        fn current_frame_len(&self) -> Option<usize> {
            None
        }

        fn total_duration(&self) -> Option<std::time::Duration> {
            None
        }
    }

    let synth = ProtoSynth::new();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(synth.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(3));

    // // Debug wavetable oscillator
    // let mut osc = WavetableOscillator::<i16>::new();
    // osc.set_wavetable(&SINE_I16);
    // osc.set_freq(Hz(2));
    // osc.set_sample_rate(Hz(100));
    // osc.set_repeat(false);
    // osc.start();
    // for x in 0..110 {
    //     let _y = osc._next();
    //     match _y {
    //         Some(y) => println!("{}: {}\n", x, (y as f64) / (i16::MAX as f64)),
    //         None => {
    //             println!("Generator stopped at {}", x);
    //             break;
    //         }
    //     }
    // }

    // // Debug LFSR
    // let mut lfsr16 = LFSR::<u16>::default();
    // let mut lfsr32 = LFSR::<u32>::default();
    // for i in 0..33 {
    //     println!("lfsr16: {:?}", lfsr16.next());
    //     println!("lfsr32: {:?}", lfsr32.next());
    // }
}
