---
title: Things rewrites their server architecture in Swift
blog_section:
  - micro-blog
date: "2025-05-23"
description: Things rewrites their server architecture in Swift
link: https://www.swift.org/blog/how-swifts-server-support-powers-things-cloud/
tags: ["software-engineering", "design"]
---

I've been a long time user of Cultured Code's [Things](https://culturedcode.com/things/) to-do app. It's slick, has well designed ergonomics, and is perfectly minimalistic. Things' Markdown support is tasteful and its approach to task management structured but pared back.

They've just announced a rewrite of their existing server-side infrastructure stack in Swift, the [linked post](https://www.swift.org/blog/how-swifts-server-support-powers-things-cloud/) and [blog post](https://culturedcode.com/things/blog/2025/05/a-swift-cloud/) are worth a read. 

From a technical perspective, I've always appreciated its rock-solid proprietary Things Cloud syncing service. In particular, I find it interesting the app asks for Local Network access to enable faster syncing:

> “Things” would like to find and connect to devices on your local network. Things uses the local network to provide faster sync between your devices.

I'd always thought they implemented some [CRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type) data structure and synchronised it on the LAN as well as via the server, but [according to their FAQ](https://culturedcode.com/things/support/articles/9706121/), their synchronisation is only server-side:

> None of your data is transmitted across the local network. Things merely sends a notification to your other devices telling them that new information is available, so that they can download it from the cloud.
