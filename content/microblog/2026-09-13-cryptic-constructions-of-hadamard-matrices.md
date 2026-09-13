---
title: Cryptic constructions of Hadamard matrices
date: 2026-09-13
tags: ['mathematics']
---
On August 12, 2026, Levent Alpöge of Anthropic [posted on X](https://x.com/__alpoge__/status/2087504785952182273) constructions for [Hadamard matrices](https://en.wikipedia.org/wiki/Hadamard_matrix) of orders $n \in \{668, 716, 892, 1132, 1244, 1388, 1436, 1676, 1772, 1916, 1948, 1964\}$, resolving all previously unknown orders below 2000 (but leaving the full conjecture open). A matrix $H \in \mathcal{M}^{n \times n}(\left\{ \pm 1\right\})$ is *Hadamard* of *order* $n$ if

$$ H H^\intercal = n I_n .$$

It is conjectured that such matrices exist for all orders $4n$, where $n$ is any positive integer. This is an open problem that has [long intrigued me](/blog/2025-08-09-hard-problems-the-hadamard-matrix-problem/), and it’s exciting to see progress!

However, the post was, in the unfortunate style du jour, a “[vaguepost](https://en.wikipedia.org/wiki/Vagueposting)”: an intentionally cryptic post to bait users into engagement. This is a term that I hadn't come across before but that I think is apropos (especially in light of the similar [Jacobian conjecture](https://en.wikipedia.org/wiki/Jacobian_conjecture) resolution).

To me, this was immediately reminiscent of the anonymous maths Stack Exchange user, “[Cleo](https://en.wikipedia.org/wiki/Cleo_%28mathematician%29)”, who posted closed-form solutions to intricate integral problems without providing any working or reasoning.

There is a sense in which this adds to the allure of mathematics, and perhaps elevates individual endeavours in the public imagination. However, on the whole, I find the attitude tiresome and borderline hostile to the broader community. In particular, there is no context provided on the (likely novel) method of construction.

I have taken the liberty of posting the unobfuscated matrices in plaintext as a [gist](https://gist.github.com/stochastical/bb4fa0bdd03fb80c5b5b75d0966485e3). To verify it:

```python
$ uvx --with numpy ipython
import numpy as np

for i in (668, 716, 892, 1132, 1244, 1388, 1436, 1676, 1772, 1916, 1948, 1964):
    H = np.loadtxt(f"matrices/hadamard_{i}.txt")
    n = H.shape[0]
    assert np.array_equal(H @ H.T, n * np.eye(n))
```
