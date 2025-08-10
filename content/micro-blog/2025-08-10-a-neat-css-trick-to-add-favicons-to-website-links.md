---
title: A neat CSS trick to add favicons to website links
blog_section:
  - micro-blog
publish_branch: main
tags:
  - design
  - css
date: "2025-08-10"
---

I discovered a neat CSS trick that automatically lets you prepend an SVG 'favicon' before URLs linking to a particular domain. For example, for links to `github.com`, you can embed an [octocat](https://github.com/octocat) SVG using an attribute selector:

```css
a[href*="github.com"]::before {
    content: "";
    display: inline-block;
    width: 1em;
    height: 1em;
    margin-right: 0.35em;
    vertical-align: -0.15em;
    background: currentColor;
    /* Use mask so color adapts to currentColor */
    mask: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.62 7.62 0 0 1 2-.27c.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8Z"/></svg>') center/contain no-repeat;
}
```

We apply the favicon as a [CSS mask](https://developer.mozilla.org/en-US/docs/Web/CSS/mask) to properly render the transparent components of the logo against the background.
