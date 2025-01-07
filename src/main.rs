use celestial::solar;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let mut time: f64 = 0.0;
    while time < ORBITAL_PERIOD {
        let player_position = solar::Polar {
            longitude: 0.0,
            altitude: 0.0,
            latitude: 30.0,
        };
        let sun_position = solar::sun_position(OBLIQUITY, ORBITAL_PERIOD, player_position, time);
        let solar_azimuth = sun_position.azimuth;
        let solar_elevation = sun_position.elevation;
        const PRECISION: usize = 2;
        print!("TIME : {:>6.1$} | ", time, PRECISION);
        print!("AZIMUTH : {:>6.1$} | ", solar_azimuth, PRECISION);
        print!("ELEVATION : {:>6.1$} | ", solar_elevation, PRECISION);
        println!("");
        time += 1.0 / (24.0 * 60.0);
    }
}
