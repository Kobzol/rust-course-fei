//! Run this file with `cargo test --test 01_binary_tree`.

//! TODO: Implement a binary search tree that stores an arbitrary type that can be compared.
//! Implement the following methods:
//! - `height`: return the height of the tree
//! - `size`: return the number of items stored in the tree
//! - `for_each_mut`: take a function that will be applied to each value stored in the tree. Note
//!   that it should be possible to modify the values in the three using this function.
//!   You will probably run into an ownership issue using the naive approach. Can you think of a way
//!   how to make sure that the passed function can be used both for the left and the right child?
//! - `insert`: insert a new item into the tree. This function will return a new tree containing the
//!   inserted item.
//! - `contains`: returns true if the tree contains the passed value.
//!
//! `height`, `size` and `for_each_mut` should be available on all types `T`, while `insert` and
//! `contains` can only be implemented for certain special types.
//!
//! Note that there are many ways how a binary tree could be represented in Rust.
//! The representation used here has the advantage that left/right child pointers are always valid,
//! so we don't have to deal with `Option`s. On the other hand, we have to represent all leaves with
//! an explicit node, which is a bit annoying. Every solution has trade-offs :)
//!
//! TODO(bonus): write an iterator for the tree that returns the items in sorted order. The iterator
//! should be as lazy as possible. It can store multiple items inside of it, but don̈́'t just prefill
//! the whole tree into a Vec and call that an iterator.


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn size_empty() {
        assert_eq!(leaf::<usize>().size(), 0);
    }

    #[test]
    fn size_single() {
        assert_eq!(node_leaf(0).size(), 1);
    }

    #[test]
    fn size_more() {
        assert_eq!(node(0, node_leaf(1), node_leaf(2)).size(), 3);
    }

    #[test]
    fn size_large() {
        assert_eq!(
            node(
                4,
                node(2, node_leaf(1), node_leaf(3)),
                node(6, node_leaf(5), node_leaf(7))
            )
            .size(),
            7
        );
    }

    #[test]
    fn insert_1() {
        assert_eq!(
            node(5, node(4, node_leaf(3), leaf()), leaf()),
            leaf().insert(5).insert(4).insert(3)
        );
    }

    #[test]
    fn insert_2() {
        assert_eq!(
            node(5, node_leaf(4), node_leaf(52)),
            leaf().insert(5).insert(4).insert(5).insert(52)
        );
    }

    #[test]
    fn insert_3() {
        assert_eq!(
            node(
                10,
                node(4, node_leaf(3), node_leaf(5)),
                node(12, node_leaf(11), leaf())
            ),
            leaf()
                .insert(10)
                .insert(4)
                .insert(12)
                .insert(11)
                .insert(5)
                .insert(3)
        )
    }

    #[test]
    fn contains_0() {
        assert!(!leaf().contains(&3))
    }

    #[test]
    fn contains_1() {
        assert!(leaf().insert(3).insert(1).insert(9).insert(5).contains(&3));
    }

    #[test]
    fn contains_2() {
        assert!(!leaf().insert(3).insert(1).insert(9).insert(5).contains(&7));
    }

    #[test]
    fn height() {
        assert_eq!(
            node(
                10,
                node(4, node(3, node_leaf(5), leaf()), node_leaf(5)),
                node(12, node_leaf(11), leaf())
            )
            .height(),
            4
        )
    }

    #[test]
    fn height_2() {
        let tree = node(
            10,
            node(
                5,
                node(3, node(1, node_leaf(0), node_leaf(2)), node_leaf(4)),
                node(7, node_leaf(6), node_leaf(8)),
            ),
            node(12, node_leaf(11), leaf()),
        );

        assert_eq!(tree.height(), 5)
    }

    #[test]
    fn contains_different_type() {
        assert!(leaf()
            .insert("abc")
            .insert("por")
            .insert("fei")
            .insert("das")
            .contains(&"das"));
    }

    #[test]
    fn apply_closure() {
        let mut tree = node(1, node_leaf(0), node_leaf(2));
        tree.for_each_mut(|node| *node += 1);
        insta::assert_debug_snapshot!(tree, @r###"
        Node {
            value: 2,
            left: Node {
                value: 1,
                left: Leaf,
                right: Leaf,
            },
            right: Node {
                value: 3,
                left: Leaf,
                right: Leaf,
            },
        }
        "###);
    }

    #[test]
    fn apply_closure_mut() {
        let mut tree = node(1, node_leaf(0), node_leaf(2));
        let mut iterated = 0;
        tree.for_each_mut(|node| {
            *node += 1;
            iterated += 1;
        });
        assert_eq!(iterated, 3);
    }

    #[test]
    fn closure_non_ord() {
        #[derive(Debug)]
        struct Foo(u32);

        let mut tree = node(Foo(0), node_leaf(Foo(1)), node_leaf(Foo(2)));
        tree.for_each_mut(|v| v.0 += 1);
        insta::assert_debug_snapshot!(tree, @r###"
        Node {
            value: Foo(
                1,
            ),
            left: Node {
                value: Foo(
                    2,
                ),
                left: Leaf,
                right: Leaf,
            },
            right: Node {
                value: Foo(
                    3,
                ),
                left: Leaf,
                right: Leaf,
            },
        }
        "###);
    }

    // Bonus tests
    /*
    #[test]
    fn iter_empty() {
        assert_eq!(leaf::<u32>().iter().next(), None);
    }

    #[test]
    fn iter_single() {
        assert_eq!(node_leaf(1).iter().collect::<Vec<_>>(), vec![&1]);
    }

    #[test]
    fn iter_left_heavy() {
        assert_eq!(
            build_tree(&[5, 4, 3, 2, 1]).iter().collect::<Vec<_>>(),
            vec![&1, &2, &3, &4, &5]
        );
    }

    #[test]
    fn iter_right_heavy() {
        assert_eq!(
            build_tree(&[1, 2, 3, 4, 5]).iter().collect::<Vec<_>>(),
            vec![&1, &2, &3, &4, &5]
        );
    }

    #[test]
    fn iter_backtrack_at_leaf() {
        assert_eq!(
            build_tree(&[5, 2, 4, 3]).iter().collect::<Vec<_>>(),
            vec![&2, &3, &4, &5]
        );
    }

    #[test]
    fn iter_backtrack() {
        assert_eq!(
            build_tree(&[5, 2, 1, 4, 3]).iter().collect::<Vec<_>>(),
            vec![&1, &2, &3, &4, &5]
        );
    }

    #[test]
    fn iter_backtrack_right() {
        assert_eq!(
            build_tree(&[5, 2, 1, 3, 4]).iter().collect::<Vec<_>>(),
            vec![&1, &2, &3, &4, &5]
        );
    }

    #[test]
    fn iter_backtrack_through_root() {
        assert_eq!(
            build_tree(&[5, 2, 8, 6, 7]).iter().collect::<Vec<_>>(),
            vec![&2, &5, &6, &7, &8]
        );
    }
    */
    fn leaf<T>() -> BinaryTree<T> {
        BinaryTree::Leaf
    }

    fn node<T>(t: T, s: BinaryTree<T>, l: BinaryTree<T>) -> BinaryTree<T> {
        BinaryTree::Node {
            value: t,
            left: Box::new(s),
            right: Box::new(l),
        }
    }

    fn node_leaf<T>(t: T) -> BinaryTree<T> {
        BinaryTree::Node {
            value: t,
            left: Box::new(leaf()),
            right: Box::new(leaf()),
        }
    }

    fn build_tree(items: &[u32]) -> BinaryTree<u32> {
        let mut tree = leaf();
        for item in items {
            tree = tree.insert(*item);
        }
        tree
    }
}
