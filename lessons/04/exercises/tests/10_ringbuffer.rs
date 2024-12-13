//! Run this file with `cargo test --test 10_ringbuffer`.

//! TODO: Write a ringbuffer data structure.
//!
//! Ringbuffer is a fixed-size queue that is often used to buffer data before it can be processed.
//! It can be used e.g. to remember a set of keyboard presses until the CPU can deal with them.
//! When the buffer is full, the oldest value in the queue should be overwritten.
//!
//! A ringbuffer works with two pointers (or indices, which will be much simpler).
//! One marks the end of data, while the other marks the start. The data in the ringbuffer is everything
//! from `start` to `end`. However, this range may *wrap around* the end of the array. For example:
//!
//! ```
//! _               V unused V
//! [1, 2, 3, 1, 3, _, _, _, _, 1, 2, 3, 3, 2]
//! _            ^ end          ^ start
//! ```
//!
//! - Make a `struct Ringbuffer<T>`, *which is generic* over a type T, so it can store arbitrary data.
//!    - If you need to perform certain operations on the generic type, make sure to include the
//!      corresponding generic bounds.
//! - Implement an associated function `new` that creates a new ringbuffer of a specific size.
//! - Implement a method `enqueue` which adds a new item to the ringbuffer, and optionally returns the
//!   value overwritten by this enqueue if the buffer was full.
//! - Implement a method `dequeue` which returns the next value in the queue (if there is any).
//! - Implement a method `peek` which returns a reference to the value that would be dequeued next
//!   (but doesn't actually dequeue it).
//! - Implement a method `len` which returns the number of elements that are actually present in the queue.
//! - Implement a method `iter`, which returns a read-only iterator of the ringbuffer items.
//!
//! Note: this data structure more or less corresponds to the `VecDeque` data structure from the
//! standard library. However, the goal is to implement a very simple version of it from scratch,
//! rather than use it directly. Use a `Vec` to implement the ringbuffer instead.
//!
//! Other than that, it's up to you how you implement it. Below you can find some methods for
//! ringbuffers implementations:
//! https://www.snellman.net/blog/archive/2016-12-13-ring-buffers/
//! I would recommend the "Array + two unmasked indices" approach (you don't need to worry about the
//! "power of two" capacity limitation).
//! You don't need to use `unsafe`, pointers, or anything special, just use the simplest data
//! representation that you can think of. The implementation should fit within 120 lines of code.

