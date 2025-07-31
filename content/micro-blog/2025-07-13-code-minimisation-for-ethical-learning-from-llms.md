---
title: Learning from LLMs ethically by practising code minimisation as a discipline
blog_section:
  - micro-blog
publish_branch: main
tags:
  - software-engineering
  - LLM
date: "2025-07-13"
---

Like many people nowadays, I find LLMs an invaluable tool for learning new concepts or vibecoding in unfamiliar stacks and langauages. It’s undeniably a massive accelerator when it comes to quickly iterating and learning.

But it has its downsides. There’s a strong argument to be said that by offloading the _challenge_ of learning something new, your core critical thinking atrophies and your ability to focus slides down the slippery slope of instant gratification.

However, there really are only so many hours in a day, and there are far too many days of learning to fit in. So a Faustian bargain I have done…

A useful exercise I employ is “_how far can I cut this code down before it stops working_”? LLMs seem to love overdoing things, even when instructed against it. I find the code produced by LLMs to be full of single-use functions, over-zealous try-except clauses, and overdone encapsulation. (Of course, the same could be said of any idiomatic Java programmer).

Of course, concision is not the sole determiner of ‘good code’, whatever that may be. But I think there is something to be said that parsimony is an ideal to strive for.[^1] Everything in computer science is just a long walk up a [Ladder of Abstraction](https://worrydream.com/LadderOfAbstraction/), after all.

In any case, I find that the sheer act of this “code minimisation” helps distill how the code works the way it does and I find myself learning huge amounts along the way.

[^1]: This may also be a case of Stockholm Syndrome: my statistical modelling professor was particularly zealous about parsimonious linear models…
