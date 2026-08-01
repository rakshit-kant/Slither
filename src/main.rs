#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Game {
    snake: Vec<Position>,
}

fn main() {
    let width = 80;
    let height = 20;

    let mut board = String::new();

    for _ in 0..height {
        for _ in 0..width {
            board.push('.');
        }

        board.push('\n');
    }

    println!("{board}");

    let game = Game {
        snake: vec![
            Position { x: 5, y: 5 },
            Position { x: 4, y: 5 },
            Position { x: 3, y: 5 },
        ],
    };

    // println!("{:?}", game.snake);
}
