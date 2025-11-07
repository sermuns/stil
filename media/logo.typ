#import "lib.typ": *

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
#set text(font: "Libertinus Sans", fill: darker)
#set align(center + horizon)

#image(bytes(directory-bytes), height: 80%)
