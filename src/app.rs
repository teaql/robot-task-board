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
    pub hide_logs: bool,
    pub pending_delete: Option<u64>,
    pub scroll_percent: f64,
    pub timeline_width: u16,
    pub sql_latencies: VecDeque<f64>,
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
            hide_logs: std::env::args().any(|arg| arg == "-c"),
            pending_delete: None,
            scroll_percent: 1.0,
            timeline_width: 100,
            sql_latencies: {
                let mut d = VecDeque::new();
                for _ in 0..100 {
                    d.push_back(0.0);
                }
                d
            },
        };
        app.service.log_info("TeaQL traces one business request into generated SQL, facets, and audit records.");
        app.service.log_info("System successfully initialized.");
        app.service.log_info("Pre-loaded SQLite database 'robot_kanban.db'.");
        app.service.log_info("TeaQL v1.0.3: Comment Propagation is fully active.");
        app
    }

    pub fn add_log(&mut self, msg: &str) {
        if self.logs.len() >= MAX_LOGS {
            self.logs.pop_front();
        }
        self.logs.push_back(msg.to_owned());

        // Parse latency from log line e.g. [1234µs] or [5.6ms]
        let mut parsed_latency = None;
        if let Some(start_idx) = msg.find('[') {
            if let Some(end_idx) = msg[start_idx..].find(']') {
                let tag = &msg[start_idx + 1..start_idx + end_idx];
                if tag.ends_with("µs") {
                    if let Ok(val) = tag[..tag.len() - 3].parse::<f64>() {
                        parsed_latency = Some(val / 1000.0);
                    }
                } else if tag.ends_with("ms") {
                    if let Ok(val) = tag[..tag.len() - 2].parse::<f64>() {
                        parsed_latency = Some(val);
                    }
                }
            }
        }

        let latency = parsed_latency.unwrap_or(0.0);
        self.sql_latencies.push_back(latency);
        if self.sql_latencies.len() > 100 {
            self.sql_latencies.pop_front();
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
                        if let Some(id) = self.pending_delete {
                            match key.code {
                                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                                    if let Err(e) = self.service.delete_task(id).await {
                                        self.add_log(&format!("Error deleting task: {}", e));
                                    } else {
                                        let _ = self.reload_data().await;
                                    }
                                    self.pending_delete = None;
                                    self.input.clear();
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                    self.pending_delete = None;
                                    self.input.clear();
                                }
                                _ => {}
                            }
                            continue;
                        }

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
                            KeyCode::Left => {
                                let inner_w = self.timeline_width.saturating_sub(2).max(10) as f64;
                                self.scroll_percent = (self.scroll_percent - 1.0 / inner_w).max(0.0);
                            }
                            KeyCode::PageUp => {
                                self.scroll_percent = (self.scroll_percent - 0.1).max(0.0);
                            }
                            KeyCode::Right => {
                                let inner_w = self.timeline_width.saturating_sub(2).max(10) as f64;
                                self.scroll_percent = (self.scroll_percent + 1.0 / inner_w).min(1.0);
                            }
                            KeyCode::PageDown => {
                                self.scroll_percent = (self.scroll_percent + 0.1).min(1.0);
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
