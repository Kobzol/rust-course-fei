//! Run this file with `cargo test --test 03_factorio`.

//! TODO: Implement a Factorio-like assembly pipeline
//! Welcome to the Space Age!
//!
//! Implement a struct called `FactorioBuilder`, which will allow configuring a *pipeline* of
//! *assembly lines*.
//! A pipeline has an input (a queue) that receives *items* and an output (a queue) that produces
//! items. Between these two, there is a set of assembly lines connected in a series.
//! Each assembly line represents a computation that receives an item, does something and
//! produces an item, which it then forwards to the following assembly line (or
//! to the output, if it is the last assembly line in the pipeline).
//!
//! ## Creation of the builder
//! The builder should offer a `new` method, which will create a builder representing an empty
//! pipeline, which simply forwards all input items directly to the output.
//! The `new` method receives a number representing queue size, which will be applied to all
//! assembly lines and also the input/output queue.
//!
//! Note that the type of items that are sent as inputs to the pipeline might be different than the
//! type of outputs produced by the pipeline. However, the initial empty pipeline **must** have the
//! same input and output type (because the items in an empty pipeline are directly forwarded to
//! the output), so you should enable the `new` method only if that is the case.
//!
//! ## Execution rules of assembly lines
//! Below, you will find four kinds of assembly lines that you should implement.
//! Here is a list of shared properties for all of them:
//! - Each assembly line has an input queue of a certain size.
//!   The queue is filled with items coming from a previous assembly line (or from the input).
//! - When an assembly line receives an item, it performs some computation on it, and then forwards
//!   the result to the next assembly line in the pipeline, or to the output queue, if the assembly
//!   line is at the very end of the pipeline.
//! - Each assembly line should run in parallel w.r.t. the other assembly lines.
//! - Items cannot "skip ahead" in the pipeline. Each item has to be either sent to the output
//!   in the same order as it was sent on the input, or discarded (using `filter/filter_map`).
//! - When you send an input into the pipeline, it should apply backpressure. In other words, if the
//!   input queue is full, the send should block. The same behavior should be applied for all
//!   intermediate queues between individual assembly lines.
//!
//! ## What kinds of assembly lines should be implemented
//! Implement the following four kinds of assembly lines.
//! `map/filter/filter_map/fork_join` should be methods on `FactorioBuilder`, which consume the
//! builder and return a new builder, possibly with different input/output types.
//! The returned builder should contain the corresponding assembly line **at its end**.
//! In other words, the previous output becomes the input of the newly added assembly line, and the
//! newly added assembly line becomes the output of the pipeline.
//!
//! In the following ASCII diagrams, `<item X>` means "an item with type X".
//!
//! ### map
//! Adds a `map` assembly line to the end of the pipeline.
//! `map` receives a function that should be applied to each item that goes through it.
//! The function will produce a new item that will be forwarded further through the pipeline.
//!
//! ```
//! <item A> --> map(A) --> <item B>
//! ```
//!
//! Note that `map` might change the output type of the pipeline.
//!
//! ### filter
//! Adds a `filter` assembly line to the end of the pipeline.
//! `filter` receives a function that should be applied to each item that goes through it.
//! The function will receive a shared reference to the input item and return a boolean.
//! If it returns `true`, then the input item is forwarded further through the pipeline.
//! If it returns `false`, then the item will be discarded.
//!
//! ```
//!                          [true]
//! <item A> --> filter(&A) -------> <item A>
//!                 |
//!                 | [false]
//!                 |
//!                 x (discard)
//! ```
//!
//! ### filter_map
//! Adds a `filter_map` assembly line to the end of the pipeline.
//! `filter_map` receives a function that should be applied to each item that goes through it.
//! The function will receive an item and return an `Option` of a possibly different type.
//! If it returns `Some`, then the item wrapped within the Option is forwarded further through the
//! pipeline.
//! If it returns `None`, then the item will be discarded.
//!
//! ```
//!                            [Some(B)]
//! <item A> --> filter_map(A) -------> <item B>
//!                  |
//!                  | [None]
//!                  |
//!                  x (discard)
//! ```
//!
//! Note that `filter_map` might change the output type of the pipeline.
//!
//! ### fork_join
//! Adds a `fork_join` assembly line to the end of the pipeline.
//! `fork_join` creates `N` parallel internal assembly lines that will process each input item
//! separately, **in parallel**. Each input item will be processed by all `N` lines.
//! Each internal assembly line has an input queue with size that is stored in the builder.
//! `fork_join` receives a fork function that will run on each internal assembly line, the number
//! of internal assembly lines (`N`), and a join function.
//! The fork function will receive a reference to the input item and a zero-based index of the
//! internal assembly line that executes the fork function.
//!
//! The results of the internal assembly lines will be joined together using the provided join
//! function, which will receive a `Vec` of the intermediate results.
//! This combined result will be then passed forward through the pipeline.
//! The internal assembly lines produce an item of some type, which is then
//! sent to the join function. The output of the join function then determines what type will be
//! passed to the rest of the pipeline.
//!
//! TODO(bonus): The internal lines need to be synchronized. They can handle
//! additional items even if some other line is still processing the previous item, but the join
//! function needs to receive the intermediate results **in the original order**.
//!
//! ```
//!                         --> fork(&A, 0)   --> <item R> --v
//!                         |                                |
//! <item A> --> fork_join                                   --> join(Vec<R>) --> <item B>
//!                         |                                |
//!                         --> fork(&A, 1)   --> <item R> --^
//!                         |                                |
//!                         ...                              |
//!                         |                                |
//!                         --> fork(&A, N-1) --> <item R> --^
//! ```
//!
//! Note that `fork_join` might change the output type of the pipeline.
//!
//! Note: this assembly line will require you to create a bunch of channels.
//! Don't be afraid of it :)
//!
//! ## Creation of the pipeline
//! Once all assembly lines of the pipeline have been configured, the builder should allow creating
//! the actual pipeline using a `build` method. This method returns a struct representing the
//! pipeline, a channel that can be used to send inputs to the pipeline, and a channel that can be
//! used to read the outputs from the end of the pipeline.
//!
//! **All threads should be created when configuring the pipeline. After the `build` function
//! returns a completed pipeline, no new threads should be spawned when items go through the
//! pipeline!**
//!
//! ## Closing of the pipeline
//! The created pipeline should have a `close` method, which consumes it and waits until all
//! assembly lines have terminated.
//! Think carefully about the receive and send channels and what happens when they are closed.
//!
//! See tests for more details. Some tests also contain simple ASCII diagrams that render the
//! pipelines created in the test.
//!
//! **DO NOT** use Rayon or any other crate, implement the pipeline manually using only libstd.
//!
//! Hint: When writing parallel code, you might run into deadlocks (which will be presented as a
//! "blank screen" with no output and maybe a spinning wheel :) ).
//! If you want to see interactive output during the execution of a test, you can add stderr print
//! statements (e.g. using `eprintln!`) and run tests with `cargo test -- --nocapture`, so that you
//! see the output interactively. Alternatively, you can try to use a debugger (e.g. GDB, LLDB or
//! GDB/LLDB integrated within an IDE).

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{FactorioBuilder, Pipeline};
    use rand::Rng;
    use std::collections::{HashSet, VecDeque};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::TrySendError;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    #[test]
    fn different_type_after_build() {
        let (factorio, _, _): (Pipeline, _, _) = FactorioBuilder::<u32, u32>::new(4).build();
        factorio.close();
    }

    /// I --> O
    #[test]
    fn passthrough() {
        let (factorio, tx, rx) = FactorioBuilder::new(4).build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), 5);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn queue_size_zero() {
        let (factorio, tx, rx) = FactorioBuilder::new(0).build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), 5);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn passthrough_non_empty() {
        let (factorio, tx, rx) = FactorioBuilder::new(4).build();

        tx.send(5).unwrap();

        // Drop the reader while the queue is non-empty
        // In this case, the assembly line should still continue to work, even if it has nowhere
        // to send the results.
        drop(rx);

        tx.send(5).unwrap();

        drop(tx);
        factorio.close();
    }

    /// I --> Map --> O
    #[test]
    fn map_simple() {
        let (factorio, tx, rx) = FactorioBuilder::new(4).map(|v| v + 1).build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), 6);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_no_clone() {
        struct Foo(u32);

        let (factorio, tx, rx) = FactorioBuilder::<Foo, Foo>::new(4)
            .map(|v| Foo(v.0 + 1))
            .build();

        tx.send(Foo(5)).unwrap();
        assert_eq!(rx.recv().unwrap().0, 6);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_no_skip_shead() {
        let count = AtomicUsize::new(0);

        // Even though the processing of the first item will take longer than the processing of the
        // second item, the second item should **not** skip ahead of the first one.
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .map(move |v| {
                if count.fetch_add(1, Ordering::SeqCst) == 0 {
                    std::thread::sleep(Duration::from_secs(1));
                }
                v
            })
            .build();

        tx.send(5).unwrap();
        tx.send(10).unwrap();
        assert_eq!(rx.recv().unwrap(), 5);
        assert_eq!(rx.recv().unwrap(), 10);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_change_type() {
        let (factorio, tx, rx) = FactorioBuilder::<u32, u32>::new(4)
            .map(|v| v.to_string())
            .build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), "5".to_string());

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_nested() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .map(|v| v + 1)
            .map(|v| v * 2)
            .build();

        tx.send(50).unwrap();
        assert_eq!(rx.recv().unwrap(), 102);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_non_empty() {
        let (factorio, tx, rx) = FactorioBuilder::new(4).map(|v| v + 1).build();
        tx.send(5).unwrap();

        // Drop the reader while the queue is non-empty
        // In this case, the assembly line should still continue to work, even if it has nowhere
        // to send the results.
        drop(rx);

        tx.send(5).unwrap();

        drop(tx);
        factorio.close();
    }

    #[test]
    fn map_is_bounded() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .map(|v| {
                std::thread::sleep(Duration::from_secs(1));
                v + 1
            })
            .build();

        for _ in 0..4 {
            tx.send(5).unwrap();
        }
        assert_eq!(tx.try_send(4).unwrap_err(), TrySendError::Full(4));

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn map_different_thread() {
        let thread_id = std::thread::current().id();
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .map(move |v| {
                assert_ne!(std::thread::current().id(), thread_id);
                v + 1
            })
            .build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), 6);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    #[should_panic]
    fn map_panic() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .map(move |_| panic!("Assembly failed"))
            .build();

        tx.send(5).unwrap();

        drop(tx);
        drop(rx);
        // The code should panic here
        factorio.close();
    }

    #[test]
    fn map_assemble_after_panic() {
        let counter = AtomicUsize::new(0);
        let (factorio, tx, rx) = FactorioBuilder::new(2)
            .map(move |v| {
                // Here, we panic after the first call
                // However, the data that we already sent forward in the pipeline should still
                // be processed
                if counter.fetch_add(1, Ordering::SeqCst) != 0 {
                    panic!("Assembly failed")
                }
                v * 2
            })
            .map(|v| {
                std::thread::sleep(Duration::from_secs(1));
                v + 1
            })
            .build();

        tx.send(5).unwrap();
        tx.send(6).unwrap();
        assert_eq!(rx.recv().unwrap(), 11);

        drop(tx);
        drop(rx);

        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| factorio.close())).is_err()
        );
    }

    /// I --> Filter --> O
    #[test]
    fn filter_simple() {
        let (factorio, tx, rx) = FactorioBuilder::new(1).filter(|v| v % 2 == 0).build();

        tx.send(5).unwrap();
        tx.send(6).unwrap();
        tx.send(4).unwrap();
        tx.send(1).unwrap();
        tx.send(8).unwrap();

        assert_eq!(rx.recv().unwrap(), 6);
        assert_eq!(rx.recv().unwrap(), 4);
        assert_eq!(rx.recv().unwrap(), 8);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn filter_no_clone() {
        struct Foo(u32);

        let (factorio, tx, rx) = FactorioBuilder::<Foo, Foo>::new(1)
            .filter(|v| v.0 > 2)
            .build();

        tx.send(Foo(1)).unwrap();
        tx.send(Foo(4)).unwrap();

        assert_eq!(rx.recv().unwrap().0, 4);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn filter_different_thread() {
        let thread_id = std::thread::current().id();
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .filter(move |&v| {
                assert_ne!(std::thread::current().id(), thread_id);
                v > 5
            })
            .build();

        tx.send(5).unwrap();
        tx.send(6).unwrap();
        tx.send(3).unwrap();
        tx.send(8).unwrap();
        assert_eq!(rx.recv().unwrap(), 6);
        assert_eq!(rx.recv().unwrap(), 8);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    /// I --> Map --> Filter --> Map --> O
    #[test]
    fn map_filter_map() {
        let (factorio, tx, rx) = FactorioBuilder::new(1)
            .map(|v| v * 2)
            .filter(|&v| v > 10)
            .map(|v| v + 1)
            .build();

        tx.send(5).unwrap();
        tx.send(6).unwrap();
        tx.send(4).unwrap();
        tx.send(1).unwrap();
        tx.send(8).unwrap();

        assert_eq!(rx.recv().unwrap(), 13);
        assert_eq!(rx.recv().unwrap(), 17);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    /// I --> FilterMap --> Map --> Filter --> O
    #[test]
    fn filter_map_map() {
        let (factorio, tx, rx) = FactorioBuilder::new(1)
            .filter_map(|v| match v {
                10.. => Some(v.to_string()),
                _ => None,
            })
            .map(|s| s.len())
            .filter(|&v| v > 2)
            .build();

        tx.send(5).unwrap();
        tx.send(16).unwrap();
        tx.send(4).unwrap();
        tx.send(123).unwrap();

        assert_eq!(rx.recv().unwrap(), 3);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn filter_map_no_clone() {
        struct Foo(u32);

        let (factorio, tx, rx) = FactorioBuilder::<Foo, Foo>::new(1)
            .filter_map(|v| Some(Foo(v.0 + 1)))
            .build();

        tx.send(Foo(5)).unwrap();

        assert_eq!(rx.recv().unwrap().0, 6);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    ///    --> Fork --v
    /// I -|          |-> Join --> O
    ///    --> Fork --^
    #[test]
    fn fork_join_simple() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .fork_join(
                move |v, _| v + 1,
                2,
                |results| results.into_iter().sum::<u64>(),
            )
            .build();

        tx.send(5u64).unwrap();
        assert_eq!(rx.recv().unwrap(), 12);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fork_join_no_clone() {
        struct Foo(u32);

        let (factorio, tx, rx) = FactorioBuilder::<Foo, Foo>::new(4)
            .fork_join(
                move |v, _| Foo(v.0),
                2,
                |results| results.into_iter().map(|v| v.0).sum::<u32>(),
            )
            .build();

        tx.send(Foo(5)).unwrap();
        assert_eq!(rx.recv().unwrap(), 10);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fork_join_different_threads() {
        let ids: Arc<Mutex<HashSet<(usize, std::thread::ThreadId)>>> =
            Arc::new(Mutex::new(Default::default()));
        let ids2 = ids.clone();
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .fork_join(
                move |v, worker_id| {
                    ids.lock()
                        .unwrap()
                        .insert((worker_id, std::thread::current().id()));
                    *v
                },
                4,
                |results| results.into_iter().sum::<u32>(),
            )
            .build();

        tx.send(5).unwrap();
        rx.recv().unwrap();

        assert_eq!(ids2.lock().unwrap().len(), 4);

        let worker_ids: HashSet<_> = ids2
            .lock()
            .unwrap()
            .iter()
            .map(|(worker_id, _)| *worker_id)
            .collect();
        assert_eq!(worker_ids.len(), 4);

        let thread_ids: HashSet<_> = ids2
            .lock()
            .unwrap()
            .iter()
            .map(|(_, thread_id)| *thread_id)
            .collect();
        assert_eq!(thread_ids.len(), 4);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fork_join_keep_ordering() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .fork_join(
                move |v, worker_id| {
                    if worker_id == 1 {
                        std::thread::sleep(Duration::from_secs(1));
                    }
                    v + worker_id + 1
                },
                4,
                |results| {
                    // The second worker finishes last, but that should not affect the order of the
                    // items passed to the mjoinerge function.
                    assert_eq!(results, vec![6, 7, 8, 9]);
                    0
                },
            )
            .build();

        tx.send(5).unwrap();
        assert_eq!(rx.recv().unwrap(), 0);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fork_join_check_parallelism() {
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .fork_join(
                move |v, worker_id| {
                    std::thread::sleep(Duration::from_secs(1));
                    v + worker_id as u64
                },
                2,
                |results| results.into_iter().sum::<u64>(),
            )
            .build();

        tx.send(101).unwrap();

        let start = Instant::now();
        assert_eq!(rx.recv().unwrap(), 203);
        let duration = start.elapsed().as_secs_f64();
        assert!(duration < 1.5);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fill_single_thread() {
        let queue_size = 10;
        let worker_count = 4;
        let (factorio, tx, rx) = FactorioBuilder::new(queue_size)
            .fork_join(
                move |v, _| *v,
                worker_count,
                |results| results.into_iter().sum::<u64>(),
            )
            .map(|v| v + 1)
            .build();

        let worker_count = worker_count as u64;
        let mut inflight = VecDeque::new();
        for i in 0..10000 {
            tx.send(i).unwrap();
            inflight.push_back(i);
            if inflight.len() == queue_size {
                // In order to avoid deadlock, we have to read from the pipeline
                assert_eq!(
                    rx.recv().unwrap(),
                    inflight.pop_front().unwrap() * worker_count + 1
                );
            }
        }

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fill_multiple_threads() {
        let queue_size = 10;
        let worker_count = 4;
        let (factorio, tx, rx) = FactorioBuilder::<(u64, u64), (u64, u64)>::new(queue_size)
            .fork_join(
                move |(input_id, v), _| (*input_id, *v),
                worker_count,
                |results| {
                    let input_id = results[0].0;
                    (input_id, results.into_iter().map(|v| v.1).sum::<u64>())
                },
            )
            .map(|(input_id, v)| (input_id, v + 1))
            .build();

        let handles = (0..3)
            .map(|id| {
                let tx = tx.clone();
                std::thread::spawn(move || {
                    for i in 0..1000 {
                        tx.send((id, (id * 10000) + i)).unwrap();
                        std::thread::sleep(Duration::from_millis(1));
                    }
                })
            })
            .collect::<Vec<_>>();

        let worker_count = worker_count as u64;
        for _ in 0..3000 {
            let (input_id, result) = rx.recv().unwrap();
            assert!((input_id * 10000) * worker_count + 1 <= result);
            assert!(result < (input_id * 10000 + 1000) * worker_count + 1);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn axe_maker() {
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct Ore {
            weight: u64,
        }

        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct RefinedOre(Ore);

        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct SmeltedOre(Ore);

        struct Axe(SmeltedOre);

        let (factorio, tx, rx) = FactorioBuilder::new(4)
            // Axes need some amount of ore to be made
            .filter(|ore: &Ore| ore.weight >= 50)
            // Generate refined ore with some probability
            .filter_map(|ore| {
                std::thread::sleep(Duration::from_millis(1));
                if rand::thread_rng().gen_bool(0.5) {
                    Some(RefinedOre(Ore {
                        weight: (ore.weight as f64 * 0.8).round() as u64,
                    }))
                } else {
                    None
                }
            })
            // Try to smelt the refined ore in four furnaces in parallel
            // Select the output that produced the most smelted ore
            .fork_join(
                |ore, furnace_id| {
                    let amount =
                        rand::thread_rng().gen_range((furnace_id * 2) as u64..ore.0.weight);
                    std::thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..3)));
                    SmeltedOre(Ore { weight: amount })
                },
                4,
                |ores| ores.into_iter().max().unwrap(),
            )
            // Create the final axe
            .map(|ore| Axe(ore))
            // Keep only axes that have enough weight
            .filter(|axe| axe.0 .0.weight >= 10)
            .build();

        // Try to build a thousand axes
        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                let ore_amount = rand::thread_rng().gen_range(10..150);
                tx.send(Ore { weight: ore_amount }).unwrap();
            }
        });

        let mut axes = vec![];
        // Collect axes as long as the generating thread is alive
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(axe) => {
                    axes.push(axe);
                }
                Err(_) => {
                    // timeout, check thread
                    if handle.is_finished() {
                        handle.join().unwrap();
                        while let Ok(axe) = rx.try_recv() {
                            axes.push(axe);
                        }
                        break;
                    }
                }
            }
        }

        // Check that at least some axes were made
        assert!(!axes.is_empty());

        drop(rx);
        factorio.close();
    }

    // TODO(bonus): uncomment the following tests and make them pass :)
    /*
    #[test]
    #[should_panic]
    fn fork_join_panic() {
        // When one of the forked workers panics, the pipeline should also panic when trying to
        // receive a message.
        let (factorio, tx, rx) = FactorioBuilder::new(4)
            .fork_join(
                move |v, worker_id| {
                    if worker_id == 1 {
                        panic!("Assembly fail");
                    }
                    *v
                },
                4,
                |results| results.into_iter().sum::<u64>(),
            )
            .build();

        tx.send(101).unwrap();
        assert_eq!(rx.recv().unwrap(), 203);

        drop(tx);
        drop(rx);
        factorio.close();
    }

    #[test]
    fn fork_join_stragglers() {
        // This test checks if the forked assembly line can function properly when one of the
        // workers is slow.
        let queue_size = 4;
        let (factorio, tx, rx) = FactorioBuilder::new(queue_size)
            .fork_join(
                move |v, worker_id| {
                    if worker_id == 1 {
                        // Worker 1 is slow...
                        std::thread::sleep(Duration::from_secs(1));
                    }
                    *v
                },
                4,
                |results| results.into_iter().sum::<u64>(),
            )
            .build();

        tx.send(1).unwrap();
        // ...so what happens when the next batch gets ahead?
        tx.send(100).unwrap();
        assert_eq!(rx.recv().unwrap(), 4);
        assert_eq!(rx.recv().unwrap(), 400);

        drop(tx);
        drop(rx);
        factorio.close();
    }
    */
}
