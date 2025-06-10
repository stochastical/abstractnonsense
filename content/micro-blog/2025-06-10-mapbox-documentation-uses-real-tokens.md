---
title: Mapbox documentation uses real tokens
blog_section:
  - micro-blog
publish_branch: main
description: A neat trick to allow rapid prototyping
tags:
  - design
  - software-engineering
date: "2025-06-10"
---

I've been working on a small side project that requires embedding an interactive map. I spent some time evaluating different mapping providers - initially [OpenStreetMap](https://www.openstreetmap.org/) with [leaflet](https://leafletjs.com) for annotations. OpenStreetMap is great for rapid prototyping since it's free and doesn't require any API key, but it doesn't look great (by default, at least) and it doesn't have the same rich POI data integration.

I dabbled with [Mapbox](https://www.mapbox.com) next and was really impressed. It has end-to-end API coverage for heaps of mapping use cases, and provides lots of examples of how to integrate Mapbox into your choice of language/platform/framework. However, the thing that really stood out to was their onboarding design. Once you create an account and login, Mapbox injects your API keys directly into the example code on the webpage. They literally inject your personal API key into the standard examples for logged in users so that you can copy/paste and immediately start working. 

I ended up settling on Apple's [Mapkit JS](https://developer.apple.com/maps/web/). It's got a generous free tier allowance (as per Apple usual, only if you pay for an Apple Developer account, which is fair) and a nice UI. The downsides are that it doesn't have any React bindings, so you have to roll your own wrapper, and the overall documentation is lacking. The dearth of documentation is especially noticeable when contrasted to the tomes provided by Mapbox.
