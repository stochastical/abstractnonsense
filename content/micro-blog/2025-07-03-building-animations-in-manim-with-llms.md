---
title: Building animations in Manim with LLMs
blog_section:
  - micro-blog
publish_branch: main
description: Building animations in Manim with LLMs
date: "2025-07-03"
tags: ["design", "mathematics"]
---

[Manim](https://github.com/3b1b/manim) is a Python animation engine designed to build animated mathematical explainer videos ([examples here](https://www.manim.community/awesome/)) created by Grant Sanderson of the superb YouTube channel [3Blue1Brown](https://www.youtube.com/c/3blue1brown).

A couple years ago I wanted to write my own mathematics explainer videos with Manim, but found the learning curve to be pretty steep at the outset.

The user experience has since improved dramatically - in large part because of the [Manim Community Edition](https://github.com/manimCommunity/manim), but also because there's now enough samples in recent LLM training runs that they can produce workable Manim code!

I stumbled across a [neat website](https://github.com/marcelo-earth/generative-manim) ([GitHub](https://github.com/marcelo-earth/generative-manim)) that allows you to prompt either a SOTA LLM or a fine-tuned GPT-3.5 model and have Manim code generated and rendered in-browser. The quality is passable at best (and I've observed better results prompting whatever default model ChatGPT is serving up as of writing), but the barrier to entry just dropped remarkably. I can jump right in with a MVP and start tweaking!

Also shoutout to a cool project I stumbled along on the way: [ManimWeb](https://github.com/manim-web/manim-web), a Dart-backed engine to build _interactive_ maths animations for the web. Development seems to have died a few years ago, but the [Fourier Transform example](https://manim-web.hugos29.dev/examples/fourier-transform) is neat to play around with.
