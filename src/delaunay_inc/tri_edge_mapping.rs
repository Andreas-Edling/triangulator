use crate::{edge::Edge, types::TriIdx, Triangle};
use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, Debug)]
pub(crate) struct TriangleEdgeMapping {
    edge_tri_map: HashMap<Edge, HashSet<TriIdx>>,
    tri_edge_map: HashMap<TriIdx, HashSet<Edge>>,
}

impl TriangleEdgeMapping {
    pub fn new() -> Self {
        Self {
            edge_tri_map: HashMap::new(),
            tri_edge_map: HashMap::new(),
        }
    }

    pub fn add_triangle(&mut self, triangle_index: TriIdx, triangles: &[Triangle]) {
        let tri = &triangles[triangle_index];
        let mut edges = HashSet::with_capacity(3);
        edges.insert(Edge::new(tri.index0, tri.index1));
        edges.insert(Edge::new(tri.index1, tri.index2));
        edges.insert(Edge::new(tri.index2, tri.index0));
        debug_assert!(edges.len() == 3);

        //update triangle-edge mapping
        let res = self.tri_edge_map.insert(triangle_index, edges.clone());
        debug_assert!(res.is_none());

        //update edge-triangle mapping
        for edge in edges.into_iter() {
            if let Some(triangle_set) = self.edge_tri_map.get_mut(&edge) {
                debug_assert!(triangle_set.len() == 1);
                triangle_set.insert(triangle_index);
            } else {
                let mut triangle_set = HashSet::with_capacity(1);
                triangle_set.insert(triangle_index);
                self.edge_tri_map.insert(edge, triangle_set);
            }
        }
    }

    pub fn remove_triangle(&mut self, triangle_index: TriIdx) {
        debug_assert!(self.tri_edge_map.contains_key(&triangle_index));

        //update edge-triangle mapping
        for edge in &self.tri_edge_map[&triangle_index] {
            if let Some(triangle_set) = self.edge_tri_map.get_mut(edge) {
                let existed = triangle_set.remove(&triangle_index);
                debug_assert!(existed);
            }
            if self.edge_tri_map.contains_key(edge) && self.edge_tri_map[edge].is_empty() {
                self.edge_tri_map.remove(edge);
            }
        }

        //update triangle-edge mapping
        self.tri_edge_map.remove(&triangle_index);
    }

    pub fn get_edges(&self, triangle_index: TriIdx) -> Vec<&Edge> {
        self.tri_edge_map[&triangle_index].iter().collect()
    }

    pub fn get_triangles(&self, edge: &Edge) -> Vec<TriIdx> {
        self.edge_tri_map[edge].iter().copied().collect()
    }

    pub fn neighbouring_triangles(&self, triangle_index: TriIdx) -> Vec<TriIdx> {
        let mut neighbours = Vec::with_capacity(3);

        for edge in self.get_edges(triangle_index) {
            for tri in self.get_triangles(edge) {
                if tri != triangle_index {
                    neighbours.push(tri);
                }
            }
        }
        neighbours
    }
}

#[cfg(test)]
mod tests {
    use crate::Triangle;

    use super::{Edge, TriangleEdgeMapping};

    #[test]
    fn add_triangle() {
        let mut mapping = TriangleEdgeMapping::new();
        let triangles = [Triangle::new(0, 1, 2)];
        mapping.add_triangle(0, &triangles);

        assert_eq!(mapping.tri_edge_map.len(), 1);
        assert_eq!(mapping.tri_edge_map[&0].len(), 3);
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(0, 1)));
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(1, 2)));
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(2, 0)));

        assert_eq!(mapping.edge_tri_map.len(), 3);
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(0, 1)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(1, 2)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(2, 0)));
        assert_eq!(mapping.edge_tri_map[&Edge::new(0, 1)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(1, 2)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(2, 0)].len(), 1);
        assert!(mapping.edge_tri_map[&Edge::new(0, 1)].contains(&0));
        assert!(mapping.edge_tri_map[&Edge::new(1, 2)].contains(&0));
        assert!(mapping.edge_tri_map[&Edge::new(2, 0)].contains(&0));
    }

    #[test]
    fn remove_triangle() {
        let mut mapping = TriangleEdgeMapping::new();
        let triangles = [Triangle::new(0, 1, 2)];
        mapping.add_triangle(0, &triangles);

        mapping.remove_triangle(0);
        assert_eq!(mapping.edge_tri_map.len(), 0);
        assert_eq!(mapping.tri_edge_map.len(), 0);
    }

    #[test]
    fn add_two_triangles() {
        let mut mapping = TriangleEdgeMapping::new();
        let triangles = [Triangle::new(0, 1, 2), Triangle::new(0, 2, 3)];
        mapping.add_triangle(0, &triangles);
        mapping.add_triangle(1, &triangles);

        assert_eq!(mapping.tri_edge_map.len(), 2);
        assert_eq!(mapping.tri_edge_map[&0].len(), 3);
        assert_eq!(mapping.tri_edge_map[&1].len(), 3);
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(0, 1)));
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(1, 2)));
        assert!(mapping.tri_edge_map[&0].contains(&Edge::new(2, 0)));
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(0, 2)));
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(2, 3)));
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(3, 0)));

        assert_eq!(mapping.edge_tri_map.len(), 5);
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(0, 1)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(1, 2)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(2, 0)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(2, 3)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(3, 0)));

        assert_eq!(mapping.edge_tri_map[&Edge::new(0, 1)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(1, 2)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(2, 0)].len(), 2);
        assert_eq!(mapping.edge_tri_map[&Edge::new(2, 3)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(3, 0)].len(), 1);

        assert!(mapping.edge_tri_map[&Edge::new(0, 1)].contains(&0));
        assert!(mapping.edge_tri_map[&Edge::new(1, 2)].contains(&0));
        assert!(mapping.edge_tri_map[&Edge::new(2, 0)].contains(&0));
        assert!(mapping.edge_tri_map[&Edge::new(2, 0)].contains(&1));
        assert!(mapping.edge_tri_map[&Edge::new(2, 3)].contains(&1));
        assert!(mapping.edge_tri_map[&Edge::new(3, 0)].contains(&1));
    }

    #[test]
    fn add_two_remove_one_triangle() {
        let mut mapping = TriangleEdgeMapping::new();
        let triangles = [Triangle::new(0, 1, 2), Triangle::new(0, 2, 3)];
        mapping.add_triangle(0, &triangles);
        mapping.add_triangle(1, &triangles);

        mapping.remove_triangle(0);

        assert_eq!(mapping.tri_edge_map.len(), 1);
        assert_eq!(mapping.tri_edge_map[&1].len(), 3);
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(0, 2)));
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(2, 3)));
        assert!(mapping.tri_edge_map[&1].contains(&Edge::new(3, 0)));

        assert_eq!(mapping.edge_tri_map.len(), 3);
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(2, 0)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(2, 3)));
        assert!(mapping.edge_tri_map.contains_key(&Edge::new(3, 0)));

        assert_eq!(mapping.edge_tri_map[&Edge::new(2, 0)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(2, 3)].len(), 1);
        assert_eq!(mapping.edge_tri_map[&Edge::new(3, 0)].len(), 1);

        assert!(mapping.edge_tri_map[&Edge::new(2, 0)].contains(&1));
        assert!(mapping.edge_tri_map[&Edge::new(2, 3)].contains(&1));
        assert!(mapping.edge_tri_map[&Edge::new(3, 0)].contains(&1));
    }
}
