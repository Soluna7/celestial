pub struct Polar {
    pub longitude: f64,
    pub altitude: f64,
    pub latitude: f64,
}

pub struct Celestial {
    pub azimuth: f64,
    pub elevation: f64,
}

fn hour_angle(time: f64) -> f64 {
    let hour_angle = 360.0 * (time % 1.0);
    return hour_angle - hour_angle.signum() * 180.0;
}

fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = obliquity * ecliptic_longitude.to_radians().sin();
    return declination;
}

fn elevation_angle(latitude: f64, declination: f64, time: f64) -> f64 {
    let hour_angle = hour_angle(time);
    let elevation_angle = latitude.to_radians().sin() * declination.to_radians().sin()
        + latitude.to_radians().cos()
            * declination.to_radians().cos()
            * hour_angle.to_radians().cos();
    return elevation_angle.asin().to_degrees();
}

fn azimuth_angle(latitude: f64, declination: f64, time: f64) -> f64 {
    if latitude.abs() >= 90.0 {
        return 360.0 * (-latitude.signum() * time + (latitude.signum() + 1.0) / 4.0) % 1.0;
    }
    let elevation_angle = elevation_angle(latitude, declination, time);
    let azimuth_angle = (latitude.to_radians().sin() * elevation_angle.to_radians().sin()
        - declination.to_radians().sin())
        / (latitude.to_radians().cos() * elevation_angle.to_radians().cos());
    let azimuth_angle = azimuth_angle.acos().to_degrees();
    if hour_angle(time) > 0.0 {
        let azimuth_angle = azimuth_angle + 180.0;
        return azimuth_angle % 360.0;
    } else {
        let azimuth_angle = 540.0 - azimuth_angle;
        return azimuth_angle % 360.0;
    }
}

pub fn sun_position(obliquity: f64, orbital_period: f64, position: Polar, time: f64) -> Celestial {
    let longitude = position.longitude;
    let latitude = position.latitude;
    let time = time + longitude / 180.0;
    let declination = declination(obliquity, orbital_period, time);
    let elevation_angle = elevation_angle(latitude, declination, time);
    let azimuth_angle = azimuth_angle(latitude, declination, time);
    let sun_position = Celestial {
        azimuth: azimuth_angle,
        elevation: elevation_angle,
    };
    return sun_position;
}
