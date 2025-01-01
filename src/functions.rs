use num_traits::sign::signum;

pub fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = obliquity * ecliptic_longitude.to_radians().sin();
    return declination;
}

pub fn hour_angle(longitude: f64, time: f64) -> f64 {
    let day_angle: f64 = 360.0*(time%1.0)+longitude;
    let day_angle = day_angle.rem_euclid(360.0);
    return day_angle+180.0*signum(day_angle);
}

pub fn zenith_angle(latitude:f64, declination:f64, hour_angle: f64) -> f64 {
    let zenith_angle:f64 = 
        latitude.to_radians().sin()*declination.to_radians().sin() +
        latitude.to_radians().cos()*declination.to_radians().cos()*hour_angle.to_radians().cos();
    let zenith_angle = zenith_angle.acos().to_degrees();
    return zenith_angle;
}

pub fn elevation_angle(latitude:f64, declination:f64, hour_angle:f64) -> f64 {
    let elevation_angle: f64 = 90.0 - zenith_angle(latitude, declination, hour_angle);
    return elevation_angle;
}

pub fn azimuth_angle(latitude:f64, declination:f64, hour_angle:f64) -> f64 {
    let zenith_angle = zenith_angle(latitude, declination, hour_angle);
    if hour_angle > 0.0 {
        let azimuth_angle = (
            latitude.to_radians().sin()*zenith_angle.to_radians().cos() - 
            declination.to_radians().sin() 
            ) / latitude.to_radians().cos()*declination.to_radians().sin();
            println!("{azimuth_angle}");
        let azimuth_angle = ((azimuth_angle+1.0).rem_euclid(2.0))-1.0;
        let azimuth_angle = azimuth_angle.acos().to_degrees() + 180.0;
        let azimuth_angle = azimuth_angle.rem_euclid(360.0);
        return azimuth_angle;
    } else {
        let azimuth_angle = (
            latitude.to_radians().sin()*zenith_angle.to_radians().cos() - 
            zenith_angle.to_radians().sin()
            ) / latitude.to_radians().cos()*zenith_angle.to_radians().sin();
            println!("{azimuth_angle}");
        let azimuth_angle = ((azimuth_angle+1.0).rem_euclid(2.0))-1.0;
        let azimuth_angle = 540.0 - azimuth_angle.acos().to_degrees();
        let azimuth_angle = azimuth_angle.rem_euclid(360.0);
        return azimuth_angle;
    }
}