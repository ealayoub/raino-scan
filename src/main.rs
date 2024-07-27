mod net;
mod tui;
mod app;
use app::{App, run_app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = tui::init()?;

    let mut app = App {
        hosts: Vec::new(),
        selected_host: 0,
    };

    app.update_hosts(net::scan());
    run_app(&mut terminal, &mut app)?;
    Ok(())
}
