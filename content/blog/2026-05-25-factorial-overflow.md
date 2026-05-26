---
title: Factorial overflow
tags:
  - mathematics
  - distro
date: 2026-05-25
description: |
    Exploring the factorial function and determining the first $n$ such that $n!$ exceeds the maximum size of an unsigned integer, for various bitwidths. We'll take a look at Stirling's approximation and the Lambert W function.
---

I've been working on a statistics library in Typst recently, [`distro`](https://github.com/stochastical/distro).

One fun outcome of this has been the chance to revisit some special functions in mathematics, and explore how to implement them in a numerically safe way. For instance, Typst uses `i64` under the hood when [implementing](https://github.com/typst/typst/blob/main/crates/typst-library/src/foundations/calc.rs#L612) the [`calc.binom`](https://typst.app/docs/reference/foundations/calc/#functions-binom) function, and it overflows for moderately large `n` ([Issue #4993](https://github.com/typst/typst/issues/4993)).

Now, the factorial function grows super-exponentially, so the recursive computation of

$$
n! = \begin{cases}n(n-1)!& n \ge 1 \\ 1& n = 0 \end{cases}
$$

quickly exceeds the range of fixed-width integer values.

I was curious about just how quickly this actually occurs. That is, for what values of $n$ does $n!$ overflow a $b$-bit integer?

## An inequality using Stirling's approximation

[Stirling's approximation](https://en.wikipedia.org/wiki/Stirling's_approximation) gives an asymptotic approximation![*](fn "Asymptotic here means that the approximation becomes increasingly accurate as $n\to\infty$") for the factorial function:

$$
n!\sim {\sqrt {2\pi n}}\left({\frac {n}{e}}\right)^{n}
$$

I've been enamoured by this formula for a long time! Not to commit mathematical heresy, but to me it's even more beautiful than [Euler's identity](https://en.wikipedia.org/wiki/Euler's_identity) of $e^{i\pi}+1=0$.

Now, the range of a $b$-bit unsigned integer type is $\{0, \dots, 2^b-1\}$. So, precisely, we're looking for the first $n$ such that $n! > 2^b-1$.

If we substitute in the Stirling approximation and take the (binary) logarithm:

$$
\begin{align*}
           & n!                                                      && > 2^b-1 \\
\implies\; & \log_2(n!)                                              && > b \\
\implies\; & n\log_2(n) - n\log_2(e) + \tfrac{1}{2}\log_2(2\pi n)    && > b
\end{align*}
$$

The last term in the inequality is pretty small, but we'll keep it in anyway. This gives us a quick way to find $\min\{n\in \mathbb N: n!> 2^b-1\}$ programmatically.  We'll consider [Rust's unsigned integers](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types): `u8`, `u16`, `u32`, `u64` and `u128`.


```rust
use std::f64::consts::{E, PI};

fn main() {
    const UNSIGNED_INT_TYPES: &[(u32, u128)] = &[
        (8, u8::MAX as u128),
        (16, u16::MAX as u128),
        (32, u32::MAX as u128),
        (64, u64::MAX as u128),
        (128, u128::MAX),
    ];

    for &(b, max_val) in UNSIGNED_INT_TYPES {
        let n = (1u32..)
            .find(|&n| {
                let n = n as f64;
                n * n.log2() - n * E.log2() + 0.5 * (2.0 * PI * n).log(2.0)
                    >= b as f64
            })
            .unwrap();

        let factorial: u128 = (1..n as u128).product(); // (n-1)!
        println!("u{b:<4} {n:>3} {factorial:>40} {max_val:>40}");
    }
}
```

This is perhaps overly functional, but effectively we're lazily iterating over an infinite range of integers `(1u32..)` and applying the above inequality as our stopping criterion in the [`.find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) predicate.

| Type   | $n$  | $(n-1)!$                                  | $2^b - 1$                                 |
|--------|------|-------------------------------------------|-------------------------------------------|
| `u8`   | `6`  | `120`                                     | `255`                                     |
| `u16`  | `9`  | `40320`                                   | `65535`                                   |
| `u32`  | `13` | `479001600`                               | `4294967295`                              |
| `u64`  | `21` | `2432902008176640000`                     | `18446744073709551615`                    |
| `u128` | `35` | `295232799039604140847618609643520000000` | `340282366920938463463374607431768211455` |


Of course, we could also just directly compute the factorials and stop when we encounter an overflow, but this is more fun.

## A transcendental expression with Lambert's W function

If you prefer a transcendental expression to the trial-and-error inequality solving, we can (loosely) introduce [Lambert's W function ](https://en.wikipedia.org/wiki/Lambert_W_function) using the principal branch $W_0$ on positive reals, as the [monotonic](https://math.stackexchange.com/questions/1075118/montonicity-of-lambert-w) function $W$ that satistifies

$$
ye^y = x \implies y = W(x)
$$

and has a ["super root"](https://en.wikipedia.org/wiki/Lambert_W_function#Super_root) property that $W(x\ln x) = \ln x$.

If we take the natural logarithm on Stirling's approximation

$$
\ln n! \approx n\ln n - n + \cancel{\frac 12 \ln (2\pi n)} > \ln(2^b -1)
$$

and this time we'll ignore the small, last term in the $\ln n!$ approximation.

We then define $x \equiv \frac n e$, noting that $\ln(x) = \ln(n/e) = \ln(n)-1$, and factor it out:

$$
\begin{align*}
           & \frac{n}{e}(\ln n - 1) && > \frac{\ln(2^b-1)}{e} \\
\implies\; & x\ln x                 && > \frac{\ln(2^b-1)}{e} \\
\implies\; & \ln x                  && > W\left(\frac{\ln(2^b-1)}{e}\right) \\
\implies\; & \ln(n/e)               && > W\left(\frac{\ln(2^b-1)}{e}\right) \\
\implies\; & n                      && > \exp\left(W\left(\frac{\ln(2^b-1)}{e}\right) + 1\right)
\end{align*}
$$

Unfortunately, the $2^b$ term overflows for $b=128$, and the exponential blows up. But we can remedy this by exploiting [that](https://en.wikipedia.org/wiki/Lambert_W_function#Identities):

$$
e^{W(x)} = \frac{x}{W(x)}
$$

and rewriting the logarithm term $\ln(2^b-1)=\ln(2^b(1-2^{-b}))=b\ln(2)+\ln(1-2^{-b})$. We can drop the last term, as it's negligible.

Pulling it all together, we get the approximate bound:

$$
n > \frac{b\ln 2+\ln(1-2^{-b})}{W\left(\frac{b\ln 2+\ln(1-2^{-b})}{e} \right)} \approx \frac{b\ln 2}{W\left(\frac{b\ln 2}{e}\right)}
$$

Now we can numerically evaluate this!

I'll switch over to Python since it's nice and quick to grab the Lambert function from `scipy` using [`uvx`](https://docs.astral.sh/uv/guides/tools/#running-tools):

```python
$ uvx --with scipy ipython
In [1]: from scipy.special import lambertw
In [2]: from numpy import e, log, exp
In [3]: def lower_bound(b):
   ...:     C = b * log(2)
   ...:     return C / lambertw(C / e)
In [4]: [lower_bound(b).real for b in [8, 16, 32, 64, 128]]
Out[4]: 
[np.float64(6.434894908913895),
 np.float64(9.143000222484407),
 np.float64(13.708552087952512),
 np.float64(21.46683755450863),
 np.float64(34.79891651275784)]
```

You could also use `np.log1p(-2**(-b))` for the tiny term $\ln(1-2^{-b})$, but it's much of a muchness.

## Calculating the Poisson distribution probability mass function

In my library, I was calculating the probability mass function for a Poisson random variable $X \sim \operatorname{Pois}(\lambda )$ using the standard form:

$$
P(X = k) = \frac{\lambda^k e^{-\lambda}}{k!}
$$

but both the $\lambda^k$ and $k!$ factors can rapidly overflow.

[Re-writing it equivalently](https://en.wikipedia.org/wiki/Poisson_distribution#Evaluating_the_Poisson_distribution) as

$$
P(X = k) = \exp(k \ln(\lambda) - \lambda - \ln\Gamma(k+1))
$$

means that we can preserve precision!

Of course, this means we need a way to compute the $\ln \Gamma$ function, but I already had that defined for my Gamma distribution implementation. Alternatively, since the support of the Poisson random variable is $\mathbb{N}_0$ you can keep things discrete and expand the log-factorial into a sum-of-logs:

$$
\ln(k!) = \ln(1(2)(3)\cdots (k)) =\ln(1) + \ln(2) + \ln(3) + \cdots + \ln(k) \\
\implies P(X = k) = \exp\left(k \ln(\lambda) - \lambda - \sum_{i=1}^{k} \ln(i)\right)
$$
