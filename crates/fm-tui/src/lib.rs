use fm_core::*;
use crossterm::event;
use ratatui::widgets::{Paragraph, Block, ListDirection, ListItem, List};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};

/*
This layout is temporary, functions will be moved into separate files,
I am just testing with it right now
*/



pub fn run() -> Result<()> {
    println!("fm-tui running on fm-core v{}", fm_core::version());

    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn render(frame: &mut ratatui::Frame) {
    use Constraint::{Fill, Length, Min};

    // let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    // let [title_area, main_area, status_area] = vertical.areas(frame.area());
    // let horizontal = Layout::horizontal([Fill(1); 2]);
    // let [left_area, right_area] = horizontal.areas(main_area);
    let Ok(entrys) = fm_core::list_dir(".") else {
        return;
    };

    let mut string_entrys : Vec<String> = Vec::new();

    for entry in entrys {
        string_entrys.push(entry.name);   
    } 

    let list = List::new(string_entrys)
        .block(Block::bordered().title("List"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

        
    // frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    // frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    // frame.render_widget(Block::bordered().title("Left"), left_area);
    // frame.render_widget(Block::bordered().title("Right"), right_area);
    frame.render_widget(list, frame.area());

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