---
title: "Migrating CDN to Cloudflare R2"
date: 2026-08-03
tags:
  - abstractnonsense.xyz
---

I've migrated my static images to use Cloudflare [R2](https://www.cloudflare.com/products/r2/) as my object store and image CDN. R2 exposes an [S3-compatible API](https://developers.cloudflare.com/r2/examples/aws/aws-cli/), which makes it nice and easy to use the `aws` CLI to interact with the bucket.

Previously I was (ab)using GitHub's free image CDN via their user media upload support in Issues (`https://github.com/user-attachments/assets/<hash>`). However, this is slow and not very idiomatic. For R2, I've [attached the bucket](https://developers.cloudflare.com/r2/buckets/public-buckets/) to my public sub-domain, so raster images are now served from `cdn.abstractnonsense.xyz` and should load much faster.

I've also migrated my [inline-footnote Hugo Markdown render hook](/microblog/2025-08-11-writing-collapsible-inline-footnotes-in-hugo-with-markdown) from overriding image-links to overriding regular-links. Now, relative image paths are prepended with the CDN.

The publishing workflow still involves manually uploading images to the desired path and ensuring the Hugo path reflects it, but I might make this more ergonomic in the future. I'm thinking about building a folder-add triggered Shortcut on my macOS that'll take care of the bucket syncing.

This closes out a longstanding issue on my blog ([`#35`](https://github.com/stochastical/abstractnonsense/issues/35)), with the exception of supporting web-optimised compressed image formats ([example solution](https://rednafi.com/misc/behind-the-blog/#oxipng)). There are also some built-in Cloudflare offerings to do image transformations, but I believe they'd cost, and are overkill at this point.

I'm a little bit torn as to whether to start publishing more photos here. On the one hand, I enjoy seeing other bloggers' photo snaps (especially of nature). On the other hand, I'd quite like to preserve the ethos of having a snappy and minimalist blog. Maybe I can use a subdomain or my [`library`](/library) section to compartmentalise my photos. There's a cool image of a [Hummingbird hawk-moth](https://en.wikipedia.org/wiki/Hummingbird_hawk-moth) I took in Korea that I'd love to publish...
