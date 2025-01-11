pub struct Planet {
    //radius: f64,
    pub obliquity: f64,
    pub orbital_period: f64,
}

impl Planet {
    pub fn new(obliquity: f64, orbital_period: f64) -> Self {
        Planet {
            obliquity: obliquity,
            orbital_period: orbital_period,
        }
    }
}

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
