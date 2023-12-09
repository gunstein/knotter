use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Wrapper struct for Color that implements Eq and Hash
#[derive(Clone, Debug)]
pub struct ColorKey(pub Color);

impl PartialEq for ColorKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Resource)]
pub struct ColorMaterialMap {
    pub map: HashMap<ColorKey, Handle<StandardMaterial>>,
}

impl Eq for ColorKey {}

impl Hash for ColorKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.r().to_bits().hash(state);
        self.0.g().to_bits().hash(state);
        self.0.b().to_bits().hash(state);
        self.0.a().to_bits().hash(state);
    }
}

impl ColorMaterialMap {
    pub fn new() -> Self {
        ColorMaterialMap {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, color: Color, material_handle: Handle<StandardMaterial>) {
        self.map.insert(ColorKey(color), material_handle);
    }

    pub fn get(&self, color: &Color) -> Option<&Handle<StandardMaterial>> {
        self.map.get(&ColorKey(*color))
    }
}