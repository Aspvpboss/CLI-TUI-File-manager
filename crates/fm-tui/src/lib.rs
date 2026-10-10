use fm_core::*;
use crossterm::event;
use ratatui::widgets::Paragraph;

pub fn run() -> Result<()> {
    println!("fm-tui running on fm-core v{}", fm_core::version());

    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn render(frame: &mut ratatui::Frame) {
    let text = Paragraph::new("Hello World!");
    frame.render_widget(text, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::read()?.is_key_press() {
        return Ok(true);
    }

    Ok(false)
}


fn run_app(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if should_quit()? {
            break Ok(());
        }
    }
}