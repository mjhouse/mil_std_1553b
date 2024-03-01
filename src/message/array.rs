/// An array with a fixed maximum size
///
/// This struct has a maximum size and a
/// limit that can be increased or decreased
/// within that size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<T, const S: usize>
where
    T: Clone + Copy,
{
    initial: T,
    count: usize,
    limit: usize,
    data: [T; S],
}

impl<T, const S: usize> Array<T, S>
where
    T: Clone + Copy,
{
    /// Create a new array instance
    ///
    /// The default value for the array elements is
    /// given as a param, and the maximum size of the
    /// array is given by the const generic `S`.
    pub fn new(v: T) -> Self {
        Self {
            initial: v,
            count: 0,
            limit: S,
            data: [v; S],
        }
    }

    /// Get the current maximum size
    ///
    /// This value will be equal to or less
    /// than `S`.
    pub fn size(&self) -> usize {
        self.limit
    }

    /// Get the number of elements in the array
    ///
    /// This value will be equal to or less
    /// than [Array::size].
    pub fn len(&self) -> usize {
        self.count
    }

    /// Get the number of remaining positions in the array
    ///
    /// This value will be equal to the size - len.
    pub fn space(&self) -> usize {
        self.size() - self.len()
    }

    /// Get an element from the array by index
    ///
    /// If the given index is larger than the number
    /// of elements stored, this method returns None.
    pub fn get(&self, index: usize) -> Option<&T> {
        if !self.is_empty() && index < self.len() {
            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Append an element to the array
    ///
    /// If the array is full, this method does
    /// nothing. See [Array::is_full] for details.
    pub fn push(&mut self, value: T) {
        if !self.is_full() {
            self.data[self.count] = value;
            self.count += 1;
        }
    }

    /// Count the number of elements that satisfy a predicate
    pub fn count<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&&T) -> bool,
    {
        self.data.iter().filter(predicate).count()
    }

    /// Change the maximum size of the array
    ///
    /// The given value must be greater than 0 and
    /// less than or equal to the *initial* maximum size
    /// given by the const generic `S`.
    pub fn resize(&mut self, value: usize) {
        if value <= S {
            self.limit = value;

            // if current size is larger than limit then
            // truncate the array and update the count
            if self.count > self.limit {
                for i in self.limit..self.count {
                    self.data[i] = self.initial;
                }
                self.count = self.limit;
            }
        }
    }

    /// Get the first element of the array
    ///
    /// If the array has no elements, this method will
    /// return None.
    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    /// Get the last element of the array
    ///
    /// If the array has no elements, this method will
    /// return None.
    pub fn last(&self) -> Option<&T> {
        self.get(self.count.saturating_sub(1))
    }

    /// Remove all elements from the array
    pub fn clear(&mut self) {
        self.data = [self.initial; S];
        self.count = 0;
    }

    /// Check if the array is full
    pub fn is_full(&self) -> bool {
        self.count == self.limit
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_new_len() {
        let array: Array<u8, 10> = Array::new(0);
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_array_resized_smaller() {
        let mut array: Array<u8, 10> = Array::new(0);

        array.push(1);
        array.push(2);
        array.push(3);
        array.push(4);
        array.push(5);

        array.resize(2);

        assert_eq!(array.len(), 2); // the current length
        assert_eq!(array.size(), 2); // the maximum length
        assert_eq!(array.first(), Some(&1));
        assert_eq!(array.last(), Some(&2));

        // verify extra elements were reinitialized
        assert_eq!(array.data[2], 0);
    }

    #[test]
    fn test_array_resized_larger() {
        let mut array: Array<u8, 10> = Array::new(0);
        array.resize(2);

        array.push(1);
        array.push(2);

        array.resize(4);

        assert_eq!(array.len(), 2); // the current length
        assert_eq!(array.size(), 4); // the maximum length
        assert_eq!(array.first(), Some(&1));
        assert_eq!(array.last(), Some(&2));
    }

    #[test]
    fn test_array_fill_len() {
        let mut array: Array<u8, 10> = Array::new(0);

        array.push(1);
        array.push(2);
        array.push(3);
        array.push(4);
        array.push(5);

        assert_eq!(array.len(), 5);
    }

    #[test]
    fn test_array_overflow_len() {
        let mut array: Array<u8, 2> = Array::new(0);

        array.push(1);
        array.push(2);
        array.push(3);

        assert_eq!(array.len(), 2);
    }

    #[test]
    fn test_array_get_values() {
        let mut array: Array<u8, 2> = Array::new(0);

        array.push(1);
        array.push(2);

        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(1), Some(&2));
        assert_eq!(array.get(2), None);
    }

    #[test]
    fn test_array_get_first_and_last() {
        let mut array: Array<u8, 3> = Array::new(0);

        array.push(1);
        array.push(2);
        array.push(3);

        assert_eq!(array.first(), Some(&1));
        assert_eq!(array.last(), Some(&3));
    }

    #[test]
    fn test_array_clear() {
        let mut array: Array<u8, 3> = Array::new(0);

        array.push(1);
        array.push(2);
        array.push(3);

        assert_eq!(array.len(), 3);

        array.clear();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_array_is_empty() {
        let mut array: Array<u8, 3> = Array::new(0);
        assert!(array.is_empty());

        array.push(1);
        assert!(!array.is_empty());

        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_is_full() {
        let mut array: Array<u8, 2> = Array::new(0);
        assert!(!array.is_full());

        array.push(1);
        array.push(2);

        assert!(array.is_full());

        array.clear();
        assert!(!array.is_full());
    }
}
