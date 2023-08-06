use nalgebra::*;
use reqwest::blocking::get;
use std::collections::HashMap;
use std::f64::consts::*;
use std::fs::File;
use std::io::copy;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct RinexNavHeader {
    ion_alpha: [f64; 4],
    ion_beta: [f64; 4],
    delta_utc: (f64, f64, i32, i32),
    leap_seconds: i32,
}

#[derive(Clone, Debug)]
pub struct SatelliteData {
    prn: i32,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: f64,
    sv_clock_bias: f64,
    sv_clock_drift: f64,
    sv_clock_drift_rate: f64,
    iode: f64,
    crs: f64,
    delta_n: f64,
    m0: f64,
    cuc: f64,
    e: f64,
    cus: f64,
    sqrt_a: f64,
    toe: f64,
    cic: f64,
    raan: f64,
    cis: f64,
    i0: f64,
    crc: f64,
    aop: f64,
    raandot: f64,
    idot: f64,
    l2_codes: f64,
    gps_week: f64,
    l2_p_data_flag: f64,
    sv_accuracy: f64,
    sv_health: f64,
    tgd: f64,
    iodc: f64,
    transmission_time: f64,
    fit_interval: f64,
}

#[allow(unused_variables)]
pub fn rinex2_nav_sv(filename: &str, prn: usize, gps_time: f64) -> Result<SatelliteData, Error> {
    // gps_time is time (sec) of week
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file).lines();
    let prn_string = format!("{: >2}", prn);

    let mut iode = 0.;
    let mut crs = 0.;
    let mut delta_n = 0.;
    let mut m0 = 0.;
    let mut cuc = 0.;
    let mut e = 0.;
    let mut cus = 0.;
    let mut sqrt_a = 0.;
    let mut toe = 0.;
    let mut cic = 0.;
    let mut raan = 0.;
    let mut cis = 0.;
    let mut i0 = 0.;
    let mut crc = 0.;
    let mut aop = 0.;
    let mut raandot = 0.;
    let mut idot = 0.;
    let mut l2_codes = 0.;
    let mut gps_week = 0.;
    let mut l2_p_data_flag = 0.;
    let mut sv_accuracy = 0.;
    let mut sv_health = 0.;
    let mut tgd = 0.;
    let mut iodc = 0.;
    let mut transmission_time = 0.;
    let mut fit_interval = 0.;

    let mut sat = SatelliteData {
        prn: prn as i32,
        year: 0,
        day: 0,
        month: 0,
        hour: 0,
        minute: 0,
        second: 0.,
        sv_clock_bias: 0.,
        sv_clock_drift: 0.,
        sv_clock_drift_rate: 0.,
        iode,
        crs,
        delta_n,
        m0,
        cuc,
        e,
        cus,
        sqrt_a,
        toe,
        cic,
        cis,
        i0,
        crc,
        aop,
        raan,
        raandot,
        idot,
        l2_p_data_flag,
        l2_codes,
        gps_week,
        sv_accuracy,
        sv_health,
        tgd,
        iodc,
        transmission_time,
        fit_interval,
    };

    let mut found_prn = false;

    while let Some(line_result) = reader.next() {
        let line = line_result?;
        if line[0..=1] == prn_string {
            found_prn = true;
        }
        if found_prn {
            // line 0
            let tokens = [
                &line[3..=4],   // year
                &line[5..=7],   // month
                &line[8..=10],  // day
                &line[11..=13], // hour
                &line[14..=16], // minute
                &line[17..=21], // second
                &line[22..=40], // SV clock bias
                &line[41..=59], // SV clock drift
                &line[60..=78], // SV clock drift rate
            ];

            let year = tokens[0].replace(" ", "").parse::<i32>().unwrap();
            let month = tokens[1].replace(" ", "").parse::<i32>().unwrap();
            let day = tokens[2].replace(" ", "").parse::<i32>().unwrap();
            let hour = tokens[3].replace(" ", "").parse::<i32>().unwrap();
            let minute = tokens[4].replace(" ", "").parse::<i32>().unwrap();
            let second = tokens[5].replace(" ", "").parse::<f64>().unwrap();
            let sv_clock_bias = tokens[6]
                .replace(" ", "")
                .replace("D", "e")
                .parse::<f64>()
                .unwrap();
            let sv_clock_drift = tokens[7]
                .replace(" ", "")
                .replace("D", "e")
                .parse::<f64>()
                .unwrap();
            let sv_clock_drift_rate = tokens[8]
                .replace(" ", "")
                .replace("D", "e")
                .parse::<f64>()
                .unwrap();

            if let Some(line_result) = reader.next() {
                // line 1
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                iode = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                crs = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                delta_n = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                m0 = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }
            if let Some(line_result) = reader.next() {
                // line 2
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                cuc = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                e = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                cus = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                sqrt_a = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }

            if let Some(line_result) = reader.next() {
                // line 3
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                toe = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                cic = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                raan = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                cis = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }

            if let Some(line_result) = reader.next() {
                // line 4
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                i0 = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                crc = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                aop = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                raandot = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }

            if let Some(line_result) = reader.next() {
                // line 5
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                idot = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                l2_codes = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                gps_week = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                l2_p_data_flag = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }

            if let Some(line_result) = reader.next() {
                // line 2
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                sv_accuracy = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                sv_health = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                tgd = tokens[2]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                iodc = tokens[3]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }

            if let Some(line_result) = reader.next() {
                // line 7
                let line = line_result?;
                let tokens = [&line[3..22], &line[22..41], &line[41..60], &line[60..79]];
                transmission_time = tokens[0]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
                fit_interval = tokens[1]
                    .replace(" ", "")
                    .replace("D", "e")
                    .parse::<f64>()
                    .unwrap();
            }
            // change sat no matter what, in gps_time is outside of of updates
            sat = SatelliteData {
                prn: prn as i32,
                year,
                day,
                month,
                hour,
                minute,
                second,
                sv_clock_bias,
                sv_clock_drift,
                sv_clock_drift_rate,
                iode,
                crs,
                delta_n,
                m0,
                cuc,
                e,
                cus,
                sqrt_a,
                toe,
                cic,
                cis,
                i0,
                crc,
                aop,
                raan,
                raandot,
                idot,
                l2_p_data_flag,
                l2_codes,
                gps_week,
                sv_accuracy,
                sv_health,
                tgd,
                iodc,
                transmission_time,
                fit_interval,
            };

            if gps_time == 0. || (gps_time < (2. * 60. * 60. + toe) && gps_time >= toe) {
                // gps_time is within two hours of toe
                break; // exit loop once correct epoch is found
            } else {
                found_prn = false;
                continue;
            }
        }
        if gps_time > 24. * 60. * 60. {
            panic!("GPS time is not within the day of the file given.")
        }
    }

    Ok(sat)
}

