use std::*;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees
const TAU: f64 = 6.2831853071;

fn main() {
    let time: f64 = 357.0; //day in year (mod ORBITAL_PERIOD)
    let latitude: f64 = 33.3532905478; //between -90 and 90
    let longitude: f64 = -97.8763568637; //between -180 and 180
    let _altitude: f64 = 302.0; //altitude from radius (sea level) in meters
    let declination = declination(OBLIQUITY, ORBITAL_PERIOD, time);
    let hour_angle = hour_angle(latitude, declination);
    let max_elevation_angle = max_elevation_angle(latitude, declination);
    let elevation_angle = elevation_angle(OBLIQUITY, ORBITAL_PERIOD, latitude, longitude, time);
    println!("DECLINATION = {declination}");
    println!("HOUR ANGLE = {hour_angle}");
    println!("MAX ELEVATION ANGLE = {max_elevation_angle}");
    println!("ELEVATION ANGLE = {elevation_angle}");
}

fn arcsin_bandpass(mut num: f64) -> f64 {
    if num < -1.0 {
        num = -90.0;
    } else if num > 1.0 {
        num = 90.0;
    } else {
        num = num.asin().to_degrees();
    }
    return num;
}

fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = obliquity * ecliptic_longitude.to_radians().sin();
    return declination;
}

fn hour_angle(latitude: f64, declination: f64) -> f64 {
    return -arcsin_bandpass(latitude.to_radians().tan() * declination.to_radians().tan());
}

fn max_elevation_angle(latitude: f64, declination: f64) -> f64 {
    let max_elevation_angle =
        latitude.to_radians().sin()*declination.to_radians().sin()+
        latitude.to_radians().cos()*declination.to_radians().cos();
    let max_elevation_angle = max_elevation_angle.asin().to_degrees();
    return max_elevation_angle;
}

fn elevation_angle(obliquity:f64, orbital_period:f64, latitude:f64, longitude:f64, time:f64) -> f64 {
    let declination = declination(obliquity, orbital_period, time+longitude/orbital_period);
    let maximum_elevation_angle = max_elevation_angle(latitude, declination);
    let minimum_elevation_angle = max_elevation_angle(-latitude, declination);
    let oscillator = (TAU * (time+(longitude/orbital_period))) % TAU;
    let elevation_angle = 
        (maximum_elevation_angle - minimum_elevation_angle)/2.0 +
        (maximum_elevation_angle + minimum_elevation_angle)/2.0 * oscillator.sin();
    return elevation_angle;
}

fn azimuth_angle(obliquity:f64, orbital_period:f64, latitude:f64, longitude:f64, time:f64) -> f64 {
    let declination = declination(obliquity, orbital_period, time + longitude/orbital_period);
    let hour_angle = hour_angle(latitude, declination);
    let azimuth_angle =
        (declination.to_radians().sin()*latitude.to_radians().cos() + 
        declination.to_radians().cos()*latitude.to_radians().sin()*
        latitude.to_radians().cos()
        )/declination.to_radians().cos();
    return azimuth_angle.to_degrees();
}
