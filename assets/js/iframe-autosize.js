// Sizes same-origin embeds to their content height.
//
// There is no CSS-only way to fit an iframe to its document: the element has no
// intrinsic size, so any fixed height or aspect-ratio either clips a tall
// component or leaves dead space under a short one. Embeds here are served from
// this site, so the parent can read the child document directly and measure it.
//
// Cross-origin frames throw on contentDocument access; those keep the CSS
// fallback height.
(() => {
  const fit = (frame) => {
    let doc;
    try {
      doc = frame.contentDocument;
    } catch {
      return false; // cross-origin
    }
    if (!doc?.body) return false;

    // Suppress the child's own scrollbar before measuring. While it is present
    // it both steals width (forcing a reflow that changes the height we are
    // trying to read) and is the exact artefact that gives the embed away.
    doc.documentElement.style.overflow = 'hidden';

    // scrollHeight over getBoundingClientRect: the latter reports the viewport
    // height of the child's root, not the height of its content.
    const height = Math.ceil(Math.max(
      doc.body.scrollHeight,
      doc.body.offsetHeight,
      doc.documentElement.scrollHeight,
      doc.documentElement.offsetHeight,
    ));
    if (height <= 0) return false;

    // Only write when the value actually moves. Resizing the frame reflows the
    // document inside it, which re-triggers the observer — without this guard
    // sub-pixel rounding can oscillate forever.
    if (Math.abs(parseFloat(frame.style.height || '0') - height) > 1) {
      frame.style.height = `${height}px`;
    }
    return true;
  };

  const attach = (frame) => {
    if (!fit(frame)) return;

    const doc = frame.contentDocument;

    // The embedded component styles itself with prefers-color-scheme but never
    // declares the color-scheme property, so the UA paints its canvas white in
    // dark mode. Setting it on the child root lets the canvas go transparent
    // and the page background show through. Done here rather than in the
    // component because that directory is a git submodule.
    doc.documentElement.style.colorScheme = 'light dark';

    if (!('ResizeObserver' in window)) return;
    const observer = new ResizeObserver(() => fit(frame));
    observer.observe(doc.documentElement);
    // The component redraws its SVG on viewport changes, which can alter height
    // without the documentElement box itself being re-observed.
    frame.contentWindow?.addEventListener('resize', () => fit(frame));
  };

  for (const frame of document.querySelectorAll('.iframe-embed iframe')) {
    frame.addEventListener('load', () => attach(frame));
    if (frame.contentDocument?.readyState === 'complete') attach(frame);
  }
})();
