use crate::{block::Block, mino::{self, Mino}, mino_rotation::MinoRotation};

#[derive(PartialEq, Debug, Clone, Copy)]
enum RotateDirection {
    Right,
    Left,
}

pub struct SuperRotation {
    pub field: Vec<Vec<Block>>,
    pub target: Mino,
    pub mino_position: (i32, i32),
    pub origin_position: (i32, i32),
    pub field_size: (usize, usize),
    pub now_rotation: MinoRotation,
    pub origin_rotation: MinoRotation,
    pub next_rotation: MinoRotation,
    rotate_direction: RotateDirection,
}

impl SuperRotation {
    pub fn new(field: Vec<Vec<Block>>, target: Mino, mino_position: (i32, i32), field_size: (usize, usize), now_rotation: MinoRotation, next_rotation: MinoRotation) -> SuperRotation {
        SuperRotation {
            field, 
            target,
            mino_position, 
            origin_position: mino_position.clone(),
            field_size,
            now_rotation,
            origin_rotation: now_rotation.clone(),
            next_rotation,
            rotate_direction: SuperRotation::get_rotate_direction(now_rotation.clone(), now_rotation.clone())
        }
    }

    pub fn rotate(&mut self) -> bool {
        if self.target.id == "I" {
            if self.step0() {
                return true;
            }
            if self.i_step1() {
                return true;
            }
            if self.i_step2() {
                return true;
            }
            if self.i_step3() {
                return true;
            }
            if self.i_step4() {
                return true;
            }
        } else {
            if self.step0() {
                return true;
            }
            if self.step1() {
                return true;
            }
            if self.step2() {
                return true;
            }
            if self.step3() {
                return true;
            }
            if self.step4() {
                return true;
            }
        }

        false
    }

