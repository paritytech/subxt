use std::fmt::Display;

/// A map that associates ranges of keys with values.
#[derive(Debug, Clone)]
pub struct RangeMap<K, V> {
    // (range_start, range_ended, value). This is
    // guaranteed to be sorted and have non-overlapping ranges.
    mapping: Vec<(K, K, V)>,
}

impl<K: Clone + Copy + Display + PartialOrd + Ord, V> RangeMap<K, V> {
    /// Build an empty [`RangeMap`] as a placeholder.
    pub fn empty() -> Self {
        RangeMap {
            mapping: Vec::new(),
        }
    }

    /// Build a [`RangeMap`].
    pub fn builder() -> RangeMapBuilder<K, V> {
        RangeMapBuilder {
            mapping: Vec::new(),
        }
    }

    /// Return the value whose key is within the range, or None if not found.
    pub fn get(&self, key: K) -> Option<&V> {
        let idx = self
            .mapping
            .binary_search_by_key(&key, |&(start, end, _)| {
                if key >= start && key < end {
                    key
                } else {
                    start
                }
            })
            .ok()?;

        self.mapping.get(idx).map(|(_, _, val)| val)
    }
}

/// A builder for constructing a [`RangeMap`]. Use [``RangeMap::builder()`] to create one.
#[derive(Debug, Clone)]
pub struct RangeMapBuilder<K, V> {
    mapping: Vec<(K, K, V)>,
}

impl<K: Clone + Copy + Display + PartialOrd + Ord, V> RangeMapBuilder<K, V> {
    /// Try to add a range, mapping block numbers to a spec version.
    ///
    /// Returns an error if the range is empty or overlaps with an existing range.
    pub fn try_add_range(
        &mut self,
        start: K,
        end: K,
        val: V,
    ) -> Result<&mut Self, RangeMapError<K>> {
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        if start == end {
            return Err(RangeMapError::EmptyRange(start));
        }

        if let Some(&(s, e, _)) = self.mapping.iter().find(|&&(s, e, _)| start < e && end > s) {
            return Err(RangeMapError::OverlappingRanges {
                proposed: (start, end),
                existing: (s, e),
            });
        }

        self.mapping.push((start, end, val));
        Ok(self)
    }

    /// Add a range of blocks with the given spec version.
    ///
    /// # Panics
    ///
    /// This method will panic if the range is empty or overlaps with an existing range.
    pub fn add_range(mut self, start: K, end: K, val: V) -> Self {
        if let Err(e) = self.try_add_range(start, end, val) {
            panic!("{e}")
        }
        self
    }

    /// Finish adding ranges and build the [`RangeMap`].
    pub fn build(mut self) -> RangeMap<K, V> {
        self.mapping.sort_by_key(|&(start, _, _)| start);
        RangeMap {
            mapping: self.mapping,
        }
    }
}

/// An error that can occur when calling [`RangeMapBuilder::try_add_range()`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RangeMapError<K: Display> {
    /// An error indicating that the proposed block range is empty.
    #[error("Block range cannot be empty: start and end values must be different, but got {} for both", .0)]
    EmptyRange(K),
    /// An error indicating that the proposed block range overlaps with an existing one.
    #[error("Overlapping block ranges are not allowed: proposed range is {}..{}, but we already have {}..{}", proposed.0, proposed.1, existing.0, existing.1)]
    OverlappingRanges { proposed: (K, K), existing: (K, K) },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rangemap_get() {
        let spec_version = RangeMap::builder()
            .add_range(0, 100, 1)
            .add_range(100, 200, 2)
            .add_range(200, 300, 3)
            .build();

        assert_eq!(spec_version.get(0), Some(&1));
        assert_eq!(spec_version.get(50), Some(&1));
        assert_eq!(spec_version.get(100), Some(&2));
        assert_eq!(spec_version.get(150), Some(&2));
        assert_eq!(spec_version.get(200), Some(&3));
        assert_eq!(spec_version.get(250), Some(&3));
        assert_eq!(spec_version.get(300), None);
    }

    #[test]
    fn test_rangemap_set() {
        let mut spec_version = RangeMap::builder()
            .add_range(0, 100, 1)
            .add_range(200, 300, 3);

        assert_eq!(
            spec_version.try_add_range(99, 130, 2).unwrap_err(),
            RangeMapError::OverlappingRanges {
                proposed: (99, 130),
                existing: (0, 100),
            }
        );
        assert_eq!(
            spec_version.try_add_range(170, 201, 2).unwrap_err(),
            RangeMapError::OverlappingRanges {
                proposed: (170, 201),
                existing: (200, 300),
            }
        );
    }
}
