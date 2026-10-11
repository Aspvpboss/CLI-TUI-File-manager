use std::env;
use std::thread::current;

use pfad_core::*;
use crossterm::event;
use ratatui::widgets::{Paragraph, Block, ListDirection, ListItem, List};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};

/*
This layout is temporary, functions will be moved into separate files,
I am just testing with it right now. I am aware it is super bad practice
*/



pub fn run() -> Result<()> {
    println!("pfad_tui running on pfad_core v{}", pfad_core::version());

    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn render(frame: &mut ratatui::Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1), Fill(2)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let Ok(entrys) = pfad_core::list_dir(".") else {
        return;
    };

    let Ok(current_dir) = env::current_dir() else {
        return;
    };

    // do not ask
    let mut parent_dir_paths: Vec<String> = current_dir
        .ancestors()
        .map(|path| {
            // Get the name just like before
            let mut name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            
            // Append a '/' if it doesn't already end with one (to protect the root "/")
            if !name.ends_with('/') && !name.ends_with('\\') {
                name.push('/');
            }
            
            name
        })
        .collect();

    parent_dir_paths.reverse();

    let mut current_dir_paths : Vec<String> = Vec::new();

    for entry in entrys {
        current_dir_paths.push(entry.convert_to_string());   
    } 

    let current_list = List::new(current_dir_paths)
        .block(Block::bordered().title("Files"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let parent_dir_list = List::new(parent_dir_paths)
        .block(Block::bordered().title("Parent Directories"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(Block::bordered().title(format!("current_dir: {current_dir:?}")), title_area);
    frame.render_widget(parent_dir_list, left_area);
    frame.render_widget(current_list, right_area);

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