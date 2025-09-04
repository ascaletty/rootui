use crossterm::event::KeyEvent;
use crossterm::event::{self, KeyCode};
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::backend::ClearType;
use ratatui::crossterm::terminal;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::*;
use ratatui::widgets::{Block, Borders};

use crate::structs::ClearingType;
use crate::structs::Map;
use crate::structs::MapTypes;
use crate::structs::{Clearing, Focus};
use std::io::*;

impl Focus {
    /// Cycle to the next pane
    fn next(&self) -> Self {
        use Focus::*;
        match self {
            Left => Right,
            Right => Left,
        }
    }

    ///// Cycle to the previous pane
    //fn prev(&self) -> Self {
    //    use Focus::*;
    //    match self {
    //        Right=>
    //        Top => Left,
    //        Bottom => Top,
    //    }
    //}
}
fn focused_block<'a>(title: &'a str, is_focused: bool) -> Block<'a> {
    let border_style = if is_focused {
        Style::default().fg(Color::Yellow) // highlighted
    } else {
        Style::default().fg(Color::White) // normal
    };

    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style)
}

impl Map {
    pub fn blank(map: MapTypes) -> Self {
        match map {
            MapTypes::Lake => {
                let one = Clearing {
                    buildings: vec![],
                    suit: ClearingType::None,
                    id: 1,
                    inhabitants: vec![],
                    adjacent: vec![2, 4, 5],
                    river_adj: false,
                    build_slots: 1,
                    token: vec![],
                    ruins: false,
                };
                let two = Clearing {
                    buildings: vec![],
                    suit: ClearingType::None,
                    id: 2,
                    inhabitants: vec![],
                    adjacent: vec![1, 3, 4],
                    river_adj: false,
                    build_slots: 1,
                    token: vec![],
                    ruins: false,
                };
                let three = Clearing {
                    buildings: vec![],
                    suit: ClearingType::None,
                    id: 3,
                    inhabitants: vec![],
                    adjacent: vec![2, 6],
                    river_adj: false,
                    build_slots: 2,
                    token: vec![],
                    ruins: false,
                };
                let four = Clearing {
                    buildings: vec![],
                    suit: ClearingType::None,
                    id: 4,
                    inhabitants: vec![],
                    adjacent: vec![1, 2],
                    river_adj: true,
                    build_slots: 3,
                    ruins: true,
                    token: vec![],
                };
                let five = Clearing {
                    buildings: vec![],
                    id: 5,
                    inhabitants: vec![],
                    adjacent: vec![1, 8, 9],
                    river_adj: false,
                    build_slots: 1,
                    ruins: false,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let six = Clearing {
                    buildings: vec![],
                    id: 6,
                    inhabitants: vec![],
                    river_adj: true,
                    adjacent: vec![5, 9, 10],
                    build_slots: 3,
                    ruins: true,
                    token: vec![],
                    suit: ClearingType::None,
                };
                //six-seven
                let seven = Clearing {
                    buildings: vec![],
                    id: 7,
                    inhabitants: vec![],
                    river_adj: true,
                    adjacent: vec![8, 10, 11],
                    build_slots: 3,
                    token: vec![],
                    suit: ClearingType::None,
                    ruins: true,
                };
                let eight = Clearing {
                    buildings: vec![],
                    id: 8,
                    inhabitants: vec![],
                    river_adj: false,
                    adjacent: vec![3, 7, 12],
                    build_slots: 3,
                    ruins: true,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let nine = Clearing {
                    buildings: vec![],
                    id: 9,
                    inhabitants: vec![],
                    river_adj: false,
                    adjacent: vec![5, 6, 10],
                    build_slots: 1,
                    ruins: false,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let ten = Clearing {
                    buildings: vec![],
                    id: 10,
                    inhabitants: vec![],
                    river_adj: false,
                    adjacent: vec![6, 7, 9, 11],
                    build_slots: 1,
                    ruins: false,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let eleven = Clearing {
                    buildings: vec![],
                    id: 11,
                    inhabitants: vec![],
                    river_adj: false,
                    adjacent: vec![7, 10, 12],
                    build_slots: 2,
                    ruins: false,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let twelve = Clearing {
                    buildings: vec![],
                    id: 12,
                    inhabitants: vec![],
                    river_adj: false,
                    adjacent: vec![8, 11],
                    build_slots: 1,
                    ruins: false,
                    token: vec![],
                    suit: ClearingType::None,
                };
                let clearing_vec = vec![
                    one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve,
                ];
                Map {
                    board: MapTypes::Lake,
                    clearings: clearing_vec,
                    exit: false,
                    focus: Focus::Left,
                }
            }
            _ => panic!("only lake rn"),
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let [base, bar] = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)])
            .areas(frame.area());
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(base);

        let main = focused_block("root", matches!(self.focus, Focus::Left));
        let command = focused_block("actions", matches!(self.focus, Focus::Right));
        let scoreboard = focused_block("scoreboard", true);
        frame.render_widget(main, left);
        frame.render_widget(command, bar);
        frame.render_widget(scoreboard, right);
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::Left => {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => (),
                };
            }
            Focus::Right => {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Tab => {
                        Focus::next(&self.focus);
                    }
                    _ => (),
                };
            }
        }
    }
    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}
pub fn ui() -> Result<()> {
    let mut terminal = ratatui::init();

    let app_result = Map::blank(MapTypes::Lake).run(&mut terminal);
    ratatui::restore();
    app_result
}