/// TODO(bonus): write a simple "DSL" (domain-specific language) that can
/// be used to test the ringbuffer in a more visual way.
/// For example, you could create a function that "renders" the ringbuffer
/// to a string like this:
/// ```
/// [1, 2, 3, 1, 3, _, _, _, _, 1, 2, 3, 3, 2]
/// _            ^ end          ^ start
/// ```
/// and then you can rewrite the tests to assert that the ringbuffer
/// has a specific state represented by that string. This would make
/// checking that the ringbuffer behaves as you expect much easier.
/// The render function can assume that the buffer only contains unsigned
/// integers smaller than 10, to simplify the rendering code.
///
/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::RingBuffer;

    #[test]
    fn empty_length() {
        assert_eq!(RingBuffer::<i32>::new(10).len(), 0)
    }

    #[test]
    fn one_length() {
        let mut rb = RingBuffer::new(10);
        rb.enqueue(5);
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn keep_fixed_size() {
        let mut rb = RingBuffer::new(10);
        for i in 0..20 {
            rb.enqueue(i);
        }

        assert_eq!(rb.len(), 10);
    }

    #[test]
    fn enqueue_dequeue_one() {
        let mut rb = RingBuffer::new(10);
        rb.enqueue(5);
        assert_eq!(rb.dequeue(), Some(5));
    }

    #[test]
    fn enqueue_dequeue_two() {
        let mut rb = RingBuffer::new(10);
        rb.enqueue(5);
        rb.enqueue(6);
        assert_eq!(rb.dequeue(), Some(5));
        assert_eq!(rb.dequeue(), Some(6));
    }

    #[test]
    fn enqueue_dequeue_almost_all() {
        let mut rb = RingBuffer::new(10);
        for i in 0..9 {
            rb.enqueue(i * 10);
        }
        for i in 0..9 {
            assert_eq!(rb.dequeue(), Some(i * 10));
        }
    }

    #[test]
    fn enqueue_dequeue_all() {
        let mut rb = RingBuffer::new(10);
        for i in 0..10 {
            assert_eq!(rb.len(), i);
            rb.enqueue(i * 10);
        }
        assert_eq!(rb.len(), 10);
        for i in 0..10 {
            assert_eq!(rb.dequeue(), Some(i * 10));
        }
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn enqueue_dequeue_all_overwrite() {
        let mut rb = RingBuffer::new(10);
        for i in 0..100 {
            assert_eq!(rb.len(), i.min(10));
            rb.enqueue(i);
        }
        assert_eq!(rb.len(), 10);
        for i in 0..10 {
            assert_eq!(rb.dequeue(), Some(i + 90));
        }
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn enqueue_dequeues_first_when_full() {
        let mut rb = RingBuffer::new(10);
        for i in 0..10 {
            assert!(rb.enqueue(i).is_none());
        }
        for i in 0..10 {
            assert_eq!(rb.enqueue(5), Some(i));
        }
    }

    #[test]
    fn dequeue_empty() {
        let mut rb = RingBuffer::<i32>::new(10);
        assert_eq!(rb.dequeue(), None)
    }

    #[test]
    fn zero_size() {
        let mut rb = RingBuffer::<i32>::new(0);
        assert_eq!(rb.enqueue(5), None);
        assert_eq!(rb.dequeue(), None);
        assert_eq!(rb.peek(), None);
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn one_size() {
        let mut rb = RingBuffer::<i32>::new(1);
        assert_eq!(rb.len(), 0);
        rb.enqueue(0);
        assert_eq!(rb.len(), 1);
        for i in 0..1000 {
            assert_eq!(rb.enqueue(i + 1), Some(i));
        }
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.dequeue(), Some(1000));
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn peek_empty() {
        assert_eq!(RingBuffer::<i32>::new(10).peek(), None);
    }

    #[test]
    fn peek_one() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(10);
        assert_eq!(rb.peek(), Some(&10));
    }

    #[test]
    fn peek_more() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(10);
        rb.enqueue(11);
        assert_eq!(rb.peek(), Some(&10));
    }

    #[test]
    fn peek_after_dequeue() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(10);
        rb.enqueue(11);
        rb.dequeue();
        assert_eq!(rb.peek(), Some(&11));
    }

    #[test]
    fn peek_multiple_times() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(10);
        assert_eq!(rb.peek(), Some(&10));
        assert_eq!(rb.peek(), Some(&10));
    }

    #[test]
    fn peek_after_reempty() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(10);
        rb.enqueue(11);
        rb.dequeue();
        rb.dequeue();
        assert_eq!(rb.peek(), None);
    }

    #[test]
    fn different_type() {
        let mut rb = RingBuffer::<String>::new(10);
        rb.enqueue(String::from("foo"));
        rb.enqueue(String::from("bar"));
        assert_eq!(rb.dequeue(), Some(String::from("foo")));
        assert_eq!(rb.dequeue(), Some(String::from("bar")));
    }

    #[test]
    fn iter_empty() {
        let rb = RingBuffer::<i32>::new(10);
        assert_eq!(rb.iter().next(), None);
    }

    #[test]
    fn iter_single() {
        let mut rb = RingBuffer::<i32>::new(10);
        rb.enqueue(5);
        assert_eq!(rb.iter().next(), Some(&5));
    }

    #[test]
    fn iter_full() {
        let mut rb = RingBuffer::<i32>::new(4);
        for i in 1..=4 {
            rb.enqueue(i);
        }
        assert_eq!(rb.iter().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    }

    #[test]
    fn iter_after_overwrite() {
        let mut rb = RingBuffer::<i32>::new(4);
        for i in 1..=6 {
            rb.enqueue(i);
        }
        assert_eq!(rb.iter().collect::<Vec<_>>(), vec![&3, &4, &5, &6]);
    }
}
