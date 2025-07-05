#![allow(dead_code)] // TODO: disallow dead_code when ready

mod app;
mod graphics;
mod state;
mod user_events;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    app::App::run()
}
