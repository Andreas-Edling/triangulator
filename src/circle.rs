use crate::Point;
use crate::Triangle;

#[derive(Clone, Debug)]
pub struct Circle {
    pub pos: Point,
    pub radius_sqr: f32,
}

impl Circle {
    fn new(pos: Point, radius_sqr: f32) -> Self {
        Self { pos, radius_sqr }
    }

    /// creates a circle with the triangle vertices on the circumference
    pub fn from_triangle(tri: &Triangle, points: &[Point]) -> Self {
        let a = &points[tri.index0];
        let b = &points[tri.index1];
        let c = &points[tri.index2];

        let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
        let a_s = a.x * a.x + a.y * a.y;
        let b_s = b.x * b.x + b.y * b.y;
        let c_s = c.x * c.x + c.y * c.y;

        let circle_x = (a_s * (b.y - c.y) + b_s * (c.y - a.y) + c_s * (a.y - b.y)) / d;
        let circle_y = (a_s * (c.x - b.x) + b_s * (a.x - c.x) + c_s * (b.x - a.x)) / d;

        let rad_sqr = (circle_x - a.x) * (circle_x - a.x) + (circle_y - a.y) * (circle_y - a.y);
        Self::new(Point::new(circle_x, circle_y), rad_sqr)
    }

    pub fn contains(&self, point: &Point) -> bool {
        let dx = point.x - self.pos.x;
        let dy = point.y - self.pos.y;
        let d_sqr = dx * dx + dy * dy;
        d_sqr <= self.radius_sqr
    }
}
