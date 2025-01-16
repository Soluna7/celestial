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

pub struct Celestial {
    pub azimuth: f64,
    pub elevation: f64,
}

pub trait ToPolar {
    fn polar(self) -> Polar;
}

impl ToPolar for DVec3 {
    fn polar(self) -> Polar {
        return Polar::new(-self.x, -self.y, -self.z);
    }
}

pub trait ToCartesian {
    fn cartesian(self) -> DVec3;
}

impl ToCartesian for Polar {
    fn cartesian(self) -> DVec3 {
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
