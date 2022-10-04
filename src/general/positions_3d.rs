#[derive(Debug, PartialEq, Clone)]
/**
 * z is forwards, y is up, x is to the right.
 */
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub fill_char: u8,
}

#[derive(Debug)]
pub struct Degrees(pub f32);

// TODO check if nämnare is 0, both get_horizontal_angle and vertical

impl Point {
    // TODO rewrite so that dx is x in the trigonometry, now -dz acts as x in the trigonometry calcualtions
    pub fn get_horizontal_angle(&self, origin: &Point) -> Degrees {
        let dx = self.x - origin.x;
        let dz = self.z - origin.z;
        // dist_xz is distance in x-z plane, meaning distance between points if they would have same y value.
        let dist_xz = (dx.powi(2) + dz.powi(2)).sqrt();

        let mut angle = (dx / dist_xz).acos().to_degrees();
        if dz < 0.0 {
            angle  = 360.0 - angle;
        }
        let angle2 = (90.0 - angle).rem_euclid(360.0);
        Degrees(angle2)
    }
    
    /**
     * up is 90 deg, horizontal is 0 deg, down is -90 deg.
     */
    pub fn get_vertical_angle(&self, origin: &Point) -> Degrees {
        let dy = self.y - origin.y;
        let distance = distance(self, &origin);
        Degrees((dy/distance).asin().to_degrees())
    }
}

impl Triangle {
    pub fn points(&self) -> (&Point, &Point, &Point) {
        (&self.p1, &self.p2, &self.p3)
    }

    // TODO
    // pub fn rotate(&mut self, angle_deg: f32, origin: &Point) {
    //     self.p1.rotate(angle_deg, origin);
    //     self.p2.rotate(angle_deg, origin);
    //     self.p3.rotate(angle_deg, origin);
    // }
}

pub fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}