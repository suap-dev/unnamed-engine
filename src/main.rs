mod app;
mod graphics;
mod user_events;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    app::App::run()
}
