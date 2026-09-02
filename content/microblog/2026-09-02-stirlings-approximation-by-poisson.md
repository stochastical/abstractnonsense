---
title: "Deriving Stirling's approximation using the Poisson distribution"
date: 2026-09-02
tags: ['mathematics']
---

I discovered a neat way to derive Stirling's approximation for the factorial in a succinct manner. I came across this neat derivation in _Information Theory, Inference and Learning Algorithms_ by David J.C. MacKay (though I've also just noticed that it's listed on the [Wikipedia page](https://en.wikipedia.org/wiki/Stirling's_approximation#Using_the_Central_Limit_Theorem_and_the_Poisson_distribution) too). This is an addendum of sorts to my [*Factorial Overflow*](/blog/2026-05-25-factorial-overflow) post.

Consider an indexed family of Poisson random variables $N_{\lambda }\sim \operatorname{Pn}(\lambda), \lambda \in \mathbb N$ with probability mass functions

$$
f_{N_\lambda}(n) = \frac{e^{-\lambda} \lambda^n}{n!}, \quad n \in \{0, 1,\dots \}
$$
Now, the Poisson distribution has a divisibility property which means that the sum of i.i.d Poisson random variables is itself a Poisson random variable. So we can decompose $N_\lambda$ as $N_{\lambda}=\sum_{i=1}^\lambda N_i$ where $N_i \sim \operatorname{Pn}(1)$ i.i.d and $\mathbb{E}[N_i]=\operatorname{Var}[N_i]=1$. Thus, by the Central Limit Theorem, as $\lambda \to \infty$:
$$
\frac{N_{\lambda}-\lambda}{\sqrt{\lambda}} \dot{\sim} \mathcal{N}\left(0,1 \right)
$$
and so[1](fn " You actually need a local limit theorem to justify that the PMF pointwise converges too.") we can approximate the Poisson mass function, for large $\lambda$, by the corresponding normal density for $\mathcal{N}(\lambda, \lambda)$:

$$
\frac{e^{-\lambda} \lambda^n}{n!} \approx \dfrac{1}{\sqrt{2\pi \lambda}} \exp\left(-\frac{(n-\lambda)^2}{2\lambda}\right)
$$
If we let $n=\lambda$ the exponential term drops away and we can re-arrange for $n!$:

$$
\frac{e^{-n} n^n}{n!} \approx \frac{1}{\sqrt{2\pi n}} \implies n! \approx \sqrt{2\pi n} e^{-n}n^n= \sqrt{2\pi n}\left( \frac{n}{e}\right)^n
$$
