use crate::coordinates;
use crate::world;
use crate::world::*;
use std::f64::consts::TAU;

pub trait Solar {
    fn declination(self, time: f64) -> f64;
}

impl Solar for World {
    fn declination(self, time: f64) -> f64 {
        let ecliptic_longitude = (TAU * time) / self.orbital_period;
        let declination: f64 = self.obliquity * ecliptic_longitude.sin();
        return declination;
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

fn equation_of_time(time: f64) -> f64 {
    let hour_angle = hour_angle(time);
    return hour_angle;
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
