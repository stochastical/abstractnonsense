---
title: "Xteink X4 Pro review"
date: 2026-09-07
tags: ['books', 'reading']
---

![](xteink/xteink-x4-pro.jpeg)

I just bought an [Xteink X4 Pro](https://www.xteink.com/products/xteink-x4-pro-pocket-ereader) e-reader that snaps onto the back of my phone using MagSafe. 

I've been trying to do more reading as of late, and I hoped that having a dedicated reading device would aid me in that venture. Well, I'm very glad to say that it has! 

Here are some thoughts about the device.

## Hardware

- The touchscreen is responsive, with smooth scrolling support via swiping.
- The frontlight is superb, with granular control over the brightness and colour temperature.
- Battery life is indefatigable: the advantage of E Ink displays is that they only sip power when actively updating the page.
- The refresh rate is by no means instantaneous, but it's fast enough that it's not bothersome.
- The page turning buttons are accessible and easy to navigate with one hand.
- On-screen typography suffers a bit due to the lower screen resolution, but I didn't experience any eye strain. I like to read on the smallest font size, and I still found it legible.
- The X4 Pro comes equipped with a replacable microSD card. I think it'd be pretty hard to fill up all the storage with books!
- It's extremely lightweight, so you can hold it for hours on end without tiring.
- The lack of USB-C is my only gripe. It comes equipped with a (finicky) USB-C to pogo-pin adapter that provides both power and data transfer. It does the job, but it's by no means ergonomic. And with all my other devices on USB-C now, it's frustrating to have to keep around an adapter.

## Firmware

- The stock OS is ... not great. But, thankfully, the device is unlocked and can be easily flashed using the browser via the [Web Serial API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Serial_API). I'm using the superb [CrossPoint](https://crosspointreader.com) firmware, and there's a wonderful ecosystem of forks and plugins that have sprouted around it.
- The device runs on an ESP32, which is a wonderfully low-cost microcontroller. It's pretty incredible that the firmware runs on just a few hundred kilobytes of RAM. I love these examples of constrained computing devices!
- Since the ESP32 has WiFi support, there's a plethora of ways to transfer books: USB-C, local network, WiFi hotspot, and [Calibre plugin](https://github.com/crosspoint-reader/calibre-plugins). There's even a way to connect it to [Libby](https://libbyapp.com) to borrow DRM-protected books from your library.
- There's a lot to customise, if you so desire. I found the stock fonts to be a bit grainy, but I've been enjoying using [Lexend Deca](https://fonts.google.com/specimen/Lexend+Deca)

As the old adage goes, the best book you have is the one you're carrying. If it broke today, I'd probably go out and buy another one.
