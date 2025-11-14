---
title: 'Talk: Implementing Efficient Language Models under Homomorphic Encryption'
date: 2025-11-11
tags: ['homomorphic-encryption', 'LLMs', 'mathematics', 'lecture']
description: 'A summary of a talk on implementing efficient language models under homomorphic encryption by Donghwan Rho at Seoul National University.'
---

Today ([11/11 Pepero Day](https://en.wikipedia.org/wiki/Pepero_Day) in Korea), I had the pleasure of attending a fascinating talk on *Implementing Efficient Language Models under Homomorphic Encryption* by [Donghwan Rho (노동환)](https://donghwan-rho.github.io) at the Seoul National University Research Institute of Mathematics' [2025 11.11 Symposium](https://rim.math.snu.ac.kr/bbs/board.php?bo_table=Math_Event&wr_id=81).

I am on holiday in Korea and didn't want to miss this opportunity to learn more about homomorphic encryption - an area I've been been increasingly fascinated by as of late!

## Talk Abstract

> As large language models (LLMs) become ubiquitous, users routinely share sensitive information with them, raising pressing privacy concerns. Homomorphic encryption (HE) is a promising solution to privacy-preserving machine learning (PPML), enabling computations on encrypted data. However, many core components of LLMs are not HE-friendly, limiting practical deployments. In this talk, we investigate the main bottleneck - softmax, matrix multiplications, and next-token prediction - and how we address them, moving toward the practical implementation of LLMs under HE.

## What is Homomorphic Encryption?

The core premise of homomorphic encryption is that it allows computations to be performed on *encrypted* data without needing to decrypt it first. Loosely speaking, the homomorphism property of HE allows for addition and multiplication to be preserved under the ciphertext transform. From these primitive operations (and some others, depending on the scheme), you can compile homomorphic *circuits* that perform semi-arbitrary computations on encrypted data. The big obstacle to overcome is that homomorphic encryption schemes are typically very computationally expensive, and so making them efficient enough for practical use cases (such as LLM inference) is an active area of research.

There are various schemes for homomorphic encryption, each with their own set of permissible operations and associated trade-offs, but I'll delegate a more thorough treatment of the intricacies to this excellent [blog post series](https://www.jeremykun.com/2024/05/04/fhe-overview/) by [Jeremy Kun](https://www.jeremykun.com).

## Talk Summary

Full disclaimer: The talk was delivered in Korean, which I sadly do not speak, so I followed the talk partially via live English translation and partially via the English slides. Donghwan was kind enough to share his slides with me afterwards, but they're not currently publicly available. 

The talk was partitioned into three sections:

1. An overview of the CKKS homomorphic encryption scheme and its applications to private transformer inference.
2. The application of HE to fine-tuning LLMs: the replacement of the `softmax` in the attention layer with Gaussian-kernel attention and the use of LoRA to reduce the number of ciphertext-ciphertext matrix multiplications. This section is based on the paper *Encryption-Friendly LLM Architecture* by Rho et al., ICLR 2025 ([OpenReview](https://openreview.net/forum?id=pbre0HKsfE)).
3. How to address the lack of random sampling algorithms in CKKS HE schemes when performing next-token prediction. There's a lot of intricacy here that I sadly couldn't follow, but a particularly interesting component that stood out to me was applying the Travelling Salesman Problem to define an optimisation problem over the token indices to minimise cosine embedding distances between adjacent tokens. This section is based on the paper *Traveling Salesman-Based Token Ordering Improves Stability in Homomorphically Encrypted Language Models* by Rho et al ([arXiv](https://arxiv.org/abs/2510.12343)).

## What's next?
Many thanks to Donghwan Rho for the talk! I'm looking forward to reading the papers in more detail when I have some time and hope to update this page or blog more about what I learn in the future!

<!-- TODO: Add in photos I took of campus -->

![Seoul National University Campus](/micro-blog/IMG_9308.jpeg)