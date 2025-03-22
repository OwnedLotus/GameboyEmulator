use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

mod cpu;
mod display;
mod memory;

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
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &Srgba::WHITE.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    img.set_color_at(23, 23, Color::srgb(1.0, 0.0, 1.0))
        .expect("Error Writing Color");

    img.set_color_at(10, 10, Color::srgb(0.3, 0.2, 0.5))
        .expect("Error Writing Color");

    let bytes = img.pixel_bytes(UVec3::new(10, 10, 0)).unwrap();

    let color = img.get_color_at(10, 10);

    cmd.spawn(Sprite {
        image: images.add(img),
        ..Default::default()
    });
}

fn draw(
    my_handle: Res<MyDisplay>,
    mut images: ResMut<Assets<Image>>,
    mut i: Local<u32>,
    mut draw_color: Local<Color>
) {
    let mut rng = 
}
