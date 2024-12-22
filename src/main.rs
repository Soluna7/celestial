use std::*;

const ORBITAL_PERIOD: f64 = 365.25; //measured in rotations per complete revolution (days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let time: f64 = 52.0;
    let latitude: f64 = 37.2; //between -90 and 90
    let declination = declination(OBLIQUITY, ORBITAL_PERIOD, time);
    let hour_angle = hour_angle(latitude, declination);
    println!("HOUR ANGLE = {hour_angle}");
}

fn arcsin_bandpass(mut num: f64) -> f64 {
    if num < -1.0 {
        num = 180.0;
    } else if num > 1.0 {
        num = 0.0;
    } else {
        num = num.asin();
    }
    return num;
}

fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = obliquity * ecliptic_longitude.sin();
    return declination;
}

fn hour_angle(latitude: f64, declination: f64) -> f64 {
    return arcsin_bandpass(-latitude.tan() * declination.tan());
}
