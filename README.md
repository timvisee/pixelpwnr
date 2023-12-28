# pixelpwnr
A quick [pixelflut][pixelflut] ([video][pixelflut-video]) client in
[Rust][rust] for use at [34C3][34C3], that _pwns_ whole pixelflut panels.

For a high performance pixelflut server implementation, see:  
[→ pixelpwnr-server (server)][pixelpwnr-server]

**Note:** This is a prototype project. Some things may not work correctly.
Or some things may work slow.

## Features
* Many concurrent drawing pipes, fast multithreading
* Animated images, with multiple frame images
* Control over render sizes and offset
* Automatic image sizing and formatting
* Blazingly fast [binary protocol](https://github.com/timvisee/pixelpwnr-server#the-binary-px-command) (`PB` with `--binary`)
* Faster than most other clients :-)
* Linux, Windows and macOS

## Usage
Pixelflut a simple image:
```bash
# Flut a simple image.
# - To host 127.0.0.1 on port 8080
# - With the image: image.png
# - With 4 painting threads
# - With the size of the screen (default)
pixelpwnr 127.0.0.1:8080 -i image.png -c 4

# Other CLI syntax is also supported
pixelpwnr "127.0.0.1:8080" --image="image.png" -c=4
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

Use the `--help` flag, or see the [help](#help) section for all available
options.

## Installation
For installation, Git and Rust cargo are required.
Install the latest version of Rust with [rustup][rustup].

Then, clone and install `pixelpwnr` with:

```bash
# Clone the project
git clone https://github.com/timvisee/pixelpwnr.git
cd pixelpwnr

# Install pixelpwnr
cargo install -f

# Start using pixelpwnr
pixelpwnr --help

# or run it directly from Cargo
cargo run --release -- --help
```

Or just build it and invoke the binary directly (Linux/macOS):

```bash
# Clone the project
git clone https://github.com/timvisee/pixelpwnr.git
cd pixelpwnr

# Build the project (release version)
cargo build --release

# Start using pixelpwnr
./target/release/pixelpwnr --help
```

## Performance & speed optimization
There are many things that affect how quickly pixels can be painted on a
pixelflut server.  
Some of them are:
- Size of the image that is drawn.
- Amount of connections used to push pixels.
- Performance of the machine `pixelpwnr` is running on.
- Network interface performance of the client.
- Network interface performance of the server.
- Performance of the pixelflut server.

Things that improve painting performance:
- Use a wired connection.
- Use a LAN connection, closely linked to the pixelflut server. The lower
  latency the better, due to the connection being over TCP.
- Use as many threads (`-c` flag) as the server, your connection and your
  machine allows.
- Paint a smaller image (`-w`, `-h` flags).
- Paint in an area on the screen, where the least other things are pained.
- Use multiple machines (servers) with multiple `pixelpwnr` instances to push
  pixels to the screen.

## Future improvements
This application is still in the prototyping phase, and many things can be
improved for significantly better performance and usability.
See the [TODO](TODO.md) file for a list of future improvements.

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
    -x <PIXELS>              Draw X offset (def: 0)
    -y <PIXELS>              Draw Y offset (def: 0)
    -c, --count <COUNT>      Number of concurrent threads (def: CPUs)
    -r, --fps <RATE>         Frames per second with multiple images (def: 1)

ARGS:
    <HOST>    The host to pwn "host:port"
```

## Relevant projects
* [pixelpwnr-server (server)][pixelpwnr-server]

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.


[34C3]: https://events.ccc.de/congress/2017/wiki/index.php/Main_Page
[pixelflut]: https://cccgoe.de/wiki/Pixelflut
[pixelflut-video]: https://vimeo.com/92827556/
[pixelpwnr-server]: https://github.com/timvisee/pixelpwnr-server
[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
