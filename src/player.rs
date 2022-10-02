#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Player{
    pub position: Vector2d,
    pub speed: Vector2d,
    pub acceleration: f32,

}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Vector2d{
    pub x:f32,
    pub y:f32
}