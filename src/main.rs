mod simulation;

use simulation::{Config, World};
use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 8 {
        eprintln!(
            "Usage: {} <width> <height> <fish_breed_time> <shark_breed_time> <shark_start_energy> <initial_fish> <initial_sharks>",
            args[0]
        );
        process::exit(1);
    }

    let width: usize = args[1].parse().expect("Invalid width");
    let height: usize = args[2].parse().expect("Invalid height");
    let fish_breed_time: u32 = args[3].parse().expect("Invalid fish breed time");
    let shark_breed_time: u32 = args[4].parse().expect("Invalid shark breed time");
    let shark_start_energy: u32 = args[5].parse().expect("Invalid shark start energy");
    let initial_fish: usize = args[6].parse().expect("Invalid initial fish");
    let initial_sharks: usize = args[7].parse().expect("Invalid initial sharks");

    let config = Config {
        fish_breed_time,
        shark_breed_time,
        shark_start_energy,
        initial_fish,
        initial_sharks,
    };

    let mut world = World::new(width, height, config);

    loop {
        // Clear screen (using ANSI escape codes)
        print!("\x1B[2J\x1B[1;1H");
        println!("Tick: {}", world.tick_count());
        println!("{}", world);
        world.tick();
        thread::sleep(Duration::from_millis(200));
    }
}
