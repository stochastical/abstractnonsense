---
title: Activation functions and empiricism
blog_section:
  - micro-blog
publish_branch: main
tags:
  - llm
  - quote
date: "2026-03-04"
---

In deep learning literature, there's a veritable menagerie of different activation functions that are commonly employed betwixt layers. Proponents of one or another class of functions will usually proffer up some rationalisation for what makes their choice grounded: differentiability, smoothness, computational complexity, numerical stability, concision...

Today I was reading through [_GLU Variants Improve Transformer_](https://arxiv.org/abs/2002.05202) by Noam Shazeer (also of [_Attention is all you need_](https://proceedings.neurips.cc/paper_files/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf) fame) and came across this gem of empiricism:

> We have extended the GLU family of layers and proposed their use in Transformer. In a transfer-learning setup, the new variants seem to produce better perplexities for the de-noising objective used in pre-training, as well as better results on many downstream language-understanding tasks. These architectures are simple to implement, and have no apparent computational drawbacks. **We offer no explanation as to why these architectures seem to work; we attribute their success, as all else, to divine benevolence.**

Well, I appreciate the honesty.
