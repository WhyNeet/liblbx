use std::{
    process,
    thread::{self, JoinHandle},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub struct EventHandler {
    kill: (flume::Sender<()>, flume::Receiver<()>),
    event: flume::Sender<KeyEvent>,
}

impl EventHandler {
    pub fn new() -> (Self, flume::Receiver<KeyEvent>, flume::Sender<()>) {
        let (kill_tx, kill_rx) = flume::bounded::<()>(1);
        let (event_tx, event_rx) = flume::unbounded::<KeyEvent>();

        (
            Self {
                event: event_tx,
                kill: (kill_tx.clone(), kill_rx),
            },
            event_rx,
            kill_tx,
        )
    }

    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            if let Ok(()) = self.kill.1.try_recv() {
                break;
            }

            if let Err(e) = self.handle_event() {
                eprintln!("An error occured: {e}");
                process::exit(1)
            }
        })
    }

    fn handle_event(&self) -> anyhow::Result<()> {
        let event = event::read()?;

        if let Event::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return Ok(());
            }

            match key_event.code {
                KeyCode::Up | KeyCode::Down | KeyCode::Enter => {
                    self.event.send(key_event)?;
                }
                KeyCode::Esc => self.kill.0.send(())?,
                _ => {}
            }
        }

        Ok(())
    }
}
