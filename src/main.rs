// http://paulbourke.net/geometry/polygonise/
use bevy_marching_cubes::*;
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

#[derive(Debug, Copy, Clone)]
struct Point {
    pos: Vec3,
    value: f64,
}

#[derive(Debug, Copy, Clone)]
struct Cube([Point; 8]);

#[derive(Debug, Copy, Clone)]
struct Triangle([Vec3; 3]);

#[derive(Debug)]
struct CubeGrid(Vec<Cube>);

impl Point {
    pub fn new(pos: Vec3, value: f64) -> Self {
        Point { pos, value }
    }
}

impl Cube {
    pub fn new(vals: [Point; 8]) -> Self {
        Cube(vals)
    }

    fn process_cube(self, id: Vec3) -> Vec<Triangle> {
        let corner_coords: [Vec3; 8] = [
            id + Vec3::new(0., 0., 0.),
            id + Vec3::new(1., 0., 0.),
            id + Vec3::new(1., 0., 1.),
            id + Vec3::new(0., 0., 1.),
            id + Vec3::new(0., 1., 0.),
            id + Vec3::new(1., 1., 0.),
            id + Vec3::new(1., 1., 1.),
            id + Vec3::new(0., 1., 1.),
        ];

        // 8 bit value, one bit for each corner
        // 1 is upper, 0 is lower
        let mut cube_config = 0;
        for (x, y) in corner_coords.iter().enumerate() {
            if self.get_point_value(*y) < 0. {
                // |= is a bitwise OR assign
                cube_config |= 1 << x;
            }
        }

        let edge_indices = TRIANGULATION_TABLE[cube_config];

        let mut triangles: Vec<Triangle> = vec![];

        // a cube has 6 sides where a point can be drawn
        for x in 0..6 {
            if edge_indices[x] == -1 {
                break;
            }

            let (a0, a1) = (
                CORNER_INDEX_A_FROM_EDGE[edge_indices[x] as usize],
                CORNER_INDEX_B_FROM_EDGE[edge_indices[x] as usize],
            );
            let (b0, b1) = (
                CORNER_INDEX_A_FROM_EDGE[edge_indices[x + 1] as usize],
                CORNER_INDEX_B_FROM_EDGE[edge_indices[x + 1] as usize],
            );
            let (c0, c1) = (
                CORNER_INDEX_A_FROM_EDGE[edge_indices[x + 2] as usize],
                CORNER_INDEX_B_FROM_EDGE[edge_indices[x + 2] as usize],
            );

            let (a, b, c) = (
                (corner_coords[a0] + corner_coords[a1]) * 0.5,
                (corner_coords[b0] + corner_coords[b1]) * 0.5,
                (corner_coords[c0] + corner_coords[c1]) * 0.5,
            );
            triangles.push(Triangle([a, b, c]));
        }
        triangles
    }

    fn get_point_value(self, pos: Vec3) -> f64 {
        let mut res: f64 = -1.;
        for x in self.0 {
            if x.pos == pos {
                res = x.value
            }
        }
        res
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(draw_cube.system())
        .add_plugin(PlayerPlugin)
        .run();
}

fn draw_cube(mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = Cube::new([
        Point::new(Vec3::new(0., 0., 0.), 0.2),
        Point::new(Vec3::new(1., 0., 0.), -0.6),
        Point::new(Vec3::new(1., 0., 1.), 2.4),
        Point::new(Vec3::new(0., 0., 1.), -0.7),
        Point::new(Vec3::new(0., 1., 0.), 0.3),
        Point::new(Vec3::new(1., 1., 0.), -2.5),
        Point::new(Vec3::new(1., 1., 1.), 3.1),
        Point::new(Vec3::new(0., 1., 1.), 0.9),
    ]);
    let processed = cube.process_cube(Vec3::splat(0.));
    for x in processed {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(createtri(x)),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_translation(Vec3::splat(-0.5)),
            ..Default::default()
        });
        println!("{:?}", x)
    }
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(0.8, 0.7, 0.6, 0.2).into()),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn createtri(vertices: Triangle) -> Mesh {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for position in vertices.0 {
        positions.push([position.x, position.y, position.z]);
        normals.push([0.0, 0.0, 0.0]);
        uvs.push([1.0, 1.0]);
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}