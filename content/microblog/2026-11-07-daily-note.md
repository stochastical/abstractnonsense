---
title: "Daily Note: 7 November 2026"
date: 2026-11-07
tags: ["daily-note", "cambridge", "julia"]
---

I don't normally re-watch lectures to study, but I spent today going over half of my [L314 Digital Signals Processing](https://www.cl.cam.ac.uk/teaching/2627/L314/) lectures in preparation for an upcoming test. The Cambridge terms are quite compressed, so it's helpful going over content to help piece it all together. I've been reading through [_Digital Signal Processing_ by Thomas Holton](https://www.cambridge.org/highereducation/books/digital-signal-processing/CFB036179B9CEDB3CB2E3B7D6E646D72) to supplement the coursework, and I'm finding it quite helpful.

I selected the DSP module somewhat on a whim: I wanted something that was a bit more mathematical, and I thought I'd learn a lot of useful tools to reason about [physical phenomena](/microblog/2026-10-09-pressure-spikes-in-aeroplanes). I was very glad to be affirmed on both counts, and the lecturer, [Markus Kuhn](https://en.wikipedia.org/wiki/Markus_Kuhn_(computer_scientist)), is fabulous.

I've also made good progress on my second assignment for DSP, and just need to put some finishing touches on it. It's my first time using [Julia](https://julialang.org) and I'm quite enjoying it (certainly moreso than MATLAB). I especially like that it's 1) open-source 2) fast. I know some people have gripes with it's "time to first plot" due to the [JIT](https://docs.julialang.org/en/v1/devdocs/jit/), but it seems to have vastly improved with more recent Julia versions.

The assignments are written with [Pluto](https://plutojl.org) notebooks, which allows for interactivity and easy export to PDF. Fortunately, unlike its dreaded cousin the Jupyter notebook, Pluto strictly enforces reproducibility. That means that the notebook format is just plain text (unlike the Kafkaesque JSON format `.ipynb` files come in); package dependencies are specified in the notebook itself; and Pluto defines a [dependency tree](https://plutojl.org/en/docs/plutodependencyexplorer/) to stop you from overwriting variables. There are some UI issues I've come across, and it's not as well-supported as Jupyter notebooks are, but it's a vast improvement nonetheless.

Today was cool and misty which made for a lovely and scenic walk into town for some groceries. Tomorrow I need to go over some past Tripos exam questions for DSP, work on my assignments for both Probabilistic Machine Learning and Machine Learning in the Physical World, and read some more papers. There're also a few neat maths things I want to comment on, but I'll see how the day progresses.

Until tomorrow.
