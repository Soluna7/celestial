use bevy::math::DVec3;
use num::complex::*;
use std::f64::consts::TAU;

pub struct Polar {
    pub longitude: f64,
    pub altitude: f64,
    pub latitude: f64,
}

impl Polar {
    pub fn new(longitude: f64, altitude: f64, latitude: f64) -> Self {
        Polar {
            longitude: longitude,
            altitude: altitude,
            latitude: latitude,
        }
    }
}

pub struct Horizontal {
    pub azimuth: f64,
    pub elevation: f64,
}

pub trait CartesianMethods {
    fn update(&self, radius: f64) -> Self;
    fn polar(&self) -> Polar;
}

fn mode(n: f64) -> f64 {
    (n - 1.0) % 2.0 - 1.0
}

fn tri(n: f64, r: f64) -> f64 {
    4.0 * (((n - r) / 4.0) % r - r / 2.0).abs() - r.abs()
}

fn flip(x: f64, y: f64, r: f64) -> f64 {
    2.0 * mode(((x + r) / r).floor() + ((y + r) / r).floor()) + 1.0
}

impl CartesianMethods for DVec3 {
    fn update(&self, radius: f64) -> Self {
        let f = flip(self.x, self.y, radius);
        let x = f * tri(self.x, radius);
        let y = f * self.y;
        let z = f * tri(self.z, radius);
        return DVec3::new(x, y, z);
    }
    fn polar(&self) -> Polar {
        return Polar::new(-self.x, -self.y, -self.z);
    }
}

pub trait PolarMethods {
    fn cartesian(&self) -> DVec3;
}

impl PolarMethods for Polar {
    fn cartesian(&self) -> DVec3 {
        let longitude = self.longitude.to_radians();
        let latitude = self.latitude.to_radians();
        let chi: f64 = (2.0 * (longitude + TAU / 2.0)).cos();
        let rho: f64 = ((2.0 * latitude + TAU / 2.0) / 4.0).tan();
        let a: f64 =
            2.0 * rho.powi(2) - ((1.0 + rho.powi(4)).powi(2) - (2.0 * rho * chi).powi(2)).sqrt();
        let b: f64 = 1.0 + 2.0 * chi * rho.powi(2) - rho.powi(4);
        let x = Complex64::new(a / b, 0.0).acos();
        let z = Complex64::new(b / a, 0.0).acos();
        return DVec3::new(x.re(), -self.altitude, z.im());
    }
}
