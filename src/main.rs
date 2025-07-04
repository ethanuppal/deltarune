// Copyright (C) 2025 Ethan Uppal. All rights reserved.

use std::{env, time::Duration};

use avian2d::{math::Vector, prelude::*};
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

const DELTARUNE_WIDTH: usize = 640;
const DELTARUNE_HEIGHT: usize = 480;
const DELTARUNE_PIXEL_SIZE: usize = 2;
const RANDOM_SCALE_ON_MY_COMPUTER: usize = 2;

// https://github.com/adrien-bon/bevy_ecs_tiled/blob/ee458ad464e8ea7cea22c7923efb911945b5d710/examples/physics_avian_controller.rs#L96C1-L117C2
#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
struct MyCustomAvianPhysicsBackend(TiledPhysicsAvianBackend);

impl TiledPhysicsBackend for MyCustomAvianPhysicsBackend {
    fn spawn_colliders(
        &self,
        commands: &mut Commands,
        tiled_map: &TiledMap,
        filter: &TiledNameFilter,
        collider: &TiledCollider,
        anchor: &TilemapAnchor,
    ) -> Vec<TiledColliderSpawnInfos> {
        let colliders = self
            .0
            .spawn_colliders(commands, tiled_map, filter, collider, anchor);
        for c in &colliders {
            commands.entity(c.entity).insert(RigidBody::Static);
        }
        colliders
    }
}

fn main() {
    let mut app = App::new();

    // Add Bevy default plugins
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Window title or something".into(),
            resolution: WindowResolution::new(
                (RANDOM_SCALE_ON_MY_COMPUTER * DELTARUNE_WIDTH) as f32,
                (RANDOM_SCALE_ON_MY_COMPUTER * DELTARUNE_HEIGHT) as f32,
            ),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    // Add bevy_ecs_tiled plugin: note that bevy_ecs_tilemap::TilemapPlugin
    // will be automatically added as well if it's not already done
    .add_plugins(TiledMapPlugin::default())
    .add_plugins(TiledPhysicsPlugin::<MyCustomAvianPhysicsBackend>::default())
    // Load Avian main plugin
    .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
    .add_plugins((
        PhysicsDebugPlugin::default(),
        PhysicsDiagnosticsPlugin,
        PhysicsDiagnosticsUiPlugin,
    ))
    .insert_resource(Gravity(Vector::NEG_Y * 100.0))
    // Add our startup function to the schedule and run the app
    .add_systems(Startup, startup)
    .add_systems(Update, (keys_to_pause_time));

    if env::var("R").as_deref() == Ok("1") {
        app.add_plugins({
            let rec = revy::RecordingStreamBuilder::new("deltaruined")
                .save(format!("log-{}.rrd", chrono::offset::Local::now()))
                .unwrap();
            revy::RerunPlugin { rec }
        });
    }

    app.run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Physics>>,
) {
    time.pause();

    // Spawn a Bevy 2D camera
    commands.spawn(Camera2d);

    // // Load a map asset and retrieve the corresponding handle
    // let map_handle: Handle<TiledMap> = asset_server.load("test_level.tmx");
    //
    // // Spawn a new entity with this handle
    // commands.spawn((
    //     TiledMapHandle(map_handle),
    //     TilemapAnchor::Center,
    //     TiledPhysicsSettings::<TiledPhysicsAvianBackend>::default(),
    // ));
}

fn keys_to_pause_time(mut time: ResMut<Time<Physics>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }

    if keys.just_pressed(KeyCode::Enter) && time.is_paused() {
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            time.advance_by(Duration::from_millis(10));
        } else {
            time.advance_by(Duration::from_millis(100));
        }
    }

