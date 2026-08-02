use std::io;

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

/// Initialize the terminal: enable raw mode, enter alternate screen, hide cursor.
pub fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::cursor::Hide,
    )?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restore the terminal: disable raw mode, leave alternate screen, show cursor.
pub fn restore(terminal: &mut Tui) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Show the cursor and enable blinking (called after startup screens, before main TUI loop).
pub fn show_cursor(terminal: &mut Tui) -> io::Result<()> {
    execute!(
        terminal.backend_mut(),
        crossterm::cursor::Show,
        crossterm::cursor::EnableBlinking
    )
}
