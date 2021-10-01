use crossterm::terminal;
use rand::prelude::*;

use crate::{block::Block, game_data::*, mino::Mino, mino_rotation::MinoRotation};

#[derive(Debug)]
pub struct Rustris {
    pub game_data: GameData
}

impl Rustris {
    pub fn new(field_size: (usize, usize)) -> Rustris {
        let game_data = GameData::new(field_size);
        Rustris { game_data }
    }

    pub fn get_next_mino(&mut self) -> Mino {
        if self.game_data.next_minos.is_empty() {
            self.game_data.next_minos = self.game_data.minos.clone();
        }

        let next_mino_index = rand::thread_rng().gen_range(0..self.game_data.next_minos.len());
        let next_mino = self.game_data.next_minos[next_mino_index].clone();
        self.game_data.next_minos.remove(next_mino_index);
        next_mino
    }

    pub fn check_field(&mut self) -> bool {
        match self.try_place_control_mino(Block::Block) {
            Ok(_) => true,
            Err(()) => false
        }
    }

    fn rotate_mino(&mut self, rotation: MinoRotation) -> bool {
        let original_rotation = self.game_data.mino_rotation.clone();
        self.game_data.mino_rotation = rotation;

        if self.check_field() {
            true
        } else {
            self.game_data.mino_rotation = original_rotation;
            false
        }
    }

    pub fn rotate_mino_to_right(&mut self) -> bool {
        self.rotate_mino(MinoRotation::get(self.game_data.mino_rotation.to_count() + 1))
    }

    pub fn rotate_mino_to_left(&mut self) -> bool {
        self.rotate_mino(MinoRotation::get(self.game_data.mino_rotation.to_count() - 1))
    }

    pub fn move_mino(&mut self, x: i32, y: i32) -> bool {
        self.game_data.mino_pos.0 += x;
        self.game_data.mino_pos.1 += y;

        if self.check_field() {
            true
        } else {
            self.game_data.mino_pos.0 -= x;
            self.game_data.mino_pos.1 -= y;
            false
        }
    }

    fn rotate_to_right<T: Clone>(vec: Vec<Vec<T>>) -> Vec<Vec<T>> {
        let x_max = vec.len();
        let y_max = vec[0].len();

        let mut buffer: Vec<Vec<T>> = vec![vec![]; y_max];
        for y in (0..y_max).rev() {
            for x in (0..x_max).rev() {
                buffer[x].push(vec[y][x].clone());
            }
        }

        buffer
    }

    fn try_place_control_mino(&mut self, block_type: Block) -> Result<Vec<Vec<Block>>, ()> {
        if let Some(control_mino) = &self.game_data.control_mino {
            let mut shape = control_mino.shape.clone();
            let offset = self.game_data.mino_pos;

            let rotate_count = self.game_data.mino_rotation.to_count();

            for _ in 0..rotate_count {
                shape = Rustris::rotate_to_right(shape);
            }

            let mut field = self.game_data.field.clone();
            
            for x in 0..shape.len() {
                for y in 0..shape[x].len() {
                    
                    if shape[x][y] {
                        if  (x as i32 + offset.0) < 0 || 
                            (x as i32 + offset.0) > (self.game_data.field_size.1 - 1) as i32 || 
                            (y as i32 + offset.1) < 0 || 
                            (y as i32 + offset.1) > (self.game_data.field_size.0 - 1) as i32 {
                            return Err(());
                        }
                        match field[(x as i32 + offset.0) as usize][(y as i32 + offset.1) as usize] {
                            Block::Block => {
                                return Err(());
                            }

                            Block::Air => {
                                field[(x as i32 + offset.0) as usize][(y as i32 + offset.1) as usize] = block_type;
                            }

                            Block::Ghost => {
                                field[(x as i32 + offset.0) as usize][(y as i32 + offset.1) as usize] = block_type;
                            }
                        }
                    }
                }
            }

            return Ok(field);
        }

        Err(())
    }

    pub fn place_control_mino(&mut self, block_type: Block) -> bool {
        match self.try_place_control_mino(block_type) {
            Ok(field) => {
                self.game_data.field = field;
                true
            }
            Err(()) => {
                false
            }
        }
    }

    pub fn show(&mut self) -> String {
        let original_field = self.game_data.field.clone();
        let original_mino_pos = self.game_data.mino_pos;

        while self.move_mino(1, 0) { }
        self.place_control_mino(Block::Ghost);
        self.game_data.mino_pos = original_mino_pos;
        self.place_control_mino(Block::Block);

        let console_size = terminal::size().expect("Error: Cannot get console size.");

        let mut print_buffer = String::default();

        for blocks in &self.game_data.field {
            print_buffer += &"　".repeat((console_size.0 / 5).into());

            for block in blocks {
                print_buffer += &block.show();
            }
            print_buffer += "\n";
        }

        for _ in 0..(console_size.1 / 6) {
            for _ in 0..console_size.0 {
                print_buffer += " ";
            }
            print_buffer += "\n";
        }

        self.game_data.field = original_field;

        print_buffer
    }
}

#[cfg(test)]
mod rustris_tests {
    use super::*;
    
    #[test]
    fn rotate_test() {
        let rotate_before = 
            vec![
                vec![0, 0, 1],
                vec![1, 1, 0],
                vec![1, 0, 0]
            ];
        let rotate_after = 
            vec![
                vec![1, 1, 0],
                vec![0, 1, 0],
                vec![0, 0, 1]
            ];
        assert_eq!(Rustris::rotate_to_right(rotate_before), rotate_after);
    }

    #[test]
    fn rotate_test2() {
        let rotate_before = 
            vec![
                vec![1, 0, 0],
                vec![1, 1, 1],
                vec![1, 0, 0]
            ];
        let rotate_after = 
            vec![
                vec![1, 1, 1],
                vec![0, 1, 0],
                vec![0, 1, 0]
            ];
        assert_eq!(Rustris::rotate_to_right(rotate_before), rotate_after);
    }

    #[test]
    fn rotate_test3() {
        let rotate_before = 
            vec![
                vec![1, 0, 0],
                vec![1, 1, 1],
                vec![0, 1, 0]
            ];
        let rotate_after = 
            vec![
                vec![0, 1, 1],
                vec![1, 1, 0],
                vec![0, 1, 0]
            ];
        assert_eq!(Rustris::rotate_to_right(rotate_before), rotate_after);
    }
}
