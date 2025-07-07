#![allow(dead_code)] // TODO: disallow dead_code when ready

mod app;
mod graphics;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    app::App::run()
}
