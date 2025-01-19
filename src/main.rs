use celestial::coordinates;
use celestial::solar;
use celestial::world;

const RADIUS: f64 = 6371.0;
const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the world, between -90 and 90 degrees
const ECCENTRICITY: f64 = 0.0; //eccentricity of the world, between 0 and 1
const PERIAPTIC_PERIOD: f64 = 0.1;

fn main() {
    let mut time: f64 = 0.5;
    while time < ORBITAL_PERIOD {
        let world = world::World::new(
            RADIUS,
            ORBITAL_PERIOD,
            OBLIQUITY,
            ECCENTRICITY,
            PERIAPTIC_PERIOD,
        );
        let player_position = coordinates::Polar::new(0.0, 0.0, 30.0);
        let sun_position = solar::sun_position(world, player_position, time);
        let solar_azimuth = sun_position.azimuth;
        let solar_elevation = sun_position.elevation;
        graph(time, solar_azimuth, solar_elevation);
        time += 1.0 / 24.0;
    }
}

fn graph(time: f64, azimuth: f64, elevation: f64) {
    const PRECISION: usize = 2;
    print!("TIME : {:>6.1$} | ", time, PRECISION);
    print!("AZIMUTH : {:>6.1$} | ", azimuth, PRECISION);
    print!("ELEVATION : {:>6.1$} | ", elevation, PRECISION);
    let elevation_display_offset = (elevation + 90.0) / 2.0;
    let azimuth_display_offset = azimuth / 4.0;
    for i in 0..elevation_display_offset.abs().floor() as i32 {
        if i == azimuth_display_offset.floor() as i32 {
            print!("@");
        } else if i == 45 {
            print!("|");
        } else {
            print!(" ");
        }
    }
    if elevation > 0.0 {
        print!("*");
    } else {
        print!("*");
    }
    for i in elevation_display_offset.abs().floor() as i32..90 {
        if i == azimuth_display_offset.floor() as i32 - 1 {
            print!("@");
        } else if i == 44 {
            print!("|");
        } else {
            print!(" ");
        }
    }
    print!("|");
    println!("");
}
