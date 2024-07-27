use crate::net;
use crate::tui;
use net::HostInfo;
use std::io;
#[allow(unused_imports)]
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::*,
    style::*,
    widgets::*,
    Terminal,
    crossterm::event,
};

pub struct App {
    pub hosts: Vec<HostInfo>,
    pub selected_host: usize,
}

impl App {
    pub fn next(&mut self) {
        if self.selected_host < self.hosts.len() - 1 {
            self.selected_host += 1;
        }
    }
    pub fn previous(&mut self) {
        if self.selected_host > 0 {
            self.selected_host -= 1;
        }
    }
    pub fn update_hosts(&mut self, hosts: Vec<HostInfo>) {
        self.hosts = hosts;
        self.selected_host = 0;
    }
}

impl Drop for App {
    fn drop(&mut self) {
        tui::restore().expect("failed to restore");
    }
}

pub fn run_app(terminal: &mut tui::Tui, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints(
                    [
                        ratatui::layout::Constraint::Percentage(30),
                        ratatui::layout::Constraint::Percentage(70),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let hosts: Vec<ratatui::widgets::ListItem> = app
                .hosts
                .iter()
                .map(|h| ratatui::widgets::ListItem::new(h.host.clone()))
                .collect();

            let hosts_list = ratatui::widgets::List::new(hosts)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title("Hosts"),
                )
                .highlight_symbol(">>")
                .highlight_style(
                    ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD),
                );

            f.render_widget(hosts_list, chunks[0]);

            if let Some(host) = app.hosts.get(app.selected_host) {
                let details = format!(
                    "Host: {}\nMAC: {}\nVendor: {}",
                    host.host,
                    host.mac.as_deref().unwrap_or("N/A"),
                    host.vendor.as_deref().unwrap_or("N/A")
                );
                let details = ratatui::widgets::Paragraph::new(details).block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title("Host Details"),
                );
                f.render_widget(details, chunks[1]);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(10))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Down => app.next(),
                    event::KeyCode::Up => app.previous(),
                    event::KeyCode::Char('r') => {
                        app.update_hosts(net::scan());
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
