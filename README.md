# Programming in Rust
This repository contains materials for the [Programming in Rust](https://edison.sso.vsb.cz/cz.vsb.edison.edu.study.prepare.web/SubjectVersion.faces?version=460-4157/02&subjectBlockAssignmentId=545221&studyFormId=1&studyPlanId=26573&locale=en&back=true) subject taught at [FEI VŠB-TUO](https://www.fei.vsb.cz/en/).

You can find the lessons recordings on [YouTube](https://www.youtube.com/playlist?list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU).

> Note that the recordings are in the Czech language 🇨🇿.

## Lessons
1. Rust intro, syntax, Cargo, rustc
    - [Lesson recording](https://www.youtube.com/watch?v=PDBT5dIVEfc&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=2&ab_channel=JakubBer%C3%A1nek)
    - [Slides](lessons/01/slides.pdf)
    - [Exercises](lessons/01/exercises)

2. Structs, enums, pattern matching
    - [Lesson recording](https://www.youtube.com/watch?v=oDvjXaP-2pU&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=2)
    - [Exercises](lessons/02/exercises)

3. Ownership, borrowing, lifetimes
    - [Lesson recording](https://www.youtube.com/watch?v=c8i9SKDfWDE&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=3)
    - [Exercises](lessons/03/exercises)

4. Polymorphism, traits, generic programming
    - [Lesson recording](https://www.youtube.com/watch?v=IY4ejueecdQ&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=4)
    - [Exercises](lessons/04/exercises)

5. Smart pointers, interior mutability, closures
    - [Lesson recording](https://www.youtube.com/watch?v=lGKSYne5DzM&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=5)
    - [Exercises](lessons/05/exercises)

6. Extended traits (coherence check, orphan rule, object safety), error handling
    - [Lesson recording](https://www.youtube.com/watch?v=XgTEIVbcTQ8&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=6)
    - [Exercises](lessons/06/exercises)

7. Parallelism, Send/Sync, atomics, locks, channels
    - [Lesson recording](https://www.youtube.com/watch?v=CYAW4khk5Nc&list=PLgoUJJFtqE9C8Ar_JgDBHQYrG-hHMlVyU&index=7)
    - [Exercises](lessons/07/exercises)

8) Shell interpreter

- [Lesson recording](https://www.youtube.com/watch?v=FQuyXAldPrI)
- [Shell interpreter](projects/shell-interpret)

9) Network chat (TCP/IP communication, message (de)serialization)

- [Lesson recording](https://www.youtube.com/watch?v=KVlrAepesMo)
- [Network chat](projects/network-chat/01)

10) Network chat (threads, reference-counting, synchronization)

- [Lesson recording](https://www.youtube.com/watch?v=OduKSTpzwUM)
- [Network chat](projects/network-chat/02)

11) Network chat (non-blocking operations, event loop)

- [Lesson recording](https://www.youtube.com/watch?v=cNQMGrzZhKs)
- [Network chat](projects/network-chat/03)

12) Network chat (async/await, pinning)

- [Lesson recording](https://www.youtube.com/watch?v=z49Ek7BOy50)
- [Network chat](projects/network-chat/04)

13) Advent of Code, [open-source contribution](https://github.com/rust-lang/glob/pull/135)

- [Lesson recording](https://www.youtube.com/watch?v=N5LlZ2L1Q3Y)

## Archive
You can find past runs of the subject in the [archive](archive).

## Inspiration
This subject takes inspiration from many other courses and online materials. A non-exhaustive list is provided below.

- [Software fundamentals](https://cese.ewi.tudelft.nl/software-fundamentals/)
    - Rust course taugh at CESE, TU Delft
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/)
    - 5-day intensive Rust course from Google
- [teach-rs](https://teach-rs.trifectatech.org/)
    - Modular university Rust course
