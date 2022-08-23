use peppi::model::{
    enums::{
        action_state::{Common, State},
        attack::Attack,
    },
    frame::{Data, Frame, PortData},
};
use std::cmp;
use std::prelude::*;

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

pub fn get_attack_string(attack: Attack) -> String {
    match attack {
        Attack::NON_STALING => "NON_STALING".to_string(),
        Attack::JAB_1 => "JAB_1".to_string(),
        Attack::JAB_2 => "JAB_2".to_string(),
        Attack::JAB_3 => "JAB_3".to_string(),
        Attack::RAPID_JABS => "RAPID_JABS".to_string(),
        Attack::DASH_ATTACK => "DASH_ATTACK".to_string(),
        Attack::SIDE_TILT => "SIDE_TILT".to_string(),
        Attack::UP_TILT => "UP_TILT".to_string(),
        Attack::DOWN_TILT => "DOWN_TILT".to_string(),
        Attack::SIDE_SMASH => "SIDE_SMASH".to_string(),
        Attack::UP_SMASH => "UP_SMASH".to_string(),
        Attack::DOWN_SMASH => "DOWN_SMASH".to_string(),
        Attack::NAIR => "NAIR".to_string(),
        Attack::FAIR => "FAIR".to_string(),
        Attack::BAIR => "BAIR".to_string(),
        Attack::UAIR => "UAIR".to_string(),
        Attack::DAIR => "DAIR".to_string(),
        Attack::NEUTRAL_SPECIAL => "NEUTRAL_SPECIAL".to_string(),
        Attack::SIDE_SPECIAL => "SIDE_SPECIAL".to_string(),
        Attack::UP_SPECIAL => "UP_SPECIAL".to_string(),
        Attack::DOWN_SPECIAL => "DOWN_SPECIAL".to_string(),
        Attack::KIRBY_HAT_MARIO_NEUTRAL_SPECIAL => "KIRBY_HAT_MARIO_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_FOX_NEUTRAL_SPECIAL => "KIRBY_HAT_FOX_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_CFALCON_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_CFALCON_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_DKNEUTRAL_SPECIAL => "KIRBY_HAT_DKNEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_BOWSER_NEUTRAL_SPECIAL => "KIRBY_HAT_BOWSER_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_LINK_NEUTRAL_SPECIAL => "KIRBY_HAT_LINK_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_SHEIK_NEUTRAL_SPECIAL => "KIRBY_HAT_SHEIK_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_NESS_NEUTRAL_SPECIAL => "KIRBY_HAT_NESS_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_PEACH_NEUTRAL_SPECIAL => "KIRBY_HAT_PEACH_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_ICE_CLIMBER_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_ICE_CLIMBER_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_PIKACHU_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_PIKACHU_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_SAMUS_NEUTRAL_SPECIAL => "KIRBY_HAT_SAMUS_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_YOSHI_NEUTRAL_SPECIAL => "KIRBY_HAT_YOSHI_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_JIGGLYPUFF_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_JIGGLYPUFF_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_MEWTWO_NEUTRAL_SPECIAL => "KIRBY_HAT_MEWTWO_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_LUIGI_NEUTRAL_SPECIAL => "KIRBY_HAT_LUIGI_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_MARTH_NEUTRAL_SPECIAL => "KIRBY_HAT_MARTH_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_ZELDA_NEUTRAL_SPECIAL => "KIRBY_HAT_ZELDA_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_YOUNG_LINK_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_YOUNG_LINK_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_DOC_NEUTRAL_SPECIAL => "KIRBY_HAT_DOC_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_FALCO_NEUTRAL_SPECIAL => "KIRBY_HAT_FALCO_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_PICHU_NEUTRAL_SPECIAL => "KIRBY_HAT_PICHU_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_GAME_AND_WATCH_NEUTRAL_SPECIAL => {
            "KIRBY_HAT_GAME_AND_WATCH_NEUTRAL_SPECIAL".to_string()
        }
        Attack::KIRBY_HAT_GANON_NEUTRAL_SPECIAL => "KIRBY_HAT_GANON_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_ROY_NEUTRAL_SPECIAL => "KIRBY_HAT_ROY_NEUTRAL_SPECIAL".to_string(),
        Attack::GET_UP_ATTACK_FROM_BACK => "GET_UP_ATTACK_FROM_BACK".to_string(),
        Attack::GET_UP_ATTACK_FROM_FRONT => "GET_UP_ATTACK_FROM_FRONT".to_string(),
        Attack::PUMMEL => "PUMMEL".to_string(),
        Attack::FORWARD_THROW => "FORWARD_THROW".to_string(),
        Attack::BACK_THROW => "BACK_THROW".to_string(),
        Attack::UP_THROW => "UP_THROW".to_string(),
        Attack::DOWN_THROW => "DOWN_THROW".to_string(),
        Attack::CARGO_FORWARD_THROW => "CARGO_FORWARD_THROW".to_string(),
        Attack::CARGO_BACK_THROW => "CARGO_BACK_THROW".to_string(),
        Attack::CARGO_UP_THROW => "CARGO_UP_THROW".to_string(),
        Attack::CARGO_DOWN_THROW => "CARGO_DOWN_THROW".to_string(),
        Attack::LEDGE_GET_UP_ATTACK_100 => "LEDGE_GET_UP_ATTACK_100".to_string(),
        Attack::LEDGE_GET_UP_ATTACK => "LEDGE_GET_UP_ATTACK_101".to_string(),
        Attack::BEAM_SWORD_JAB => "BEAM_SWORD_JAB".to_string(),
        Attack::BEAM_SWORD_TILT_SWING => "BEAM_SWORD_TILT_SWING".to_string(),
        Attack::BEAM_SWORD_SMASH_SWING => "BEAM_SWORD_SMASH_SWING".to_string(),
        Attack::BEAM_SWORD_DASH_SWING => "BEAM_SWORD_DASH_SWING".to_string(),
        Attack::HOME_RUN_BAT_JAB => "HOME_RUN_BAT_JAB".to_string(),
        Attack::HOME_RUN_BAT_TILT_SWING => "HOME_RUN_BAT_TILT_SWING".to_string(),
        Attack::HOME_RUN_BAT_SMASH_SWING => "HOME_RUN_BAT_SMASH_SWING".to_string(),
        Attack::HOME_RUN_BAT_DASH_SWING => "HOME_RUN_BAT_DASH_SWING".to_string(),
        Attack::PARASOL_JAB => "PARASOL_JAB".to_string(),
        Attack::PARASOL_TILT_SWING => "PARASOL_TILT_SWING".to_string(),
        Attack::PARASOL_SMASH_SWING => "PARASOL_SMASH_SWING".to_string(),
        Attack::PARASOL_DASH_SWING => "PARASOL_DASH_SWING".to_string(),
        Attack::FAN_JAB => "FAN_JAB".to_string(),
        Attack::FAN_TILT_SWING => "FAN_TILT_SWING".to_string(),
        Attack::FAN_SMASH_SWING => "FAN_SMASH_SWING".to_string(),
        Attack::FAN_DASH_SWING => "FAN_DASH_SWING".to_string(),
        Attack::STAR_ROD_JAB => "STAR_ROD_JAB".to_string(),
        Attack::STAR_ROD_TILT_SWING => "STAR_ROD_TILT_SWING".to_string(),
        Attack::STAR_ROD_SMASH_SWING => "STAR_ROD_SMASH_SWING".to_string(),
        Attack::STAR_ROD_DASH_SWING => "STAR_ROD_DASH_SWING".to_string(),
        Attack::LIPS_STICK_JAB => "LIPS_STICK_JAB".to_string(),
        Attack::LIPS_STICK_TILT_SWING => "LIPS_STICK_TILT_SWING".to_string(),
        Attack::LIPS_STICK_SMASH_SWING => "LIPS_STICK_SMASH_SWING".to_string(),
        Attack::LIPS_STICK_DASH_SWING => "LIPS_STICK_DASH_SWING".to_string(),
        Attack::OPEN_PARASOL => "OPEN_PARASOL".to_string(),
        Attack::RAY_GUN_SHOOT => "RAY_GUN_SHOOT".to_string(),
        Attack::FIRE_FLOWER_SHOOT => "FIRE_FLOWER_SHOOT".to_string(),
        Attack::SCREW_ATTACK => "SCREW_ATTACK".to_string(),
        Attack::SUPER_SCOPE_RAPID => "SUPER_SCOPE_RAPID".to_string(),
        Attack::SUPER_SCOPE_CHARGED => "SUPER_SCOPE_CHARGED".to_string(),
        Attack::HAMMER => "HAMMER".to_string(),
        _ => "UNNNAMED(".to_string() + &(attack.0 as u32).to_string() + ")",
    }
}
