use bevy::{prelude::*, window::*};
use egui::Ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (on_resize_system))
        .run();
}

#[derive(Component)]
struct Pic;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    
                    ..default()
                },
                image: UiImage::new(asset_server.load("SPUELBECKEN.png")),
                ..default()
            },
            Pic,
        ));
    });
}

fn on_resize_system(
    mut q: Query<&mut Style, With<Pic>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut pic = q.single_mut();
    // for e in resize_reader.read() {
    //     // When resolution is being changed
    //     println!("GELO");
    // }
}