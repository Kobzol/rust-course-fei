//! Run this file with `cargo test --test 04_memory_map`.

//! TODO: implement a sparse memory map
//! This is the final boss of this week :)
//!
//! Imagine that you're building a CPU emulator and you need to implement RAM memory.
//! The first obvious solution would be to use `Vec<u8>`.
//! But that's no good if you're going to emulate a 64-bit address space :)
//! You'll need to implement a sparse map instead.
//! The next solution that you could think of is using `HashMap<Address, u8>`.
//! That is indeed sparse, but it has a lot of overhead, because we store each byte separately.
//! A better approach could be to store buffers (spans of bytes) (i.e. `HashMap<Address, Vec<u8>>`,
//! to reduce the number of entries in the map.
//! However, using a hashmap becomes problematic when storing byte spans. Imagine that you store a
//! buffer `[1, 2, 3, 4]` at address `42`, and then you try to read two bytes from the address `44`.
//! How do you find out what is stored at address `44` when the hashmap key is `42`?
//! You would need to iterate the whole hashmap sequentially, which is not great, as each memory
//! read/write would have complexity O(n).
//!
//! There is a useful data structure in Rust called `BTreeMap`, which has a hashmap-like API,
//! but it is backed by a tree, so its operations have O(log(n)) complexity. An important benefit
//! is that it stores its keys in an ordered manner. Therefore, we can perform range queries on the
//! keys, which is exactly what we need here.
//!
//! Implement two methods, `read` and `write`, on the [MemoryMap] below.
//! Start with `read`, as it is simpler, and then continue on to `write`.
//! The map has to uphold an important invariant: every address that holds a written byte has to be
//! stored in the map at most one time! There may be no overlapping buffers, that would surely be
//! a bug.
//!
//! The memory map read and write operations should have complexity O(log n).
//! Avoid going through the [`BTreeMap`] sequentially, that's not the point :)
//! Note that this is an algorithmization exercise, you don't need interior mutability or smart
//! pointers.
//!
//! Hint: take a look at the [`BTreeMap::range`] method. Think about how to use this method to
//! perform a range query like "which buffer lies at an address or below/above it".
//!
//! Btw, it would probably be better to store `Box<[u8]>` instead of `Vec<u8>`, because we do not
//! need dynamic reallocation. But I didn't want to complicate this further.
//!
//! TODO(bonus): implement defragmentation
//! After performing several writes, it is possible that there will be separate buffers stored after
//! one another. For example, if we write the following buffers at the following addresses:
//! - (4, [1])
//! - (5, [2])
//! - (6, [3])
//! - (7, [4])
//!
//! The map will unnecessarily store four buffers, instead of storing just `(4, [1, 2, 3, 4])`.
//! Implement a simple defragmentation pass after each write, which will merge all such consecutive
//! buffers together.
//! Note that merging can only happen at the place where a write was just performed.
//! Do not go through the whole map when defragmenting!

use std::collections::BTreeMap;

/// This is just a type alias, not a new type.
/// It can be useful to start with it if you want to give a new name
/// to an existing type, but don't want to deal with newtype wrapping.
type Address = usize;

#[derive(Default)]
struct MemoryMap {
    map: BTreeMap<Address, Vec<u8>>,
}

/// Allow creating the memory map from an iterator of (address, bytes) tuples.
impl<T> From<T> for MemoryMap
where
    T: Iterator<Item = (Address, Vec<u8>)>,
{
    fn from(value: T) -> Self {
        Self {
            map: value.collect(),
        }
    }
}

impl MemoryMap {
    /// TODO: implement a method that reads the given number of bytes from the specified
    /// address. Note that it is possible that the read will span multiple separate buffers!
    /// In that case, the contents of such buffers have to be combined together.
    /// For example, if you read at `40` with count `8`, and the map stores `[1, 2, 3, 4]` at
    /// `40` and `[5, 6, 7, 8]` at `44`, the read should return `[1, 2, 3, 4, 5, 6, 7, 8]`.
    ///
    /// If **any** bytes are missing in the specified address range
    /// `[address, address + count)`, return `None`.
    /// For example, in the situation described above, if the second buffer started at `45` and
    /// not at `44`, there would be a missing byte at address `44`, and `read` would return `None`.
    ///
    /// Remember: this method should have complexity O(log(n)), where `n` is the number of buffers
    /// stored in the map.
    fn read(&self, address: Address, count: usize) -> Option<Vec<u8>> {
        todo!()
    }

