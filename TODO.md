# TODO
- Add downloadable binary (don't require users to install Rust) 
- Resolve relative paths, or paths with a `~` correctly.
- Add alpha support.
- Instantly update images in painter threads,
  not just when the stopped drawing.
- Create a small listening server, to benchmark throughput.
- Properly handle connection errors, try to reconnect, show a proper message.

# Further optimizations
- Process and slice all images before starting, don't process them each frame
  again.
- Create a pixel map at start, instead of continuously getting pixels from the
  image.
- Convert whole image blocks to a single large command string, to push in one
  piece to the pixelflut server. Instead of pushing each pixel command
  separately.
- Do not draw transparant (alpha) pixels.
- Do not draw pixels outside the screen size.
- Further control buffering in drawing pipes.
- Allow UDP mode (for pixelflut servers that support it).
