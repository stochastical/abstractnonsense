---
title: TIL Hyperlinks in terminal emulators
link: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
tags:
  - TIL
date: "2026-04-13"
---

TIL that terminal emulators can opt to [support hyperlinks](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda) via the OSC 8 escape sequence. In [terminals that support it](https://github.com/Alhadis/OSC8-Adoption/?tab=readme-ov-file), the following snippet should produce a clickable link:

```shell
$ printf '\e]8;;https://abstractnonsense.xyz\e\\This is a link to this blog\e]8;;\e\\\n'
This is a link to this blog
```

I tested this to work in iTerm and VS Code's terminal, but not in the native macOS Terminal. Conceivably, this could be used nefariously, since a URL rendered may not match the URL _opened_. The [linked post](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda#security) has some valuable points re mitigation and security rationale, though.
