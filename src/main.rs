extern crate game_engine;
use game_engine::{Engine};

fn main() {
    let mut engine = Engine::new();

    fn update(dt: u64) {
        println!("{}", dt);
    }

    engine.register_to_update(update);
    engine.run_game();
}
