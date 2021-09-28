use crate::{point::cross, Point, TriangulatorError};
use std::cmp::Ordering;

// creates indices sorted on x, y secondary
fn get_sorted_indices(points: &[Point]) -> Result<Vec<usize>, TriangulatorError> {
    let mut point_indices = points
        .iter()
        .enumerate()
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    sort_indices(&mut point_indices, points)?;
    Ok(point_indices)
}

fn sort_indices(point_indices: &mut [usize], points: &[Point]) -> Result<(), TriangulatorError> {
    // check for NAN
    for (i, p) in points.iter().enumerate() {
        if p.x.is_nan() || p.y.is_nan() {
            return Err(TriangulatorError::NANInInput(i));
        }
    }

    // unwraps are ok, since NAN is checked above
    point_indices.sort_by(
        |a, b| match points[*a].x.partial_cmp(&points[*b].x).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => points[*a].y.partial_cmp(&points[*b].y).unwrap(),
        },
    );

    Ok(())
}

pub(crate) fn convex_hull(points: &[Point]) -> Result<(Vec<usize>, Vec<usize>), TriangulatorError> {
    // monotone chain algorithm:
    // https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain

    let point_indices = get_sorted_indices(points)?;
    let (mut lower, mut points_left) = half_hull(point_indices.iter(), points)?;

    //add beginning of lower hull so upper hull will close the total hull
    points_left.push(lower[lower.len() - 1]);
    points_left.push(lower[0]);

    sort_indices(&mut points_left, points)?;
    let (mut upper, points_left) = half_hull(points_left.iter().rev(), points)?;

    // remove duplicate end/begin
    upper.pop();
    upper.remove(0);

    lower.append(&mut upper);
    Ok((lower, points_left))
}

fn half_hull<'a, T: Iterator<Item = &'a usize>>(
    point_indices: T,
    points: &[Point],
) -> Result<(Vec<usize>, Vec<usize>), TriangulatorError> {
    let mut hull = Vec::new();
    let mut points_left = Vec::new();

    // next point should make hull 'turn' clockwise, otherwise pop point(s)
    for point_index in point_indices {
        while hull.len() >= 2
            && cross(
                &points[hull[hull.len() - 2]],
                &points[hull[hull.len() - 1]],
                &points[*point_index],
            ) < 0.0
        {
            points_left.push(*hull.last().unwrap());
            hull.pop();
        }
        hull.push(*point_index);
        if let Some(pos) = points_left.iter().position(|p| p == point_index) {
            points_left.swap_remove(pos);
        }
    }

    Ok((hull, points_left))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_from_triangle() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(0.5, 0.5),
            Point::new(1.0, -0.5),
        ];

        let (hull, points_left) = convex_hull(&points).unwrap();

        assert_eq!(hull.len(), 3);
        assert_eq!(hull[0], 0);
        assert_eq!(hull[1], 2);
        assert_eq!(hull[2], 1);
        assert_eq!(points_left.len(), 0);
    }

    #[test]
    fn works_with_horizontal_line() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
        ];

        let (hull, points_left) = convex_hull(&points).unwrap();

        assert_eq!(hull.len(), 3);
        assert_eq!(hull[0], 0);
        assert_eq!(hull[1], 1);
        assert_eq!(hull[2], 2);
        assert_eq!(points_left.len(), 0);
    }

    #[test]
    fn works_with_horizontal_line_4_points() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 0.0),
        ];

        for permution in permutations(&points) {
            let (hull, points_left) = convex_hull(&permution).unwrap();
            assert_eq!(hull.len(), 4);
            assert_eq!(points_left.len(), 0);
        }
    }

    #[test]
    fn works_with_vertical_line() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(0.0, 2.0),
        ];

        let (hull, points_left) = convex_hull(&points).unwrap();

        assert_eq!(hull.len(), 3);
        assert_eq!(hull[0], 0);
        assert_eq!(hull[1], 1);
        assert_eq!(hull[2], 2);
        assert_eq!(points_left.len(), 0);
    }

    #[test]
    fn works_with_vertical_line_4_points() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(0.0, 2.0),
            Point::new(0.0, 3.0),
        ];

        for permutation in permutations(&points) {
            let (hull, points_left) = convex_hull(&permutation).unwrap();
            assert_eq!(hull.len(), 4);
            assert_eq!(points_left.len(), 0);
        }
    }

    #[test]
    fn works_with_diagonal_line() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
        ];

        let (hull, points_left) = convex_hull(&points).unwrap();

        assert_eq!(hull.len(), 3);
        assert_eq!(hull[0], 0);
        assert_eq!(hull[1], 1);
        assert_eq!(hull[2], 2);
        assert_eq!(points_left.len(), 0);
    }

    #[test]
    fn point_not_part_of_hull() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.5, 1.0),
            Point::new(0.5, 0.5),
        ];

        let (hull, points_left) = convex_hull(&points).unwrap();

        assert_eq!(hull.len(), 3);
        assert_eq!(hull[0], 0);
        assert_eq!(hull[1], 1);
        assert_eq!(hull[2], 2);
        assert_eq!(points_left.len(), 1);
        assert_eq!(points_left[0], 3);
    }

    fn permutations<T: std::clone::Clone>(slice: &[T]) -> Vec<Vec<T>> {
        use itertools::Itertools;
        let perms = slice
            .iter()
            .cloned()
            .permutations(slice.len())
            .collect_vec();
        perms
    }
}
