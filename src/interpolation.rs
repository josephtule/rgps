use nalgebra::*;

pub fn interp_n_points<T: Copy + Into<f64>>(
    x: &DVector<T>,
    y: &DVector<T>,
    n: usize,
) -> (DVector<f64>, DVector<f64>) {
    let xi: DVector<f64> = DVector::from_vec(
        (0..n)
            .map(|i| {
                x[0].into() + (x[x.len() - 1].into() - x[0].into()) * (i as f64) / (n as f64 - 1.0)
            })
            .collect(),
    );
    let yi: DVector<f64> = DVector::from_vec(xi.iter().map(|&xi| prev_interp(x, y, xi)).collect());
    (xi, yi)
}

pub fn nn_interp<T: Copy + Into<f64>>(x: &DVector<T>, y: &DVector<T>, xi: f64) -> f64 {
    // good enough for prn interpolation
    // Find the closest value in 'x' to 'xi'
    let closest_idx = (0..x.len())
        .min_by(|&i, &j| {
            let dist_i: f64 = (x[i].into() - xi).abs();
            let dist_j: f64 = (x[j].into() - xi).abs();
            dist_i
                .partial_cmp(&dist_j)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    y[closest_idx].into()
}

pub fn prev_interp<T: Copy + Into<f64>>(x: &DVector<T>, y: &DVector<T>, xi: f64) -> f64 {
    if xi <= x[0].into() {
        return y[0].into();
    }

    for i in 1..x.len() {
        if xi < x[i].into() {
            return y[i - 1].into();
        } else if (xi - x[i - 1].into()).abs() < f64::EPSILON {
            return y[i - 1].into();
        }
    }

    y[x.len() - 1].into()
}

pub fn linspace<T: Copy + Into<f64>>(start: T, end: T, n: usize) -> DVector<f64> {
    let start_f64 = start.into();
    let end_f64 = end.into();

    let values: Vec<f64> = (0..n)
        .map(|i| start_f64 + (end_f64 - start_f64) * (i as f64) / (n as f64 - 1.0))
        .collect();

    DVector::from_vec(values)
}

pub fn range_vec<T: Copy + Into<f64>>(start: T, end: T, step: T) -> DVector<f64> {
    let mut step_val = step.into();
    if end.into() < start.into() {
        step_val = -step_val;
    }
    let mut values = Vec::new();
    let mut val = start.into();
    let end_val: f64 = end.into();

    while val <= end_val {
        values.push(val);
        val += step_val;
    }

    DVector::from_vec(values)
}
