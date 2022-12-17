use std::io::stdout;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use crossterm::cursor::RestorePosition;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::{execute, ExecutableCommand};
use retro::cli::RetroArgs;
use retro::network::actions::NetworkAction;
use retro::network::network::Network;
use retro::state::mode::Mode;
use retro::state::state::State;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key};

#[tokio::main]
async fn start_tokio<'a>(io_rx: Receiver<NetworkAction>, network: &mut Network) {
    while let Ok(event) = io_rx.recv() {
        network.handle_event(event).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = RetroArgs::new();
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<NetworkAction>();

    let state = Arc::new(Mutex::new(State::new(sync_io_tx)));

    let cloned_state = Arc::clone(&state);
    let cloned_args = args.clone();
    std::thread::spawn(move || {
        let mut network = Network::new(&cloned_args.room);
        start_tokio(sync_io_rx, &mut network);
    });

    start_ui(args, &cloned_state).await?;

    Ok(())
}

fn quit() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        RestorePosition
    )?;

    Ok(())
}

async fn start_ui(args: RetroArgs, state: &Arc<Mutex<State>>) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle(args.room))?;

    let mut terminal = Terminal::new(backend)?;

    loop {
        let mut state = state.lock().unwrap();
        terminal.draw(|ui| {
            ui.render_widget(
                Block::default().title("title").borders(Borders::all()),
                Rect::new(10, 10, 10, 5),
            );
        })?;

        let input: Input = crossterm::event::read()?.into();

        if state.mode == Mode::Normal {
            match input {
                Input {
                    key: Key::Char('q'),
                    ..
                } => {
                    quit()?;
                    return Ok(());
                }
                Input {
                    key: Key::Char('g'),
                    ..
                } => state.dispatch(NetworkAction::GetNotes),
                _ => {}
            }
        }
    }
}
