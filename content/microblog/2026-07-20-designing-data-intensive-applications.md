---
title: Designing Data-Intensive Applications by Martin Kleppmann
tags: ['textbook', 'programming', 'quotes']
date: 2026-07-20
---

I completed reading through [_Designing Data-Intensive Applications_](https://dataintensive.net) by Martin Kleppmann.

This was the [2nd edition](https://www.youtube.com/watch?v=SVOrURyOu_U), which has been updated to incorporate changes in cloud computing design patterns and other developments in the database world (e.g. vector and geospatial indices). I particularly enjoyed the commentary on the philosophy of stream processing and event sourcing.

The book doesn't shy away from theory, but the emphasis is still placed on application. I enjoyed seeing many of the patterns I'd encountered at work consolidated together with a balanced exploration of benefits and tradeoffs of each. It's worth spending some time reading through a selection of the references: you can glean much from these alone. 

For an assortment of excerpts:

The book opens each chapter with a pithy quote, which I really enjoyed. A favourite of mine was

> The Internet was done so well that most people think of it as a natural resource like the Pacific Ocean, rather than something that was man-made. When was the last time a technology with a scale like that was so error-free?
> -- [Alan Kay](https://web.archive.org/web/20121018004959/http://www.drdobbs.com/architecture-and-design/interview-with-alan-kay/240003442)

and a great joke on the nature of functional programming:

> *We believe in the separation of Church and state*.

Something that really surprised me:

> Counterintuitively, the performance advantage of in-memory databases is not due to the fact that they don’t need to read from disk. Even a disk-based storage engine may never need to read from disk if you have enough memory, because the operating system caches recently used disk blocks in memory anyway. Rather, they are faster because they avoid the overheads of encoding in-memory data structures in a form that can be written to disk 

I haven't yet talked about it on the blog, but in my spare time I've been writing a [server](https://github.com/stochastical/dictd-rs) for the [DICT protocol](https://en.wikipedia.org/wiki/DICT) that allows for dictionary lookups. I've had to consider various lookup approaches and how best to handle in-memory and from-disk reads, so this was pertinent information!

Finally, a very poignant call-to-action in a world that's rapidly changing due to the effects of technology:

> Given the large impact that software and data have on the world, we as engineers must remember that we carry a responsibility to work toward the kind of world that we want to live in: a world that treats people with humanity and respect. Let’s work together toward that goal.
