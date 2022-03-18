use cosmwasm_std::StdResult;

/// Type alias of a function that fetch a page given as first value an
/// offset and as second argument how many items to fetch from the given offset.
pub type Fetcher<'a, T> = Box<dyn Fn(u64, u64) -> StdResult<Page<T>> + 'a>;

/// A page of a elements.
pub struct Page<T> {
    /// List of elements present in the page.
    pub items: Vec<T>,
}

/// Iterator that fetch paginated elements and allow to iterate over
/// them as a continuous sequence of elements.
pub struct PageIterator<'a, T: Clone> {
    /// Function to fetch a page
    fetch_page: Fetcher<'a, T>,
    /// Optional cached page
    current_page: Option<Page<T>>,
    /// Position to the current element of the current page
    page_item_index: usize,
    /// Current offset
    offset: u64,
    /// Size of each page
    page_size: u64,
    /// Tells if the iterator has iterated over all the items.
    consumed: bool,
}

impl<'a, T: Clone> PageIterator<'a, T> {
    /// Creates a new iterator that fetch paginated items and allow to iterate over them
    /// as a continuous sequence of elements.
    ///
    /// * `fetch_page` - Function that fetch a page given an offset and the amount of items to
    /// fetch from the given offset
    /// * `page_size` - Size of each page.
    ///
    /// # Examples
    ///
    /// ```
    /// use desmos_std::iter::page_iterator::{PageIterator, Page};
    ///
    /// // Creates an iterator that return the numbers from 0 to 99.
    /// let it = PageIterator::new(Box::new(|offset: u64, items: u64| {    ///
    ///     Ok(Page {
    ///         items: if offset >= 100 {
    ///             Vec::new()
    ///         } else {
    ///             (offset..offset + items).collect()
    ///         }
    ///     })
    /// }), 10);
    /// ```
    pub fn new(fetch_page: Fetcher<'a, T>, page_size: u64) -> PageIterator<'a, T> {
        PageIterator {
            fetch_page,
            current_page: None,
            page_item_index: 0,
            offset: 0,
            page_size,
            consumed: false,
        }
    }
}

impl<'a, T: Clone> Iterator for PageIterator<'a, T> {
    type Item = StdResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the iterator is consumed just return None.
        if self.consumed {
            return None;
        }

        // If we have a page and we have iterate over all the items let's check
        // if we need to fetch a new page or we have iterate over all the possible items
        if let Some(page) = &self.current_page {
            if page.items.len() == self.page_item_index {
                // If the previously fetched page had less then
                // page_size elements means there will not be another page
                // to avoid fetch an empty page just mark the iterator as
                // consumed and return None
                if page.items.len() < self.page_size as usize {
                    self.consumed = true;
                    // Release the memory of the current page
                    self.current_page = None;
                    return None;
                } else {
                    // Clear the current page.
                    self.current_page = None;
                }
            }
        }

        match &self.current_page {
            None => {
                // No page available fetch a new one
                let fetch_result: StdResult<Page<T>> =
                    (self.fetch_page)(self.offset, self.page_size);

                match fetch_result {
                    Ok(page) => {
                        // No items on the new page so no more items, flag the iterator as consumed.
                        if page.items.is_empty() {
                            self.consumed = true;
                            None
                        } else {
                            // Items available, update the offset field for the next fetch
                            self.offset += page.items.len() as u64;
                            // Clone the first item.
                            let first_item = page.items[0].clone();
                            // Save the fetched page
                            self.current_page = Some(page);
                            // Update the index for the next iteration.
                            self.page_item_index = 1;
                            // Return the first item of the new fetched page
                            Some(Ok(first_item))
                        }
                    }
                    // An error occurred, propagate it to the caller.
                    Err(e) => {
                        // Set the iterator as consumed to prevent other invocations.
                        self.consumed = true;
                        Some(Err(e))
                    }
                }
            }
            Some(page) => {
                // A page is available, return the current item index
                let result = Some(Ok(page.items[self.page_item_index].clone()));
                // Update the iterator index.
                self.page_item_index += 1;
                result
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::iter::page_iterator::{Page, PageIterator};
    use cosmwasm_std::StdError;

    #[test]
    fn test_iterations_without_errors() {
        let it = PageIterator::new(
            Box::new(|offset, items| {
                if offset == 20 {
                    Ok(Page { items: Vec::new() })
                } else {
                    Ok(Page {
                        items: (offset..offset + items).collect(),
                    })
                }
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
        let mut it =
            PageIterator::<i32>::new(Box::new(|_, _| Err(StdError::generic_err("ERROR :("))), 10);

        let item = it.next();

        // First item should be some with the error that happen fetching the page.
        assert!(item.is_some());
        assert!(item.unwrap().is_err());

        // Since an error occurred when fetching the page the second item should be None
        let item = it.next();
        assert!(item.is_none());
    }

    #[test]
    fn test_iterations_with_partial_page() {
        let it = PageIterator::new(Box::new(|_, _| Ok(Page { items: vec![1, 2] })), 10);

        let mut it_counter = 0;
        for element in it {
            element.unwrap();
            it_counter += 1;
        }

        assert_eq!(2, it_counter);
    }
}
