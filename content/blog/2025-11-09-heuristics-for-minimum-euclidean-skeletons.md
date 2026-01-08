---
title: Heuristics for Minimum Euclidean Skeletons
date: "2025-11-09"
tags: ['mathematics', 'graph-theory', 'algorithms', 'research']
description: |
    A retrospective look at my 2021 University of Melbourne vacation research project on minimum Euclidean skeletons, a geometric graph theory optimisation problem.
---

> **Disclaimer**: This blog post is a recap of exploration I did as part of the undergraduate [Mathematics and Statistics Vacation Scholarship](https://ms.unimelb.edu.au/engage/vacation-scholarships) program at the University of Melbourne in January 2021. It's been a while since then, so please forgive any inaccuracies that are almost assuredly present.
>
> The poster I presented at the conclusion of the program was hosted on the School of Mathematics' website but has suffered the demise of linkrot. For posterity, I'm hosting my poster PDF and an HTML version on my blog here.

<figure aria-labelledby="pdf-caption">
    <object
        data="/files/unimelb-vacation-research-poster-heuristics-for-minimum-euclidean-skeletons.pdf"
        type="application/pdf"
        aria-label="Embedded PDF: UniMelb research poster: Heuristics for Minimum Euclidean Skeletons"
        style="width:100%;height:70vh;border:0;display:block;"
        role="document">
        <p>Your browser does not support inline PDFs. <a href="/files/unimelb-vacation-research-poster-heuristics-for-minimum-euclidean-skeletons.pdf" target="_blank" rel="noopener">Open or download the PDF</a>.</p>
    </object>
    <figcaption id="pdf-caption">UniMelb research poster: Heuristics for Minimum Euclidean Skeletons. <a href="/files/unimelb-vacation-research-poster-heuristics-for-minimum-euclidean-skeletons.pdf" target="_blank" rel="noopener">Open PDF in a new tab</a>.</figcaption>
</figure>

## Introduction

Imagine you’re a robot navigating your way through an obstacle field, a captain steering around a coast, or a lithographer packing transistors into a microchip. You can model obstacles as polygons in the plane and construct a 'skeleton' to help avoid collisions.

The scope of this project is to find an approximation algorithm to the Minimum Euclidean Skeleton problem. This project lies at the intersection of Graph theory, Euclidean geometry, and combinatorial optimisation. We embed a graph (representing a polygon) into a Euclidean metric space, where we care about the *geometric* relationships between the vertices and edges.

### Polygons and Edges

- Let $\Omega$ be a *simple* polygon in the plane (no holes or self-intersections).
- Let $V$ be the set of $n$ vertices of $\Omega$, where no three vertices of $V$ are collinear; a *convex* vertex of $\Omega$ has interior angle less than $\pi \ \mathrm{rad}$.
- An *edge* of a skeleton is a Euclidean line segment; a *maximal length* edge
has its end-nodes on the boundary of $\Omega$.
- A set of edges $E$ is *connected* if there exists a polygonal chain (piecewise linear curve) between any two edges in $E$ that lies entirely along edges in $E$ (i.e. you can trace a path between any two points in E that uses only edges in $E$).
 
### Skeletons

- A *skeleton* $S$ of $\Omega$ is a set of edges such that the line segment $P_1-P_2$ between any two points $P_1, P_2$ that lie in the exterior to $\Omega$ intersects $\Omega$ iff $P_1-P_2$ intersects (at least one) edge in $S$.
- A *minimum skeleton* is the skeleton $S'$ of $\Omega$ with smallest cardinality.

## An Equivalent Problem

In [^1] it is shown, with sufficiency and necessity, that any set $S$ of maximal length edges that:
1. intersects every convex vertex of $\Omega$; and
2. is connected

is a skeleton of $\Omega$.

There exists a (computationally expensive) exact algorithm[^1] to generate a minimum Euclidean skeleton, and this project seeks an approximation algorithm to minimise the skeleton size.

**Heuristic**: The problem of approximating a minimum skeleton of $\Omega$ reduces to finding some way to cover all convex vertices with as small a set of connected edges as possible. To do this we can employ a combination of the *Steiner tree* on graphs and the notion of a *visibility graph* of a polygon.

## Visibility Graphs and Steiner Trees in Graphs

The **visibility graph** of a polygon $\Omega$ is the graph $G$ where each node in $G$ represents a vertex of $\Omega$, and nodes $u, v$ are adjacent iff the line segment $u − v$ lies entirely within the interior or on the boundary of $\Omega$ (i.e. there is a 'line-of-sight' between vertex $u$ and vertex $v$).

**Steiner Trees in Graphs**: Let $G = (V,E)$ be an undirected graph with non-negative edge weights $c$ and let $S \subseteq V$ be a subset of vertices, called terminals. The Steiner tree problem asks for the minimum-weight tree of $G$ that covers all terminals.

**Applying the Steiner Tree**

1. Let the set of terminals be $S = \operatorname{Conv}{\Omega}$, the set of convex vertices of the polygon.
2. Let the edge-weights $c = 1$ be constant.
3. Let $G$ be the visibility graph of the polygon.
4. Then the minimum-weight Steiner tree of $G$ will approximate the minimum skeleton of $\Omega$!

However, the Steiner tree problem is NP-Hard! So to utilise the Steiner tree, we need to approximate the Steiner tree problem itself![^2]

## Metric Closures and Minimum Spanning Trees

The *metric closure* of a graph $G$ is the complete graph in which each edge is weighted by the shortest path distance between the nodes in $G$.

A *minimum spanning tree* (MST) is a subset of the edges of a connected, edge-weighted undirected graph that connects all the vertices together, without any cycles and with the minimum possible total edge weight

**Complete Heuristic:** Compute the MST of the subgraph induced by the convex vertices of $\Omega$ of the metric closure of the visibility graph of $\Omega$.

## Results

**Bounds on Minimum Euclidean skeletons:** The number of vertices of $\Omega$ is an upper bound for $|S'|$, since the boundary of $\Omega$ is a feasible skeleton. Additionally, $|\operatorname{Conv}{Ω}|/2$ is a lower bound for $|S'|$, since we need to intersect every convex vertex. This heuristic seemed to produce a skeleton with $\approx0.6n$ edges, compared to the optimal solution using the exact algorithm[^1] of $\approx 0.4n$ edges.

## Next Steps

The (naive) visibility graph computation employed was the most computationally
expensive part of the algorithm; but there exist alternate faster methods that could be implemented. The Steiner tree approximation was very fast (approximately 2 minutes on an $n = 4000$ instance), and returns a solution with weight within a $2−2/|\operatorname{Conv}{Ω}|$ factor of the optimal Steiner tree [^2]. In fact, from my data, it appears that the cardinality
of the skeleton produced by the heuristic is within the same factor of the exact solution.

It would be interesting to explore alternate heuristics for the Minimum Euclidean Skeleton problem, and to adapt the existing heuristic, perhaps by using a non-constant cost function to weight the edges, or by generating a vertex-edge or edge-edge visibility or intersection graph to reduce the number of edges required.

[^1]: N. Andrés-Thió, M. Brazil, C. Ras, D. Thomas, and M. Volz. "An Exact Algorithm for Constructing Minimum Euclidean Skeletons of Polygons".
[^2]: L. Kou, G. Markowsky, and L. Berman. "A fast algorithm for Steiner trees". Acta Informatica (1981), pp. 141–145.
