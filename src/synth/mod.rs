trait Synth<T> {
    fn init(&mut self);
    fn _next(&mut self) -> Option<T>;

    fn set_sample_rate(&mut self, sample_rate: u32);
    fn get_sample_rate(&self) -> u32;
    fn is_running(&self) -> bool;
}
