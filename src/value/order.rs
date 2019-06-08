use std::cmp::{Ord, Ordering, PartialOrd};

use crate::to_vec;
use crate::value::Value;

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        let a = to_vec(self).expect("lhs serialization succeeds");
        let b = to_vec(other).expect("rhs serialization succeeds");
        a.cmp(&b)
    }
}
