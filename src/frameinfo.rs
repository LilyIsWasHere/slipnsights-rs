use peppi::model::{
    enums::action_state::{Common, State},
    frame::{Data, Frame, PortData},
};
use std::prelude::*;
use std::cmp;

pub trait PlayerFrame {
    fn is_damaged(&self) -> bool;
    fn is_grabbed(&self) -> bool;
    fn is_command_grabbed(&self) -> bool;
    fn is_grounded_actionable(&self) -> bool;

    fn percent(&self) -> f32;
    fn stocks(&self) -> u8;
    fn damage_taken(&self, prev_frame: &impl PlayerFrame) -> f32;
    fn action_state_id(&self) -> u16;
    fn did_lose_stock(&self, prev_frame: &impl PlayerFrame) -> bool;


}

