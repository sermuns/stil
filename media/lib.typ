#let rusty-red = rgb(228, 58, 37)
#let darker = rusty-red.darken(80%)
#let directory-bytes = read("directory.svg").replace(
  "fill='#00000000'",
  "fill='" + darker.to-hex() + "'",
)
