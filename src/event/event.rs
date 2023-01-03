use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::event::{self, KeyEvent};

#[derive(Debug)]
pub struct Events {
    pub tick_rate: Duration,
    rx: Receiver<Event<KeyEvent>>,
    _tx: Sender<Event<KeyEvent>>,
}

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
    Tick,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    event_tx.send(Event::Input(key)).unwrap();
                }
            }
            event_tx.send(Event::Tick).unwrap();
        });

        Events {
            tick_rate,
            _tx: tx,
            rx,
        }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}
