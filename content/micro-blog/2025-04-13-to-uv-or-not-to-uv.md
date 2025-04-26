---
title: To uv or not to uv
blog_section:
  - micro-blog
date: "2025-04-13"
description: The magic behind uv's speed
link: https://www.youtube.com/watch?v=gSKTfG1GXYQ
tags: ["computer-science", "software-engineering"]
---

[`uv`](https://github.com/astral-sh/uv) is a blazing fast Python package manager that aims to displace `pip`. It's a really slick tool that lets you go from `git clone` to executing code with all the dependencies seamlessly. All the standard accolades apply: written in Rust, beautiful terminal UI, well thought-out user ergonomics ... all written by [Astral](https://astral.sh), the same company that gave the Python community [`ruff`](https://github.com/astral-sh/ruff).

But what makes it so damn fast? Under the rusty hood, the magic is even more impressive. Charlie Marsh, the project lead, presented at Jane Street and revealed some of the inner workings. The whole talk is super interesting, but some standout highlights are:

1. `uv` has it's own [SAT solver](https://en.wikipedia.org/wiki/SAT_solver) implementation to resolve dependency package versions much faster than e.g. `pip` or `conda`. ([`@16:09`](https://youtu.be/gSKTfG1GXYQ?feature=shared&t=969))
2. Interesting tricks around implementing efficient representations of dependency metadata in Rust data structures to allow super-fast solves. The team effectively wrote their own package version number parser and represented them cleverly inside a single `u64` to enable fast dependency comparisons as scalar integer comparisons. ([`@25:47`](https://youtu.be/gSKTfG1GXYQ?feature=shared&t=1547))
3. Some very clever usage of [HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Range_requests) to heuristically slice into compressed [PyPi](https://pypi.org) `.zip` package archives to parse package metadata from package registries, without having to retrieve the _entire_ `.zip` from PyPi first. ([`@31:42`](https://youtu.be/gSKTfG1GXYQ?feature=shared&t=1902))
4. A very clever cache design to enable super fast ephemeral Python [venv](https://docs.python.org/3/library/venv.html) creation, with some Rust zero-copy serialisation magic. ([`@34:49`](https://youtu.be/gSKTfG1GXYQ?feature=shared&t=2089), [`@37:27`](https://youtu.be/gSKTfG1GXYQ?feature=shared&t=2247))

So, to `uv` or not to `uv`? I'd definitely give `uv` a try - it really makes Python feel like magic.

<iframe
    width="560"
    height="315"
    src="https://www.youtube.com/embed/gSKTfG1GXYQ?si=1d7T1zEokffr4G-O"
    title="YouTube video player"
    frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen
></iframe>
