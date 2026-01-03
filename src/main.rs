use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use std::default;
use std::io::{self, Result, Stdout};
use tui_textarea::{Input, Key, TextArea};

#[derive(Debug, Clone)]
pub struct Model {
    input: String,
    running_state: RunningState,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            input: String::new(),
            running_state: RunningState::Running,
        }
    }
}

#[derive(PartialEq)]
enum Message {
    Increment,
    Decrement,
    Reset,
    Quit,
    SetInput(String),
    Enter,
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::SetInput(input) => {
            model.input = input;
            None
        }
        Message::Enter => {
            println!("Input: {}", model.input);
            model.input.clear();
            None
        }
        _ => None,
    }
}

fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        // .margin(1)
        .constraints([Constraint::Length(3), Constraint::Max(3)])
        .areas(frame.area());

    let [index_area, input_area] = layout;

    let input = Paragraph::new(model.input.clone())
        .style(Style::default())
        .block(Block::bordered().title("Input"));

    frame.render_widget(input, input_area);
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Exit,
}

struct App {
    model: Model,
}

impl Default for App {
    fn default() -> Self {
        Self {
            model: Model::default(),
        }
    }
}

impl App {
    pub fn run(mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Enter a valid float (e.g. 1.56)");

        loop {
            term.draw(|f| {
                // let chunks = layout.split(f.area());
                // f.render_widget(&textarea, chunks[0]);
                // self.render(f).unwrap();
                view(&self.model, f);
            })
            .unwrap();

            match crossterm::event::read().unwrap().into() {
                Input { key: Key::Esc, .. } => break,
                Input {
                    key: Key::Enter, ..
                } => {
                    update(&mut self.model, Message::Enter);
                }
                input => {
                    if textarea.input(input) {
                        update(
                            &mut self.model,
                            Message::SetInput(textarea.lines()[0].clone()),
                        );
                    }
                }
            }
        }

        disable_raw_mode()?;
        crossterm::execute!(
            term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        term.show_cursor()?;

        println!("Input: {:?}", textarea.lines()[0]);
        Ok(())
    }

    // pub fn render(&mut self, frame: &mut Frame) -> io::Result<()> {
    //     let layout = Layout::default()
    //         .direction(Direction::Vertical)
    //         // .margin(1)
    //         .constraints([Constraint::Length(3), Constraint::Max(3)])
    //         .areas(frame.area());

    //     let [index_area, input_area] = layout;

    //     let input = Paragraph::new(self.input.clone())
    //         .style(Style::default())
    //         .block(Block::bordered().title("Input"));

    //     frame.render_widget(input, input_area);

    //     Ok(())
    // }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture, Show)?;
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    App::default().run(terminal)
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    run_app(&mut terminal)?;
    teardown_terminal(&mut terminal)?;
    Ok(())
}
