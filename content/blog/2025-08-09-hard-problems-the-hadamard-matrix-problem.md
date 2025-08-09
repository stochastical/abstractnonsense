---
title: "Hard problems: the Hadamard matrix problem"
date: "2025-08-09"
tags: ['mathematics', 'unsolved-problems']
---

There's a lot of [unsolved problems in mathematics](https://en.wikipedia.org/wiki/List_of_unsolved_problems_in_mathematics). A lot. It'd be a bit imprecise to define a measure on the set of open problems, but I'd wager that it is much larger than the set of solved problems. In fact, for every solved problem, I'd conjecture that the process of solving it simply creates more open problems.

One such open problem that I find deeply unsettling is the [Hadamard conjecture](https://en.wikipedia.org/wiki/Hadamard_matrix#Hadamard_conjecture). Let's define a Hadamard matrix and explain the conjecture:

> **Definition**: A Hadamard matrix of order \(n\) is a square matrix whose entries are either \(+1\) or \(-1\), and whose rows are *mutually orthogonal*.

A succinct way to phrase this is that \(H \in \mathcal{M}^{n \times n}(\left\{ \pm 1\right\}) \) is *Hadamard* of *order* \(n\) if

$$ H H^\intercal = n I_n .$$

You can use this to show[^1] that \(\det(H) = \pm n^{n / 2}\) which is the maximum determinant value that any matrix with elements bounded in magnitude by \(1\) can achieve, by the [Hadamard inequality](https://en.wikipedia.org/wiki/Hadamard%27s_inequality).

The Hadamard conjecture posits that

> **Conjecture**: Hadamard matrices exist *for all* orders \(4n\), where \(n\) is any positive integer.

You can [prove by contradiction](https://en.wikipedia.org/wiki/Hadamard_matrix#Proof) that Hadamard matrices can only exist for dimensions \(1\), \(2\), or multiple of \(4.\)

There are constructions of Hadamard matrices for specific sets of orders, such as the [Sylvester construction](https://en.wikipedia.org/wiki/Hadamard_matrix#Sylvester's_construction), which generates Hadamard matrices of order \(2^n\) for all non-negative integer \(n\), but a general solution for all orders \(4n\) remains elusive.

What I find more astonishing than the lack of a general solution, is that the current **smallest unknown Hadamard matrix is of order `668`**. That is, to date, no one has been able to successfully construct a Hadamard of this order, or prove that it does not exist (in which case, the entire conjecture collapses!).

As always, combinatorial explosion defies comprehension: it feels like given enough compute power and time, you could simply brute-force your way to a solution. But the search space grows exponentially with the size of the matrix, making it infeasible to check all possibilities even with an astronomical compute budget[^2]. If each element can be either \(+1\) or \(-1\), and there are \(n^2\) elements in a Hadamard matrix of order \(n\), then the search space has \(2^{n^2}\) possible configurations to check. For \(n = 668\), this is a number far, far greater than the number of atoms in the universe - clocking in at a measly \(\approx 10^{80}\) atoms.

When I find some time, I'll try to update this post with more content about the Hadamard matrix problem.

[^1]: Applying standard properties of the determinant, if \(HH^\intercal = n I_n\), then \( \det(HH^\intercal) = \det(n I_n) \implies \det(H)^2 = n^n \implies \det(H) = \pm n^{n / 2} \)
[^2]: There are various observations on symmetry you can apply to reduce the search space, but even still, a brute-force approach is intractable.
