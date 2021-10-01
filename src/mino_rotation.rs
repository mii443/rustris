#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MinoRotation {
    Up,
    Right,
    Down,
    Left
}

impl MinoRotation {
    pub fn to_count(&self) -> i32 {
        match self {
            MinoRotation::Up => 0,
            MinoRotation::Right => 1,
            MinoRotation::Down => 2,
            MinoRotation::Left => 3
        }
    }

    pub fn get(rotation: i32) -> MinoRotation {
        let mut rotation = rotation;
        if rotation < 0 {
            rotation += 4;
        }
        rotation %= 4;

        match rotation {
            0 => MinoRotation::Up,
            1 => MinoRotation::Right,
            2 => MinoRotation::Down,
            3 => MinoRotation::Left,
            _ => MinoRotation::Up
        }
    }
}

#[cfg(test)]
mod mino_rotation_test {
    use super::*;

    #[test]
    fn convert_rotate_test() {
        assert_eq!(MinoRotation::get(-1), MinoRotation::Left);
    }

    #[test]
    fn convert_rotate_test1() {
        assert_eq!(MinoRotation::get(0), MinoRotation::Up);
    }

    #[test]
    fn convert_rotate_test2() {
        assert_eq!(MinoRotation::get(1), MinoRotation::Right);
    }

    #[test]
    fn convert_rotate_test3() {
        assert_eq!(MinoRotation::get(2), MinoRotation::Down);
    }

    #[test]
    fn convert_rotate_test4() {
        assert_eq!(MinoRotation::get(3), MinoRotation::Left);
    }

    #[test]
    fn convert_rotate_test5() {
        assert_eq!(MinoRotation::get(4), MinoRotation::Up);
    }
}