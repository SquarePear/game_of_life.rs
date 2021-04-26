mod life;
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let mut size = (16, 16);

    if let Some((Width(w), Height(h))) = terminal_size() {
        size = (w / 2, h - 1);
    }

    let size = size;

    let mut life = life::Life::new(size);

    life.randomize(64);

    life.print();

    let delay = time::Duration::from_millis(1000 / 10);

    loop {
        life.update();
        life.print();

        thread::sleep(delay);
    }
}
