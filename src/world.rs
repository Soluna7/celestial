pub struct World {
    //radius: f64,
    pub obliquity: f64,
    pub orbital_period: f64,
}

impl World {
    pub fn new(obliquity: f64, orbital_period: f64) -> Self {
        World {
            obliquity: obliquity.to_radians(),
            orbital_period: orbital_period,
        }
    }
}
