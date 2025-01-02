mod solar;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let time: f64 = 0.0; //day in year (mod ORBITAL_PERIOD)
    let latitude: f64 = -90.0; //between -90 and 90
    let longitude: f64 = -180.0; //between -180 and 180
    let sun_position = solar::sun_position(OBLIQUITY, ORBITAL_PERIOD, latitude, longitude, time);
    println!("{:?}", sun_position);
}
