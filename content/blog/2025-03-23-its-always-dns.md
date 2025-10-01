---
title: "It's always DNS"
date: "2025-03-23"
description: "Debugging an internet issue led me me into the dark arts of DNS and an afternoon benchmarking DNS resolvers."
---

I recently moved to the city. As usual, considering the technological ruin that is the [NBN](https://en.wikipedia.org/wiki/National_Broadband_Network), I was fully expecting to go another round against the ISP gods.

I was not disappointed.

My old apartment used [HFC](https://en.wikipedia.org/wiki/Hybrid_fiber-coaxial), which was ... not _bad_. But since my new apartment supported [FTTP](https://en.wikipedia.org/wiki/Fiber_to_the_x), I figured I may as well upgrade my plan.

Oh boy.

First up, the new apartment already had an Optus Ultra Wi-Fi Gen 2 modem-router along with a terrible Optus NBN plan. Since I'm lucky enough to get a 50% discount on [More NBN](https://www.more.com.au) through work, the first step was to switch ISPs.

Small problem: Optus gives you the modem for "free"... if you stay with them for 36 months. Unfortunately for them, I had _no_ intention of staying locked-in for another 24 months. So now we owed Optus ~$200 for the privilege of keeping a terrible modem.

A few escalations, an increasingly tired me, multiple mentions of the [Telecommunications Industry Ombudsman](https://www.tio.com.au) ![](fn "ISPs really do hate this one simple trick"), and the fee was waived. They even let us keep the modem. Success?

Not a chance.

It turns out More NBN is just a reseller of Vocus NBN, which uses [PPoE](https://en.wikipedia.org/wiki/Point-to-Point_Protocol_over_Ethernet) authentication. Unfortunately for me, PPoE isn't supported by the $306 OPTUS Ultra Wi-Fi Gen 2™️ modem-router.

Yay.

One quick Facebook Marketplace sale later, and I was the proud owner of a Telstra Smart Modem Gen 2 for $40![](fn "I actually like these modems - they've got a little 4G chip that activates if the NBN goes down. Amazingly, this also works for free even if you don't have an active Telstra NBN plan!"). Phew. Long day, but problem solved, right?

Well, almost.

I plug in my new modem, authenticate to my More account using the PPoE settings, fuck around with the WAN/LAN cables for a few minutes (as is tradition), hit an obligatory speed test ...

Everything looks perfect - a steady 100Mb/s. ![](fn "Yeah, I know. 100Mb/s isn't *the* future. But it was *my* future, and I was loving every megabit of it.")

I open up YouTube ... wait a few **seconds** 
...  
...  
...  
... and finally my video starts loading.

Hmm. That's not right.

I try a few other websites and quickly notice: whenever anything buffers, there's a couple-second lag before the content is received over the network.
YouTube, Netflix, Hacker News, Websites on the High Seas .... all the same.

Damn. Maybe it's [DNS](https://en.wikipedia.org/wiki/Domain_Name_System)?

For those new to networking (that's me!), DNS (Domain Name System) is essentially the internet's phone book. When you type `youtube.com` into your browser, your computer needs to translate that human-friendly domain name into a machine-readable IP address (like `142.250.70.238`) before it can load anything.

This translation process works something like this:

1. Your device asks a DNS resolver, "Hey, what's the IP address for `youtube.com`?"
2. The DNS resolver looks up the answer and replies, "It's `142.250.70.238`"
3. Your device then connects to that IP address to load the website.

I check the DNS server in my modem's settings panel - I'm connected to the Telstra Londsale Exchange at `61.9.133.193` by default. But something's clearly not right...

After some quick digging, I find a simple little shell script called [`dnsperftest`](https://github.com/cleanbrowsing/dnsperftest) that benchmarks different DNS resolvers.

I add in Vocus' `111.220.1.1` and get something like the following (where `1000.00` represents a 1-second timeout):

```shell
% dnsperftest/dnstest.sh | sort -k 22 -n
                     test1   test2   test3   test4   test5   test6   test7   test8   test9   test10  test11  test12  test13  Average
1.0.0.1              6 ms    5 ms    8 ms    6 ms    7 ms    5 ms    6 ms    6 ms    7 ms    6 ms    6 ms    6 ms    48 ms     9.38
1.1.1.1              8 ms    6 ms    8 ms    6 ms    6 ms    7 ms    6 ms    18 ms   5 ms    6 ms    6 ms    6 ms    29 ms     9.00
vocus                41 ms   14 ms   11 ms   11 ms   9 ms    9 ms    7 ms    8 ms    7 ms    78 ms   7 ms    11 ms   17 ms     17.69
opendns              28 ms   6 ms    9 ms    6 ms    5 ms    20 ms   5 ms    7 ms    5 ms    6 ms    24 ms   6 ms    5 ms      10.15
google               25 ms   5 ms    23 ms   5 ms    6 ms    94 ms   5 ms    7 ms    7 ms    6 ms    92 ms   5 ms    21 ms     23.15
telstra-lonsdale     1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms 1000 ms   1000.00
```

Damn - turns out it **was** a DNS issue!

In my case, the Telstra modem was set to use Telstra's DNS resolver at the Lonsdale Exchange. Based on those timeout results, maybe their DNS was throttled for non-Telstra customers?

This explained the strange lag: whenever I tried to visit a new website or start streaming a video, my computer would send a DNS request and then......

I'm still not sure *why* this was happening, but manually changing my modem's DNS to Vocus' DNS resolver fixed all my latency woes. I actually ended up using Cloudflare's `1.1.1.1` on my Mac, and now everything's buttery smooth!

Moral of the story - it's not _always_ DNS, but when it is, hopefully this helps?
