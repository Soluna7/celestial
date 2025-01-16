use crate::coordinates;
use crate::world;
use crate::world::*;
use std::f64::consts::TAU;

pub trait Solar {
    fn declination(&self, time: f64) -> f64;
    fn ecliptic_longitude(&self, time: f64) -> f64;
    fn mean_anomaly(&self, time: f64) -> f64;
    fn annual_periapsis(&self) -> f64;
    fn periaptic_longitude(&self, time: f64) -> f64;
    fn equation_of_time(&self, time: f64) -> f64;
}

impl Solar for World {
    fn declination(&self, time: f64) -> f64 {
        self.obliquity * self.ecliptic_longitude(time).sin()
    }
    fn ecliptic_longitude(&self, time: f64) -> f64 {
        (TAU * time) / self.orbital_period
    }
    fn mean_anomaly(&self, time: f64) -> f64 {
        TAU * (time - self.annual_periapsis()) / self.orbital_period
    }
    fn annual_periapsis(&self) -> f64 {
        self.orbital_period * self.periaptic_period
    }
    fn periaptic_longitude(&self, time: f64) -> f64 {
        self.ecliptic_longitude(time) * self.mean_anomaly(time)
    }
    fn equation_of_time(&self, time: f64) -> f64 {
        let obliquity = self.obliquity;
        let eccentricity = self.eccentricity;
        let mean_anomaly = self.mean_anomaly(time);
        let periaptic_longitude = self.periaptic_longitude(time);
        let first_pass = -2.0 * eccentricity * mean_anomaly.sin()
            + (obliquity / 2.0).tan().powi(2) * (2.0 * (mean_anomaly + periaptic_longitude)).sin();
        let second_pass = -1.25 * eccentricity.powi(2) * (2.0 * mean_anomaly).sin()
            + 4.0
                * eccentricity
                * (obliquity / 2.0).tan().powi(2)
                * mean_anomaly.sin()
                * (2.0 * (mean_anomaly + periaptic_longitude)).cos()
            - 0.5
                * (obliquity / 2.0).tan().powi(4)
                * (2.0 * (2.0 * mean_anomaly + periaptic_longitude));
        return first_pass + second_pass;
    }
}

fn hour_angle(time: f64) -> f64 {
    let hour_angle = TAU * (time % 1.0);
    return hour_angle - hour_angle.signum() * TAU / 2.0;
}

fn elevation_angle(latitude: f64, declination: f64, time: f64) -> f64 {
    let hour_angle = hour_angle(time);
    let elevation_angle =
        latitude.sin() * declination.sin() + latitude.cos() * declination.cos() * hour_angle.cos();
    return elevation_angle.asin();
}

fn azimuth_angle(latitude: f64, declination: f64, time: f64) -> f64 {
    if latitude.abs() >= TAU / 4.0 {
        return TAU * (-latitude.signum() * time + (latitude.signum() + 1.0) / 4.0) % 1.0;
    }
    let elevation_angle = elevation_angle(latitude, declination, time);
    let azimuth_angle = (latitude.sin() * elevation_angle.sin() - declination.sin())
        / (latitude.cos() * elevation_angle.cos());
    if azimuth_angle.abs() > 1.0 {
        return TAU / 2.0 * (azimuth_angle.signum() + 1.0) / 2.0;
    }
    let azimuth_angle = azimuth_angle.acos();
    if hour_angle(time) > 0.0 {
        let azimuth_angle = azimuth_angle + TAU / 2.0;
        return azimuth_angle % TAU;
    } else {
        let azimuth_angle = 1.5 * TAU - azimuth_angle;
        return azimuth_angle % TAU;
    }
}

pub fn sun_position(
    world: world::World,
    position: coordinates::Polar,
    time: f64,
) -> coordinates::Celestial {
    let longitude = position.longitude.to_radians();
    let latitude = position.latitude.to_radians();
    let time = time + longitude / TAU / 2.0;
    let declination = world.declination(time);
    let elevation_angle = elevation_angle(latitude, declination, time);
    let azimuth_angle = azimuth_angle(latitude, declination, time);
    let sun_position = coordinates::Celestial {
        azimuth: azimuth_angle.to_degrees(),
        elevation: elevation_angle.to_degrees(),
    };
    return sun_position;
}
