use crate::codes::*;
use crate::interpolation::*;
use nalgebra::*;
use std::f64::consts::*;

#[allow(dead_code, unused_variables, non_snake_case)]
pub fn correlation_magnitude(
    test_delay: DVector<f64>,
    test_doppler: DVector<f64>,
    prn_number: usize,
    received_signal: DVector<f64>,
    carrier_freq: f64,
    time: DVector<f64>,
) -> DMatrix<f64> {
    let prn_code = gen_prn(prn_number, true, 1023).map(|val| val as f64);
    let temp = linspace(1., 1023., 1023);
    let (_, prn_code_interp) = interp_n_points(&temp, &prn_code, time.len());

    let mut I_cc = DMatrix::zeros(time.len(), test_doppler.len());
    let mut Q_cc = DMatrix::zeros(time.len(), test_doppler.len());
    for i in 0..test_doppler.len() {
        let I_interp = interp_I(carrier_freq, test_doppler[i], &time, &prn_code_interp);
        let Q_interp = interp_Q(carrier_freq, test_doppler[i], &time, &prn_code_interp);
        let (corr_I, _) = circ_corr(&received_signal, &I_interp, 1.);
        let (corr_Q, _) = circ_corr(&received_signal, &Q_interp, 1.);
        I_cc.set_column(i, &corr_I); // Assuming circcorr returns DVector<f64>
        Q_cc.set_column(i, &corr_Q); // Assuming circcorr returns DVector<f64>
    }

    I_cc.map(|x| x * x) + Q_cc.map(|x| x * x)
}

#[allow(non_snake_case)]
pub fn interp_I(
    carrier: f64,
    doppler_value: f64,
    time: &DVector<f64>,
    interp_prn: &DVector<f64>,
) -> DVector<f64> {
    let factor = 2.0 * PI * (carrier + doppler_value);

    let result: Vec<f64> = time
        .iter()
        .zip(interp_prn.iter())
        .map(|(&t, &prn)| (factor * t).cos() * prn)
        .collect();

    DVector::from_vec(result)
}

#[allow(non_snake_case)]
pub fn interp_Q(
    carrier: f64,
    doppler_value: f64,
    time: &DVector<f64>,
    interp_prn: &DVector<f64>,
) -> DVector<f64> {
    let factor = 2.0 * PI * (carrier + doppler_value);

    let result: Vec<f64> = time
        .iter()
        .zip(interp_prn.iter())
        .map(|(&t, &prn)| (factor * t).sin() * prn)
        .collect();

    DVector::from_vec(result)
}
