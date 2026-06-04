use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::models::TaskModel;
use crate::service::TaskService;
use crate::tui::Tui;

const MAX_LOGS: usize = 1000;

pub struct App {
    pub input: String,
    pub logs: VecDeque<String>,
    pub planned_tasks: Vec<TaskModel>,
    pub ready_tasks: Vec<TaskModel>,
    pub executing_tasks: Vec<TaskModel>,
    pub verified_tasks: Vec<TaskModel>,
    pub planned_count: usize,
    pub ready_count: usize,
    pub executing_count: usize,
    pub verified_count: usize,
    pub service: TaskService,
    pub search_term: Option<String>,
    pub should_quit: bool,
    pub cpu_model: String,
    pub mem_size: String,
    pub log_scroll_offset: usize,
    pub hide_logs: bool,
}

impl App {
    pub fn new(service: TaskService) -> Self {
        let sys_info = crate::utils::get_system_info();
        let app = Self {
            input: String::new(),
            logs: VecDeque::new(),
            planned_tasks: Vec::new(),
            ready_tasks: Vec::new(),
            executing_tasks: Vec::new(),
            verified_tasks: Vec::new(),
            planned_count: 0,
            ready_count: 0,
            executing_count: 0,
            verified_count: 0,
            service,
            search_term: None,
            should_quit: false,
            cpu_model: sys_info.cpu_model,
            mem_size: sys_info.mem_size,
            log_scroll_offset: 0,
            hide_logs: std::env::args().any(|arg| arg == "-c"),
        };
        app.service.emit_ui_message("TeaQL traces one business request into generated SQL, facets, and audit records.");
        app.service.emit_ui_message("System successfully initialized.");
        app.service.emit_ui_message("Pre-loaded SQLite database 'robot_kanban.db'.");
        app.service.emit_ui_message("TeaQL v1.0.3: Comment Propagation is fully active.");
        app
    }

    pub fn add_log(&mut self, msg: &str) {
        let was_scrolled = self.log_scroll_offset > 0;
        if self.logs.len() >= MAX_LOGS {
            self.logs.pop_front();
            if self.log_scroll_offset > 0 {
                self.log_scroll_offset -= 1;
            }
        }
        self.logs.push_back(msg.to_owned());
        if was_scrolled {
            self.log_scroll_offset += 1;
        }
    }

    pub fn check_sql_logs(&mut self) {
        let new_logs = self.service.check_sql_logs();
        for log in new_logs {
            self.add_log(&log);
        }
    }

    pub async fn reload_data(&mut self) -> Result<(), Box<dyn Error>> {
        let res = self.service.reload_data(&self.search_term).await?;

        self.planned_tasks = res.planned_tasks;
        self.ready_tasks = res.ready_tasks;
        self.executing_tasks = res.executing_tasks;
        self.verified_tasks = res.verified_tasks;
        self.planned_count = res.planned_count;
        self.ready_count = res.ready_count;
        self.executing_count = res.executing_count;
        self.verified_count = res.verified_count;

        Ok(())
    }

    /// Main application loop: draw UI, handle keyboard input, dispatch commands.
    pub async fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        loop {
            self.check_sql_logs();
            terminal.draw(|f| crate::ui::ui(f, self))?;

            if crossterm::event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char(c) => {
                                self.input.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input.pop();
                            }
                            KeyCode::Enter => {
                                if let Err(e) = crate::commands::execute(self).await {
                                    self.add_log(&format!("Error: {}", e));
                                }
                            }
                            KeyCode::Esc => {
                                self.should_quit = true;
                            }
                            KeyCode::Up => {
                                let log_height = if let Ok(size) = terminal.size() {
                                    ((size.height / 2) as usize).saturating_sub(2)
                                } else {
                                    10
                                };
                                let max_scroll = self.logs.len().saturating_sub(log_height);
                                self.log_scroll_offset = (self.log_scroll_offset + 1).min(max_scroll);
                            }
                            KeyCode::PageUp => {
                                let log_height = if let Ok(size) = terminal.size() {
                                    ((size.height / 2) as usize).saturating_sub(2)
                                } else {
                                    10
                                };
                                let max_scroll = self.logs.len().saturating_sub(log_height);
                                self.log_scroll_offset = (self.log_scroll_offset + 10).min(max_scroll);
                            }
                            KeyCode::Down => {
                                self.log_scroll_offset = self.log_scroll_offset.saturating_sub(1);
                            }
                            KeyCode::PageDown => {
                                self.log_scroll_offset = self.log_scroll_offset.saturating_sub(10);
                            }
                            _ => {}
                        }
                    }
                }
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }
}
