//! Run this file with `cargo test --test 02_hashmap_constructor`.

//! TODO: implement a macro named `make_map`, which will construct a hash map out of key-value
//! pairs.
//! The macro will receive an arbitrary number of key-value pairs separated by `=>` (see tests).
//! The macro will construct a hashmap, prefill it with the key-value pairs and return the map.
//! If the keys contain a duplicate, the macro should panic.
//!
//! The HashMap should be preallocated for the specific number of arguments passed to the macro
//! (see [std::collections::HashMap::with_capacity]). You will need to count the number of
//! arguments passed to the macro to figure out the preallocation size.
//!
//! Again, the macro should be hygienic in regards to the `HashMap` type.
#![allow(unused)]


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let map: HashMap<u32, u32> = make_map!();
    }

    #[test]
    fn single() {
        let map: HashMap<u32, bool> = make_map!(
            5 => true
        );
        assert_eq!(map.get(&5), Some(&true));
    }

    #[test]
    fn multiple() {
        let map: HashMap<u32, u32> = make_map!(
            1 => 2,
            3 => 4,
            5 => 8
        );
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));
        assert_eq!(map.get(&5), Some(&8));
    }

    #[test]
    fn different_type() {
        let map: HashMap<String, &str> = make_map!(
            "foo".to_string() => "bar",
            "baz".to_string() => "foo"
        );
        assert_eq!(map.get("foo"), Some(&"bar"));
        assert_eq!(map.get("baz"), Some(&"foo"));
    }

    #[test]
    #[should_panic]
    fn panic_on_duplicate() {
        let _: HashMap<u64, u32> = make_map!(
            1 => 2,
            2 => 3,
            1 => 4
        );
    }

    #[test]
    fn hygiene() {
        struct HashMap;

        let map: std::collections::HashMap<u32, u16> = make_map!(
            1 => 2,
            2 => 3
        );
    }
}
