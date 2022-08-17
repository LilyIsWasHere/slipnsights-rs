use std::prelude::*;
use std::{fs, io};
use peppi::model::frame::{Frame, Data};

fn main() {
    let mut buf = io::BufReader::new(
        fs::File::open("replays/game.slp").unwrap());
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
    let mut player_punishes: [Option<ActivePunish>; N] = [(); N].map(|_| None);
    for frame in frames {
        for port in 0..N {
            match &mut player_punishes[port] {
                Some(punish) => {
                    let conversion: Option<Conversion> = punish.processFrame(&frame);
                    if let Some(c) = conversion {
                        conversions.push(c);
                    }
                },
                None => {
                    // Identify new conversions
                    todo!();
                },
            }
        }
    }
}

struct ActivePunish {
    advantage_player: usize,
    disadvantage_player: usize,
    frames_since_last_hit: usize,
    has_landed: bool,
    has_been_actionable: bool,
    conversion_completed: bool,
}

impl ActivePunish {
    fn new(advantage_player_index: usize, disadvantage_player_index: usize) -> ActivePunish {
        ActivePunish {
            advantage_player: advantage_player_index,
            disadvantage_player: disadvantage_player_index,
            frames_since_last_hit: 0,
            has_landed: false,
            has_been_actionable: false,
            conversion_completed: false,
        }
    }


    fn processFrame<const N: usize>(&mut self, frame: &Frame<N>) -> Option<Conversion> {
        self.frames_since_last_hit += 1;

        let advantage_player_post = frame.ports[self.advantage_player].leader.post;
        let disadvantage_player_post = frame.ports[self.disadvantage_player].leader.post;

        let dp_is_grounded = is_player_grounded(self.disadvantage_player, frame);
        let dp_is_actionable = is_player_actionable(self.disadvantage_player, frame);
        let dp_is_hit = is_player_hit(self.disadvantage_player, frame);

        if dp_is_grounded {
            self.has_landed = true;
        }

        if dp_is_actionable {
            self.has_been_actionable = true;    
        }

        if dp_is_hit {
            self.frames_since_last_hit = 0;
            self.has_landed = false;
            self.has_been_actionable = false;
        }

        if self.frames_since_last_hit >= 45 && self.has_landed && self.has_been_actionable {
            self.conversion_completed = true;
        }

        todo!();

    }
}

struct GameStats {
    conversions: Vec<Conversion>,
}

struct Conversion {
    advantage_player_index: usize,
    disadvantage_player_index: usize,
    active_frames: std::ops::Range<usize>
}

fn is_player_grounded<const N: usize>(player: usize, frame: &Frame<N>) -> bool {
    todo!()
}

fn is_player_actionable<const N: usize>(player: usize, frame: &Frame<N>) -> bool {
    todo!()
}

fn is_player_hit<const N: usize>(player: usize, frame: &Frame<N>) -> bool {
    todo!()
}

