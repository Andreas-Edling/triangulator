mod circle;
mod convex_hull;
mod delaunay_inc;
mod edge;
mod point;
mod triangle;
mod types;
mod utils;

pub use circle::Circle;
pub use point::Point;
pub use triangle::Triangle;

use delaunay_inc::DelaunayIncremental;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TriangulatorError {
    #[error("Too few points given")]
    TooFewPoints,

    #[error("NAN found in input, index {0}")]
    NANInInput(usize),

    #[error("Point found outside of hull")]
    PointOutsideOfHull,
}

pub fn triangulate(points: &[Point]) -> Result<Vec<Triangle>, TriangulatorError> {
    if points.len() < 3 {
        return Err(TriangulatorError::TooFewPoints);
    }

    let triangles = delaunay_inc::triangulate(points)?;
    Ok(triangles)
}

#[derive(Default, Clone, Debug)]
pub struct Triangulator {
    triangulator: DelaunayIncremental,
}

impl Triangulator {
    pub fn new() -> Self {
        Self {
            triangulator: DelaunayIncremental::new(),
        }
    }

    pub fn initial_triangulation(
        &mut self,
        points: &[Point],
    ) -> Result<Vec<Triangle>, TriangulatorError> {
        if points.len() < 3 {
            return Err(TriangulatorError::TooFewPoints);
        }

        self.triangulator.initial_triangulation(points)
    }

    pub fn do_step(&mut self, points: &[Point]) -> bool {
        self.triangulator.do_step(points)
    }

    pub fn get_triangles(&self) -> &[Triangle] {
        self.triangulator.triangles.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::{triangulate, Point, Triangle, TriangulatorError};

    #[test]
    fn returns_too_few_points() {
        let points = [Point::new(0.0, 0.0), Point::new(1.0, 1.0)];

        let res = triangulate(&points);

        assert_eq!(res, Err(TriangulatorError::TooFewPoints));
    }

    #[test]
    fn fails_gracefully_on_nan() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(f32::NAN, 0.0),
            Point::new(0.5, 1.0),
        ];

        let res = triangulate(&points);

        assert_eq!(res, Err(TriangulatorError::NANInInput(1)));
    }

    #[test]
    fn returns_one_triangle() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
        ];

        let res = triangulate(&points);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1);
    }

    #[test]
    fn returns_three_triangles() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
            Point::new(0.5, 0.5),
        ];

        let res = triangulate(&points);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 3);
    }

    #[test]
    fn handles_identical_values() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
            Point::new(0.5, 0.5),
            Point::new(0.5, 0.5),
        ];

        let res = triangulate(&points);
        assert!(res.is_ok());
    }

    #[test]
    fn avoids_obtuse_triangles() {
        // result should be:
        //   /\
        //  /__\
        //  \  /
        //   \/
        // and not:
        //   /|\
        //  / | \
        //  \ | /
        //   \|/

        let points = [
            Point::new(0.0, 0.0),
            Point::new(0.0, 4.0),
            Point::new(-1.0, 2.0),
            Point::new(1.0, 2.0),
        ];
        let expected_0 = Triangle::new(0, 3, 2);
        let expected_1 = Triangle::new(1, 2, 3);

        let triangles = triangulate(&points).unwrap();

        assert_eq!(triangles.len(), 2);
        assert!(triangles[0].equivalent(&expected_0));
        assert!(triangles[1].equivalent(&expected_1));
    }

    #[test]
    fn avoids_obtuse_triangles_2() {
        // same as previous, but laid on its side
        let points = [
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(2.0, -1.0),
            Point::new(2.0, 1.0),
        ];
        let expected_0 = Triangle::new(1, 2, 3);
        let expected_1 = Triangle::new(0, 2, 3);

        let triangles = triangulate(&points).unwrap();

        assert_eq!(triangles.len(), 2);
        assert!(triangles[0].equivalent(&expected_0));
        assert!(triangles[1].equivalent(&expected_1));
    }

    #[test]
    fn offending_points() {
        use rand::seq::SliceRandom;

        // these are offending combinations of points, caused triangulate to fail:
        let fails = [
            Point::new(22., 20.),
            Point::new(141., 20.),
            Point::new(245., 169.),
            Point::new(268., 134.),
            Point::new(314., 133.),
            Point::new(69., 20.),
        ];

        for _ in 0..720 {
            let subset: Vec<Point> = fails
                .choose_multiple(&mut rand::thread_rng(), 6)
                .map(|x| *x)
                .collect();

            let triangles = triangulate(&subset);
            assert!(triangles.is_ok());
        }
    }
}