pub fn rinex2_nav_header(filename: &str) -> Result<RinexNavHeader, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut ion_alpha = [0.0; 4];
    let mut ion_beta = [0.0; 4];
    let mut delta_utc = (0.0, 0.0, 0, 0);
    let mut leap_seconds = 0;

    for line in reader.lines() {
        let line = line?;

        if line.contains("ION ALPHA") {
            let tokens: Vec<_> = line
                .replace("ION ALPHA", "")
                .split_whitespace()
                .map(|x| x.replace("D", "e").parse::<f64>().unwrap())
                .collect();
            ion_alpha.copy_from_slice(&tokens[0..4]);
        } else if line.contains("ION BETA") {
            let tokens: Vec<_> = line
                .replace("ION BETA", "")
                .split_whitespace()
                .map(|x| x.replace("D", "e").parse::<f64>().unwrap())
                .collect();
            ion_beta.copy_from_slice(&tokens[0..4]);
        } else if line.contains("DELTA-UTC") {
            // have to parse manually because there can be a
            // negative sign between the first and second items
            // delta_utc = ();
            let tokens = line.replace("DELTA-UTC: A0,A1,T,W", "").replace("D", "e");
            let tokens: Vec<_> = tokens.split_whitespace().collect();
            let a0 = tokens[0][0..18].parse::<f64>();
            let a1 = tokens[0][19..].parse::<f64>();
            delta_utc.0 = a0.unwrap().clone();
            delta_utc.1 = a1.unwrap().clone();
            delta_utc.2 = tokens[1].parse::<i32>().unwrap();
            delta_utc.3 = tokens[2].parse::<i32>().unwrap();
        } else if line.contains("LEAP SECONDS") {
            leap_seconds = line
                .replace("LEAP SECONDS", "")
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
        } else if line.contains("END OF HEADER") {
            break;
        }
    }

    Ok(RinexNavHeader {
        ion_alpha,
        ion_beta,
        delta_utc,
        leap_seconds,
    })
}

