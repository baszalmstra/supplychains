#[macro_use]
extern crate log;

mod app;
mod camera;
mod states;
mod timing;
mod world;

fn main() {
    pretty_env_logger::init();

    app::Application::new(states::GameState)
        .with_title("Supply Chains")
        .with_dimensions((1280, 800).into())
        .build()
        .unwrap()
        .run();
}