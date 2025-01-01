mod functions;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let mut time: f64 = 0.0; //day in year (mod ORBITAL_PERIOD)
    while time <= ORBITAL_PERIOD {
        let mut latitude: f64 = -90.0; //between -90 and 90
        while latitude <= 90.0 {
            let mut longitude: f64 = -180.0; //between -180 and 180
            while longitude <= 180.0 {
                let declination = functions::declination(OBLIQUITY, ORBITAL_PERIOD, time);
                let hour_angle = functions::hour_angle(longitude, time);
                let _zenith_angle = functions::zenith_angle(latitude, declination, hour_angle);
                let elevation_angle = functions::elevation_angle(latitude, declination, hour_angle);
                let azimuth_angle = functions::azimuth_angle(latitude, declination, hour_angle);
                println!("TIME = {time}");
                //println!("DECLINATION = {declination}");
                //println!("HOUR ANGLE = {hour_angle}");
                //println!("ZENITH ANGLE = {zenith_angle}");
                println!("ELEVATION ANGLE = {elevation_angle}");
                println!("AZIMUTH ANGLE = {azimuth_angle}");
                longitude = longitude + 1.0;
            }
            latitude = latitude + 1.0;
        }
        time = time + 1.0;
    }
}
