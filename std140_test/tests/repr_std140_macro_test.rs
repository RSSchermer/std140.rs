#[std140::repr_std140]
struct PointLight {
    position: std140::vec3,
    // Test repeating the same type twice
    intensity: std140::float,
    falloff: std140::float,
}

#[std140::repr_std140]
struct Uniforms {
    transform: std140::mat4x4,
    ambient_light_color: std140::vec3,
    lights: std140::array<PointLight, 2>,
}

fn main() {}
