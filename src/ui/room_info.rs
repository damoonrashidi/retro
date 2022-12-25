use tui::widgets::Paragraph;

pub fn room_info<'a>(display_name: &'a String, room_name: &'a String) -> Paragraph<'a> {
    Paragraph::new(format!("{display_name} @ {room_name}"))
}
