use bevy::prelude::*;
use super::movevalidator::Piece;
#[derive(Event)]
pub struct BoardUpdated{}

#[derive(Event)]
pub struct SquareSelect{
    pub square:(usize,usize)
}
#[derive(Event)]
pub struct SquareDeselect;

#[derive(Event)]
pub struct PromotionClick{pub piece:Piece}

#[derive(Event)]
pub struct CallForPromotion{pub square:(usize,usize)}