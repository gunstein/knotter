use bevy::prelude::*;

#[derive(Resource)]
pub struct HandleForBallMesh {
    pub handle: Handle<Mesh>,
}

#[derive(Resource)]
pub struct HandleForBallMaterial {
    pub handle: Handle<StandardMaterial>,
}
