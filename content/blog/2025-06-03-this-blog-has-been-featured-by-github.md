---
title: This blog has been featured by GitHub!
blog_section:
  - blog
publish_branch: main
description: This blog has been featured by GitHub!
link: https://issue-ops.github.io/docs/reference/examples
tags:
  - abstract-nonsense
date: "2025-06-03"
---

This blog has been featured by GitHub!*

Well, sort of.

To publish posts to this blog, I’m abusing GitHub’s excellent Issue tracking capability. I have a [GitHub Issue Form Template](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/syntax-for-issue-forms) at [`.github/ISSUE_TEMPLATE/new-blog-post.yml`](https://github.com/stochastical/abstractnonsense/blob/main/.github/ISSUE_TEMPLATE/new-blog-post.yml) which gives me a nice template to complete with fields like blog `title`, `date`, `section` etc.

The form template automatically labels the issue as a `new-blog-post` which triggers a [GitHub Actions workflow](https://github.com/stochastical/abstractnonsense/blob/main/.github/workflows/new-blog-post.yml) to (very hackily) dump the contents of the Issue into a Markdown file and commit.

The commit triggers a [Cloudflare Pages](https://pages.cloudflare.com/) hook that builds my site with [Hugo](https://gohugo.io/) and deploys it to my `abstractnonsense.xyz` domain.

To parse the issue template, I’m using the excellent [`issue-ops/parser`](https://github.com/issue-ops/parser) workflow maintained by [Nick Alteen](https://github.com/ncalteen), a GitHub staffer! Nick’s very kindly [featured my blog](https://issue-ops.github.io/docs/reference/examples) as an example of [_IssueOps_](https://issue-ops.github.io/docs/), described as:

> the process of interacting with GitHub Issues and Pull Requests to invoke commands. For example, approving requests to access important systems.

The key advantages of this workflow are:
- I can publish posts on mobile! In fact, this post was written all on mobile! This was probably the biggest requirement. I wanted to be able to make the writing experience as frictionless as possible and push content from anywhere without having to have my laptop on-hand.
- I get rich Markdown rendering support with [GitHub Flavoured Markdown](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax)
- I can sort of abuse GitHub’s user-upload-file-to-Issue CDN for storing media. This is probably sub-optimal, and I should probably migrate to an idiomatic pattern like Cloudflare’s R2 or AWS S3[^1] at some point. But for now, it just works 😉.

And yes, this is indeed an obligatory blog post about how I write blog posts. I spend far too much time thinking about blogging, tweaking my blog internals, or blogging about blogging instead of _actually blogging_. I’m trying to change that, I swear.


[^1]: Or just plain old git - premature optimisation is the root of all evil, after all.
