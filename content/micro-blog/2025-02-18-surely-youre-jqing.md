---
title: Surely you're `jq`ing
blog_section:
  - Microblog
date: "2025-02-18"
description: Learning `jq` by reading the manual end-to-end.
link: https://jqlang.org/manual/
tags: ["software-engineering"]
---

Today I read through the [`jq`](https://jqlang.org) [manual](https://jqlang.org/manual/) cover-to-cover. For those unaware, `jq` is a popular CLI tool to query and manipulate JSON. It's also a Turing-complete mini-language with nice functional semantics that fits well into the ethos of composable CLI tools.

It was an exemplar of well-written technical documentation. Concise, well-written, littered with examples, and linking to an interactive playground to test-and-learn.

Some learnings:
1. It's surprisingly functional! You can implement recursive functions and use higher-order functions! For example, here's factorial in `jq`:
```bash
$ jq '[.,1]|until(.[0] < 1; [.[0] - 1, .[1] * .[0]])|.[1]'
```
1. It supports [string interpolation](https://jqlang.org/manual/#string-interpolation) - this is really nice if you're piping stuff from JSON into a string. Coupled with [format strings](https://jqlang.org/manual/#format-strings-and-escaping) this becomes frictionless:
```bash
$ echo '{"search":"hello; world"}' | jq -r '@uri "https://www.google.com/search?q=\(.search)"'
# https://www.google.com/search?q=hello%3B%20world
```
1. You can define functions that accept functions[^1], and [control structures](https://jqlang.org/manual/#breaking-out-of-control-structures) that allow labelling.
```bash
$ echo '[[1,2],[10,20]]' | jq -r 'def addvalue(f): . + [f]; map(addvalue(.[0]))'
#[[1,2,1], [10,20,10]]
```
1. You can traverse complex data structures with [first-class pathing](https://jqlang.org/manual/#path) support. And you can easily [modify nested structures](https://jqlang.org/manual/#complex-assignments) to extend objects.
2. For the category theorists/polyglots, there's a [denotational semantics paper](https://arxiv.org/pdf/2302.10576) written about `jq`.
3. _Bonus_: You can build a [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter [in `jq`](https://github.com/01mf02/jaq/blob/main/examples/bf.jq); and you can build a [`jq` interpreter in `jq`](https://github.com/wader/jqjq) - how's that for bootstrapping!

**Side note**: This is one of my goals for 2025 - read through documentation end-to-end to develop mastery over tools. I'm trying to prioritise selectively depth over breadth.

[^1]: The line between "functions" and "filters" is a little blurry to me.
