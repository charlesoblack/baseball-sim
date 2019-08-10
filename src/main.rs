
use rand::prelude::*;
mod player;
use player::Stats;
use player::Actions;
extern crate rand;


fn main() {

    let mut rng = thread_rng();
    let mut my_player = player::Player { abs: 0,
                                         hits: 0,
                                         coordination: rng.gen_range(0, 101)};

    for n in 0..100 {
        let rng_value = rng.gen_range(0, 101);
        my_player.at_bat(rng_value);
        println!("batting average is {:.3}!", my_player.ba());
    }

}
