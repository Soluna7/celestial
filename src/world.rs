pub struct World {
    pub radius: f64,
    pub orbital_period: f64,   //length of sidereal year in ephemeris days
    pub obliquity: f64,        //axial tilt, between -90 and 90 degrees
    pub eccentricity: f64,     //orbital eccentricity, between 0 and 1
    pub periaptic_period: f64, //proportional position of orbital period where periapsis occurs, between 0 and 1
}

impl World {
    pub fn new(
        radius: f64,
        obliquity: f64,
        orbital_period: f64,
        eccentricity: f64,
        periaptic_period: f64,
    ) -> Self {
        World {
            radius: radius,
            obliquity: obliquity.to_radians(),
            orbital_period: orbital_period,
            eccentricity: eccentricity,
            periaptic_period: periaptic_period,
        }
    }
}
