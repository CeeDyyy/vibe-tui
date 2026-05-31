use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::*,
    DefaultTerminal,
};
use std::io;

fn main() -> io::Result<()> {
    // 1. Setup the terminal to capture raw keyboard input
    let mut terminal = ratatui::init();
    
    // 2. Run the main application loop
    let app_result = run_app(&mut terminal);
    
    // 3. Cleanly restore the terminal when we quit
    ratatui::restore();
    app_result
}

fn run_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut vibe_score: u16 = 50;

    // This loop runs continuously, drawing the screen and waiting for keys
    loop {
        terminal.draw(|frame| {
            // Split the screen into three horizontal slices
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Top: 3 lines tall
                    Constraint::Min(1),    // Middle: Takes remaining space
                    Constraint::Length(3), // Bottom: 3 lines tall
                ])
                .split(frame.area());

            // --- TOP GAUGE ---
            let gauge = Gauge::default()
                .block(Block::bordered().title(" Current Vibe Level "))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(vibe_score);
            frame.render_widget(gauge, chunks[0]);

            // --- MIDDLE MESSAGE ---
            let message = match vibe_score {
                0..=30 => "Low Energy... Need Coffee. \n\n(x_x)",
                31..=70 => "Getting there! \n\n(^_^)",
                _ => "MAXIMUM VIBE DETECTED! \n\n\\(^O^)/",
            };
            let main_content = Paragraph::new(message)
                .centered()
                .block(Block::bordered());
            frame.render_widget(main_content, chunks[1]);

            // --- BOTTOM HELP ---
            let help = Paragraph::new("Press 'j' to boost vibe | 'k' to chill | 'q' to quit")
                .centered()
                .block(Block::bordered());
            frame.render_widget(help, chunks[2]);
        })?;

        // --- KEYBOARD CONTROLS ---
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()), // Quit the app
                KeyCode::Char('j') => vibe_score = vibe_score.saturating_add(10).min(100), // Increase
                KeyCode::Char('k') => vibe_score = vibe_score.saturating_sub(10), // Decrease
                _ => {}
            }
        }
    }
}