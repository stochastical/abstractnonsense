---
title: Pseudorandom number generation in JAX
tags:
  - typst
  - distro
  - mathematics
  - statistics
  - quote
date: 2026-05-17
---

Recently I've been working on a statistics library in [Typst](https://github.com/typst/typst) called [`distro`](https://github.com/stochastical/distro)![*](fn "More to come soon"). Typst is architected with an [incremental](https://github.com/typst/typst/blob/main/docs/dev/architecture.md#compilation) [compiler](https://github.com/typst/comemo/), and pairs it with pure functions![*](fn "Almost all functions in Typst are pure, with a [few exceptions](https://typst.app/docs/reference/foundations/function/#:~:text=Note%20on%20function%20purity)") to facilitate snappy real-time compilation.

The problem is, what to do about random number generation when your functions are supposed to be referentially transparent? Well, thankfully that's a solved problem with a neat solution. For instance, [JAX relies](https://docs.jax.dev/en/latest/random-numbers.html#random-numbers-in-jax) on an explicit 'key' value, that is consumed by the PRNG, and returns an updated state that is then passed back into the PRNG to generate the next random sample.

> PRNG-generated sequences are not truly random because they are actually determined by their initial value, which is typically referred to as the seed, and each step of random sampling is a deterministic function of some state that is carried over from a sample to the next.

Importantly, they [warn you](https://docs.jax.dev/en/latest/random-numbers.html#explicit-random-state) to never re-use the same key, else:

> The rule of thumb is: never reuse keys (unless you want identical outputs). **Reusing the same state will cause sadness and monotony, depriving the end user of lifegiving chaos.**

Maybe I should start a [collection](/microblog/2026-03-04-activation-functions-and-empiricism/) of humorous academic refrains....

In Typst, there's a package called [`suiji`](https://typst.app/universe/package/suiji/) that provides pure functions for random-number generation. That allows me to use it within my `distro` package to produce random variates from distributions using [inverse transform sampling](https://en.wikipedia.org/wiki/Inverse_transform_sampling):

```typst
#import "@preview/suiji:0.5.1": *
#import "../lib.typ": categorical // my `distro` package

#let n_samples = 1000
#let Cat = categorical.new((0.1, 0.3, 0.2, 0.4))
#let counts = (0,) * Cat.weights.len()
#let (rng, u) = (gen-rng-f(42), none)

#for _ in range(n_samples) {
  (rng, u) = uniform-f(rng)
  let result = categorical.sample(Cat, u)
  counts.at(result) += 1
}
```
