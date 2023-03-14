// Build file to create wavetables for the various synthesizers

use std::f64::consts::PI;
use std::fmt::Display;
use std::{fs, path::Path};

/// Directory used to store wavetable files generated during compile time
const WAVETABLES_DIRECTORY: &str = "src/osc/luts/";

struct Wavetable<T> {
    table: Vec<T>,
    len: usize,
}

trait MaxAmp {
    fn max_amp(&self) -> f64;
    fn cast(&self, value: f64) -> Self;
    fn type_string(&self) -> String;
}

impl MaxAmp for i32 {
    fn max_amp(&self) -> f64 {
        i32::MAX as f64
    }
    fn cast(&self, value: f64) -> Self {
        value as Self
    }
    fn type_string(&self) -> String {
        String::from("i32")
    }
}

impl MaxAmp for i16 {
    fn max_amp(&self) -> f64 {
        i16::MAX as f64
    }
    fn cast(&self, value: f64) -> Self {
        value as Self
    }
    fn type_string(&self) -> String {
        String::from("i16")
    }
}

impl MaxAmp for i8 {
    fn max_amp(&self) -> f64 {
        i8::MAX as f64
    }
    fn cast(&self, value: f64) -> Self {
        value as Self
    }
    fn type_string(&self) -> String {
        String::from("i8")
    }
}

fn generate_wavetable_sine<T: Sized + MaxAmp>(t: T, len: usize) -> Wavetable<T> {
    let mut sine_table = Wavetable::<T> {
        table: Vec::with_capacity(len),
        len: len,
    };

    for i in 0..len {
        let mut _sine: f64 = (2_f64 * PI * ((i as f64) / (len as f64))).sin();

        _sine *= t.max_amp();
        sine_table.table.push(t.cast(_sine.round()));
    }
    sine_table
}

fn generate_wavetable_exp<T: Sized + MaxAmp>(t: T, len: usize) -> Wavetable<T> {
    let mut exp_table = Wavetable::<T> {
        table: Vec::with_capacity(len),
        len: len,
    };

    for i in 0..len {
        // let mut _exp: f64 = ((i as f64)/(len as f64)).exp();
        let mut _exp: f64 = t.max_amp().powf(1_f64 - ((i as f64) / (len as f64)));

        exp_table.table.push(t.cast(_exp.round()));
    }
    exp_table
}

fn write_table_to_file<T: Sized + MaxAmp + Display>(wavetable: Wavetable<T>, fname: &str) {
    let type_string = wavetable.table[0].type_string();
    let wave_string = fname.split("_").collect::<Vec<&str>>()[0].to_uppercase();

    // Create string
    let mut array_string = String::from("pub static ");

    // Construct a name like SINE_i32 or EXP_i8 etc.
    array_string.push_str(&wave_string);
    array_string.push_str("_");
    array_string.push_str(&type_string.to_uppercase());

    // Specify array type and length
    array_string.push_str(": [");
    array_string.push_str(&type_string);
    array_string.push_str("; ");
    array_string.push_str(wavetable.len.to_string().as_str());

    // Fill array
    array_string.push_str("] = [");
    for s in &wavetable.table {
        array_string.push_str("");
        array_string.push_str(s.to_string().as_str());
        array_string.push_str(", ");
    }
    array_string.push_str("];\r\n");

    // It it's an exponential, we'll add a time constant tau where the exponential drops to 1/e.
    // tau is given in terms of the closest integer index
    if wave_string == "EXP" {
        let t = &wavetable.table[0];
        let tau = t.cast(((wavetable.len as f64) / t.max_amp().ln()).round());
        let max = t;
        array_string.push_str("\r\n");

        array_string.push_str("pub const ");
        array_string.push_str(&wave_string);
        array_string.push_str("_");
        array_string.push_str(&type_string.to_uppercase());
        array_string.push_str("_TAU: usize = ");
        array_string.push_str(tau.to_string().as_str());
        array_string.push_str(";\r\n");

        array_string.push_str("pub const ");
        array_string.push_str(&wave_string);
        array_string.push_str("_");
        array_string.push_str(&type_string.to_uppercase());
        array_string.push_str("_MAX: usize = ");
        array_string.push_str(max.to_string().as_str());
        array_string.push_str(";\r\n");
    }

    // write the string to a file. OUT_DIR environment variable is defined by cargo
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join(fname);
    // fs::write(&dest_path, array_string).unwrap();

    let dest_path = Path::new(&WAVETABLES_DIRECTORY).join(fname);
    fs::write(&dest_path, array_string).unwrap();
}

fn main() {
    let sine_table8 = generate_wavetable_sine(0_i8, 256);
    write_table_to_file(sine_table8, "sine_i8.rs");

    let sine_table16 = generate_wavetable_sine(0_i16, 1024);
    write_table_to_file(sine_table16, "sine_i16.rs");

    let sine_table32 = generate_wavetable_sine(0_i32, 1024);
    write_table_to_file(sine_table32, "sine_i32.rs");

    let exp_table8 = generate_wavetable_exp(0_i8, 256);
    write_table_to_file(exp_table8, "exp_i8.rs");

    let exp_table16 = generate_wavetable_exp(0_i16, 1024);
    write_table_to_file(exp_table16, "exp_i16.rs");

    let exp_table32 = generate_wavetable_exp(0_i32, 1024);
    write_table_to_file(exp_table32, "exp_i32.rs");
}
