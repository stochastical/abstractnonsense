---
title: "Seminar: Scaling AI at the Speed of Light: Rethinking Compute, Memory, and Networks Through Optics"
date: 2026-10-08
tags: ["seminar", "talk", "cambridge"]
---

I attended my first [seminar talk](https://talks.cam.ac.uk/talk/index/273180/) today at Cambridge, for the Computer Lab Systems Research Group by [Dr Paolo Costa](https://www.microsoft.com/en-us/research/people/pcosta/) of Microsoft Research. Paolo introduced quite a fascinating idea and here's my recollection of it (mistakes all mine).

One of the biggest bottlenecks for inference workloads on GPUs is [High Bandwidth Memory](https://en.wikipedia.org/wiki/High_Bandwidth_Memory) to load weights for processing by tensor cores. HBM has been rapidly increasing in price due to scarcity (hyperscalers are consuming HBM at an insatiable rate), even impacting the consumer market as manufacturing resources are diverted to more valuable lines. It’s difficult to physically scale the layers of HBM up, partially due to lower manufacturing yield and partially because cooling the system quickly becomes a bottleneck.

Paolo’s team proposed the use of [microLEDs](https://en.wikipedia.org/wiki/MicroLED) to augment existing copper wiring in data centres, to allow for memory banks to be placed at a greater distance whilst preserving communication throughput. MicroLEDs have the advantage of being cheaper to manufacture and are significantly less power-hungry than lasers. However, they emit light with [Lambertian diffuse reflectance](https://en.wikipedia.org/wiki/Lambertian_reflectance), so Microsoft built a "lens-cap" to mitigate light spillage using [total internal reflection](https://en.wikipedia.org/wiki/Total_internal_reflection) to collimate the beam. MicroLEDs are also more prone to failures over their lifetime than lasers or copper, but this can be accounted for by populating more channels than strictly required for a given transmission bandwidth.

The broader vision is a decoupling of compute from memory using a network of cheap microLED connectors. This would necessitate a complete rethinking of systems architecture, since compute and memory become ephemeral interlinked components that can scale up and down as required for a particular workload. This breaks the assumption of rack-fixed GPUs that require large capital expenditures and careful power budgeting to allocate resources to maximise utilisation.

I found this to be quite an intriguing idea, you can read more about it via the [SIGCOMM paper](https://www.microsoft.com/en-us/research/publication/mosaic-breaking-the-optics-versus-copper-trade-off-with-a-wide-and-slow-architecture-and-microleds/) or the [Microsoft Research Newsroom](https://news.microsoft.com/source/features/innovation/using-inexpensive-microleds-microsoft-networking-innovation-aims-to-make-datacenters-more-efficient/).
