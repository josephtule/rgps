use nalgebra::*;

use std::fs::File;
use std::io::Read;

mod codes;
mod doppler;
mod earth;
mod interpolation;

use codes::*;
use doppler::*;
use earth::*;
use interpolation::*;

#[allow(unused_variables, non_snake_case)]
fn main() {
    // let fs = 10e6; // sample frequency, Hz
    // let fl1 = 1575.42e6; // L1 carrier frequency, Hz
    // let fl2 = 1227.6e6; // L2 carrier frequency, Hz
    // let fi = 2.716e6; // intermediate carrier frequency, Hz
    // let Ts = 1. / fs; // sample time, sec
    // let Td = 1e-3; // datalength (1023 chips), sec
    // let Ts_vec = linspace(0., Td, (Td * fs) as usize);
    // let received_signal = read_bin_to_dvector("gpstestdata.bin")
    //     .expect("Error in reading bin or converting to vector");
    //
    // println!("{}", received_signal.fixed_rows::<20>(0));
    //
    // let fD_test = range_vec(-5000, 5000, 100);
    // let t_test = range_vec(-5000, 4999, 1) * Ts;
    // // println!("{}", t_test.fixed_rows::<20>(0));
    //
    // correlation_magnitude(t_test, fD_test, 1, received_signal, fi, Ts_vec);
    //
    // let test = gen_prn(1, true, 1050, -10);
    // println!("{}", test.fixed_rows::<10>(0));

    let r = Vector3::new(-2694685.473, -4293642.366, 3857878.924);
    let geodetic = ecef2geodetic(r, 0);
    println!("{}", geodetic);
}

fn read_bin_to_dvector(filename: &str) -> Result<DVector<f64>, std::io::Error> {
    // Open the binary file
    let mut file = File::open(filename)?;

    // Read the entire content of the file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Convert the bytes into f64 numbers
    let data: Vec<f64> = unsafe {
        let ptr = buffer.as_ptr() as *const f64;
        std::slice::from_raw_parts(ptr, buffer.len() / std::mem::size_of::<f64>()).to_vec()
    };

    Ok(DVector::from_vec(data))
}
