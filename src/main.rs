use core::panic::PanicInfo;
use std::io::stdout;
use std::panic;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use crossterm::cursor::RestorePosition;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use crossterm::{execute, ExecutableCommand};
use retro::event::event::{Event, Events};
use retro::handlers::handle_input;
use retro::ui::help::help;
use retro::ui::new_note::new_note;
use retro::ui::room_info::room_info;
use retro::{
    app::{mode::Mode, state::State},
    cli::RetroArgs,
    network::{actions::NetworkAction, remote::Remote},
    ui::{notes_list::notes_list, status_bar::status_bar},
};
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Terminal;
use tui_textarea::TextArea;

#[tokio::main]
async fn start_tokio(io_rx: Receiver<NetworkAction>, network: &Remote) {
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
    let mut textarea = new_note();

    let state = Arc::new(Mutex::new(State::new(sync_io_tx, args.clone())));

    state
        .lock()
        .expect("cannot do stuff")
        .dispatch(NetworkAction::GetNotes);

    let cloned_state = Arc::clone(&state);
    let cloned_args = args.clone();

    std::thread::spawn(move || {
        let network = Remote::new(&cloned_args.room, &state);
        start_tokio(sync_io_rx, &network);
    });

    start_ui(args, &cloned_state, &mut textarea).await?;

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

async fn start_ui(
    args: RetroArgs,
    state: &Arc<Mutex<State>>,
    textarea: &mut TextArea<'static>,
) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let mut backend = CrosstermBackend::new(stdout);
    backend.execute(SetTitle(args.room.clone()))?;

    let events = Events::new(Duration::from_millis(200));

    let mut terminal = Terminal::new(backend)?;

    loop {
        let size = terminal.size()?;
        let mut state = state.lock().expect("Could not lock state");

        terminal.draw(|ui| {
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

            ui.render_widget(
                Paragraph::new(state.tick_count.to_string()),
                Rect::new(0, 0, 10, 1),
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

        match events.next()? {
            Event::Input(i) => {
                if let (
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    },
                    Mode::Normal,
                ) = (i, &state.mode)
                {
                    return quit();
                }
                handle_input(i, &mut state, textarea);
            }
            Event::Tick => state.tick(),
        }
    }
}
