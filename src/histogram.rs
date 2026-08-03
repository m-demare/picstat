use std::collections::BTreeMap;

pub struct Histogram<V: PartialOrd + PartialEq> {
    items: BTreeMap<V, usize>,
}

impl<V: PartialOrd + PartialEq> Histogram<V> {
    pub(crate) fn new() -> Self {
        Self { items: Default::default() }
    }
}

