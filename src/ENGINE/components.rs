use bevy_ecs::prelude::Component;
use crate::ENGINE::core::{Point2D, Rect, Vector2D};


#[derive( Component)]
pub struct SpriteComp{
    pub srs: Rect,
    pub pos: Point2D,
}


#[derive( Component)]
pub struct PlayerComp{
}



#[derive( Component)]
pub struct MobComp{
    pub rotate_dir: Vector2D,
}



#[derive(Component, Debug)]
pub struct AccelerationComp {
    pub acceleration: Vector2D,
}

#[derive(Component)]
pub struct MovementComp {
    pub position: Vector2D,
}