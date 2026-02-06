use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct SnakeGame {
    width: i32,
    height: i32,
    snake: VecDeque<Point>,
    food: Point,
    direction: Direction,
    game_over: bool,
}

impl SnakeGame {
    fn new(width: i32, height: i32) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Point { x: width / 2, y: height / 2 });
        
        SnakeGame {
            width,
            height,
            snake,
            food: Point { x: 2, y: 2 },
            direction: Direction::Right,
            game_over: false,
        }
    }

    fn draw(&self) {
        // Clear screen using ANSI escape codes
        print!("\x1B[2J\x1B[1;1H");
        
        // Draw top border
        for _ in 0..self.width + 2 { print!("#"); }
        println!();

        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                let p = Point { x, y };
                if self.snake.contains(&p) {
                    if p == *self.snake.front().unwrap() {
                        print!("H"); // Head
                    } else {
                        print!("S"); // Body
                    }
                } else if p == self.food {
                    print!("F");
                } else {
                    print!(" ");
                }
            }
            println!("#");
        }

        // Draw bottom border
        for _ in 0..self.width + 2 { print!("#"); }
        println!();
    }

    fn update(&mut self) {
        let head = self.snake.front().unwrap();
        let mut new_head = *head;

        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Check collisions with walls
        if new_head.x < 0 || new_head.x >= self.width || new_head.y < 0 || new_head.y >= self.height {
            self.game_over = true;
            return;
        }

        // Check collisions with self
        if self.snake.contains(&new_head) {
            // If we are just chasing our tail (which will move), it's fine, 
            // but simple logic says if new_head exists in current snake, crash.
            // Technically if new_head == tail and we don't eat, tail moves, so it's safe.
            // But for simplicity, let's just crash if we hit any part of the current body.
            // To be precise: if we eat, we grow, so tail stays. If we don't eat, tail moves.
            // Let's check collision excluding the tail if we are not eating?
            // Simplest: just check collision.
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            // Generate new food using simple pseudo-randomness
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            let nanos = since_the_epoch.subsec_nanos();
            
            // Simple LCG-like behavior or just use nanos
            self.food = Point {
                x: (nanos as i32 % self.width).abs(),
                y: ((nanos / 100) as i32 % self.height).abs(),
            };
        } else {
            self.snake.pop_back();
        }
    }
}

fn main() {
    let mut game = SnakeGame::new(20, 10);

    println!("Welcome to Snake! (Turn-based)");
    println!("Controls: w (up), s (down), a (left), d (right) + Enter");
    println!("Press Enter to start...");
    let _ = io::stdin().read_line(&mut String::new());

    loop {
        game.draw();

        if game.game_over {
            println!("Game Over!");
            break;
        }

        print!("Move (w/a/s/d): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "w" => game.direction = Direction::Up,
            "s" => game.direction = Direction::Down,
            "a" => game.direction = Direction::Left,
            "d" => game.direction = Direction::Right,
            _ => {} // Keep current direction
        }

        game.update();
    }
}
