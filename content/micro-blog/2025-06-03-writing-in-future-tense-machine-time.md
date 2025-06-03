---
title: 'Writing in Future Tense: Machine Time'
blog_section:
  - micro-blog
publish_branch: main
description: Beware UTC vs local timezones
tags:
  - software-engineering
date: "2025-06-03"
---

I [published a blog post](https://abstractnonsense.xyz/blog/2025-06-03-this-blog-has-been-featured-by-github/) last night but it never appeared on the site. My GitHub Actions workflow kicked in, my commit hit the server, my Cloudflare build completed with no warnings or errors  - everything looked good.

The culprit? Timezone mismatch. I'm writing from AEST (+10, I'm in Melbourne), but Cloudflare Pages Workers builds in UTC ("server time"). Hugo saw my future timestamp and politely ignored the post.

**The fix:** Use `hugo --buildFuture` as the build command in Cloudflare Pages settings to include posts "in the future". I'll consider this a cautionary tale ... it's not the first time timezones have caused me havoc in production.
