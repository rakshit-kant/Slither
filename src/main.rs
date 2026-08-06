#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
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

    let width = 80;
    let height = 20;

    let mut board = String::new();

    for current_y in 0..height {
        for current_x in 0..width {
            let mut is_snake = false;

            for part in &game.snake {
                if part.x == current_x && part.y == current_y {
                    is_snake = true;
                }
            }

            if is_snake {
                board.push('#');
            } else {
                board.push('.');
            }
        }

        board.push('\n');
    }

    println!("{:?}", game.snake);
    println!("{:?}", game);
}
