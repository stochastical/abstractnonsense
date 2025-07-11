---
title: Better Python path handling with Pathlib and Git
blog_section:
  - blog
publish_branch: main
description: Better Python path handling using Pathlib and Git
tags:
  - software-engineering
date: "2025-07-11"
---

One of the most frustrating problems in ad-hoc data science projects is broken file paths.

You write a script that loads a model, grabs data, or instantiates a config from your local disk. It works perfectly on your machine, then someone else runs it and… catastrophe: `FileNotFoundError: No such file or directory`. Uh oh, looks like someone just got bit by hard-coded paths or assumptions about where the script is being run from.

Since I'm normally working inside a `git` repo, I've started writing all my file paths as relative to the _repo root_, and dynamically finding that path with [GitPython](https://github.com/gitpython-developers/GitPython). For example, supposing I have the following project structure:

```text
my-project/
├── README.md
├── data
│   └── iris.csv
└── src
    ├── model.py
    └── preprocess.py
```

and I want to grab `data/iris.csv` from within `preprocess.py` (or perhaps, much more heavily nested sub-folders).

Instead of using a relative path (`../data/iris.csv`) or an absolute path (`/Users/yossi/my-project/data/iris.csv'`), I can do the following:

```python
# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "gitpython",
# ]
# ///

from pathlib import Path
import git

repo_root = Path(git.Repo(search_parent_directories=True).git.rev_parse("--show-toplevel"))

print((repo_root / 'data' / 'iris.csv').read_text())
```

> Side note: I really love [pathlib](https://docs.python.org/3/library/pathlib.html)'s [overloading](https://github.com/python/cpython/blob/175ada2806abef16187361ba4ad5242fb9284f60/Lib/pathlib/_local.py#L148) of the `/` operator to allow syntactic sugar for path augmentation.

Also, if you haven't seen it before, I'm using [Python inline script metadata](https://packaging.python.org/en/latest/specifications/inline-script-metadata/) here to specify the dependencies. You can run this script with `uv run --script src/preprocess.py` and [`uv`](https://github.com/astral-sh/uv) will take care of resolving and installing the dependencies within a `venv`.

I'm anticipating that this will probably trigger a lot of people:
1. But now you have an extra dependency on the `GitPython` package!
2. What happens if `git` isn't installed on the system? (Looking at you, Windows).
3. What happens if there's `git` [submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules)?

And to all this, I say: "yes, that's true". I don't believe that this is idiomatic or good for production or anything close to it. But for adhoc collaborative projects, littered with `*.ipynb` notebooks (this, even I cannot stand), it restores _some_ level of sanity amidst the throes of data science passion.

Please let me know what your preferred path-wrangling design pattern is. I'm just trying to find my path here...
