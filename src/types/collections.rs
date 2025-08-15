//! Collection-related type definitions for the SDK.
//! 
//! This module defines the types used to represent collections
//! and data structures.

use serde::{Deserialize, Serialize};

/// A collection of items with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection<T> {
    /// The items in the collection.
    pub items: Vec<T>,
    /// The total count of items.
    pub total_count: usize,
    /// The current page number.
    pub page: usize,
    /// The page size.
    pub page_size: usize,
    /// Whether there are more pages.
    pub has_more: bool,
}

impl<T> Collection<T> {
    /// Create a new collection.
    pub fn new(items: Vec<T>, total_count: usize, page: usize, page_size: usize) -> Self {
        let has_more = (page + 1) * page_size < total_count;
        Self {
            items,
            total_count,
            page,
            page_size,
            has_more,
        }
    }

    /// Get the number of items in the current page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an item by index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Iterate over the items.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