    /// TODO: implement a method that writes the given byte buffer at the specified address.
    /// You will need to overwrite (or even outright remove) existing buffers (or their parts)
    /// if there is any overlap between existing buffers and the specified address range
    /// `[address, address + buffer.len())`.
    ///
    /// If you are not implementing defragmentation, then the following behaviour should be applied
    /// when overwriting buffers:
    /// write(40, [1, 2, 3, 4])
    /// write(42, [5, 6, 7, 8])
    ///
    /// should result in the following buffer state:
    /// 40 -> [1, 2]
    /// 42 -> [5, 6, 7, 8]
    ///
    /// Remember: this method should have complexity O(log(n)).
    fn write(&mut self, address: Address, buffer: Vec<u8>) {
    }
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{Address, MemoryMap};

    #[test]
    fn read_empty() {
        let map = make_map(&[]);
        assert_eq!(map.read(40, 8), None);
        assert_eq!(map.read(0, 1000), None);
    }

    // Hope you have a lot of RAM if you implement the map densely :)
    #[test]
    fn read_large_address() {
        let map = make_map(&[(12654768764387, &[1, 2, 3, 4])]);
        assert_eq!(map.read(12654768764387, 4), Some(vec![1, 2, 3, 4]));
    }

    // Use with caution, the test can spend *a lot* of time printing the error message
    // in debug mode if you get it wrong :)
    // #[test]
    // fn read_large_data() {
    //     let buffer = vec![42; 64 * 1024 * 1024];
    //     let map = make_map(&[(12654768764387, &buffer)]);
    //     assert_eq!(map.read(12654768764387, 4), Some(vec![42, 42, 42, 42]));
    // }

    #[test]
    fn read_missing_data_at_start() {
        let map = make_map(&[(40, &[1, 2, 3, 4])]);
        for addr in 36..40 {
            assert_eq!(map.read(addr, 4), None);
        }
    }

    #[test]
    fn read_missing_data_at_end() {
        let map = make_map(&[(40, &[1, 2, 3, 4])]);
        for addr in 41..45 {
            assert_eq!(map.read(addr, 4), None);
        }
    }

    #[test]
    fn read_missing_data_in_the_middle() {
        let map = make_map(&[(40, &[1, 2, 3, 4]), (45, &[5, 6, 7, 8])]);
        assert_eq!(map.read(40, 10), None);
        for addr in 41..44 {
            assert_eq!(map.read(addr, 4), None);
        }
    }

    #[test]
    fn read_missing_with_data_before() {
        let map = make_map(&[(40, &[1, 2, 3, 4])]);
        assert_eq!(map.read(50, 4), None);
    }

    #[test]
    fn read_missing_with_data_after() {
        let map = make_map(&[(60, &[1, 2, 3, 4])]);
        assert_eq!(map.read(50, 4), None);
    }

    #[test]
    fn read_missing_with_data_around() {
        let map = make_map(&[(40, &[1, 2, 3, 4]), (60, &[1, 2, 3, 4])]);
        assert_eq!(map.read(50, 4), None);
    }

