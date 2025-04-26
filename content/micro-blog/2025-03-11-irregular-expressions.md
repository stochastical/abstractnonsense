---
title: Irregular Expressions
blog_section:
  - Microblog
date: "2025-03-11"
description: Can you encode Luhn's algorithm into regex?
tags: ["computer-science"]
---

Someone at work asked if it's possible to validate credit card numbers with the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) in regex.

Technically, a [regular language](https://en.wikipedia.org/wiki/Regular_language) could recognise valid fixed-length credit card numbers by brute-force enumerating all possible sequences. But as a more general solution, I don't think [DFA](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)s can support the modular arithmetic required for arbitrary sequence lengths...

Possible or not, I feel incredibly [nerd sniped](https://xkcd.com/356/).

![](https://imgs.xkcd.com/comics/nerd_sniping.png)
