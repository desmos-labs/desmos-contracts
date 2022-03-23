use cosmwasm_std::StdResult;

/// Type alias of a function that fetch a page given as first argument an optional key
/// that references the next page to fetch and as second argument how many items to fetch.
/// If the first argument is None means that this function should fetch the first page.
pub type Fetcher<'a, T, K> = Box<dyn Fn(Option<K>, u64) -> StdResult<Page<T, K>> + 'a>;

/// A page of elements.
pub struct Page<T, K> {
    /// List of elements present in the page.
    pub items: Vec<T>,
    /// Optional key to the next page to fetch.
    /// If this is None means that there aren't other pages to fetch.
    pub next_page_key: Option<K>,
}

/// Iterator that fetch paginated elements and allow to iterate over
/// them as a continuous sequence of elements.
pub struct PageIterator<'a, T: Clone, K: Clone> {
    /// Function to fetch a page.
    fetch_page: Fetcher<'a, T, K>,
    /// Optional cached page.
    current_page: Option<Page<T, K>>,
    /// Position to the current element of the current page.
    page_item_index: usize,
    /// Size of each page.
    page_size: u64,
    /// Tells if the iterator has iterated over all the items.
    consumed: bool,
}

impl<'a, T: Clone, K: Clone> PageIterator<'a, T, K> {
    /// Creates a new iterator that fetch paginated items and allow to iterate over them
    /// as a continuous sequence of elements.
    ///
    /// * `fetch_page` - Function that fetch the pages.
    /// * `page_size` - Size of each page.
    ///
    /// # Examples
    ///
    /// ```
    /// use desmos_std::iter::page_iterator::{PageIterator, Page};
    ///
    /// // Creates an iterator that return the numbers from 0 to 19.
    /// let it: PageIterator<u64, u64> = PageIterator::new(
    ///            Box::new(|key, limit| {
    ///                let start = key.unwrap_or(0);
    ///                Ok(Page {
    ///                    items: (start..start + limit).collect(),
    ///                    next_page_key: if start + limit >= 20 {
    ///                        None
    ///                    } else {
    ///                        Some(start + limit)
    ///                    }
    ///                })
    ///            }),
    ///            10,
    ///        );
    /// ```
    pub fn new(fetch_page: Fetcher<'a, T, K>, page_size: u64) -> PageIterator<'a, T, K> {
        PageIterator {
            fetch_page,
            current_page: None,
            page_item_index: 0,
            page_size,
            consumed: false,
        }
    }
}

impl<'a, T: Clone, K: Clone> Iterator for PageIterator<'a, T, K> {
    type Item = StdResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the iterator is consumed just return None
        if self.consumed {
            return None;
        }

        if self.current_page.is_none()
            || self.current_page.as_ref().unwrap().items.len() == self.page_item_index
        {
            // Get the next page key
            let next_key = self
                .current_page
                .as_ref()
                .and_then(|page| page.next_page_key.as_ref().cloned());

            if next_key.is_none() && self.current_page.is_some() {
                // We have fetched at least on page but there isn't a new page to fetch,
                // to prevent an empty fetch just mark this iterator as consumed and return None
                self.consumed = true;
                self.current_page = None;
                return None;
            }

            // Fetch a new page
            let fetch_result: StdResult<Page<T, K>> = (self.fetch_page)(next_key, self.page_size);

            match fetch_result {
                Ok(page) => {
                    // No items on the new page so no more items, flag the iterator as consumed
                    if page.items.is_empty() {
                        self.consumed = true;
                        None
                    } else {
                        // Clone the first item
                        let first_item = page.items[0].clone();
                        // Save the fetched page
                        self.current_page = Some(page);
                        // Update the index for the next iteration
                        self.page_item_index = 1;
                        // Return the first item of the new fetched page
                        Some(Ok(first_item))
                    }
                }
                // An error occurred, propagate it to the caller
                Err(e) => {
                    // Set the iterator as consumed to prevent other invocations
                    self.consumed = true;
                    Some(Err(e))
                }
            }
        } else {
            // A page is available and we don't have iterated over all the items
            let page = self.current_page.as_ref().unwrap();
            let result = Some(Ok(page.items[self.page_item_index].clone()));
            // Update the iterator index
            self.page_item_index += 1;
            result
        }
    }
}

#[cfg(test)]
mod test {
    use crate::iter::page_iterator::{Page, PageIterator};
    use cosmwasm_std::StdError;

    #[test]
    fn test_iterations_without_errors() {
        let it: PageIterator<u64, u64> = PageIterator::new(
            Box::new(|key, limit| {
                let start = key.unwrap_or(0);
                Ok(Page {
                    items: (start..start + limit).collect(),
                    next_page_key: if start + limit >= 20 {
                        None
                    } else {
                        Some(start + limit)
                    },
                })
            }),
            10,
        );

        let mut it_counter = 0;
        for element in it {
            element.unwrap();
            it_counter += 1;
        }

        assert_eq!(20, it_counter);
    }

    #[test]
    fn test_iterations_with_error() {
        let mut it = PageIterator::<i32, i32>::new(
            Box::new(|_, _| Err(StdError::generic_err("ERROR :("))),
            10,
        );

        let item = it.next();

        // First item should be some with the error that happen fetching the page
        assert!(item.is_some());
        assert!(item.unwrap().is_err());

        // Since an error occurred when fetching the page the second item should be None
        // to signal that the iterator don't have more items.
        let item = it.next();
        assert!(item.is_none());
    }

    #[test]
    fn test_iterations_with_empty_page() {
        // Create an that just return an empty page
        let mut it: PageIterator<i32, i32> = PageIterator::new(
            Box::new(|_, _| {
                Ok(Page {
                    items: Vec::new(),
                    next_page_key: None,
                })
            }),
            10,
        );

        // First items should be None since we returned an empty page
        let first = it.next();
        assert!(first.is_none());

        // Second items should also be None since the page don't contains any element
        let second = it.next();
        assert!(second.is_none());
    }

    #[test]
    fn test_iterations_with_partial_page() {
        // Create an iterator that just return a page of 2 elements
        // even if we requested a page of 10 elements.
        let mut it: PageIterator<i32, i32> = PageIterator::new(
            Box::new(|_, _| {
                Ok(Page {
                    items: vec![1, 2],
                    next_page_key: None,
                })
            }),
            10,
        );

        // First items should be 1
        let first = it.next();
        assert_eq!(1, first.unwrap().unwrap());

        // Second items should be 1
        let second = it.next();
        assert_eq!(2, second.unwrap().unwrap());

        // Third items should be None since we returned only 2 elements from the iterator.
        let third = it.next();
        assert_eq!(None, third)
    }

    #[test]
    fn test_iterations_with_second_partial_page() {
        // Create an iterator that just return a page of 2 elements
        // even if we requested a page of 10 elements.
        let it: PageIterator<u64, u64> = PageIterator::new(
            Box::new(|key, limit| {
                let range_start = key.unwrap_or(0);
                let range_end = if range_start == 0 {
                    range_start + limit
                } else {
                    range_start + 2
                };
                Ok(Page {
                    items: (range_start..range_end).collect(),
                    next_page_key: if range_start == 0 {
                        Some(range_end)
                    } else {
                        None
                    },
                })
            }),
            10,
        );

        let mut total_count = 0;
        for element in it {
            element.unwrap();
            total_count += 1;
        }
        // We should have 12 elements
        assert_eq!(12, total_count);
    }
}