// 1 Eccentricity:                             e
// 2 Time of Applicability(s):                 TOE
// 3 Orbital Inclination(rad):                 I_0
// 4 Rate of Right Ascen(r/s):                 OMEGA_DOT
// 5 SQRT(A) (m^1/2):                          SQRT_A
// 6 Right Ascen at TOA(rad):                  OMEGA_0
// 7 Argument of Perigee(rad):                 OMEGA
// 8 Mean Anom(rad):                           M_0
// 9 mean motion diff(r/s):                    DELTA_N
// 10 Rate of inclin (r/s):                    I_DOT
// 11 lat cosine ampl (r):                     CUC
// 12 Lat sine ampl   (r):                     CUS
// 13 radius cos ampl (m):                     CRC
// 14 radius sin ampl (m):                     CRS
// 15 inclin cos ampl(r):                      CIC
// 16 inclin sin ampl(r):                      CIS
// 17 t_gd:                                    T_GD
// 18 t_oc:                                    T_OC
// 19 Af0(s):                                  af0
// 20 Af1(s/s):                                af1
// 21 Af2(s/s/s):                              af2

pub fn read_sp3() {
    todo!()
}

// almanacs (low precision): https://celestrak.org/GPS/almanac/Yuma/2023/
// ephemeris (med precision - daily): https://cddis.nasa.gov/archive/gnss/data/daily
// ephemeris (med precision - hourly): https://cddis.nasa.gov/archive/gnss/data/daily
// igs precise ephemeris (high precision):
//
//
//
// pub fn parse_rinex_nav(
//     filename: &str,
//     gps_time: f64,
//     target_sv: &[i32],
// ) -> Result<(RinexNavHeader, Vec<SatelliteData>), Error> {
//     let data = File::open(filename).unwrap();
//     let mut reader = BufReader::new(data);
//     // Parse Header
//     let ion_alpha: Vec<f64> = reader
//         .by_ref()
//         .lines()
//         .next()
//         .ok_or(Error::new(
//             std::io::ErrorKind::UnexpectedEof,
//             "Unexpected end of file",
//         ))?
//         .unwrap()
//         .split_whitespace()
//         .map(|x| x.replace("D", "e").parse::<f64>().unwrap())
//         .collect();
//     let ion_beta: Vec<f64> = reader
//         .by_ref()
//         .lines()
//         .next()
//         .ok_or(Error::new(
//             std::io::ErrorKind::UnexpectedEof,
//             "Unexpected end of file",
//         ))?
//         .unwrap()
//         .split_whitespace()
//         .map(|x| x.replace("D", "e").parse::<f64>().unwrap())
//         .collect();
//
//     let delta_utc_data: Vec<String> = reader
//         .by_ref()
//         .lines()
//         .next()
//         .ok_or(Error::new(
//             std::io::ErrorKind::UnexpectedEof,
//             "Unexpected end of file",
//         ))?
//         .unwrap()
//         .split_whitespace()
//         .map(|x| x.replace("D", "e"))
//         .collect();
//
//     let delta_utc = (
//         delta_utc_data[0].parse::<f64>().unwrap(),
//         delta_utc_data[1].parse::<f64>().unwrap(),
//         delta_utc_data[2].parse::<i32>().unwrap(),
//         delta_utc_data[3].parse::<i32>().unwrap(),
//     );
//     let leap_seconds = reader
//         .by_ref()
//         .lines()
//         .next()
//         .ok_or(Error::new(
//             std::io::ErrorKind::UnexpectedEof,
//             "Unexpected end of file",
//         ))?
//         .unwrap()
//         .trim()
//         .parse::<i32>()
//         .unwrap();
//
//     let header = RinexNavHeader {
//         ion_alpha: [ion_alpha[0], ion_alpha[1], ion_alpha[2], ion_alpha[3]],
//         ion_beta: [ion_beta[0], ion_beta[1], ion_beta[2], ion_beta[3]],
//         delta_utc,
//         leap_seconds,
//     };
//
//     let mut satellites = Vec::new();
//     let mut first_data_for_sat = HashMap::new();
//
//     while let Some(line) = reader.by_ref().lines().next() {
//         let line = line?;
//         let prn: i32 = line.split_whitespace().next().unwrap().parse().unwrap();
//
//         // If the current SV number is not in target_sv, skip its data
//         if !target_sv.contains(&prn) {
//             for _ in 0..7 {
//                 reader.by_ref().lines().next();
//             }
//             continue;
//         }
//
//         // Continue processing for selected SVs
//         let epoch_data: Vec<&str> = line.split_whitespace().skip(1).collect();
//         let year = epoch_data[0].parse::<i32>().unwrap();
//         let month = epoch_data[1].parse::<i32>().unwrap();
//         let day = epoch_data[2].parse::<i32>().unwrap();
//         let hour = epoch_data[3].parse::<i32>().unwrap();
//         let minute = epoch_data[4].parse::<i32>().unwrap();
//         let second = epoch_data[5].parse::<f64>().unwrap();
//         let sv_clock_values: Vec<f64> = line
//             .split_whitespace()
//             .skip(7)
//             .map(|x| x.replace("D", "e").parse::<f64>().unwrap())
//             .collect();
//         let sv_clock_bias = sv_clock_values[0];
//         let sv_clock_drift = sv_clock_values[1];
//         let sv_clock_drift_rate = sv_clock_values[2];
//
//         let mut orbit_values = vec![];
//         for _ in 0..7 {
//             orbit_values.extend(
//                 reader
//                     .by_ref()
//                     .lines()
//                     .next()
//                     .ok_or(Error::new(
//                         std::io::ErrorKind::UnexpectedEof,
//                         "Unexpected end of file",
//                     ))?
//                     .unwrap()
//                     .split_whitespace()
//                     .skip(1)
//                     .map(|x| x.replace("D", "e").parse::<f64>().unwrap()),
//             );
//         }
//
//         // Populate the SatelliteData struct
//         let current_data = SatelliteData {
//             prn,
//             year,
//             month,
//             day,
//             hour,
//             minute,
//             second,
//             sv_clock_bias,
//             sv_clock_drift,
//             sv_clock_drift_rate,
//             iode: orbit_values[0],
//             crs: orbit_values[1],
//             delta_n: orbit_values[2],
//             m0: orbit_values[3],
//             cuc: orbit_values[4],
//             e: orbit_values[5],
//             cus: orbit_values[6],
//             sqrt_a: orbit_values[7],
//             toe: orbit_values[8],
//             cic: orbit_values[9],
//             omega0: orbit_values[10],
//             cis: orbit_values[11],
//             i0: orbit_values[12],
//             crc: orbit_values[13],
//             omega: orbit_values[14],
//             omega_dot: orbit_values[15],
//             idot: orbit_values[16],
//             codes_on_l2_channel: orbit_values[17],
//             gps_week: orbit_values[18],
//             l2_p_data_flag: orbit_values[19],
//             sv_accuracy: orbit_values[20],
//             sv_health: orbit_values[21],
//             tgd: orbit_values[22],
//             iodc: orbit_values[23],
//             transmission_time_of_message: orbit_values[24],
//             fit_interval: orbit_values[25],
//         };
//
//         // Save satellite data if the gps_time is valid
//         if gps_time == 0.0 {
//             if !first_data_for_sat.contains_key(&prn) {
//                 first_data_for_sat.insert(prn, current_data.clone());
//                 satellites.push(current_data);
//             }
//         } else if current_data.transmission_time_of_message <= gps_time {
//             satellites.push(current_data);
//         }
//     }
//
//     Ok((header, satellites))
// }
