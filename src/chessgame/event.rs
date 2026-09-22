use bevy::prelude::*;
#[derive(Event)]
pub struct BoardUpdated{}
#[derive(Event)]
pub struct MakeBoard{}

#[derive(Event)]
pub struct SquareSelect{
    pub square:(usize,usize)
}
#[derive(Event)]
pub struct SquareDeselect;