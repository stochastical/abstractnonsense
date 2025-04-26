---
title: btop of your resources
date: 2025-04-26
blog_section:
  - micro-blog
description: 'btop: the modern terminal resource monitor with an exceptional TUI'
link: https://github.com/aristocratos/btop
tags: ["design"]
---

[`btop`](https://github.com/aristocratos/btop) is now my default terminal resource monitor, supplanting `top`, `htop`, and all the others of that ilk. I wanted to spare a few words for its beautiful (and functional!) text-based user interface (TUI):

<img width="1010" alt="Image" src="https://github.com/user-attachments/assets/3256e7be-6cdc-44ca-ab54-ec60835ad5e6" />

- **pane management**: `btop` divides your terminal window into multiple information-dense _panes_ displaying CPU, memory, network, and process information simultaneously. What's fantastic about `btop` is that user ergonomics and customisation are clearly front and centre: each pane is numbered, and toggling off/on a pane is as simple as pressing the corresponding pane number. Instead of fiddling with a config file and refreshing (as many other command-line tools require), you can effortlessly switch between panes on-the-fly.
- **command input**: In a similar vein, attached to each pane is a set of commands that configure that view. A single letter of the command is highlighted in red, and pressing that letter will toggle that filter/sort/configuration in that panel. Putting the commands front-and-centre shifts the mental burden of recalling "What command displays my processes hierarchically" (`e`) from searching the manual to just _looking_ at the screen.
- **global configuration**: if you want your customisations to be sticky across sessions, there's a cleanly navigable and expressive configuration window that lets you apply globally persistent configurations. This is much nicer than setting command line flags or editing a config file.
- **cursor support**: Despite running in a terminal, you can simply click on processes to select them, or use scroll wheels to navigate long lists. This blending of terminal efficiency with GUI-like interactions creates a really slick experience that respects both keyboard purists and those who don't mind the forbidden practice of mouse navigation.
- **process management**: As an added bonus, selecting any process will allow you to send any signal straight from the TUI.

<img width="912" alt="Image" src="https://github.com/user-attachments/assets/d69fba1d-3e7d-4dc9-8954-f597472b73b2" />

What makes `btop` stand out is its intuitive keyboard navigation system. Unlike many other CLIs, `btop` maps essential functions to single keystrokes. This design philosophy means the interaction mode gets out of the way - toggling through complex system information and controls is always just a keystroke away. To borrow from [The Design of Everyday Things](https://en.wikipedia.org/wiki/The_Design_of_Everyday_Things), this feels like a set of masterfully crafted [_affordances_](https://en.wikipedia.org/wiki/Affordance).

Of course, this only works because of the constraints of what `btop` provides - unlike other CLIs with more complex combinations of configurations, resource management is effectively a set of independent components tied together into a master 'view controller'.
