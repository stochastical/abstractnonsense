---
title: Networking Concepts by Beej
date: "2026-01-24"
tags: ['networking', 'textbook', 'study']
---

Computer networking always felt like an area in which I was lacking after completing my undergraduate CS education. A good while ago I stumbled across the excellent and freely accessible [_Beej's Guide to Networking Concepts_](https://beej.us/guide/bgnet0/) and worked through it, tracking my solutions in this [`solutions-guide-to-networking-concepts`](https://github.com/stochastical/solutions-guide-to-networking-concepts) repository.

All solutions herein were produced without any LLM assistance. Where any cloze-style template is provided by the textbook for the project, I've adopted that as the basis for my solution. Any errors or lapses of understanding are solely my own.

Please note that the selection of chapters for which solutions are presented is non-contiguous as not all chapters in the textbook had programmatic questions or projects associated. The list of projects is as follows:

| **Chapter** | **Textbook link**                                                                                                         | **Solutions**                                                                                           |
| ----------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| 5           | [HTTP Client and Server](https://beej.us/guide/bgnet0/html/split/project-http-client-and-server.html)                     | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter05) |
| 9           | [A Better Web Server](https://beej.us/guide/bgnet0/html/split/project-a-better-web-server.html)                           | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter09) |
| 12          | [Atomic Time](https://beej.us/guide/bgnet0/html/split/project-atomic-time.html)                                           | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter12) |
| 13          | [The Word Server](https://beej.us/guide/bgnet0/html/split/project-the-word-server.html)                                   | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter13) |
| 16          | [Validating a TCP Packet](https://beej.us/guide/bgnet0/html/split/project-validating-a-tcp-packet.html)                   | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter16) |
| 19          | [Computing and Finding Subnets](https://beej.us/guide/bgnet0/html/split/project-computing-and-finding-subnets.html)       | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter19) |
| 22          | [Routing with Dijkstra’s](https://beej.us/guide/bgnet0/html/split/project-6-routing-with-dijkstras.html)                  | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter22) |
| 30          | [Using Select](https://beej.us/guide/bgnet0/html/split/project-using-select.html)                                         | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter30) |
| 39          | [Multiuser Chat Client and Server](https://beej.us/guide/bgnet0/html/split/project-multiuser-chat-client-and-server.html) | [Solutions](https://github.com/stochastical/solutions-guide-to-networking-concepts/blob/main/chapter39) |

With the exception of [`uni-curses`](https://github.com/unicurses/unicurses)'s for the last project, there's no dependencies other than Python and the standard library.

My focus in these solutions is on simplicity and straightforward, imperative code, following the style used by Beej in the textbook. For production applications, you would certainly want to add more abstraction and robust parsing and error handling. These solutions are meant to be educational and illustrative.

I really enjoyed this little foray into the dark arts of networking, and learned a lot along the way. Most importantly, I learned a lot about what I still don't know, which I find equally valuable. Here's a little section-by-section retrospective on working through the exercises:

## 5 HTTP Client and Server

Here you build your own HTTP client and server, and practice opening sockets, sending bytes through and receiving bytes back. It was interesting reading through the [`socket`](https://docs.python.org/3/library/socket.html) docs to see what's exposed by the OS, and learning about arcane incantation option flags like `socket.SOL_SOCKET, socket.SO_REUSEADDR...`.

## 9 A Better Web Server

This was a fun little exercise, and got me thinking about how I would construct type-safe objects to represent the usual menagerie of HTTP headers, requests, paths, methods, protocols... For simplicity I went with [`TypeAlias`](https://typing.python.org/en/latest/spec/aliases.html) to have something more explicit to put into function signatures. I can't recall why I didn't use the (relatively new) `type` statement, though.

It was also a little scary (and fun!) to think about just how many vulnerabilities that a naive implementation of a web server would expose, and just how much complexity is required to remediate those security holes.

## 12 Atomic Time

```python {hl_lines=[3, 7]}
from contextlib import closing
def get_nist_time() -> int:
    with closing(socket.socket()) as s:
        s.connect((TIME_SERVER, TIME_PROTOCOL_PORT))
        logger.info(f'Connected to {TIME_SERVER}:{TIME_PROTOCOL_PORT}')
        
        response = b''.join(iter(lambda: s.recv(RESPONSE_BUFFER_SIZE), b''))
        logger.debug(f'Response: {response}')
        
        return int.from_bytes(response, byteorder='big')
```

I initially thought that using the [`closing`](https://docs.python.org/3/library/contextlib.html#contextlib.closing) utility was a smart way to automatically [`.close`](https://docs.python.org/3/library/socket.html#socket.close) the socket after exiting the context manager, but I just realised that:

> [Changed in version 3.2](https://docs.python.org/3/library/socket.html#socket-objects): Support for the context manager protocol was added. Exiting the context manager is equivalent to calling `close()`.

I also enjoyed using the functional-esque form of [`iter(callable, sentinel, /)`](https://docs.python.org/3/library/functions.html#iter). The docs use this example that I borrowed to read data into the buffer from the socket:

> One useful application of the second form of `iter()` is to build a block-reader. For example, reading fixed-width blocks from a binary database file until the end of file is reached:
```python
from functools import partial
with open('mydata.db', 'rb') as f:
    for block in iter(partial(f.read, 64), b''):
        process_block(block)
```

## 13 The Word Server

In this project you have to build up a server-side packet stream that encodes words (English words that is, not machine words) and a client that consumes the packets and parses them. I don't think my construction here is the cleanest, but I think it does the job for working within the scaffold of the solution and keeping it imperative.

## 16 Validating a TCP Packet

This was an exercise in finagling with bit twiddling operations and coercing Python to use bit arithmetic instead of integer arithmetic. But quite enjoyable once you squash the inevitable off-by-*n* errors.

## 19 Computing and Finding Subnets

More bit-twiddling exercises! This all came together quite nicely though, and made for some fun arithmetic. [`doctests`](https://docs.python.org/3/library/doctest.html) was my friend here for helping me validate the functions as I wrote them.

## 22 Routing with Dijkstra’s

This was a straight-forward implementation of [Dijkstra’s's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) for route finding. I haven't dug into it much, but I know there's a positively inordinate amount of complexity re efficient network routing. I quite enjoy reading the [Cloudflare Blog](https://blog.cloudflare.com) to expose myself to more about the wide, wild and wonderful world of the web.

## 30 Using Select

Not much to say here, just a small exercise on using [`select`](https://docs.python.org/3/library/select.html#select.select). This was instructive in the broader I/O and OS file integration sense, though.

## 39 Multiuser Chat Client and Server

By far my favourite exercise, and one of those "you've got to build it yourself once" programmer exercises I can tick off the list!

There's a veritable endless set of extensions I wanted to add to the project, but got called away by the pursuit of something else in the knowledge-verse. It was the perfect confluence of everything the book had worked through: bits and bytes, sockets, threads, error-handling, packet-wrangling... It was nice handling concurrent users and distributing packets between clients. As a small bonus, I could sneak in my favourite and sparsely utilised walrus operator:

```python {hl_lines=[1]}
while (cmd := read_command(f'{args.client_nickname}> ')) != CLIENT_QUIT_COMMAND:
    if cmd.startswith('/message'):
        _cmd, recipient, message = cmd.split(maxsplit=2)
        packet: bytes = chat_packet.dict_to_packet({
            'type': 'private_message',
            'recipient_nickname': recipient,
            'message': message
        })
    ...
```

## Conclusion

I'm glad I went through the textbook end-to-end and wrote up solutions for the learning experience, but I don't think I'll revisit this much. I think my time is probably better spent on learning other things now. At some point it might be nice to revisit these exercises in Rust, though. I think it provides better affordances and abstractions for writing cleaner and more expressive networking code than Python.
