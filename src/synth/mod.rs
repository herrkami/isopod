use crate::util::units::Hz;

pub trait Synth {
    fn new() -> Self;
    fn _next(&mut self) -> Option<i16>;

    fn get_sample_rate(&self) -> Hz;
    fn set_sample_rate(&mut self, sample_rate: Hz);
}
