use std::collections::HashSet;

use crate::{
    circle::Circle,
    convex_hull::convex_hull,
    edge::Edge,
    point::cross,
    types::{PointIdx, TriIdx},
    Point, Triangle, TriangulatorError,
};

mod ordered_pair;
mod tri_edge_mapping;

use ordered_pair::OrderedPair;
use tri_edge_mapping::TriangleEdgeMapping;

pub(crate) fn triangulate(points: &[Point]) -> Result<Vec<Triangle>, TriangulatorError> {
    let mut triangulator = DelaunayIncremental::new();
    let _ = triangulator.initial_triangulation(points)?;

    while triangulator.do_step(points) {}

    Ok(triangulator.triangles)
}

#[derive(Default, Clone, Debug)]
pub(crate) struct DelaunayIncremental {
    pub triangles: Vec<Triangle>,
    points_to_add: Vec<PointIdx>,
    tri_edge_mapping: TriangleEdgeMapping,
}

impl DelaunayIncremental {
    pub(crate) fn new() -> Self {
        Self {
            triangles: Vec::new(),
            points_to_add: Vec::new(),
            tri_edge_mapping: TriangleEdgeMapping::new(),
        }
    }

    pub(crate) fn initial_triangulation(
        &mut self,
        points: &[Point],
    ) -> Result<Vec<Triangle>, TriangulatorError> {
        let (hull, points_inside_hull) = convex_hull(points)?;
        self.points_to_add = points_inside_hull;

        self.triangles = generate_triangles_from_hull(&hull);
        for i in 0..self.triangles.len() {
            self.tri_edge_mapping.add_triangle(i, &self.triangles);
        }

        if self.points_to_add.is_empty() {
            let check_stack = (0..self.triangles.len()).collect();
            self.flip_pairs(check_stack, points);
        }

        Ok(self.triangles.clone())
    }

    pub(crate) fn do_step(&mut self, points: &[Point]) -> bool {
        if let Some(point_idx) = self.points_to_add.pop() {
            self.add_point(point_idx, points);
            true
        } else {
            false
        }
    }

    fn add_point(&mut self, point_idx: PointIdx, points: &[Point]) {
        // find all triangles whose circumcircle contains the point, starting with the triangle containing the point.
        // work outwards from there, keeping track of the edges of the cavity.
        // delete those triangles, which is guaranteed to create a convex cavity.
        // join the new point with the vertices of the cavity

        let point = &points[point_idx];

        let mut containing_triangle_idx = 0;
        for (idx, triangle) in self.triangles.iter().enumerate() {
            if point_in_triangle(
                point,
                &points[triangle.index0],
                &points[triangle.index1],
                &points[triangle.index2],
            ) {
                containing_triangle_idx = idx;
                break;
            }
        }

        let mut triangles_to_check = vec![containing_triangle_idx];
        let mut triangles_to_remove = Vec::new();
        let mut cavity_edges = HashSet::<Edge>::new();

        // build cavity edge list and flag triangles for removal
        while let Some(triangle_to_check) = triangles_to_check.pop() {
            let circle = Circle::from_triangle(&self.triangles[triangle_to_check], points);
            if circle.contains(point) {
                //flag triangle for removal
                triangles_to_remove.push(triangle_to_check);

                // remove or add edges to cavity
                for edge in self.tri_edge_mapping.get_edges(triangle_to_check) {
                    let tris_of_edge = self.tri_edge_mapping.get_triangles(edge);

                    if tris_of_edge.len() == 2
                        && triangles_to_remove.contains(&tris_of_edge[0])
                        && triangles_to_remove.contains(&tris_of_edge[1])
                    {
                        // edge is now *inside* of cavity, remove it
                        cavity_edges.remove(edge);
                    } else {
                        cavity_edges.insert(*edge);
                    }
                }

                // add neighbours for checking
                for tri_idx in self
                    .tri_edge_mapping
                    .neighbouring_triangles(containing_triangle_idx)
                {
                    if !triangles_to_remove.contains(&tri_idx)
                        && !triangles_to_check.contains(&tri_idx)
                    {
                        triangles_to_check.push(tri_idx);
                    }
                }
            }
        }

        debug_assert!(cavity_edges.len() >= triangles_to_remove.len());
        let mut cavity_edges = cavity_edges.into_iter().collect::<Vec<_>>();
        let mut changed_triangles = triangles_to_remove.clone();

        // remove old triangles
        for tri_idx in triangles_to_remove.iter() {
            self.tri_edge_mapping.remove_triangle(*tri_idx);
        }

        //create new triangles
        for tri_idx in triangles_to_remove.iter() {
            let cavity_edge = cavity_edges.pop().unwrap();
            let new_tri = Triangle::new(point_idx, cavity_edge.index_0, cavity_edge.index_1);
            self.triangles[*tri_idx] = new_tri;
            self.tri_edge_mapping
                .add_triangle(*tri_idx, &self.triangles);
        }

        // append any remaining new triangles
        while let Some(cavity_edge) = cavity_edges.pop() {
            let new_tri = Triangle::new(point_idx, cavity_edge.index_0, cavity_edge.index_1);
            self.triangles.push(new_tri);
            let tri_idx = self.triangles.len() - 1;
            self.tri_edge_mapping.add_triangle(tri_idx, &self.triangles);
            changed_triangles.push(tri_idx);
        }

        // swap pairs if necessary
        self.flip_pairs(changed_triangles, points);
    }

