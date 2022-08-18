use peppi::model::enums::action_state::{State, Common};
use peppi::model::enums::attack::Attack;
use peppi::model::frame::{Data, Frame, PortData};
use frameinfo::PlayerFrame;
use core::fmt::{self, Display};
use std::prelude::*;
use std::{fs, io};

pub mod frameinfo;

fn main() {
    let mut buf = io::BufReader::new(fs::File::open("replays/game.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();
    let frames_enum = game.frames;

    for player in game.metadata.players.unwrap().iter() {
        let port = match player.port {
            peppi::model::primitives::Port::P1 => 1,
            peppi::model::primitives::Port::P2 => 2,
            peppi::model::primitives::Port::P3 => 3,
            peppi::model::primitives::Port::P4 => 4,
        };
    }

    match frames_enum {
        peppi::model::game::Frames::P1(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P2(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P3(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P4(f) => handle_frames_enum(f),
    }
}

fn handle_frames_enum<const N: usize>(frames: Vec<Frame<N>>) {
    let mut conversions: Vec<Conversion> = Vec::new();
    let mut active_conversions: [Option<Conversion>; N] = [(); N].map(|_| None);
    for (i, frame) in frames.iter().enumerate() {
        let prev_frame = if i > 0 {&frames[i - 1]} else {frame};
        for port in 0..N {

            match &mut active_conversions[port] {
                Some(active_conversion) => {
                    let player_frame = &frame.ports[port];

                    active_conversion.frames_since_last_hit += 1;
                    active_conversion.has_been_grounded_actionable = active_conversion.has_been_grounded_actionable || player_frame.is_grounded_actionable();

                    let mut conversion_complete = false;

                    if active_conversion.frames_since_last_hit > 45 && active_conversion.has_been_grounded_actionable {
                        conversion_complete = true;
                    }
                    else if player_frame.did_lose_stock(&prev_frame.ports[port]) {
                        conversion_complete = true;
                    }
                    else {
                        let is_damaged = player_frame.is_damaged();
                        let is_grabbed = player_frame.is_grabbed();
                        let is_command_grabbed = player_frame.is_command_grabbed();    
                        let damage_taken = player_frame.damage_taken(&prev_frame.ports[port]);

                        if (is_damaged || is_grabbed || is_command_grabbed) && damage_taken > 0.0 {
                            active_conversion.frames_since_last_hit = 0;
                            active_conversion.has_been_grounded_actionable = false;

                            let last_hit_by = player_frame.leader.post.last_hit_by;
                            let adv_index: Option<usize> = if let Some(adv_port) = last_hit_by {
                                match adv_port {
                                    peppi::model::primitives::Port::P1 => Some(0),
                                    peppi::model::primitives::Port::P2 => Some(1),
                                    peppi::model::primitives::Port::P3 => Some(2),
                                    peppi::model::primitives::Port::P4 => Some(3),
                                }
                            } else {
                                None
                            };


                            let landed_attack: Option<Attack> = if let Some(adv_i) = adv_index {
                                frame.ports[adv_i].leader.post.last_attack_landed
                            } else {
                                None
                            };
                            let adv_attack: PlayerAttack = PlayerAttack {
                                player_index: adv_index,
                                attack: landed_attack,
                                frame: i
                            };
    
                            active_conversion.add_attack(adv_attack);

                            if let Some(adv_i) = adv_index {
                                if let None = active_conversion.adv_index {
                                    active_conversion.adv_index = Some(adv_i);
                                }
                            }
                        }

                    }

                    if conversion_complete {
                        active_conversion.end_frame = Some(i);
                        active_conversion.end_percent = Some(frame.ports[port].percent());

                        conversions.push(active_conversion.clone());
                        active_conversions[port] = None;
                    }
                }
                None => {
                    let player_frame = &frame.ports[port];
                    let is_damaged = player_frame.is_damaged();
                    let is_grabbed = player_frame.is_grabbed();
                    let is_command_grabbed = player_frame.is_command_grabbed();

                    if is_damaged || is_grabbed || is_command_grabbed {
                        let last_hit_by = player_frame.leader.post.last_hit_by;
                        let adv_index: Option<usize> = if let Some(adv_port) = last_hit_by {
                            match adv_port {
                                peppi::model::primitives::Port::P1 => Some(0),
                                peppi::model::primitives::Port::P2 => Some(1),
                                peppi::model::primitives::Port::P3 => Some(2),
                                peppi::model::primitives::Port::P4 => Some(3),
                            }
                        } else {
                            None
                        };

                        let disadv_index = port;
                        let start_frame = i;
                        let start_percent = prev_frame.ports[port].percent();

                        let mut conversion = Conversion::new(adv_index, disadv_index, start_frame, start_percent); 

                        let landed_attack: Option<Attack> = if let Some(adv_i) = adv_index{
                            frame.ports[adv_i].leader.post.last_attack_landed
                        } else {
                            None
                        };
                        let adv_attack: PlayerAttack = PlayerAttack {
                            player_index: adv_index,
                            attack: landed_attack,
                            frame: i
                        };

                        conversion.add_attack(adv_attack);
                        active_conversions[port] = Some(conversion);
                    }
                }
            }
        }
    }

    println!("Total conversions: {}", conversions.len());
    for conversion in conversions {
        println!("{}", conversion);
    }
}

#[derive(Clone, Debug)]
struct Conversion {
    adv_index: Option<usize>,
    disadv_index: usize,

    has_been_grounded_actionable: bool,
    frames_since_last_hit: usize,

    start_frame: usize,
    end_frame: Option<usize>,

    start_percent: f32,
    end_percent: Option<f32>,

    attacks: Vec<PlayerAttack>,
    did_kill: bool,
    opening_type: Option<String>,
}

impl Conversion {
    fn new(adv_index: Option<usize>, disadv_index: usize, start_frame: usize, start_percent: f32) -> Conversion {
        Conversion {
            adv_index,
            disadv_index,
            has_been_grounded_actionable: false,
            frames_since_last_hit: 0,
            start_frame,
            end_frame: None,
            start_percent,
            end_percent: None,
            attacks: Vec::new(),
            did_kill: false,
            opening_type: None,
        }
    }

    fn add_attack(&mut self, attack: PlayerAttack) {
        self.attacks.push(attack);
        self.frames_since_last_hit = 0;
        self.has_been_grounded_actionable = false;
    }

}
impl Display for Conversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let adv_player = match self.adv_index {
            Some(i) => format!("{}", i+1),
            None => "Unknown".to_string(),
        };

        let attacks_vec = self.attacks.iter().map(|a| {
            format!("{}", a)
        }).collect::<Vec<String>>().join(", ");

        write!(f, "Conversion: Player {} hit Player {}!\n   They dealt {:.2} damage in {} hits.\n   Attacks: {}", adv_player, self.disadv_index+1, self.end_percent.unwrap() - self.start_percent, self.attacks.len(), attacks_vec)
    }
}

#[derive(Clone, Debug)]
struct PlayerAttack {
    player_index: Option<usize>,
    attack: Option<Attack>,
    frame: usize,
}

impl Display for PlayerAttack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let player = match self.player_index {
            Some(i) => format!("{}", i),
            None => "Unknown".to_string(),
        };
        let attack = match self.attack {
            Some(a) => {
                get_attack_string(a)
            }
            None => "Unknown".to_string(),
        };
        write!(f, "{} on frame {}", attack, self.frame)
    }
}




impl PlayerFrame for PortData {
    fn is_damaged(&self) -> bool {
        // The range of action state IDs that correspond with damage taken.
        let damaged_range_start = Common::DAMAGE_HI_1.0;
        let damaged_range_end = Common::DAMAGE_FLY_ROLL.0;

        let damaged_fall = Common::DAMAGE_FALL.0;

        let state = self.leader.post.state;

        if let State::Common(c) = state {
            let state_id = c.0;
            return state_id >= damaged_range_start && state_id <= damaged_range_end
                || state_id == damaged_fall;
        } else {
            return false;
        }
    }

    fn is_grabbed(&self) -> bool {
        // The range of action state IDs that correspond with being grab.
        let grab_range_start = Common::CAPTURE_PULLED_HI.0;
        let grab_range_end = Common::CAPTURE_FOOT.0;

        let state = self.leader.post.state;

        if let State::Common(c) = state {
            let state_id = c.0;
            return state_id >= grab_range_start && state_id <= grab_range_end;
        } else {
            return false;
        }
    }

    fn is_command_grabbed(&self) -> bool {
        // The ranges of action state IDs that correspond with being command grab (UGH why are there multiple)
        let cmd_grab_range_start_1 = Common::SHOULDERED_WAIT.0;
        let cmd_grab_range_end_1 = Common::THROWN_MEWTWO_AIR.0;

        let cmd_grab_range_start_2 = Common::CAPTURE_MASTER_HAND.0;
        let cmd_grab_range_end_2 = Common::CAPTURE_WAIT_CRAZY_HAND.0;

        let cmd_grab_barrel_wait = Common::BARREL_WAIT.0;

        let state = self.leader.post.state;

        if let State::Common(c) = state {
            let state_id = c.0;
            return (state_id >= cmd_grab_range_start_1 && state_id <= cmd_grab_range_end_1)
                || (state_id >= cmd_grab_range_start_2 && state_id <= cmd_grab_range_end_2)
                || state_id == cmd_grab_barrel_wait;
        } else {
            return false;
        }
    }

    fn is_grounded_actionable(&self) -> bool {
        // The range of action state IDs that correspond with being grounded and actionable.
        let ground_control_start = Common::WAIT.0;
        let ground_control_end = Common::KNEE_BEND.0;

        let squat_start = Common::SQUAT.0;
        let squat_end = Common::SQUAT_RV.0;

        let ground_attack_start = Common::ATTACK_11.0;
        let ground_attack_end = Common::ATTACK_LW_4.0;

        let grab = Common::CATCH.0;

        let state = self.leader.post.state;

        if let State::Common(c) = state {
            let state_id = c.0;
            return state_id >= ground_control_start && state_id <= ground_control_end
                || state_id >= squat_start && state_id <= squat_end
                || state_id >= ground_attack_start && state_id <= ground_attack_end
                || state_id == grab;
        } else {
            return false;
        }
    }

    fn percent(&self) -> f32 {
        self.leader.post.damage
    }

    fn stocks(&self) -> u8 {
        self.leader.post.stocks
    }

    fn damage_taken(&self, prev_frame: &impl PlayerFrame) -> f32 {
        let frame_damage = self.leader.post.damage;
        let prev_frame_damage = prev_frame.percent();

        (frame_damage - prev_frame_damage).max(0.0)
    }

    fn did_lose_stock(&self, prev_frame: &impl PlayerFrame) -> bool {
        let frame_stocks = self.stocks();
        let prev_frame_stocks = prev_frame.stocks();

        frame_stocks < prev_frame_stocks
    }

    fn action_state_id(&self) -> u16 {
        match self.leader.post.state {
            State::Unknown(state) => state,
            State::Common(state) => state.0,
            State::Bowser(state) => state.0,
            State::CaptainFalcon(state) => state.0,
            State::DonkeyKong(state) => state.0,
            State::DrMario(state) => state.0,
            State::Falco(state) => state.0,
            State::Fox(state) => state.0,
            State::GameAndWatch(state) => state.0,
            State::Ganondorf(state) => state.0,
            State::Jigglypuff(state) => state.0,
            State::Kirby(state) => state.0,
            State::Link(state) => state.0,
            State::Luigi(state) => state.0,
            State::Mario(state) => state.0,
            State::Marth(state) => state.0,
            State::Mewtwo(state) => state.0,
            State::Nana(state) => state.0,
            State::Ness(state) => state.0,
            State::Peach(state) => state.0,
            State::Pichu(state) => state.0,
            State::Pikachu(state) => state.0,
            State::Popo(state) => state.0,
            State::Roy(state) => state.0,
            State::Samus(state) => state.0,
            State::Sheik(state) => state.0,
            State::Yoshi(state) => state.0,
            State::YoungLink(state) => state.0,
            State::Zelda(state) => state.0,
        }
    }
}

fn get_attack_string(attack: Attack) -> String {
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
        Attack::KIRBY_HAT_CFALCON_NEUTRAL_SPECIAL => "KIRBY_HAT_CFALCON_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_DKNEUTRAL_SPECIAL => "KIRBY_HAT_DKNEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_BOWSER_NEUTRAL_SPECIAL => "KIRBY_HAT_BOWSER_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_LINK_NEUTRAL_SPECIAL => "KIRBY_HAT_LINK_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_SHEIK_NEUTRAL_SPECIAL => "KIRBY_HAT_SHEIK_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_NESS_NEUTRAL_SPECIAL => "KIRBY_HAT_NESS_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_PEACH_NEUTRAL_SPECIAL => "KIRBY_HAT_PEACH_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_ICE_CLIMBER_NEUTRAL_SPECIAL => "KIRBY_HAT_ICE_CLIMBER_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_PIKACHU_NEUTRAL_SPECIAL => "KIRBY_HAT_PIKACHU_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_SAMUS_NEUTRAL_SPECIAL => "KIRBY_HAT_SAMUS_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_YOSHI_NEUTRAL_SPECIAL => "KIRBY_HAT_YOSHI_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_JIGGLYPUFF_NEUTRAL_SPECIAL => "KIRBY_HAT_JIGGLYPUFF_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_MEWTWO_NEUTRAL_SPECIAL => "KIRBY_HAT_MEWTWO_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_LUIGI_NEUTRAL_SPECIAL => "KIRBY_HAT_LUIGI_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_MARTH_NEUTRAL_SPECIAL => "KIRBY_HAT_MARTH_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_ZELDA_NEUTRAL_SPECIAL => "KIRBY_HAT_ZELDA_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_YOUNG_LINK_NEUTRAL_SPECIAL => "KIRBY_HAT_YOUNG_LINK_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_DOC_NEUTRAL_SPECIAL => "KIRBY_HAT_DOC_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_FALCO_NEUTRAL_SPECIAL => "KIRBY_HAT_FALCO_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_PICHU_NEUTRAL_SPECIAL => "KIRBY_HAT_PICHU_NEUTRAL_SPECIAL".to_string(),
        Attack::KIRBY_HAT_GAME_AND_WATCH_NEUTRAL_SPECIAL => "KIRBY_HAT_GAME_AND_WATCH_NEUTRAL_SPECIAL".to_string(),   
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