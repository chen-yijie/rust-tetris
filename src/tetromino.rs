use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::tetromino::Color::*;
use crate::tetromino::Rotation::*;

#[derive(Copy, Clone)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Lime,
    Purple,
    Red,
    Grey,
}

impl Color {
    pub fn as_rgba(&self) -> [f32; 4] {
        match *self {
            Cyan => [0.0, 1.0, 1.0, 1.0],
            Blue => [0.0, 0.5, 1.0, 1.0],
            Orange => [1.0, 0.6, 0.0, 1.0],
            Yellow => [1.0, 1.0, 0.0, 1.0],
            Lime => [0.5, 1.0, 0.0, 1.0],
            Purple => [0.8, 0.0, 1.0, 1.0],
            Red => [1.0, 0.0, 0.0, 1.0],
            Grey => [0.3, 0.3, 0.3, 1.0],
        }
    }
}

pub struct Tetromino {
    color: Color,
    points: [[(usize, usize); 4]; 4],
}

impl Tetromino {
    pub fn get_random_shape() -> &'static Tetromino {
        let mut rng = thread_rng();
        SHAPES.choose(&mut rng).unwrap()
    }
    pub fn points(&self, rotation: Rotation) -> &[(usize, usize); 4] {
        &self.points[rotation as usize]
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
}

pub static SHAPES: [Tetromino; 7] = [Tetromino {
                                         color: Cyan,
                                         points: [[(0, 2), (1, 2), (2, 2), (3, 2)],
                                                  [(2, 0), (2, 1), (2, 2), (2, 3)],
                                                  [(0, 2), (1, 2), (2, 2), (3, 2)],
                                                  [(2, 0), (2, 1), (2, 2), (2, 3)]],
                                     },
                                     Tetromino {
                                         color: Blue,
                                         points: [[(0, 1), (1, 1), (2, 1), (2, 2)],
                                                  [(1, 0), (1, 1), (0, 2), (1, 2)],
                                                  [(0, 0), (0, 1), (1, 1), (2, 1)],
                                                  [(1, 0), (2, 0), (1, 1), (1, 2)]],
                                     },
                                     Tetromino {
                                         color: Orange,
                                         points: [[(0, 1), (1, 1), (2, 1), (0, 2)],
                                                  [(0, 0), (1, 0), (1, 1), (1, 2)],
                                                  [(2, 0), (0, 1), (1, 1), (2, 1)],
                                                  [(1, 0), (1, 1), (1, 2), (2, 2)]],
                                     },
                                     Tetromino {
                                         color: Yellow,
                                         points: [[(1, 1), (2, 1), (1, 2), (2, 2)],
                                                  [(1, 1), (2, 1), (1, 2), (2, 2)],
                                                  [(1, 1), (2, 1), (1, 2), (2, 2)],
                                                  [(1, 1), (2, 1), (1, 2), (2, 2)]],
                                     },
                                     Tetromino {
                                         color: Lime,
                                         points: [[(1, 1), (2, 1), (0, 2), (1, 2)],
                                                  [(1, 0), (1, 1), (2, 1), (2, 2)],
                                                  [(1, 1), (2, 1), (0, 2), (1, 2)],
                                                  [(1, 0), (1, 1), (2, 1), (2, 2)]],
                                     },
                                     Tetromino {
                                         color: Purple,
                                         points: [[(0, 1), (1, 1), (2, 1), (1, 2)],
                                                  [(1, 0), (0, 1), (1, 1), (1, 2)],
                                                  [(1, 0), (0, 1), (1, 1), (2, 1)],
                                                  [(1, 0), (1, 1), (2, 1), (1, 2)]],
                                     },
                                     Tetromino {
                                         color: Red,
                                         points: [[(0, 1), (1, 1), (1, 2), (2, 2)],
                                                  [(2, 0), (1, 1), (2, 1), (1, 2)],
                                                  [(0, 1), (1, 1), (1, 2), (2, 2)],
                                                  [(2, 0), (1, 1), (2, 1), (1, 2)]],
                                     }];

#[derive(Copy, Clone)]
pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

impl Rotation {
    pub fn increase(&self) -> Rotation {
        match *self {
            R0 => R1,
            R1 => R2,
            R2 => R3,
            R3 => R0,
        }
    }
    pub fn decrease(&self) -> Rotation {
        match *self {
            R0 => R3,
            R1 => R0,
            R2 => R1,
            R3 => R2,
        }
    }
}