use isopoda::osc::luts::SINE_I16;
use isopoda::osc::noise::LFSR;
use isopoda::osc::wavetable::WavetableOscillator;
use isopoda::util::units::Hz;

fn main() {
    // Debug wavetable oscillator
    let mut osc = WavetableOscillator::<i16>::new();
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

    // // Debug LFSR
    // let mut lfsr16 = LFSR::<u16>::default();
    // let mut lfsr32 = LFSR::<u32>::default();
    // for i in 0..33 {
    //     println!("lfsr16: {:?}", lfsr16.next());
    //     println!("lfsr32: {:?}", lfsr32.next());
    // }
}
