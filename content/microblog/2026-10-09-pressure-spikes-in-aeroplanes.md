---
title: Pressure spikes in aeroplanes
date: 2026-10-09
tags: ["dsp"]
---

Today I had my first lecture for my [MPhil](/blog/2026-06-20-mphil-advanced-computer-science), [Digital Signals Processing](https://www.cl.cam.ac.uk/teaching/2627/L314/), and the lecturer ended with a small puzzle: a graph of pressure over time, with a mean value of approximately 775 hPa and a sharp impulse. What physical event is this waveform a measurement of?

Well, pressure at sea level is approximately 1000 hPa, so the air must be thinner here. After some nudging we discover that we're in an aircraft! Aeroplanes have [pressurised cabins](https://en.wikipedia.org/wiki/Cabin_pressurization) to prevent a bunch of physiological issues. The impulse spike in the signal is the sudden drop in pressure as the bathroom toilet is flushed! There's a valve that briefly allows a pressure differential between the air in the cabin and the outside air to produce a suction force to [flush the bowl](https://en.wikipedia.org/wiki/Vacuum_sewer). The [environmental control system](https://en.wikipedia.org/wiki/Environmental_control_system) is constantly monitoring and stabilising the cabin pressure, hence the spike and gradual tapering off of pressure after the flush.

The lesson is: the barometer in your mobile phone is remarkably sensitive, play around with the various sensors! It's a marvel of modern engineering that we've managed to miniaturise so many components and pack them into such a small form factor. Next time I take a flight I'll have to capture this on my phone and attach a screenshot.
