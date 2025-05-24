---
title: View Transition Web API
blog_section:
  - micro-blog
publish_branch: main
description: View Transition Web API is really slick
link: https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API
tags:
  - design
  - css
date: "2025-05-24"
---

The (relatively) new [View Transition API](https://developer.mozilla.org/en-US/docs/Web/API/View_Transition_API) is really slick! Simply adding the following CSS to my blog enabled [same-document view transitions](https://developer.chrome.com/docs/web-platform/view-transitions#same-document_view_transitions), no JavaScript required! Go ahead and give it a try now! Simply click a link to another page on this site and you should observe a seamless transition occur.

```css
@view-transition {
  navigation: auto;
}
```

If you want to add even more pizzaz, you can declare CSS keyframe animations:

```css
/* Create a custom animation */
@keyframes move-out {
  from {
    transform: translateX(0%);
  }

  to {
    transform: translateX(-100%);
  }
}

@keyframes move-in {
  from {
    transform: translateX(100%);
  }

  to {
    transform: translateX(0%);
  }
}

/* Apply the custom animation to the old and new page states */
::view-transition-old(root) {
  animation: 0.4s ease-in both move-out;
}

::view-transition-new(root) {
  animation: 0.4s ease-in both move-in;
}
```

For a blog like this there's no real use, but for more complex web applications, the View Transition API is a really seamless way to integrate smooth transitions.

As of writing, it's supported by the major browsers, excepting Firefox 😔.
