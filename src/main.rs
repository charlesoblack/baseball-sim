
extern crate rand;
use rand::prelude::*;

extern crate sdl2;
use sdl2::pixels::Color;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod player;
use player::Stats;
use player::Actions;


fn main() {

    let width = 800;
    let height = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("baseball-sim", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font("/usr/share/fonts/truetype/abyssinica/AbyssinicaSIL-R.ttf", 32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let surface = font.render("Hello Rust!")
            .blended(Color::RGB(255, 255, 255)).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let target = sdl2::rect::Rect::new(120, 120, 120, 120);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = thread_rng();
    'running: loop {
        canvas.clear();
        canvas.copy(&texture, None, Some(target)).unwrap();
        let r = rng.gen_range(0, 255);
        let g = rng.gen_range(0, 255);
        let b = rng.gen_range(0, 255);
        canvas.set_draw_color(Color::RGB(r, g, b));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }

    for _ in 0..10 {
        let mut my_player = player::Player { abs: 0,
                                             hits: 0,
                                             coordination: rng.gen_range(0, 35)};

        for _ in 0..1000 {
            let rng_value = rng.gen_range(0, 101);
            my_player.at_bat(rng_value);
        }
        println!("batting average is {:.3}!", my_player.ba());
    }
}
