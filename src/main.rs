use taceo_merces_tui::App;

fn main() -> eyre::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new()?.run(terminal);
    ratatui::restore();
    app_result
}
