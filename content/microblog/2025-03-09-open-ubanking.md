---
title: Open uBanking
date: 2025-03-09
description: Exploring OpenBanking API access in Python
link: https://github.com/eidorb/ubank
aliases:
  - /micro-blog/2025-03-09-open-ubanking/
---

In Australia, we have the [Consumer Data Right](https://www.cdr.gov.au), a government mandated interoperability standard for the Banking (and now Energy) sector.

Also known as [Open Banking](https://www.ausbanking.org.au/priorities/open-banking/), the idea is to ensure that banks expose APIs that allow safe access to your transactions history and account data without having to rely on hacky methods like screen scraping[^1].

Unfortunately, due to how the standard is enforced, it's pretty tough to get access to your own transactions without going through an "approved data broker" that charges a premium for it (like [Basiq](https://www.basiq.io/home.html)), or a free app with limited export support like [Frollo](https://frollo.com.au).

Well, if you're a [Ubank](http://ubank.com.au/) customer, it appears some genius has reverse-engineered the internal Ubank API from examining the API requests the frontend makes, and has exposed it as a Python-consumable API with Passkey authentication support. Genius.

As an aside, the API mentions Python "descriptors", which I'd not heard of, but seems like an intriguing functionality worth exploring...

[^1]: A big player in this space is [Yodlee](https://www.yodlee.com): you provide your bank login password, and they scrape your transactions data on some interval. For obvious reasons, this breaches most banks' T&Cs.
