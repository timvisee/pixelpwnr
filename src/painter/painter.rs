use std::io::Error;
use std::sync::mpsc::Receiver;

use image::{DynamicImage, Pixel};

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
    pub fn new(
        client: Client,
        area: Rect,
        offset: (u32, u32),
        image: Option<DynamicImage>,
    ) -> Painter {
        Painter {
            client,
            area,
            offset,
            image,
        }
    }

    /// Perform work.
    /// Paint the whole defined area.
    pub fn work(&mut self, img_receiver: &Receiver<DynamicImage>) -> Result<(), Error> {
        // Wait for an image, if no image has been set yet
        if self.image.is_none() {
            // Show a warning
            println!("Painter thread is waiting for an image...");

            // Sleep a little
            // TODO: Do a proper error return here
            match img_receiver.recv() {
                Ok(image) => self.set_image(image),
                Err(_) => return Ok(()),
            }

            // We may now continue
            println!("Painter thread received an image, painting...");
        }

        // Get an RGB image
        let image = self.image.as_mut().unwrap().to_rgb();

        // Loop through all the pixels, and set their color
        for x in 0..self.area.w {
            for y in 0..self.area.h {
                // Update the image to paint
                if let Ok(image) = img_receiver.try_recv() {
                    self.set_image(image);
                }

                // Get the pixel at this location
                let pixel = image.get_pixel(x, y);

                // Get the channels
                let channels = pixel.channels();

                // Define the color
                let color = Color::from(channels[0], channels[1], channels[2]);

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
