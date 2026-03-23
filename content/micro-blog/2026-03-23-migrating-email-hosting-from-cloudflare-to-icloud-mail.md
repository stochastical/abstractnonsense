---
title: Migrating email hosting from Cloudflare to iCloud Mail
tags:
  - abstractnonsense.xyz
date: "2026-03-23"
---

Up until now, I've been using [Cloudflare Email Routing](https://developers.cloudflare.com/email-routing/) to forward all emails sent to the `hello@abstractnonsense.xyz` email address to my personal iCloud email. This is seamless to configure, as Cloudflare is my domain registrar and keeper of my DNS fiefdom. However, this service is a one-way pipe: if I want to respond to emails, I have to do so from my personal email.

I've now migrated to Apple Mail's [Custom Email Domain](https://support.apple.com/en-au/102540) support, which is bi-directional. This fits in nicely alongside iCloud [Hide My Email](https://support.apple.com/en-au/guide/icloud/mm9d9012c9e8/icloud) that I've been using to create ephemeral email addresses to sign up to services and obfuscate my real iCloud address. You do require a paid iCloud+ subscription for both these services, however.

Both Cloudflare Email Forwarding and iCloud+ Custom Email Domains support subaddressing too, which is neat. Wrt spam, Cloudflare also offers [Email Address Obfuscation](https://developers.cloudflare.com/waf/tools/scrape-shield/email-address-obfuscation/) to automagically hide emails from bots. I find that I still receive some crypto-spam emails, but nothing egregious (and I suspect those mostly stem from it being on my GitHub profile page).

I also considered [Proton Mail](https://proton.me/support/custom-domain)'s (paid) custom domain support, but opted for Apple Mail since that's my default client and I already subscribe to iCloud+. In any case, this is just a small quality-fof-life improvement!
