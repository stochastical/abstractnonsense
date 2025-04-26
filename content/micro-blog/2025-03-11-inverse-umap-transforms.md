---
title: Inverse UMAP transforms
blog_section:
  - Microblog
date: "2025-03-11"
description: Inverse transforms with UMAP dimensionality reduction
link: https://umap-learn.readthedocs.io/en/latest/inverse_transform.html
tags: ["data-science", "mathematics"]
---

TIL that the dimensionality reduction algorithm [UMAP](https://umap-learn.readthedocs.io/en/latest/)[^1] has support for [inverse transforms](https://umap-learn.readthedocs.io/en/latest/inverse_transform.html)!

Naturally, these are lossy inverses, but being able to generate novel examples of, for example, handwritten digits (yep, classic MNIST once again) is exceedingly cool.

The example images of handwritten digits here are sampled from the compressed _planar_ space, and "inverted" into the original "image space".

![](https://umap-learn.readthedocs.io/en/latest/_images/inverse_transform_13_0.png)

On a related note, the whole [explanatory article for UMAP](https://umap-learn.readthedocs.io/en/latest/how_umap_works.html) is a beautiful work of exposition, full of rich ideas that make one ponder deeply. I only wish I understood everything in here end-to-end.

[^1]: See also [Week 21](https://abstractnonsense.xyz/micro-blog/2023-05-21-week-21/) from the micro-blog.
