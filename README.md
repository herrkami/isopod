# Minimal audio synthesis library for non-FPU architectures

## Architecture
The central component in isopoda is the `Synth` structure which implements the
`Iterator` trait and connects the signal chain to the audio stream handler.
Depending on the hardware setup it can also implement the `rodio::source::Source` trait. The `next()` function in `Synth` returns the
next sample and updates the internal state. It also contains the definition
of the signal chain:
```rust
// Sawtooth signal with frequency modulated low pass filter
fn next() -> Option<i16> {
    self.sin.set_freq(Hertz(5));
    let f = Hertz(100) + Hertz(30) * self.sin.next();
    self.low_pass.set_freq(f);
    Some(self.low_pass.next(self.sawtooth.next()))
}
```

isopoda provides a set of simple and efficient DSP primitives which can be
chained, added and multiplied together. They are grouped into three
categories, oscillators (`osc`), effects (`fx`), and envelopes (`env`).

## Primitives
Those are the primitives in isopoda. The checkboxes indicate whether they are implemented already.

- Oscillators (Osc)
    - Wavetable oscillators
        - [x] Sine
        - [x] Arbitrary
    - Algorithmic
        - [ ] Saw
        - [ ] Square
        - [ ] Chaos
    - Noise
        - [ ] WhiteNoise
        - [ ] PinkNoise
        - [ ] BitFlipNoise
        - [ ] CrackleNoise
- Effects (FX)
    - Linear filters (small buffer sizes)
        - [ ] LPF
        - [ ] BPF
        - [ ] HPF
    - Delays and Reverbs (requires large buffer )
        - [ ] Delay
        - [ ] Reverb
- Envelops (Env)
    - Envelops
        - [x] LinExp (parametrized piecewise linear exponential decay)
        - [ ] AD
        - [ ] ADSR
        - [ ] Arbitrary


## Signal formats

There are three integer and one float signal format in isopoda.

### i32
Most common data type on MCUs. Common and efficient also on CPUs. Dynamic
range of 192 dB is more than enough for most purposes.

### i16
Dynamic range of 96 dB is adequate for most purposes.


### i8
Constrained dynamic range of 48 dB is only relevant for lo-fi applications on 8-bit microcontrollers.


### f32
Only usable if your architecture has an FPU. Dynamic range extends to 1529 dB
which is far above audio requirements. Some computations are more efficient with
`f32` and FPU, others are less efficient as compared to integer operations.


## Other formats

### frequency
Frequencies are represented in Hz oder mHz and implemented as `u32`. For some
necessary internal operations frequencies have to be multiplied together and
hence some `i64`-operations occur. Truncating the precision to `u16` or so
for increased performance would reduce to frequency resolution to almost 1 Hz
and is therefore inacceptable.
