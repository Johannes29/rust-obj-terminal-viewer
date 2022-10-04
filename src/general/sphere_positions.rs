use super::positions_3d::Degrees;

#[derive(Debug)]
/**
 * z is forwards, y is up, x is to the right.
 */
pub struct SpherePos {
    pub lat: Degrees,
    pub lon: Degrees,
}

#[derive(Debug)]
pub struct TriangleSphere {
    pub p1: SpherePos,
    pub p2: SpherePos,
    pub p3: SpherePos,
    pub fill_char: u8,
}