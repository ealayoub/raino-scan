use crate::net;
use crate::tui;
use net::HostInfo;
#[allow(unused_imports)]
use ratatui::{
    backend::CrosstermBackend,
    crossterm::event,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::*,
    style::*,
    widgets::*,
    Terminal,
};
use std::io;

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
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(f.size());

            let hosts: Vec<ListItem> = app
                .hosts
                .iter()
                .map(|h| {
                    if app.hosts[app.selected_host] == *h {
                        ListItem::new(h.host.clone()).fg(Color::Gray).bg(Color::Red)
                    } else {
                        ListItem::new(h.host.clone())
                    }
                })
                .collect();

            let hosts_list = List::new(hosts)
                .block(Block::default().borders(Borders::ALL).title("Hosts"))
                .highlight_symbol(">>")
                .highlight_style(Style::default().add_modifier(Modifier::BOLD));

            f.render_widget(hosts_list, chunks[0]);

            if let Some(host) = app.hosts.get(app.selected_host) {
                let details = format!(
                    "Host: {}\nMAC: {}\nVendor: {}",
                    host.host,
                    host.mac.as_deref().unwrap_or("N/A"),
                    host.vendor.as_deref().unwrap_or("N/A")
                );
                let details = Paragraph::new(details)
                    .block(Block::default().borders(Borders::ALL).title("Host Details"));
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
