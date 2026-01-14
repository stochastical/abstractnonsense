---
title: GitHub hijacks and breaks browser search
date: "2026-01-14"
tags:
  - design
  - ui
---

I like to keep a [word list](https://abstractnonsense.xyz/blog/2025-08-01-a-living-log-of-lexical-learnings/) of any new and interesting words I come across day-to-day. Today I was curious how many entries were in my list and went to search the [YAML file](https://github.com/stochastical/abstractnonsense/blob/main/data/vocabulary.yaml) on GitHub.

To my delight, I discover that GitHub has hijacked the native `Cmd-F` browser search. To top it off, seems the maximum number of matches GitHub's search returns is limited to `200`.

I'll excuse a search function that at least reports `> 200 results matched`, but there is no indication in the UI that this is the case. Even if you navigate to the 200th result and can see additional matches in the viewport, GitHub's UI steadfastly refuses their existence.

Here's what it looks like on macOS with Safari version 26.2:

<img width="1349" height="850" alt="Safari GitHub search" src="https://github.com/user-attachments/assets/36deaefd-d290-4aae-902a-6193e5d55ce1" />

A quickly-disappearing hint on the GitHub search model reported that hitting `Cmd-F` _again_ brings up the native browser search. I gave that a whirl, but it still wasn't finding all matches. I thought I'd inspect the page source for the text elements to see what's going on under the hood:

<img width="1680" height="916" alt="Viewport bug Safari GitHub" src="https://github.com/user-attachments/assets/75a3616c-4598-40ef-8c62-b773b12317d4" />

... and hit a render error. This is another lovely bug I've been running into when the viewport changes quickly in Safari. I can see a `data-target="react-app.reactRoot"` attribute lurking in the dark: maybe I shouldn't besmirch the React-ification of GitHub's UI, though. After all, at least the [raw](https://raw.githubusercontent.com/stochastical/abstractnonsense/refs/heads/main/data/vocabulary.yaml) file searches instantly in the browser:

<img width="1680" height="959" alt="Safari GitHub raw search" src="https://github.com/user-attachments/assets/5fd44a7e-5bf7-46b7-8ddd-72a06bf370a8" />

Luckily, Firefox retains its native search experience:

<img width="1637" height="963" alt="Firefox GitHub search" src="https://github.com/user-attachments/assets/38a9d0e5-ddfd-4e4c-a7af-1c604039acac" />

Maybe I'm getting this all wrong, and I'd love to be corrected. But it sure feels like GitHub's UI&UX has become increasingly slow and unfriendly as of late. In any case, I've reported this bug and I'll update if I hear anything back.
