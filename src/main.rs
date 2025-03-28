use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, toggle_wireframe)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera
    commands.spawn((
        (
            Camera3d::default(),
            Transform::from_xyz(0., 1.5, 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ),
        PanOrbitCamera::default(),
    ));

    // Terrain
    let sub_divisions = 5800;

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(4000.0, 4000.0)
            .subdivisions(sub_divisions),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        let mut data_vec = import(positions.len().isqrt());
        println!("Data points import: {}", data_vec.len());
        println!("Data positions: {}", positions.len());

        let terrain_height = largest_f32(&data_vec).clone();
        let scale = 100.0;
        for pos in positions.iter_mut().enumerate() {
            if let Some(data) = data_vec.pop() {
                    pos.1[1] = data / scale;
            }
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| get_height_color(*g * scale, ColorSpectrum::ImhofModified))
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        terrain.compute_normals();
    }

    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(StandardMaterial {
            ..Default::default()
        })),
        Terrain,
    ));
}

#[derive(Component)]
struct Terrain;
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn import(res: usize) -> Vec<f32> {
    // let data = include_str!("./assets/srtm_38_02.asc").to_string(); // Elbe, Hamburg
    let data = include_str!("./assets/srtm_64_05.asc").to_string(); // Fuji
    let mut data_vec: Vec<f32> = vec![];
    for line in data.lines().enumerate() {
        for date in line.1.split(' ').enumerate() {
            let mut height = date.1.trim().parse::<f32>().unwrap_or(0.0);
            if height == -9999.0 {
                height = 0.0;
            }
            if date.0 >= res {
                break;
            }
            data_vec.push(height);
        }
    }
    data_vec
}

fn largest_f32(list: &Vec<f32>) -> &f32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

enum ColorSpectrum {
    Imhof,
    ImhofModified,
}

fn get_height_color(height_m: f32, colors: ColorSpectrum) -> [f32; 4] {
    match colors {
        ColorSpectrum::Imhof => {
            // Imhof colors with modifications
            // Unit of measure is meter
            // Height 0 is always blue for convenience
            match height_m {
                height if height < 0.0 => {
                    Color::srgb(0.05, 0.125, 0.075).to_linear().to_f32_array()
                }
                height_m if height_m >= 0.0 && height_m < 100.0 => {
                    Color::srgb(0.654, 0.772, 0.541).to_linear().to_f32_array()
                }
                height_m if height_m >= 100.0 && height_m < 200.0 => {
                    Color::srgb(0.753, 0.863, 0.634).to_linear().to_f32_array()
                }
                height_m if height_m >= 200.0 && height_m < 500.0 => {
                    Color::srgb(0.882, 0.879, 0.624).to_linear().to_f32_array()
                }
                height_m if height_m >= 500.0 && height_m < 1000.0 => {
                    Color::srgb(0.855, 0.783, 0.592).to_linear().to_f32_array()
                }
                height_m if height_m >= 1000.0 && height_m < 2000.0 => {
                    Color::srgb(0.829, 0.743, 0.576).to_linear().to_f32_array()
                }
                height_m if height_m >= 2000.0 && height_m < 4000.0 => {
                    Color::srgb(0.754, 0.643, 0.523).to_linear().to_f32_array()
                }
                height_m if height_m >= 4000.0 && height_m < 9000.0 => {
                    Color::srgb(0.677, 0.546, 0.473).to_linear().to_f32_array()
                }
                _ => Color::srgb(1.0, 1.0, 1.0).to_linear().to_f32_array(),
            }
        },
        ColorSpectrum::ImhofModified => {
            // Imhof colors with modifications
            // Unit of measure is meter
            // Height 0 is always blue for convenience
            match height_m {
                height if height < 0.0 => {
                    Color::srgb(0.05, 0.125, 0.075).to_linear().to_f32_array()
                }
                height if height == 0.0 => {
                    Color::srgb(0.025, 0.075, 0.275).to_linear().to_f32_array()
                }
                height if height > 0.0 && height < 100.0 => {
                    Color::srgb(0.654, 0.772, 0.541).to_linear().to_f32_array()
                }
                height if height_m >= 100.0 && height_m < 200.0 => {
                    Color::srgb(0.753, 0.863, 0.634).to_linear().to_f32_array()
                }
                height_m if height_m >= 200.0 && height_m < 500.0 => {
                    Color::srgb(0.882, 0.879, 0.624).to_linear().to_f32_array()
                }
                height_m if height_m >= 500.0 && height_m < 1000.0 => {
                    Color::srgb(0.855, 0.783, 0.592).to_linear().to_f32_array()
                }
                height_m if height_m >= 1000.0 && height_m < 2000.0 => {
                    Color::srgb(0.829, 0.743, 0.576).to_linear().to_f32_array()
                }
                height_m if height_m >= 2000.0 && height_m < 4000.0 => {
                    Color::srgb(0.754, 0.643, 0.523).to_linear().to_f32_array()
                }
                height_m if height_m >= 4000.0 && height_m < 9000.0 => {
                    Color::srgb(0.677, 0.546, 0.473).to_linear().to_f32_array()
                }
                _ => Color::srgb(1.0, 1.0, 1.0).to_linear().to_f32_array(),
            }
        }
    }
}