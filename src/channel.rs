use std::{io::stdin, thread, time::Duration};

use bevy::prelude::*;
use crossbeam_channel::*;

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
        if buff == "test" {
            sender.try_send(String::from("yay"));
        }
    }
}

pub fn listen_channel(receiver: Res<ChannelTD>) {
    if !receiver.receiver.is_empty() {
        while let Ok(msg) = receiver.receiver.try_recv() {
            info!("YAYYY");
        }
    }
}
