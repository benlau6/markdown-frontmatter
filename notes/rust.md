---
title: "Rust"
publishDate: "2024-09-19"
---

# Rust

<!--toc:start-->
- [Rust](#rust)
  - [trait object vs enum](#trait-object-vs-enum)
  - [static dispatch vs dynamic dispatch](#static-dispatch-vs-dynamic-dispatch)
  - [trait object vs generic trait bound / impl Trait](#trait-object-vs-generic-trait-bound-impl-trait)
  - [Why slice is DST but not the vector](#why-slice-is-dst-but-not-the-vector)
<!--toc:end-->

clarify the differences, maybe make some blogs

## Error handling

- [A Simpler Way to See Results](https://www.youtube.com/watch?v=s5S2Ed5T-dc)
  - How to use `?`
    - How to define a function with multiple possible errors which may be populated by `?`
      - `anyhow::Error`, which is similar but a better `Box<dyn Error>`
      - `enum` Error + `thiserror` which helps to implement the From trait
  - When to use `Result(T, E)`, when to use `Option(T)`
    - [Choose the Right Option](https://www.youtube.com/watch?v=6c7pZYP_iIE)
  - `Infallible` (bottom type) uses `!`

## trait object vs enum

- It is about different approach to Polymorphism
- [There are still a couple of cases in which I use traits. The main reason is if I want external code to be able to add types, which enums do not allow.](https://www.mattkennedy.io/blog/rust_polymorphism/)
  - I also consider using traits if the behaviour is particularly generic, such that the interface I am defining is not determined by the types that implement it.
  - Some people argue that enums produce ugly code with methods that are long and hard to read and hence favour using traits. I disagree, if the methods start to get too long one can extract the logic into separate functions, even grouping these functions into a module if there a large number of methods. In fact, I think this makes the code easier to read. The methods can be used as a table of contents allowing one to view the logic by jumping to the function definitions, rather than have to search through the code for the individual types.
- [The real fundamental difference between them is that enum is a closed set, and the trait object is an open set. And neither of these is universally better.](https://dpc.pw/posts/what-oop-gets-wrong-about-interfaces-and-polymorphism/)
  - Enums are closed sets and trait objects are open sets.
  - A trait is a collection of variants that may or may not have unknown members.
  - An enum is a collection of variants that may not have unknown implementations.

## static dispatch vs dynamic dispatch

- “When you’re given the choice between static and dynamic dispatch, there is rarely a clear-cut right answer. Broadly speaking, though, you’ll want to use static dispatch in your libraries and dynamic dispatch in your binaries. In a library, you want to allow your users to decide what kind of dispatch is best for them, since you don’t know what their needs are. If you use dynamic dispatch, they’re forced to do the same, whereas if you use static dispatch, they can choose whether to use dynamic dispatch or not.” - Rust for Ruataceans

## trait object vs generic trait bound / impl Trait

- It is mostly just static dispatch vs dynamic dispatch
- [a function that accepts a trait object doesn't need to be generic and doesn't need monomorphization: the programmer writes a function using trait objects, and the compiler outputs only a single version of that function, which can accept trait objects that come from multiple input types](https://www.lurklurk.org/effective-rust/generics.html)
  - A more significant difference is that generic trait bounds can be used to conditionally make different functionality available, depending on whether the type parameter implements multiple traits
- [Choosing Trait Objects: When you need one tool that can adjust to handle various tasks even though it might not be the most efficient tool for any single task, you choose the adjustable wrench (trait object). It’s like having a tool that can adapt to different situations without knowing the specifics beforehand.](https://medium.com/@richinex/trait-objects-vs-generics-in-rust-426a9ce22d78)
  - Choosing Generics: When you understand the tasks ahead and need the best tool for each specific task, you can choose from your specialized wrenches (generics). This approach requires planning and knowledge about what you will face, but it pays off in performance and precision.

## Why slice is DST but not the vector

[Note](/memory_management.md#dynamically-sized-type-dst-in-rust)
[[memory-management-of-languages#Dynamically sized type (DST) in rust]]
[[#Why slice is DST but not the vector]]