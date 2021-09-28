use crate::types::PointIdx;

#[derive(PartialEq, Eq, Clone)]
pub struct Triangle {
    pub index0: PointIdx,
    pub index1: PointIdx,
    pub index2: PointIdx,
}

impl Triangle {
    pub fn new(index0: PointIdx, index1: PointIdx, index2: PointIdx) -> Self {
        Self {
            index0,
            index1,
            index2,
        }
    }

    #[cfg(test)]
    pub(crate) fn equivalent(&self, other: &Triangle) -> bool {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        // checks if two triangles are indexing the same three points, but not necessarily in the same order
        let t0 =
            HashSet::<PointIdx>::from_iter([self.index0, self.index1, self.index2].iter().cloned());
        let t1 = HashSet::<PointIdx>::from_iter(
            [other.index0, other.index1, other.index2].iter().cloned(),
        );
        let intersection: Vec<_> = t0.intersection(&t1).collect();
        intersection.len() == 3
    }
}

// terser than derived Debug
impl core::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.index0, self.index1, self.index2)
    }
}
