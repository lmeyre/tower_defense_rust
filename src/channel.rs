use std::{io::stdin, thread, time::Duration};

use bevy::prelude::*;
use crossbeam_channel::*;

use crate::AppState;

#[derive(Debug, Resource)]
pub struct ChannelTD {
    pub receiver: Receiver<String>,
}

pub fn write_channel(sender: Sender<String>) {
    loop {
        let stdin = stdin();
        let mut buff = String::new();
        if stdin.read_line(&mut buff).is_err() {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        let buff = buff.trim();
        if buff == "restart" && sender.try_send(String::from("restart")).is_ok() {
            // Happyness
        }
    }
}

pub fn listen_channel(receiver: Res<ChannelTD>, mut state: ResMut<NextState<AppState>>) {
    if !receiver.receiver.is_empty() {
        while receiver.receiver.try_recv().is_ok() {
            state.set(AppState::Starting);
        }
    }
}
