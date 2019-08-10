
use rand::prelude::*;
mod player;
use player::Stats;
extern crate rand;


fn main() {

    let mut rng = thread_rng();
    let abs: i16 = rng.gen_range(0, 601);
    let hits: i16 = rng.gen_range(0, 250);
    let my_player = player::Player { abs: abs, hits: hits };

    println!("Hello, world! abs random value is {}!", abs);
    println!("Hello, world! hits random value is {}!", hits);
    println!("batting average is {:.3}!", my_player.ba())
}
