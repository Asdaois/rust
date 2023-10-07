use crate::math::vector_2::Vector2;

pub struct Transform {
    /// Center position of actor
    pub position: Vector2,
    /// Uniforms scale of actor (1. for 100%)
    pub scale: f64,
    /// Rotation angle (in radians)
    pub rotation: f64,
}
