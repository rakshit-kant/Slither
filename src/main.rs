struct Game {
    snake: Vec<(usize, usize)>,
}

fn main() {
    let game = Game {
        snake: vec![(5, 5), (4, 5), (3, 5)],
    };

    println!("{:?}", game.snake);
}
