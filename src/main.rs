use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::primitives::Cuboid;
use bevy::math::*;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy_obj::ObjPlugin;
use avian3d::prelude::*;
use bevy::color::palettes::basic::*;

#[derive(Resource, Default)]
struct Obj {
    handles: Vec<Handle<Mesh>>,
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#twitching".into()),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ObjPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, show_instructions)
        .add_systems(Startup, load_obj)
        .add_systems(Update, spawn_obj)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 10.0,
        })
        .insert_resource(Obj::default())
        .run();
}


fn show_instructions(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Press [Space] to spawn objects, Right click to move camera",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}

fn load_obj(asset_server: Res<AssetServer>, mut obj: ResMut<Obj>) {
    for i in 0..73 {
        let file = format!("obj/breakage_{i}.obj");
        let mesh_handle: Handle<Mesh> = asset_server.load(file);
        obj.handles.push(mesh_handle);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_x(-45f32.to_radians())
                * Quat::from_rotation_y(45f32.to_radians()),
            ..default()
        },
        ..default()
    },));

    let target = Vec3::ZERO;
    let eye = vec3(100.0, 200.0, 400.0);
    let up = Vec3::Y;

   let mut cam_transform = Transform::from_translation(eye);
   cam_transform.look_at(target, up);

    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 0,
                hdr: false,
                is_active: true,
                ..default()
            },
            transform: cam_transform,
            ..default()
        },
        TemporalAntiAliasBundle::default(),
    ));

    let ground_mesh = Mesh::from(Cuboid::new(10_000.0, 1.0, 10_000.0));
    let side_x_mesh = Mesh::from(Cuboid::new(1.0, 40.0, 100.0));
    let side_z_mesh = Mesh::from(Cuboid::new(81.0, 40.0, 1.0));

    // Ground
    commands.spawn((
        RigidBody::Static,
        Collider::convex_hull_from_mesh(&ground_mesh).unwrap(),
        PbrBundle {
            mesh: meshes.add(ground_mesh),
            material: materials.add(Color::rgba(0.7, 0.7, 0.8, 0.5)),
            transform: Transform {
                translation: vec3(0.0, -2.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));

    let side_x_collider =
        Collider::convex_hull_from_mesh(&side_x_mesh).unwrap();
    let side_x_handle = meshes.add(side_x_mesh);
    let westrac = materials.add(Color::rgb(1.0, 0.8, 0.067));
    // Side X
    commands.spawn((
        RigidBody::Static,
        side_x_collider.clone(),
        PbrBundle {
            mesh: side_x_handle.clone(),
            material: westrac.clone(),
            transform: Transform {
                translation: vec3(-40.0, 18.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        RigidBody::Static,
        side_x_collider.clone(),
        PbrBundle {
            mesh: side_x_handle.clone(),
            material: westrac.clone(),
            transform: Transform {
                translation: vec3(40.0, 18.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));

    // Side Z
    let side_z_collider =
        Collider::convex_hull_from_mesh(&side_z_mesh).unwrap();
    let side_z_handle = meshes.add(side_z_mesh);
    commands.spawn((
        RigidBody::Static,
        side_z_collider.clone(),
        PbrBundle {
            mesh: side_z_handle.clone(),
            material: westrac.clone(),
            transform: Transform {
                translation: vec3(0.0, 18.0, -50.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_obj(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    obj: Res<Obj>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {

        let colors = vec![
            Color::rgb(1.0, 0.0, 0.0), //red
            Color::rgb(0.0, 1.0, 0.0), //blue
            Color::rgb(1.0, 0.08, 0.58), //pink
            Color::rgb(1.0, 0.65, 0.0), //orange
            Color::rgb(0.5, 0.0, 0.5), // purple
            Color::rgb(0.0, 1.0, 1.0), //cyan
        ];

        for (i, handle) in obj.handles.iter().enumerate() {
            let mesh = meshes.get(handle).unwrap();
            let aabb = mesh.compute_aabb().unwrap();

            if let Some(collider) = Collider::convex_hull_from_mesh(&mesh) {
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: handle.clone(),
                        material: materials.add(colors[i % colors.len()]),
                        transform: Transform {
                            translation: vec3(0.0, 40.0, 0.0),
                            ..default()
                        },
                        ..default()
                    },
                    RigidBody::Dynamic,
                    collider,
                    GravityScale(20.0),
                ));
            }
        }
    }
}