    fn flip_pairs(&mut self, mut check_stack: Vec<TriIdx>, points: &[Point]) {
        let mut already_checked = HashSet::new();

        while let Some(tri) = check_stack.pop() {
            let neighbours = self.tri_edge_mapping.neighbouring_triangles(tri);
            for neighbour in neighbours.into_iter() {
                let pair = OrderedPair::new(tri, neighbour);

                if !already_checked.contains(&pair)
                    && should_flip(&self.triangles[tri], &self.triangles[neighbour], points)
                {
                    self.flip(tri, neighbour);

                    already_checked.insert(pair);

                    // the new triangles should be checked
                    check_stack.push(tri);
                    check_stack.push(neighbour);

                    break; // dont check the rest of the neighbours since maybe they arent neighbours anymore
                }
            }
        }
    }

    fn flip(&mut self, a: TriIdx, b: TriIdx) {
        let tri_a = &self.triangles[a];
        let tri_b = &self.triangles[b];

        let ((non_common_a, non_common_b), (common_0, common_1)) = commonality(tri_a, tri_b);

        self.tri_edge_mapping.remove_triangle(a);
        self.tri_edge_mapping.remove_triangle(b);

        let new_tri_a = Triangle::new(non_common_a, non_common_b, common_0);
        let new_tri_b = Triangle::new(non_common_a, non_common_b, common_1);

        self.triangles[a] = new_tri_a;
        self.triangles[b] = new_tri_b;

        self.tri_edge_mapping.add_triangle(a, &self.triangles);
        self.tri_edge_mapping.add_triangle(b, &self.triangles);
    }
}

fn should_flip(a: &Triangle, b: &Triangle, points: &[Point]) -> bool {
    let circle_a = Circle::from_triangle(a, points);
    let circle_b = Circle::from_triangle(a, points);
    debug_assert!(circle_a.radius_sqr.is_finite() && circle_b.radius_sqr.is_finite());

    let ((_point_a, point_b), (_, _)) = commonality(a, b);
    circle_a.contains(&points[point_b])
}

fn commonality(a: &Triangle, b: &Triangle) -> ((PointIdx, PointIdx), (PointIdx, PointIdx)) {
    let mut common_points = Vec::with_capacity(2);
    let mut non_common_a = PointIdx::MAX;
    let mut non_common_b = PointIdx::MAX;
    let a_indices = [a.index0, a.index1, a.index2];
    let b_indices = [b.index0, b.index1, b.index2];

    for a_idx in a_indices {
        if b_indices.contains(&a_idx) {
            common_points.push(a_idx);
        } else {
            non_common_a = a_idx;
        }
    }

    for b_idx in b_indices {
        if !a_indices.contains(&b_idx) {
            non_common_b = b_idx;
            break;
        }
    }

    debug_assert!(non_common_a != PointIdx::MAX);
    debug_assert!(non_common_b != PointIdx::MAX);
    debug_assert!(common_points.len() == 2);

    (
        (non_common_a, non_common_b),
        (common_points[0], common_points[1]),
    )
}

fn generate_triangles_from_hull(hull: &[PointIdx]) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    triangles.reserve(hull.len() / 3);

    for i in 2..hull.len() {
        triangles.push(Triangle::new(hull[0], hull[i - 1], hull[i]));
    }

    triangles
}

fn point_in_triangle(point: &Point, a: &Point, b: &Point, c: &Point) -> bool {
    if cross(a, b, c).abs() < f32::EPSILON {
        return false;
    }

    same_side_of_line(point, a, b, c)
        && same_side_of_line(point, b, a, c)
        && same_side_of_line(point, c, a, b)
}

fn same_side_of_line(p0: &Point, p1: &Point, linestart: &Point, lineend: &Point) -> bool {
    let cp1 = cross(lineend, p0, linestart);
    let cp2 = cross(lineend, p1, linestart);
    cp1.signum() == cp2.signum()
}
