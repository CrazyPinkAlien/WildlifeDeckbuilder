use bevy::prelude::{Component, Bundle, MaterialMeshBundle};
use crate::materials::{CustomMaterial};

// Define states containing card positions
#[derive(Component)]
struct InHand;

#[derive(Component)]
struct InDraw;

#[derive(Component)]
struct InDiscard;

#[derive(Component)]
struct InShop;

// Define state of card being selected by player
#[derive(Component)]
struct IsSelected;

// Permanent components of the card object
#[derive(Component)]
struct Name {
    pub name: String,
}

#[derive(Bundle)]
struct CardBundle {
    pub name: Name,
    #[bundle]
    pub mesh_bundle: MaterialMeshBundle<CustomMaterial>,
}
