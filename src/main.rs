use bevy::{
    asset::RenderAssetUsages,
    color::color_difference::EuclideanDistance,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use rand::Rng;

mod cpu;
mod display;
mod memory;

const IMAGE_WIDTH: u32 = 160 * 2;
const IMAGE_HEIGHT: u32 = 144 * 2;

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::<Fixed>::from_hz(1024.0));
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, my_img);
    app.add_systems(FixedUpdate, draw);
    app.run();
}

#[derive(Resource)]
struct MyDisplay(Handle<Image>);

fn my_img(mut images: ResMut<Assets<Image>>, mut cmd: Commands) {
    cmd.spawn(Camera2d);

    let mut img = Image::new_fill(
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::WHITE.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    /*
        img.set_color_at(23, 23, Color::srgb(1.0, 0.0, 1.0))
            .expect("Error Writing Color");

        img.set_color_at(10, 10, Color::srgb(0.3, 0.2, 0.5))
            .expect("Error Writing Color");

        let bytes = img.pixel_bytes(UVec3::new(10, 10, 0)).unwrap();

        let color = img.get_color_at(10, 10);
    */

    let handle = images.add(img);
    cmd.spawn(Sprite::from_image(handle.clone()));

    cmd.insert_resource(MyDisplay(handle));
}

fn draw(
    my_handle: Res<MyDisplay>,
    mut images: ResMut<Assets<Image>>,
    mut i: Local<u32>,
    mut draw_color: Local<Color>,
) {
    let mut rng = rand::rng();

    if *i == 0 {
        // Generate a random color on first run.
        *draw_color = Color::linear_rgb(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
        );
    }

    // Get the image from Bevy's asset storage.
    let image = images.get_mut(&my_handle.0).expect("Image not found");

    // Compute the position of the pixel to draw.

    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
    let rot_speed = 0.0123;
    let period = 0.12345;

    let r = ops::sin(*i as f32 * period) * max_radius;
    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
    let (x, y) = (xy.x as u32, xy.y as u32);

    // Get the old color of that pixel.
    let old_color = image.get_color_at(x, y).unwrap();

    // If the old color is our current color, change our drawing color.
    let tolerance = 1.0 / 255.0;
    if old_color.distance(&draw_color) <= tolerance {
        *draw_color = Color::linear_rgb(
            rng.random::<f32>(),
            rng.random::<f32>(),
            rng.random::<f32>(),
        );
    }

    // Set the new color, but keep old alpha value from image.
    image
        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
        .unwrap();

    *i += 1;
    return ();
}
