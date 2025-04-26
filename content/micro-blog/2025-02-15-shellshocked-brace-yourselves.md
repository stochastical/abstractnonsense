---
title: Shellshocked? Brace yourselves!
date: "2025-02-15"
description: A neat trick in `bash`!
tags: ["software-engineering"]
---

I just discovered that to capture multiple lines of `stdout` from a shell script and redirect them to a file, you can simply wrap them in braces!

For example, my ["Create a blog post via a GitHub Action triggered on an Issue creation"](https://github.com/stochastical/abstractnonsense/blob/main/.github/workflows/new-blog-post.yml) workflow uses this snippet:

```bash
{
  echo "---"
  jq 'del(.content)' "parsed_issue.json" | yq -P
  echo "---"
  echo ''
  
  # Inline "content" key for the body
  jq -r '.content' "parsed_issue.json"
} > content/micro-blog/"$FILENAME"
```
