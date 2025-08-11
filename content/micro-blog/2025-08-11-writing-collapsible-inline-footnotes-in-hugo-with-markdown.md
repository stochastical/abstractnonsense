---
title: Writing collapsible inline footnotes in Hugo with Markdown
blog_section:
  - micro-blog
publish_branch: main
date: "2025-08-11"
tags:
  - design
  - css
  - hugo
---

I use [Hugo](https://github.com/gohugoio/hugo) as the static site builder for my blog. I love it because it's fast (_blisteringly_ fast) and for the most part it just gets out of the way.

However, Hugo's support for customising the rendering of Markdown into HTML is ... lacking, to say the least. Hugo uses [Goldmark](https://github.com/yuin/goldmark/) under the hood, which supports [Extensions](https://github.com/yuin/goldmark/?tab=readme-ov-file#list-of-extensions), but Hugo doesn't natively plug in to those. I'd previously been using a Hugo [shortcode](https://gohugo.io/content-management/shortcodes/) to wrap around some arbitrary Markdown and convert it into a footnote HTML:

```html
<label for="fn{{ .Get "id" }}">{{ .Get "label" }}</label>
<input type="checkbox" id="fn{{ .Get "id" }}" />
<small id="fn{{ .Get "id" }}">
    {{ .Inner | markdownify }}
</small>
```

But using it in Markdown means writing something inordinately verbose and clunky like:

```markdown
This is some text,{{</* footnote id="1" label="footnote!" */>}}and here's some **markdown** in a footnote{{</* /footnote */>}}
```

I considered switching to [pandoc](https://github.com/jgm/pandoc) for my Markdown preprocessing, but that would mean changing my whole build system and using GitHub Actions with a `pandoc` workflow instead of the (super quick) [Cloudflare Pages Hugo runner](https://developers.cloudflare.com/pages/framework-guides/deploy-a-hugo-site/). I did some browsing and realised that I was not the only one [thinking deeply](https://scottstuff.net/posts/2024/12/17/more-notes-on-notes/) about footnotes and [sidenotes in Hugo](https://scottstuff.net/posts/2024/12/16/sidenotes-in-hugo-with-fixit/).

I've been struggling with this for a while, but I realised that Hugo supports [render hooks](https://gohugo.io/render-hooks/) for select Markdown elements. Unfortunately, footnotes are not included, but [images](https://gohugo.io/render-hooks/images/) are! In Markdown, the standard syntax for images looks like: `![description](destination "title")`, which is not *too* dissimilar to the footnote syntax `[^label]: footnote`. So I built a little `layouts/_markup/render-image.html` hook:

```html
{{- if eq .Destination "fn" -}}
{{- /* This is a footnote using image syntax */ -}}
{{- $label := .PlainText | default "note" -}}
{{- $content := .Title | default "Footnote content" -}}
{{- $id := .Ordinal -}}
<label for="fn{{ $id }}" class="footnote-trigger">{{ $label }}</label><input type="checkbox" id="fn{{ $id }}" class="footnote-checkbox" /><small class="footnote-aside" id="fn{{ $id }}">{{ $content | markdownify }}</small>
{{- else -}}
{{- /* This is a regular image */ -}}
<img src="{{ .Destination | safeURL }}"
  {{- with .PlainText }} alt="{{ . }}"{{ end -}}
  {{- with .Title }} title="{{ . }}"{{ end -}}
>
{{- end -}}
```

so that I can write:

```markdown
![fun fact](fn "_Abstract Nonsense_ is a blog about mathematics and computer science.")
```

and, with some pure CSS magic, have it render into a collapsible inline footnote! You can see it live on my website now: I've started using it to add maths proofs and asides as footnotes that expand inline on hover.
