use bitmaps::{get_bit_unchecked, BitmapIter};

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_baseline(slice: &[u8]) {
    let iter = BitmapIter::new(slice, 0, slice.len() * 8).map(|x| x as usize);
    let a: usize = iter.sum();
    assert!(a > 0);
}

/// An iterator over bits according to the [LSB](https://en.wikipedia.org/wiki/Bit_numbering#Least_significant_bit),
/// i.e. the bytes `[4u8, 128u8]` correspond to `[false, false, true, false, ..., true]`.
#[derive(Debug, Clone)]
pub struct BitmapIter1<'a> {
    bytes: &'a [u8],
    index: usize,
    end: usize,
}

impl<'a> BitmapIter1<'a> {
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

impl<'a> Iterator for BitmapIter1<'a> {
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

fn bench_sum_count1(slice: &[u8]) {
    let iter = (0..slice.len() * 8).map(|i| unsafe { get_bit_unchecked(slice, i) } as usize);
    let a: usize = iter.sum();
    assert!(a > 0);
}

fn bench_sum_count2(slice: &[u8]) {
    let iter = BitmapIter1::new(slice, 0, slice.len() * 8).map(|x| x as usize);
    let a: usize = iter.sum();
    assert!(a > 0);
}

fn add_benchmark(c: &mut Criterion) {
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);

        let slice = (0..size).map(|x| (x % 255) as u8).collect::<Vec<_>>();

        c.bench_function(&format!("baseline 2^{}", log2_size), |b| {
            b.iter(|| bench_baseline(&slice))
        });

        c.bench_function(&format!("get_bit1 2^{}", log2_size), |b| {
            b.iter(|| bench_sum_count1(&slice))
        });

        c.bench_function(&format!("get_bit2 2^{}", log2_size), |b| {
            b.iter(|| bench_sum_count2(&slice))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
