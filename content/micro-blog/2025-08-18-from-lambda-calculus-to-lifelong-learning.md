---
title: From Lambda Calculus to Lifelong Learning
tags:
  - mathematics
  - computer-science
  - lambda-calculus
date: "2025-08-18"
---

About a decade ago, whilst meandering through a late-night Wikipedia rabbit hole![*](fn "Some things haven't changed at all"), I stumbled across the page for the [Lambda Calculus](https://en.wikipedia.org/wiki/Lambda_calculus).

A small footnote linked to [An Introduction to Lambda Calculus and Scheme](https://www.cs.unc.edu/~stotts/723/Lambda/scheme.html), a transcript of a short talk presented by Jim Larson, and, well, I fell in love. I loved the fact that there was this beautiful correspondence between foundational mathematics and models of computation; and that you could describe it through these symbolic wrappers called Lambda terms. There was something deep and abstract about it that I felt immensely drawn to.

The fact that you could start with primitive definitions like

$$
\begin{cases}
    \textsf{true}  &= \lambda x.\lambda y.x \\
    \textsf{false} &= \lambda x.\lambda y.y
\end{cases}
$$

and with a couple of basic reduction operations ([\(\alpha\)-reduction](https://en.wikipedia.org/wiki/Lambda_calculus#α-conversion) and [\(\beta\)-reduction](https://en.wikipedia.org/wiki/Lambda_calculus#β-reduction)) construct the [ternary conditional](https://en.wikipedia.org/wiki/Ternary_conditional_operator):

$$
\textsf{if-then-else} = \lambda a.\lambda b.\lambda c.((a)b)c
$$

just blew my mind. I'm not *completely* sure, but I think looking up the [Y-combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator) is how I ended up discovering [Hacker News](https://news.ycombinator.com/news) and began my lifelong, monotonically increasing tab-count trajectory.

Fast forward to university, and I had the pleasure of [studying](https://handbook.unimelb.edu.au/2025/subjects/comp30020) the Lambda Calculus in a more crystallised form, where I fell in love a second time - this time with Haskell.

And now? Well, now you're reading this post on a blog called [_Abstract Nonsense_](https://en.wikipedia.org/wiki/Abstract_nonsense), a subject of layers of abstraction, much like the calculus that inspired it.

> Speaking of abstraction, if you haven't had a chance to read the excellent post [Up and Down the Ladder of Abstraction](https://worrydream.com/LadderOfAbstraction/) by [Bret Victor](https://worrydream.com), go and read it now. It's superb.![*](fn "He's even got a [post on the Lambda Calculus](https://worrydream.com/AlligatorEggs/)!")
