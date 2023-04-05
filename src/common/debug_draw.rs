use std::{cmp::Ordering, mem::take};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub struct DebugDrawSystem;

pub struct DebugDrawPlugin;

impl Plugin for DebugDrawPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugDrawSettings>()
            .init_resource::<DebugDraw>()
            .configure_set(
                DebugDrawSystem
                    .after(CoreSet::Update)
                    .before(CoreSet::UpdateFlush),
            )
            .add_system(debug_renderer.in_base_set(DebugDrawSystem));
    }
}

#[derive(Resource, Default)]
pub struct DebugDrawSettings {
    pub draw_hit_boxes: bool,
    pub draw_hurt_boxes: bool,
    pub draw_feelers: bool,
}

#[derive(Resource, Default)]
pub struct DebugDraw {
    meshes: Vec<DebugDrawMesh>,
}

impl DebugDraw {
    pub fn draw<T: DebugDrawDrawable>(&mut self, mesh: T) {
        self.meshes.push(mesh.to_mesh());
    }
}

pub trait DebugDrawDrawable {
    fn to_mesh(&self) -> DebugDrawMesh;
}

#[derive(Default, Debug, Clone)]
pub struct DebugDrawMesh {
    pub vertices: Vec<DebugDrawVertex>,
    pub indices: Vec<u32>,
    pub depth: f32,
}

impl DebugDrawDrawable for DebugDrawMesh {
    fn to_mesh(&self) -> DebugDrawMesh {
        self.clone()
    }
}

impl DebugDrawMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge_with(&mut self, other: &DebugDrawMesh) {
        let base_index = self.vertices.len() as u32;
        self.vertices.extend(other.vertices.iter());
        self.indices.reserve(other.indices.len());
        for index in other.indices.iter() {
            self.indices.push(base_index + *index);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DebugDrawVertex {
    pub position: Vec2,
    pub color: Color,
}

#[derive(Component)]
struct DebugDrawObject;

fn debug_renderer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut debug_render: ResMut<DebugDraw>,
    debug_query: Query<Entity, With<DebugDrawObject>>,
) {
    for debug_entity in debug_query.iter() {
        commands.entity(debug_entity).despawn();
    }

    let mut merged_mesh = DebugDrawMesh::new();
    debug_render
        .meshes
        .sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(Ordering::Equal));
    for debug_render_mesh in take(&mut debug_render.meshes).into_iter() {
        merged_mesh.merge_with(&debug_render_mesh);
    }

    let DebugDrawMesh {
        vertices, indices, ..
    } = merged_mesh;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut positions: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut uvs: Vec<[f32; 2]> = vec![];
    let mut colors: Vec<[f32; 4]> = vec![];

    for vertex in vertices.iter() {
        positions.push([vertex.position.x, vertex.position.y, 1.]);
        normals.push([0., 0., 0.]);
        uvs.push([0., 0.]);
        colors.push([
            vertex.color.r(),
            vertex.color.g(),
            vertex.color.b(),
            vertex.color.a(),
        ]);
    }

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    commands
        .spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh)),
            material: materials.add(ColorMaterial {
                color: Color::WHITE,
                texture: None,
            }),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        })
        .insert(DebugDrawObject);
}

#[derive(Clone, Copy, Debug)]
pub struct DebugRectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub depth: f32,
}

impl Default for DebugRectangle {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            rotation: 0.,
            color: Color::BLACK,
            depth: 0.,
        }
    }
}

impl DebugDrawDrawable for DebugRectangle {
    fn to_mesh(&self) -> DebugDrawMesh {
        let rotation = Vec2::from_angle(self.rotation);
        DebugDrawMesh {
            vertices: vec![
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(0.5, 0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(-0.5, 0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(0.5, -0.5)),
                    color: self.color,
                },
                DebugDrawVertex {
                    position: self.position + rotation.rotate(self.size * Vec2::new(-0.5, -0.5)),
                    color: self.color,
                },
            ],
            indices: vec![0, 1, 2, 3, 2, 1],
            depth: self.depth,
        }
    }
}
