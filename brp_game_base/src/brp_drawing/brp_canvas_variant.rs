use bevy::prelude::Resource;

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub enum BrpCanvasVariant {
    #[default]
    Landscape,
    Portrait,
}

#[derive(Resource, Default, Debug)]
pub struct BrpCurrentCanvasVariant(pub BrpCanvasVariant);
