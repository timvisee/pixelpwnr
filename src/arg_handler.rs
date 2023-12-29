extern crate clap;
extern crate num_cpus;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, disable_help_flag = true)]
pub struct Arguments {
    // manually redefine help, but without short option, because `-h`
    // is already used by the height option.
    /// Show this help
    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// The host to pwn "host:port"
    host: String,

    /// Image path(s)
    #[arg(
        short,
        long,
        value_name = "PATH",
        required = true,
        alias = "images",
        num_args(1..)
    )]
    image: Vec<String>,

    /// Draw width [default: screen width]
    #[arg(short, long, value_name = "PIXELS")]
    width: Option<u16>,
    /// Draw height [default: screen height]
    #[arg(short, long, value_name = "PIXELS")]
    height: Option<u16>,

    /// Draw X offset
    #[arg(short, value_name = "PIXELS", default_value_t = 0)]
    x: u16,
    /// Draw Y offset
    #[arg(short, value_name = "PIXELS", default_value_t = 0)]
    y: u16,

    /// Number of concurrent threads [default: number of CPUs]
    #[arg(short, long, aliases = ["thread", "threads"])]
    count: Option<usize>,

    /// Frames per second with multiple images
    #[arg(short = 'r', long, value_name = "RATE", default_value_t = 1)]
    fps: u32,

    /// Use binary mode to set pixels (`PB` protocol extension) [default: off]
    #[arg(short, long, alias = "bin")]
    binary: bool,

    /// Do not flush socket after each pixel [default: on]
    #[arg(short, long)]
    no_flush: bool,
}

/// CLI argument handler.
pub struct ArgHandler {
    data: Arguments,
}

impl ArgHandler {
    pub fn parse() -> ArgHandler {
        ArgHandler {
            data: Arguments::parse(),
        }
    }

    /// Get the host property.
    pub fn host(&self) -> &str {
        self.data.host.as_str()
    }

    /// Get the thread count.
    pub fn count(&self) -> usize {
        self.data.count.unwrap_or_else(num_cpus::get)
    }

    /// Get the image paths.
    pub fn image_paths(&self) -> Vec<&str> {
        self.data.image.iter().map(|x| x.as_str()).collect()
    }

    /// Get the image size.
    /// Use the given default value if not set.
    pub fn size(&self, def: Option<(u16, u16)>) -> (u16, u16) {
        (
            self.data
                .width
                .unwrap_or(def.expect("No screen width set or known").0),
            self.data
                .height
                .unwrap_or(def.expect("No screen height set or known").1),
        )
    }

    /// Get the image offset.
    pub fn offset(&self) -> (u16, u16) {
        (self.data.x, self.data.y)
    }

    /// Get the FPS.
    pub fn fps(&self) -> u32 {
        self.data.fps
    }

    /// Whether to use binary mode.
    pub fn binary(&self) -> bool {
        self.data.binary
    }

    /// Whether to prevent flushing after each pixel.
    pub fn no_flush(&self) -> bool {
        self.data.no_flush
    }
}