    fn i_step1(&mut self) -> bool {
        match self.origin_rotation {
            MinoRotation::Up => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_left();
                    }
                    MinoRotation::Right => {
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Right => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_right();
                        self.move_to_right();
                    }
                    MinoRotation::Down => {
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Down => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_right();
                        self.move_to_right();
                    }
                    MinoRotation::Right => {
                        self.move_to_right();
                    }
                    _ => ()
                }
            }

            MinoRotation::Left => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_left();
                        self.move_to_left();
                    }
                    MinoRotation::Down => {
                        self.move_to_right();
                    }
                    _ => ()
                }
            }
        }

        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn i_step2(&mut self) -> bool {
        self.step0();
        
        match self.origin_rotation {
            MinoRotation::Up => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_right();
                        self.move_to_right();
                    }
                    MinoRotation::Right => {
                        self.move_to_right();
                    }
                    _ => ()
                }
            }

            MinoRotation::Right => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_left();
                    }
                    MinoRotation::Down => {
                        self.move_to_right();
                        self.move_to_right();
                    }
                    _ => ()
                }
            }

            MinoRotation::Down => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_left();
                    }
                    MinoRotation::Right => {
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Left => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_right();
                    }
                    MinoRotation::Down => {
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }
        }
        
        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn i_step3(&mut self) -> bool {
        self.step0();
        
        match self.origin_rotation {
            MinoRotation::Up => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_right();
                        self.move_to_up();
                        self.move_to_up();
                    }
                    MinoRotation::Right => {
                        self.move_to_down();
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Right => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_right();
                        self.move_to_right();
                        self.move_to_up();
                    }
                    MinoRotation::Down => {
                        self.move_to_up();
                        self.move_to_up();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Down => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_up();
                        self.move_to_right();
                        self.move_to_right();
                    }
                    MinoRotation::Right => {
                        self.move_to_right();
                        self.move_to_down();
                        self.move_to_down();
                    }
                    _ => ()
                }
            }

            MinoRotation::Left => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_down();
                        self.move_to_down();
                        self.move_to_right();
                    }
                    MinoRotation::Down => {
                        self.move_to_down();
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }
        }
        
        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn i_step4(&mut self) -> bool {
        self.step0();
        
        match self.origin_rotation {
            MinoRotation::Up => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_down();
                        self.move_to_left();
                        self.move_to_left();
                    }
                    MinoRotation::Right => {
                        self.move_to_right();
                        self.move_to_up();
                        self.move_to_up();
                    }
                    _ => ()
                }
            }

            MinoRotation::Right => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_down();
                        self.move_to_down();
                        self.move_to_left();
                    }
                    MinoRotation::Down => {
                        self.move_to_right();
                        self.move_to_right();
                        self.move_to_down();
                    }
                    _ => ()
                }
            }

            MinoRotation::Down => {
                match self.now_rotation {
                    MinoRotation::Left => {
                        self.move_to_left();
                        self.move_to_down();
                        self.move_to_down();
                    }
                    MinoRotation::Right => {
                        self.move_to_up();
                        self.move_to_left();
                        self.move_to_left();
                    }
                    _ => ()
                }
            }

            MinoRotation::Left => {
                match self.now_rotation {
                    MinoRotation::Up => {
                        self.move_to_left();
                        self.move_to_left();
                        self.move_to_up();
                    }
                    MinoRotation::Down => {
                        self.move_to_right();
                        self.move_to_up();
                        self.move_to_up();
                    }
                    _ => ()
                }
            }
        }
        
        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn step0(&mut self) -> bool {
        self.now_rotation = self.next_rotation.clone();
        self.mino_position = self.origin_position.clone();
        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn step1(&mut self) -> bool {
        let rotate_direction = self.rotate_direction;
        match self.now_rotation {
            MinoRotation::Right => {
                self.move_to_left();
            }
            MinoRotation::Left => {
                self.move_to_right();
            }
            _ => {
                if self.origin_rotation == MinoRotation::Left {
                    self.move_to_left();
                } else {
                    self.move_to_right();
                }
            }
        }

        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn step2(&mut self) -> bool {
        if self.now_rotation == MinoRotation::Right || self.now_rotation == MinoRotation::Left {
            self.move_to_up();
        } else {
            self.move_to_down();
        }

        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn step3(&mut self) -> bool {
        self.now_rotation = self.next_rotation.clone();
        self.mino_position = self.origin_position.clone();

        if self.now_rotation == MinoRotation::Right || self.now_rotation == MinoRotation::Left {
            self.move_to_down();
            self.move_to_down();
        } else {
            self.move_to_up();
            self.move_to_up();
        }

        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn step4(&mut self) -> bool {
        let rotate_direction = self.rotate_direction;
        match self.now_rotation {
            MinoRotation::Right => {
                self.move_to_left();
            }
            MinoRotation::Left => {
                self.move_to_right();
            }
            _ => {
                if rotate_direction == RotateDirection::Right {
                    self.move_to_left();
                } else {
                    self.move_to_right();
                }
            }
        }

        if self.check_duplicate() {
            return true;
        }

        false
    }

    fn move_to_right(&mut self) {
        self.mino_position.1 += 1;
    }
    fn move_to_left(&mut self) {
        self.mino_position.1 -= 1;
    }
    fn move_to_up(&mut self) {
        self.mino_position.0 -= 1;
    }
    fn move_to_down(&mut self) {
        self.mino_position.0 += 1;
    }

    fn check_duplicate(&mut self) -> bool {
        let shape = self.target.shape.clone();
        let pos = self.mino_position;
        let field = self.field.clone();
        let field_size = self.field_size;
        for x in 0..shape.len() {
            for y in 0..shape[x].len() {
                if shape[x][y] {
                    if  (x as i32 + pos.0) < 0 || 
                        (x as i32 + pos.0) > (field_size.1 - 1) as i32 || 
                        (y as i32 + pos.1) < 0 || 
                        (y as i32 + pos.1) > (field_size.0 - 1) as i32 {
                        return false;
                    }
                    match field[(x as i32 + pos.0) as usize][(y as i32 + pos.1) as usize] {
                        Block::Block => {
                            return false;
                        }
                        _ => ()
                    }
                }
            }
        }

        true
    }

    fn get_rotate_direction(before: MinoRotation, after: MinoRotation) -> RotateDirection {
        match before {
            MinoRotation::Up => {
                if after == MinoRotation::Right {
                    RotateDirection::Right
                } else {
                    RotateDirection::Left
                }
            }

            MinoRotation::Right => {
                if after == MinoRotation::Down {
                    RotateDirection::Right
                } else {
                    RotateDirection::Left
                }
            }

            MinoRotation::Down => {
                if after == MinoRotation::Left {
                    RotateDirection::Right
                } else {
                    RotateDirection::Left
                }
            }

            MinoRotation::Left => {
                if after == MinoRotation::Up {
                    RotateDirection::Right
                } else {
                    RotateDirection::Left
                }
            }
        }
    }
}

#[cfg(test)]
mod super_rotation_test {
    use super::*;

    #[test]
    fn get_rotate_direction_test1() {
        assert_eq!(SuperRotation::get_rotate_direction(MinoRotation::Up, MinoRotation::Right), RotateDirection::Right);
    }

    #[test]
    fn get_rotate_direction_test2() {
        assert_eq!(SuperRotation::get_rotate_direction(MinoRotation::Down, MinoRotation::Right), RotateDirection::Left);
    }

    #[test]
    fn get_rotate_direction_test3() {
        assert_eq!(SuperRotation::get_rotate_direction(MinoRotation::Left, MinoRotation::Up), RotateDirection::Right);
    }

    #[test]
    fn get_rotate_direction_test4() {
        assert_eq!(SuperRotation::get_rotate_direction(MinoRotation::Right, MinoRotation::Up), RotateDirection::Left);
    }
}