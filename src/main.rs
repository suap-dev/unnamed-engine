mod app;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    app::App::run()
}
