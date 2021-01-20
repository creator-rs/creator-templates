use bevy::prelude::*;

#[creator::creator_main]
pub fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(audio.system())
        .add_startup_system(icon.system())
        .run();
}

fn icon(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("icon.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}
