const BIT_MASK: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// Returns whether bit at position `i` in `data` is set or not.
///
/// # Safety
/// `i >= data.len() * 8` results in undefined behavior
#[inline]
pub unsafe fn get_bit_unchecked(data: &[u8], i: usize) -> bool {
    (*data.as_ptr().add(i >> 3) & BIT_MASK[i & 7]) != 0
}

/// An iterator over bits according to the [LSB](https://en.wikipedia.org/wiki/Bit_numbering#Least_significant_bit),
/// i.e. the bytes `[4u8, 128u8]` correspond to `[false, false, true, false, ..., true]`.
#[derive(Debug, Clone)]
pub struct BitmapIter<'a> {
    bytes: &'a [u8],
    index: usize,
    end: usize,
}

impl<'a> BitmapIter<'a> {
    pub fn new(slice: &'a [u8], offset: usize, len: usize) -> Self {
        // example:
        // slice.len() = 4
        // offset = 9
        // len = 23
        // result:
        let bytes = &slice[offset / 8..];
        // bytes.len() = 3
        let index = offset % 8;
        // index = 9 % 8 = 1
        let end = len + index;
        // end = 23 + 1 = 24
        assert!(end <= bytes.len() * 8);
        // maximum read before UB in bits: bytes.len() * 8 = 24
        // the first read from the end is `end - 1`, thus, end = 24 is ok

        Self { bytes, index, end }
    }
}

impl<'a> Iterator for BitmapIter<'a> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            return None;
        }
        let old = self.index;
        self.index += 1;
        // See comment in `new`
        Some(unsafe { get_bit_unchecked(self.bytes, old) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.end - self.index;
        (exact, Some(exact))
    }
}
