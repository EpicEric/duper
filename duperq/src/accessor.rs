use std::{iter, ops::Bound};

use duper::{DuperKey, DuperValue};

use crate::filter::DuperFilter;

type AccessorReturn<'value> = Box<dyn Iterator<Item = &'value DuperValue<'value>> + 'value>;

pub(crate) trait DuperAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value>;
}

// Flattened accessor

pub(crate) struct FlattenedAccessor(pub(crate) Vec<Box<dyn DuperAccessor>>);

impl DuperAccessor for FlattenedAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        self.0.iter().fold(
            Box::new(iter::once(value)) as AccessorReturn<'value>,
            |values, accessor| Box::new(values.flat_map(|value| accessor.access(value))),
        )
    }
}

// Base accessors

pub(crate) struct SelfAccessor;

impl DuperAccessor for SelfAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        Box::new(iter::once(value))
    }
}

pub(crate) struct FieldAccessor(pub(crate) DuperKey<'static>);

impl DuperAccessor for FieldAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        match value {
            DuperValue::Object { inner: object, .. } => Box::new(object.get(&self.0).into_iter()),
            DuperValue::Array { inner: array, .. } => Box::new(array.iter().flat_map(|duper| {
                if let DuperValue::Object { inner: object, .. } = &duper {
                    object.get(&self.0)
                } else {
                    None
                }
            })),
            _ => Box::new(iter::empty()),
        }
    }
}

pub(crate) struct IndexAccessor(pub(crate) usize);

impl DuperAccessor for IndexAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        match value {
            DuperValue::Array { inner: array, .. } => Box::new(array.get(self.0).into_iter()),
            DuperValue::Tuple { inner: tuple, .. } => Box::new(tuple.get(self.0).into_iter()),
            _ => Box::new(iter::empty()),
        }
    }
}

pub(crate) struct ReverseIndexAccessor(pub(crate) usize);

impl DuperAccessor for ReverseIndexAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        match value {
            DuperValue::Array { inner: array, .. } => {
                if let Some(index) = array.len().checked_sub(self.0) {
                    Box::new(array.get(index).into_iter())
                } else {
                    Box::new(iter::empty())
                }
            }
            DuperValue::Tuple { inner: tuple, .. } => {
                if let Some(index) = tuple.len().checked_sub(self.0) {
                    Box::new(tuple.get(index).into_iter())
                } else {
                    Box::new(iter::empty())
                }
            }
            _ => Box::new(iter::empty()),
        }
    }
}

pub(crate) struct RangeIndexAccessor {
    pub(crate) start: Bound<usize>,
    pub(crate) end: Bound<usize>,
}

impl DuperAccessor for RangeIndexAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        if let DuperValue::Array { inner: array, .. } = value {
            let start = match self.start {
                Bound::Included(i) => i,
                Bound::Excluded(i) => i + 1,
                Bound::Unbounded => 0,
            };
            Box::new(array.iter().skip(start).take(match self.end {
                Bound::Included(i) => (i + 1) - start,
                Bound::Excluded(i) => i - start,
                Bound::Unbounded => usize::MAX,
            }))
        } else {
            Box::new(iter::empty())
        }
    }
}

pub(crate) struct AnyAccessor;

impl DuperAccessor for AnyAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        if let DuperValue::Array { inner: array, .. } = value {
            Box::new(array.iter())
        } else {
            Box::new(iter::empty())
        }
    }
}

pub(crate) struct FilterAccessor(pub(crate) Box<dyn DuperFilter>);

impl DuperAccessor for FilterAccessor {
    fn access<'accessor: 'value, 'value>(
        &'accessor self,
        value: &'value DuperValue<'value>,
    ) -> AccessorReturn<'value> {
        if let DuperValue::Array { inner: array, .. } = value {
            Box::new(array.iter().filter(|value| self.0.filter(value)))
        } else {
            Box::new(iter::empty())
        }
    }
}
