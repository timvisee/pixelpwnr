extern crate clap;
extern crate num_cpus;

use clap::{Arg, ArgMatches, App};

use app::*;



/// CLI argument handler.
pub struct ArgHandler<'a> {
    matches: ArgMatches<'a>,
}

impl<'a: 'b, 'b> ArgHandler<'a> {

    /// Parse CLI arguments.
    pub fn parse() -> ArgHandler<'a> {
        // Handle/parse arguments
        let matches = App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
            .arg(Arg::with_name("HOST")
                .help("The host to pwn \"host:port\"")
                .required(true)
                .index(1))
            .arg(Arg::with_name("image")
                .short("i")
                .long("image")
                .alias("images")
                .value_name("PATH")
                .help("Image paths")
                .required(true)
                .multiple(true)
                .display_order(1)
                .takes_value(true))
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("PIXELS")
                .help("Draw width (def: screen width)")
                .display_order(2)
                .takes_value(true))
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("PIXELS")
                .help("Draw height (def: screen height)")
                .display_order(3)
                .takes_value(true))
            .arg(Arg::with_name("x")
                .short("x")
                .value_name("PIXELS")
                .help("Draw X offset (def: 0)")
                .display_order(4)
                .takes_value(true))
            .arg(Arg::with_name("y")
                .short("y")
                .value_name("PIXELS")
                .help("Draw Y offset (def: 0)")
                .display_order(5)
                .takes_value(true))
            .arg(Arg::with_name("count")
                .short("c")
                .long("count")
                .alias("thread")
                .alias("threads")
                .value_name("COUNT")
                .help("Number of concurrent threads (def: CPUs)")
                .display_order(6)
                .takes_value(true))
            .arg(Arg::with_name("fps")
                .short("r")
                .long("fps")
                .value_name("RATE")
                .help("Frames per second with multiple images (def: 1)")
                .display_order(7)
                .takes_value(true))
            .get_matches();

        // Instantiate
        ArgHandler {
            matches,
        }
    }

    /// Get the host property.
    pub fn host(&'a self) -> &'b str {
        self.matches.value_of("HOST")
            .expect("Please specify a host")
    }

    /// Get the thread count.
    pub fn count(&self) -> usize {
        self.matches.value_of("count")
            .unwrap_or(&format!("{}", num_cpus::get()))
            .parse::<usize>()
            .expect("Invalid count specified")
    }

    /// Get the image paths.
    pub fn image_paths(&'a self) -> Vec<&'b str> {
        self.matches.values_of("image")
            .expect("Please specify an image paths")
            .collect()
    }

    /// Get the image size.
    /// Use the given default value if not set.
    pub fn size(&self, def: Option<(u32, u32)>) -> (u32, u32) {
        (
            self.matches.value_of("width")
                .unwrap_or(
                    &format!("{}", def.expect("No screen width set or known").0)
                )
                .parse::<u32>()
                .expect("Invalid image width"),
            self.matches.value_of("height")
                .unwrap_or(
                    &format!("{}", def.expect("No screen height set or known").1)
                )
                .parse::<u32>()
                .expect("Invalid image height"),
        )
    }

    /// Get the image offset.
    pub fn offset(&self) -> (u32, u32) {
        (
            self.matches.value_of("x")
                .unwrap_or("0")
                .parse::<u32>()
                .expect("Invalid X offset"),
            self.matches.value_of("y")
                .unwrap_or("0")
                .parse::<u32>()
                .expect("Invalid Y offset"),
        )
    }

    /// Get the FPS.
    pub fn fps(&self) -> u32 {
        self.matches.value_of("fps")
            .unwrap_or(&format!("{}", DEFAULT_IMAGE_FPS))
            .parse::<u32>()
            .expect("Invalid frames per second rate")
    }
}
