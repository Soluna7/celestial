use std::*;

const ORBITAL_PERIOD: f64 = 365.25; //measured in rotations per complete revolution (days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    println!("Hello, world!");
}

fn arccos_bandpass(mut num: f64) -> f64 {
    if num < -1.0 {
        num = 180.0;
    } else if num > 1.0 {
        num = 0.0;
    } else {
        num = num.acos();
    }
    return num;
}

fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = obliquity * ecliptic_longitude.sin();
    return declination;
}
