use cli::App;

mod cli;
mod constants;
mod custom_widgets;
mod tidy;
mod todo;
mod utils;

fn main() {
    let _ = App::run();
}
