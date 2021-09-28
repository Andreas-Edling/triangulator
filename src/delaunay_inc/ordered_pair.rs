use crate::types::TriIdx;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct OrderedPair {
    a: TriIdx,
    b: TriIdx,
}

impl OrderedPair {
    pub fn new(first: TriIdx, second: TriIdx) -> Self {
        if first > second {
            OrderedPair {
                a: second,
                b: first,
            }
        } else {
            OrderedPair {
                a: first,
                b: second,
            }
        }
    }
}
