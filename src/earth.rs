use std::f64::consts::PI;

use nalgebra::*;

#[allow(unused_variables, dead_code)]
pub fn wgs84(unit: usize) -> (f64, f64, f64, f64, f64) {
    if unit == 0 {
        // meter
        (
            6378137.,
            6356752.31424518,
            0.081819190842621,
            0.003352810664747,
            298.257223563,
        )
    } else {
        // kilometer
        (
            6378.137,
            6356.75231424518,
            0.081819190842621,
            0.003352810664747,
            298.257223563,
        )
    }
    // (semi-major axis, semi-minor axis, eccentricity, flattening, inverse flattening)
}

#[allow(dead_code)]
pub fn ecef2geodetic(x: Vector3<f64>, unit: usize) -> Vector3<f64> {
    // returns lat long altitude in (deg, deg, unit)
    let maxiter = 1000 as usize;
    let (a, _, e, _, _) = wgs84(unit);

    let e2 = e.powi(2);

    let rad2deg = 180. / PI;

    let long = if x[0] == 0. && x[1] == 0. {
        0.
    } else {
        x[1].atan2(x[0]) * rad2deg
    };

    if x.norm() < 1e-3 {
        return Vector3::zeros(); // return all zeros when position is at the center of the earth
    }

    // initial guesses based on spherical/geocentric earth
    let rho = (x[0].powi(2) + x[1].powi(2)).sqrt();
    let mut lat = x[2].atan2(rho);
    let mut h = x.norm() - a;

    let mut rho_error = 1000.;
    let mut z_error = 1000.;

    // use newtons method to iterate on lat and h
    for _i in 0..maxiter {
        if rho_error.abs() < 1e-8 || z_error.abs() < 1e-8 {
            break;
        }
        let slat = lat.sin();
        let clat = lat.cos();
        let q = 1. - e2 * slat * slat;
        let r_n = a / q.sqrt();
        let drdl = r_n * e2 * slat * clat / q;

        rho_error = (r_n + h) * clat - rho;
        z_error = (r_n * (1. - e2) + h) * slat - x[2];

        // find jacobian
        let aa = drdl * clat - (r_n + h) * slat;
        let cc = (1. - e2) * (drdl * slat + r_n * clat);
        let bb = clat;
        let dd = slat;

        let invdet = 1. / (aa * dd - bb * cc);
        lat -= invdet * (dd * rho_error - bb * z_error);
        h -= invdet * (-cc * rho_error + aa * z_error);
    }
    Vector3::new(lat * rad2deg, long, h)
}
