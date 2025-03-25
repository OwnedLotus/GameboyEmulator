pub mod display {
    use raylib::{
        RaylibHandle, RaylibThread,
        color::Color,
        math::Vector2,
        prelude::{RaylibDraw, RaylibDrawHandle},
        texture::{Image, RaylibTexture2D, Texture2D},
    };
    use rayon::vec;

    pub struct Display {
        pixel_data: Vec<u8>,
        texture: Texture2D,
        pixel_scale: f32,
        width: i32,
        height: i32,
        position: Vector2,
    }

    impl Display {
        pub fn new(
            rl: &mut RaylibHandle,
            thread: &RaylibThread,
            width: i32,
            height: i32,
            color: Color,
            pixel_scale: i32,
            position: Vector2,
        ) -> Self {
            let mut pixel_data: Vec<u8> = vec![0; (width * height * 4) as usize];
            for y in 0..height {
                for x in 0..width {
                    if x >= 0 && y >= 0 && x < width && y < height {
                        let index = ((y * width + x) * 4) as usize;
                        pixel_data[index] = color.r;
                        pixel_data[index + 1] = color.g;
                        pixel_data[index + 2] = color.b;
                        pixel_data[index + 3] = color.a;
                    }
                }
            }

            let image = Image::gen_image_color(width, height, color);
            let texture = rl.load_texture_from_image(thread, &image).unwrap();

            Self {
                pixel_data,
                texture,
                pixel_scale: pixel_scale as f32,
                width,
                height,
                position,
            }
        }

        pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
            if x >= 0 && y >= 0 && x < self.width && y < self.height {
                let index = ((y * self.width + x) * 4) as usize;
                self.pixel_data[index] = color.r;
                self.pixel_data[index + 1] = color.g;
                self.pixel_data[index + 2] = color.b;
                self.pixel_data[index + 3] = color.a;
            }
        }

        // not hot path
        pub fn draw_buffer(
            &mut self,
            pixel_data: Vec<u8>,
            width: usize,
            height: usize,
            pixel_scale: usize,
        ) {
            let scaled_width = width * pixel_scale;
            let scaled_height = width * pixel_scale;

            let mut scaled_data = vec![0; scaled_width * scaled_height * 4];

            for y in 0..height {
                for x in 0..width {
                    let original_index = (y * width + x) * 4;

                    let r = pixel_data[original_index];
                    let g = pixel_data[original_index + 1];
                    let b = pixel_data[original_index + 2];
                    let a = pixel_data[original_index + 3];

                    for i in 0..pixel_scale {
                        for j in 0..pixel_scale {
                            let scaled_x = x * pixel_scale + i;
                            let scaled_y = y * pixel_scale + j;
                            let scaled_index = (scaled_y * scaled_x + scaled_x) * 4;

                            scaled_data[scaled_index] = r;
                            scaled_data[scaled_index + 1] = g;
                            scaled_data[scaled_index + 2] = b;
                            scaled_data[scaled_index + 3] = a;
                        }
                    }
                }
            }

            self.pixel_data = pixel_data;
        }

        pub fn update_texture(&mut self) {
            self.texture.update_texture(&self.pixel_data);
        }

        pub fn draw_texture(&self, d: &mut RaylibDrawHandle, tint: Color) {
            d.draw_texture_ex(&self.texture, self.position, 0.0, self.pixel_scale, tint);
        }
    }
}
