---
title: "Rust for everyone [video]"
date: "2025-09-08"
tags: ['rust', 'software-engineering']
---

A [while ago](/micro-blog/2023-05-28-week-22/) I mentioned that I thought that Rust was a fiendishly complex language at first glance. Well, that hasn't changed. Don't get me wrong, I love Rust, and I wish I had the time to dive deeper into it, but it has a steep learning curve.

That said, I recently came across a fantastic video, ["Rust for Everyone"](https://www.youtube.com/watch?v=R0dP-QR5wQo), presented by Will Crichton at Jane Street that explores the various tools he's built to make Rust more accessible to newcomers. It's not in the video description, but here's an outline of the tools Will discusses:

1. [Aquascope](https://cel.cs.brown.edu/aquascope/): A tool that provides a visual representation of Rust's ownership model, making it easier to understand how ownership and borrowing work in Rust.
2. [Argus](https://github.com/cognitive-engineering-lab/argus): An improved Trait debugger that lets you drill down as much (or as little) as you want through a Trait type-check compiler error.
3. [Flowistry](https://github.com/willcrichton/flowistry): I think this is the coolest tool here. It allows you to trace the 'effect-flow' of your program, highlighting only the sections of code that could possibly affect a selected variable.
