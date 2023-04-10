use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MenuMode {
    pub is_first_time: bool,
}
