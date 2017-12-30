use std::io::Error;
use std::thread;
use std::time::Duration;

use image::{DynamicImage, Pixel};

use app::PAINTER_IMAGE_WAIT_DELAY_MILLIS;
use color::Color;
use pix::client::Client;
use rect::Rect;



/// A painter that paints on a pixelflut panel.
pub struct Painter {
    client: Client,
    area: Rect,
    offset: (u32, u32),
    image: Option<DynamicImage>,
}

impl Painter {
    /// Create a new painter.
    pub fn new(client: Client, area: Rect, offset: (u32, u32), image: Option<DynamicImage>) -> Painter {
        Painter {
            client,
            area,
            offset,
            image,
        }
    }

    /// Perform work.
    /// Paint the whole defined area.
    pub fn work(&mut self) -> Result<(), Error> {
        // Make sure there is an image
        if self.image.is_none() {
            // Show a warning
            println!("Painter thread has no image yet to paint, waiting...");

            // Sleep a little
            thread::sleep(Duration::from_millis(PAINTER_IMAGE_WAIT_DELAY_MILLIS));
            return Ok(());
        }

        // Get an RGB image
        let image = self.image.as_mut().unwrap().to_rgb();

        // Loop through all the pixels, and set their color
        for x in 0..self.area.w {
            for y in 0..self.area.h {
                // Get the pixel at this location
                let pixel = image.get_pixel(x, y);

				// Get the channels
				let channels = pixel.channels();

				// Define the color
				let color = Color::from(
					channels[0],
					channels[1],
					channels[2],
				);

                // Set the pixel
                self.client.write_pixel(
                    x + self.area.x + self.offset.0,
                    y + self.area.y + self.offset.1,
                    &color,
                )?;
            }
        }

        // Everything seems to be ok
        Ok(())
    }

    /// Update the image that should be painted
    pub fn set_image(&mut self, image: DynamicImage) {
        self.image = Some(image);
    }
}
