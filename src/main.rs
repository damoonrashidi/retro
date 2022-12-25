use core::panic::PanicInfo;
use std::io::stdout;
use std::panic;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use crossterm::cursor::RestorePosition;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::{execute, ExecutableCommand};

use retro::ui::help::help;
use retro::ui::new_note::new_note;
use retro::ui::room_info::room_info;
use retro::{
    app::{mode::Mode, state::State},
    cli::RetroArgs,
    handlers::handle_input,
    network::{actions::NetworkAction, remote::Remote},
    ui::{notes_list::notes_list, status_bar::status_bar},
};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Terminal;
use tui_textarea::{Input, Key};

#[tokio::main]
async fn start_tokio(io_rx: Receiver<NetworkAction>, network: &mut Remote) {
    while let Ok(event) = io_rx.recv() {
        let _ = network.handle_event(event).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let args = RetroArgs::new();
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<NetworkAction>();

    let state = Arc::new(Mutex::new(State::new(sync_io_tx, args.clone())));

    state
        .lock()
        .expect("cannot do stuff")
        .dispatch(NetworkAction::ListenForChanges);

    let cloned_state = Arc::clone(&state);
    let cloned_args = args.clone();
    std::thread::spawn(move || {
        let mut network = Remote::new(&cloned_args.room, &state);
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

fn panic_hook(info: &PanicInfo<'_>) {
    dbg!(info);
    let _ = quit();
}

async fn start_ui(args: RetroArgs, state: &Arc<Mutex<State>>) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle(args.room.clone()))?;

    let mut terminal = Terminal::new(backend)?;
    let mut textarea = new_note();

    loop {
        let size = terminal.size()?;
        let mut state = state.lock().unwrap();

        let input: Input = crossterm::event::read()?.into();

        if let (
            Input {
                key: Key::Char('q'),
                ..
            },
            Mode::Normal,
        ) = (&input, &state.mode)
        {
            quit()?;
            return Ok(());
        }

        terminal.draw(|ui| {
            handle_input(&input, &mut state, &mut textarea);

            // Notes list
            ui.render_widget(
                notes_list(&state),
                Rect::new(0, 0, size.width, size.height - 1),
            );

            // Mode info
            ui.render_widget(status_bar(&state), Rect::new(0, size.height - 1, 5, 1));
            ui.render_widget(
                room_info(&state.display_name, &args.room),
                Rect::new(6, size.height - 1, 30, 1),
            );
            ui.render_widget(
                Paragraph::new(format!("{} participants", &state.participants.len())),
                Rect::new(size.width - 17, size.height - 1, 16, 1),
            );

            if state.show_help {
                ui.render_widget(
                    help(&state),
                    Rect::new(
                        size.width - size.width / 3 - 4,
                        size.height - 15,
                        size.width / 3,
                        12,
                    ),
                );
            }

            if state.mode == Mode::Insert {
                ui.render_widget(
                    textarea.widget(),
                    Rect::new(size.width / 2 - 15, size.height / 4, 30, 5),
                );
            }
        })?;
    }
}
