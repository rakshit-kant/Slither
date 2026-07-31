#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Game {
    snake: Vec<Position>,
}

fn main() {
    let game = Game {
        snake: vec![
            Position { x: 5, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 5 },
        ],
    };

    println!("{:?}", game.snake);
}
