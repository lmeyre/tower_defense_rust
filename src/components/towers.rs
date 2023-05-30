use bevy::prelude::Component;
use hexx::Hex;

#[derive(Component)]
pub struct Tower {
    pub hex: Hex,
}
