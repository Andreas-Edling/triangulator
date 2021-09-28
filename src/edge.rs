use crate::types::PointIdx;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
pub(crate) struct Edge {
    pub index_0: PointIdx,
    pub index_1: PointIdx,
}

impl Edge {
    pub(crate) fn new(index_0: PointIdx, index_1: PointIdx) -> Self {
        if index_0 < index_1 {
            Self { index_0, index_1 }
        } else {
            Self {
                index_0: index_1,
                index_1: index_0,
            }
        }
    }
}

// Terser than derived debug
impl core::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.index_0, self.index_1)
    }
}
