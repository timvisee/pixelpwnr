# pixelpwnr
A quick [pixelflut][pixelflut] ([video][pixelflut-video]) client in
[Rust][rust] for use at [34C3][34C3], that _pwns_ whole pixelflut panels.

**Note:** This is a prototype project. Some things may not work correctly.
Or some things may work slow.

## Features
* Many concurrent drawing pipes, fast multithreading
* Animated images, with multiple frames
* Control over render sizes and offset
* Automatic image sizing and formatting
* Faster than most other clients :-)
* Linux, Windows and macOS

## Usage
Pixelflut a simple image:
```bash
# Flut a simple image.
# - To host 127.0.0.1 on port 8080
# - With the image: image.png
# - With 4 painting threads
# - With a size of (400, 400)
pixelpwnr 127.0.0.1:8080 -i image.png -c 4 -w 400 -h 400

# Other CLI syntax is also supported
pixelpwnr "127.0.0.1:8080" -i="image.png" -c=4 -w=400 -h=400
```

Pixelflut an animated image:
```bash
# Flut an animated image, with multiple frames.
# - To host 127.0.0.1 on port 8080
# - With the images: *.png
# - With 5 frames per second
# - With 4 painting threads
# - With a size of (400, 400)
# - With an offset of (100, 100)
pixelpwnr 127.0.0.1:8080 -i *.png --fps 5 -c 4 -w 400 -h 400 -x 100 -y 100
```

## Installation
For installation, Git and Rust cargo are required.
Install the latest version of Rust with [rustup][rustup].

Then, clone and install pixelpwnr with:

```bash
# Clone the project
git clone https://github.com/timvisee/pixelpwnr.git
cd pixelpwnr

# Install pixelpwnr
cargo install

# Start using pixelpwnr
pixelpwnr --help

# or run it directly from Cargo
cargo run --release -- --help
```

Or just build it and invoke the binary directly:

```bash
# Clone the project
git clone https://github.com/timvisee/pixelpwnr.git
cd pixelpwnr

# Build the project (release version)
cargo build --release

# Start using pixelpwnr
./target/release/pixelpwnr --help
```

## Help
```text
pixelpwnr --help

pixelpwnr 0.1
Tim Visee <timvisee@gmail.com>
A quick pixelflut client, that pwns pixelflut panels.

USAGE:
    pixelpwnr [OPTIONS] <HOST> --image <PATH>...

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --image <PATH>...    Image paths
    -w, --width <PIXELS>     Draw width (def: screen width)
    -h, --height <PIXELS>    Draw height (def: screen height)
    -x, --x <PIXELS>         Draw X offset (def: 0)
    -y, --y <PIXELS>         Draw Y offset (def: 0)
    -c, --count <COUNT>      Number of concurrent threads (def: 4)
    -r, --fps <RATE>         Frames per second with multiple images (def: 1)

ARGS:
    <HOST>    The host to pwn "host:port"
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.


[34C3]: https://events.ccc.de/congress/2017/wiki/index.php/Main_Page
[pixelflut]: https://cccgoe.de/wiki/Pixelflut
[pixelflut-video]: https://vimeo.com/92827556/
[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
