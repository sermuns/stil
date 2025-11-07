#let rusty-red = rgb(228, 58, 37)

#set page(
  height: 1em,
  width: 1em,
  margin: 0em,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: rusty-red,
    radius: 10%,
  ),
)
#set text(font: "Libertinus Sans")
#set place(center + horizon)

#place(image("directory.svg", height: 9pt))
// #place(dx: -2pt, text(rusty-red.desaturate(50%).lighten(70%))[L])