    #[test]
    fn read_full() {
        let map = make_map(&[(40, &[1, 2, 3, 4])]);
        assert_eq!(map.read(40, 4), Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn read_partial() {
        let map = make_map(&[(40, &[1, 2, 3, 4])]);
        assert_eq!(map.read(40, 1), Some(vec![1]));
        assert_eq!(map.read(40, 2), Some(vec![1, 2]));
        assert_eq!(map.read(40, 3), Some(vec![1, 2, 3]));
        assert_eq!(map.read(41, 1), Some(vec![2]));
        assert_eq!(map.read(41, 2), Some(vec![2, 3]));
        assert_eq!(map.read(41, 3), Some(vec![2, 3, 4]));
        assert_eq!(map.read(42, 1), Some(vec![3]));
        assert_eq!(map.read(42, 2), Some(vec![3, 4]));
        assert_eq!(map.read(43, 1), Some(vec![4]));
    }

    #[test]
    fn read_high_end() {
        let mut items: Vec<u8> = vec![];
        for number in 0..100000u64 {
            items.push((number % 256) as u8);
        }

        let map = make_map(&[(10, &items)]);
        assert_eq!(map.read(50000, 8), Some(vec![70, 71, 72, 73, 74, 75, 76, 77]));
    }

    #[test]
    fn coalesce_neighbour_reads() {
        let map = make_map(&[(40, &[1, 2, 3, 4]), (44, &[5]), (45, &[6, 7, 8])]);
        assert_eq!(map.read(40, 8), Some(vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(map.read(41, 7), Some(vec![2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(map.read(44, 3), Some(vec![5, 6, 7]));
    }

    #[test]
    fn write_no_lower_allocation() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        check_map(map, &[(40, &[1, 2, 3, 4])]);
    }

    #[test]
    fn write_lower_allocation_outside_of_range() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(50, vec![5, 6, 7, 8]);
        check_map(map, &[(40, &[1, 2, 3, 4]), (50, &[5, 6, 7, 8])]);
    }

    #[test]
    fn write_higher_allocation_outside_of_range() {
        let mut map = MemoryMap::default();
        map.write(60, vec![1, 2, 3, 4]);
        map.write(40, vec![5, 6, 7, 8]);
        check_map(map, &[(40, &[5, 6, 7, 8]), (60, &[1, 2, 3, 4])]);
    }

    #[test]
    fn write_consume_existing_allocations() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(44, vec![1, 2, 3, 4]);
        map.write(49, vec![1, 2, 3, 4]);
        map.write(55, vec![1, 2, 3, 4]);

        let arr = vec![
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37,
        ];
        map.write(32, arr.clone());

        check_map(map, &[(32, &arr)]);
    }

    #[test]
    fn write_partial_overwrite_right() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(41, vec![5, 6, 7, 8]);

        assert_eq!(map.read(40, 5), Some(vec![1, 5, 6, 7, 8]));
    }

    #[test]
    fn write_partial_overwrite_left() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(38, vec![5, 6, 7, 8]);

        assert_eq!(map.read(38, 6), Some(vec![5, 6, 7, 8, 3, 4]));
    }

    // TODO(bonus): The tests below are bonus tests that will only work if you
    // implement defragmentation.
    /*
    #[test]
    fn lower_allocation_neighbour() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(44, vec![5, 6, 7, 8]);
        check_map(map, &[(40, &[1, 2, 3, 4, 5, 6, 7, 8])]);
    }

    #[test]
    fn higher_allocation_neighbour() {
        let mut map = MemoryMap::default();
        map.write(44, vec![5, 6, 7, 8]);
        map.write(40, vec![1, 2, 3, 4]);
        check_map(map, &[(40, &[1, 2, 3, 4, 5, 6, 7, 8])]);
    }

    #[test]
    fn defragment_add_in_the_middle() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(48, vec![9, 10, 11, 12]);
        map.write(44, vec![5, 6, 7, 8]);
        check_map(map, &[(40, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]);
    }

    #[test]
    fn overwrite_allocation_at_the_start() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(42, vec![5, 6, 7, 8]);
        check_map(map, &[(40, &[1, 2, 5, 6, 7, 8])]);
    }

    #[test]
    fn overwrite_allocation_at_the_end() {
        let mut map = MemoryMap::default();
        map.write(40, vec![1, 2, 3, 4]);
        map.write(38, vec![5, 6, 7, 8]);
        check_map(map, &[(38, &[5, 6, 7, 8, 3, 4])]);
    }
    */
    fn make_map(data: &[(Address, &[u8])]) -> MemoryMap {
        let data = data.iter().map(|(addr, bytes)| (*addr, bytes.to_vec()));
        data.into()
    }

    fn check_map(map: MemoryMap, data: &[(Address, &[u8])]) {
        assert_eq!(
            map.map.len(),
            data.len(),
            "expected {} element(s) for memory map, got {}",
            data.len(),
            map.map.len(),
        );
        for (address, bytes) in data {
            let map_buffer = map.map.get(address).expect("No buffer found");
            assert_eq!(map_buffer, bytes);
        }
    }
}
