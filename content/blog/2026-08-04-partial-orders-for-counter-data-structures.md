---
title: "Partial orders for Counter data structures"
date: 2026-08-04
tags:
  - rust
  - mathematics
description: |
    Multisets can be partially, but not totally, ordered by inclusion. Here's a quick bugfix for the `counter` crate. 
---

A *counter* data structure provides an abstract data type for a *[multiset](https://en.wikipedia.org/wiki/Multiset)*. A multiset is much like a classic set, but augmented to track the multiplicity of each of its member elements. For example, $[1, 1, 1, 2, 3]$ is a multiset consisting of $1$ repeated thrice, and each of $2$ and $3$ once.

If you define $A$ and $B$ to be multisets in a universe of objects $U$, and $m_A, m_B$ their respective multiplicity functions from $U\to \mathbb{N}$, you can define multiset inclusion as

$$
A \subseteq B \iff \forall x \in U, \ m_A(x) \le m_B(x) 
$$

The inclusion relation lets us define a [partial order](https://en.wikipedia.org/wiki/Partially_ordered_set) on multisets. Note that it is *not* a [total order](https://en.wikipedia.org/wiki/Total_order) since it is not the case that any two multisets are comparable. For example, take $A=[1, 1, 2]$ and $B=[1, 2, 2]$. We have $m_A(1) = 2 > 1 = m_B(1)$ but $m_A(2) = 1 < 2 = m_B(2)$. Thus, neither multiset contains the other, and they are incomparable.

You can define other [arithmetic-like relations](https://en.wikipedia.org/wiki/Multiset#Basic_properties_and_operations) by delegating to their multiplicity functions in a similar fashion.

The Python standard library implements multisets via the [`collections.Counter`](https://docs.python.org/3/library/collections.html#collections.Counter) class, which is a subclass of the dictionary data structure. For the Rust-equivalent crate, [`counter`](https://crates.io/crates/counter) I’ve just fixed a small implementation bug and implemented the [`std::cmp::PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) trait in [PR \#58](https://github.com/coriolinus/counter-rs/pull/58).

The `counter` crate [originally defined equality](https://github.com/coriolinus/counter-rs/blob/99ff5c337fe19f33e156f33acf584da02057c577/src/lib.rs#L302-L305) on two `Counter` structs as equality on the underlying HashMap structs:

```rust
fn eq(&self, other: &Self) -> bool {
  // ignore the zero
  self.map == other.map
}
```

Unfortunately, this is a leaky abstraction. If you mutate the underlying hash map by explicitly setting a key to have count  `0`, you’d expect that to be *ignored* for the sake of equality comparisons. Note that the code comment here is misleading: it refers to disregarding a private `zero` field on the `Counter` struct required for generic integral types, not to zero-valued counts.

We can [fix the equality comparisons](https://github.com/coriolinus/counter-rs/blob/master/src/lib.rs#L297-L306) by patching the `PartialEq` trait implementation:

```rust
fn eq(&self, other: &Self) -> bool {
  self.keys().chain(other.keys()).all(|k| self[k] == other[k])
}
```

Here, the struct [implements `std::ops::Index`](https://github.com/coriolinus/counter-rs/blob/master/src/impls/index.rs) so that `self[key]` and `other[key]` to return a reference to a (generic) `0` value for *missing* keys, which ensures the comparison succeeds even if `key` is present in one map but not the other.

As a bonus, if we implement the `partial_cmp` function [as follows](https://github.com/coriolinus/counter-rs/blob/master/src/lib.rs#L308-L329), we can get nice syntax sugar for the `<`, `<=`, `>`, `>=` operators:

```rust
impl<T, N, S> PartialOrd for Counter<T, N, S>
where
    T: Eq + Hash,
    N: PartialOrd + Zero,
    S: BuildHasher,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (mut le, mut ge) = (true, true);

        for key in self.keys().chain(other.keys()) {
            le &= self[key] <= other[key];
            ge &= self[key] >= other[key];
        }

        match (le, ge) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}
```

I first came across the `counter` crate via the [*Thinking in Iterators*](https://corrode.dev/blog/iterators/) article on [`corrode.dev`](https://corrode.dev/blog/) and discovered the bug when reading through the source code. The corrode blog is worth checking out, there’s lots of interesting Rust posts!
