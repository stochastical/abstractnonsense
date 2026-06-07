// Compile with:
// typst compile --format svg change-of-variables.typ change-of-variables.svg

#import "@preview/cetz:0.5.2"

#set page(width: auto, height: auto, margin: 10pt)

#cetz.canvas({
  import cetz.draw: *

  // Key geometry points.
  let origin = (0, 0)
  let top = (3, 3)
  let right = (6, 0)
  let bottom = (3, -3)

  // Axis extents.
  let u_axis_start = (-0.8, 0)
  let u_axis_end = (7.2, 0)
  let v_axis_start = (0, -4.0)
  let v_axis_end = (0, 4.0)

  // Label anchors.
  let u_label = (7.45, 0.0)
  let v_label = (0, 4.35)
  let origin_label = (-0.5, -0.5)
  let right_label = (6.55, 0.28)
  let top_label = (3.0, 3.28)
  let bottom_label = (3.0, -3.38)
  let map_label_1 = (9.2, 1.15)
  let map_label_2 = (9.2, 0.2)

  let label(at, body) = content(at, body)

  // Axes in (u,v)-coordinates.
  set-style(stroke: 1.1pt + black)
  line(v_axis_start, v_axis_end)
  line(u_axis_start, u_axis_end)

  // Diamond image of [0,1]^2 under x = u-v, y = u+v.
  set-style(stroke: 1.5pt + rgb("#2563eb"), fill: none)
  line(origin, top, right, bottom, close: true)

  // Axis labels.
  set-style(stroke: none)
  label(u_label, $u$)
  label(v_label, $v$)

  // Vertex labels.
  label(origin_label, $(0,0)$)
  label(right_label, $(1,0)$)
  label(top_label, $(1/2,1/2)$)
  label(bottom_label, $(1/2,-1/2)$)

  // Change-of-variables equations.
  label(map_label_1, $x = u - v$)
  label(map_label_2, $y = u + v$)
})
