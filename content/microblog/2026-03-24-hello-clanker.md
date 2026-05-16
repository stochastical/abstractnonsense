---
title: Hello clanker
tags: ['GPU']
date: 2026-03-24
aliases:
  - /micro-blog/2026-03-24-hello-clanker/
---

Thank you, work, for lending me a Dell Pro Max with GB10 to hack on. The Pro Max is effectively a twin for the NVIDIA DGX Spark, and it comes with:
- NVIDIA GB10 Blackwell GPU
- 128GB LPDDR5X unified memory
- 4TB PCIe SSD storage

## First impressions

### Hardware

- My Pro Max had already been setup by a teammate, so I had to go buy a cheap wired keyboard and mouse to connect before I could connect to Wifi.
- This thing is whisper quiet, even under LLM inference workloads, I barely hear the fan.
- It's super portable (not that I plan on moving relocating it much), and has plug-and-play USB-C power and the usual trio of Wifi/Ethernet/Bluetooth connectivity.

### Software

- The Pro Max runs a custom "NVIDIA DGX OS" skin of Ubuntu. My work daily-driver OS is macOS and I normally interact with headless GPUs for day-to-day work. So I was pleasantly surprised at how nice desktop Linux is now! There's no annoying animations, things respond quickly, there's a good amount of tooling built-in to the distro...
- I first setup [NVIDIA Sync](https://docs.nvidia.com/dgx/dgx-spark/nvidia-sync.html), which is a nice wrapper over SSH that lets me connect to the Pro Max from my MacBook, over the local network. It integrates nicely with VSCode, so that's quickly become my preferred way of working.
- I then setup [Tailscale](https://tailscale.com), and in literally under 10 minutes, I can now securely connect to the Pro Max from anywhere. Neatly, NVIDIA Sync just released an update that provides native Tailscale support. I hadn't used Tailscale before, but it seems like a super slick way to get all the benefits of WireGuard, without any of the hassle.
- Setting up the usual battery of inference providers/wrappers: [Open Web UI](https://github.com/open-webui/open-webui) and [Ollama](https://github.com/ollama/ollama) was ok, but I hit some CUDA issues with [vLLM](https://github.com/vllm-project/vllm) and [Transformers](https://github.com/huggingface/transformers). I could resolve the PyTorch+CUDA toolchain dependencies with `venv`s managed by `uv`, and careful choice of the required build. But vLLM required me to use the [NVIDIA vLLM NGC Container](https://catalog.ngc.nvidia.com/orgs/nvidia/containers/vllm), which seems to be NVIDIA's suggested way to run stuff on the box.

## What's next?

I'm really just dipping my toes in here, there's so much to learn. I'm using it for a work project, but it's really quite nice to be able to run a decently-sized model like GPT-OSS 120b with native MXFP4 weight support.
