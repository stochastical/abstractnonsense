---
title: Internationali(z|a)tion is hard
blog_section:
  - micro-blog
publish_branch: main
description: Good UI design across locales is hard
tags:
  - design
  - software-engineering
date: "2025-06-09"
---

I came across a UI glitch today in my Uber app. At first glance it appears to be a preposterous oversight, the `s` in `Favourites` has been [orphaned](https://en.m.wikipedia.org/wiki/Widows_and_orphans)!

I live in Australia, where we closely follow British English spelling - meaning it’s “Fav**ou**rites” and not “Fav**o**rites”. In the world of UI/UX, it’s common to use localisation dictionaries to map strings to [locale-appropriate](https://en.m.wikipedia.org/wiki/Internationalization_and_localization) versions. I suspect some UI designer carefully crafted this screen for US English and mapped over to AU English, accidentally committing a tiny typographic crime.

![Image](https://github.com/user-attachments/assets/ab2d7dc3-0374-41c0-a3d2-d63ae8c92832)
