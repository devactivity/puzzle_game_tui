use crate::game_logic::Game;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use std::io::{self, stdout};

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl UI {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(UI { terminal })
    }

    pub fn display(&mut self, game: &Game) -> io::Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.area());

            let puzzle_block = Block::default().title("Puzzle").borders(Borders::ALL);
            f.render_widget(puzzle_block, chunks[0]);

            if let Some(puzzle) = game.puzzle() {
                let (cursor_y, cursor_x) = game.cursor();
                let mut puzzle_content = vec![];

                for row in 0..puzzle.size() {
                    let mut line_spans = vec![];
                    for col in 0..puzzle.size() {
                        let cell = puzzle.get(row, col);
                        let cell_span = match cell {
                            'X' => Span::styled(
                                "X",
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            ' ' => Span::raw("_"),
                            _ => Span::styled(cell.to_string(), Style::default().fg(Color::Blue)),
                        };

                        if row == cursor_y && col == cursor_x {
                            line_spans.push(Span::styled(
                                format!("[{}]", cell_span.content),
                                Style::default().add_modifier(Modifier::REVERSED),
                            ));
                        } else {
                            line_spans.push(Span::raw("|"));
                            line_spans.push(cell_span);
                            line_spans.push(Span::raw("|"));
                        }
                    }

                    puzzle_content.push(Line::from(line_spans));
                }

                let puzzle_paragraph = Paragraph::new(puzzle_content)
                    .block(Block::default())
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                let puzzle_area = centered_rect(80, 80, chunks[0]);

                f.render_widget(puzzle_paragraph, puzzle_area);
            }

            let info = vec![
                Line::from(format!("Difficulty: {}", game.difficulty)),
                Line::from("Use WASD to move, Enter to mark/unmark, q to quit"),
                Line::from("Goal: Mark all empty spaces with X, avoiding the pre-filled letters."),
            ];

            let info_paragraph = Paragraph::new(info)
                .block(Block::default().title("Info").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            f.render_widget(info_paragraph, chunks[1]);
        })?;

        Ok(())
    }

    pub fn get_input(&self) -> io::Result<char> {
        loop {
            if let Event::Key(key) = event::read()? {
                return Ok(match key.code {
                    KeyCode::Char(c) => c,
                    KeyCode::Enter => ' ',
                    _ => continue,
                });
            }
        }
    }

    pub fn display_victory(&mut self) -> io::Result<()> {
        self.terminal.draw(|f| {
            let chunk = f.area();
            let victory_message =
                Paragraph::new(Line::from("Congratulations! You solved the puzzle!"))
                    .style(Style::default().fg(Color::Green))
                    .alignment(ratatui::layout::Alignment::Center);

            f.render_widget(victory_message, chunk);
        })?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(())
    }

    pub fn play_again(&mut self) -> io::Result<bool> {
        self.terminal.draw(|f| {
            let chunk = f.area();
            let play_again_message = Paragraph::new(Line::from("Play again? (y/n)"))
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);

            f.render_widget(play_again_message, chunk);
        })?;

        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('N') => return Ok(false),
                    _ => continue,
                }
            }
        }
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 1),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 1),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 1),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 1),
        ])
        .split(popup_layout[1])[1]
}
