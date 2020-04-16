use termion::color::{self, AnsiValue};
use rand::prelude::*;

const TETROMINOS: [[(usize, usize); 4]; 7] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], 
    [(0, 0), (1, 0), (2, 0), (0, 1)], 
    [(0, 0), (1, 0), (2, 0), (1, 1)], 
    [(0, 0), (1, 0), (2, 0), (2, 1)], 
    [(0, 0), (1, 0), (2, 1), (1, 1)], 
    [(0, 1), (1, 0), (2, 0), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)]
];

const COLORS: [(u8, u8, u8); 6] = [
    (5, 0, 0),
    (0, 5, 0),
    (0, 0, 5),
    (5, 5, 0),
    (5, 0, 5),
    (0, 5, 5)
];

pub type Color = AnsiValue;

pub struct OverflowError;

pub enum Direction {
    Left, Right, Down
}

pub struct Tetromino {
    pub cells: [(usize, usize); 4],
    pub color: Color
}

impl Tetromino {

    pub fn new(index: usize, color: Color) -> Self {
        Tetromino {
            cells: TETROMINOS[index],
            color: color
        }
    }

    pub fn new_random(width: usize) -> Self {
        let mut rng = rand::thread_rng();
        let t = rng.gen_range(0, TETROMINOS.len());
        let c = rng.gen_range(0, COLORS.len());
        let (r, g, b) = COLORS[c];
        let mut tetromino = Self::new(t, Color::rgb(r, g, b));
        let center = rng.gen_range(0, width-2);
        for cell in tetromino.cells.iter_mut() {
            cell.1 += center;
        }
        tetromino
    }

    pub fn shift(&mut self, dir: Direction) {
        for cell in self.cells.iter_mut() {
            match dir {
                Direction::Left  => cell.1 -= 1,
                Direction::Right => cell.1 += 1,
                Direction::Down  => cell.0 += 1
            }
        }
    }

    pub fn turn(&mut self) -> Result<(), OverflowError> {
        let center_y = self.cells[1].0 as isize;
        let center_x = self.cells[1].1 as isize;
        for &i in &[0, 2, 3] {
            let y = self.cells[i].0 as isize;
            let x = self.cells[i].1 as isize;
            let (y, x) = (
                center_y + (x - center_x), 
                center_x - (y - center_y)
            );
            if y >= 0 && x >= 0 {
                self.cells[i] = (y as usize, x as usize);
            } else {
                return Err(OverflowError);
            }
        }
        Ok(())
    }
}

