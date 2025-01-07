use solar_time::solar;
use solar_time::solar::Polar;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let mut time: f64 = 0.0;
    while time < 1.0 {
        let player_position = Polar {
            longitude: 0.0,
            altitude: 0.0,
            latitude: 30.0,
        };
        let sun_position = solar::sun_position(OBLIQUITY, ORBITAL_PERIOD, player_position, time);
        let solar_azimuth = sun_position.azimuth;
        let solar_elevation = sun_position.elevation;
        println!("TIME: {time} | AZIMUTH: {solar_azimuth} | ELEVATION: {solar_elevation}");
        time += 1.0 / 24.0;
    }
}
