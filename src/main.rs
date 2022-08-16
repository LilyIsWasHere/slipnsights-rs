use std::prelude::*;
use std::{fs, io};
use peppi::model::frame::{Frame, Data};

fn main() {
    let mut buf = io::BufReader::new(
        fs::File::open("replays/game.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();
    let frames_enum = game.frames;

    let players: Vec<Player> = Vec::new();

    for player in game.metadata.players.unwrap().iter() {
        let port = match player.port {
            peppi::model::primitives::Port::P1 => 1,
            peppi::model::primitives::Port::P2 => 2,
            peppi::model::primitives::Port::P3 => 3,
            peppi::model::primitives::Port::P4 => 4,
        };
        let player = Player::new(port)
    }


    match frames_enum {
        peppi::model::game::Frames::P1(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P2(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P3(f) => handle_frames_enum(f),
        peppi::model::game::Frames::P4(f) => handle_frames_enum(f),
    }
}

fn handle_frames_enum<const N: usize>(frames: Vec<Frame<N>>) {
    for frame in frames {
        println!("{:?}", frame);
    }
}

