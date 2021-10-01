mod game_data;
mod mino;
mod block;
mod rustris;
mod mino_rotation;
use std::{io::stdout, sync::{Arc, Mutex}, thread, time::Duration};

use crossterm::{cursor, event::{Event, KeyCode, KeyEvent, KeyModifiers, read}, execute, style::{Print}, terminal::{self, Clear, ClearType, enable_raw_mode}};

use crate::{block::Block, rustris::Rustris};

fn main() {
    let stdout = Arc::new(Mutex::new(stdout()));
    enable_raw_mode().unwrap();

    {
        let mut stdout = stdout.lock().unwrap();
        execute!(
            stdout, 
            cursor::MoveTo(0, 0),
            cursor::Hide,
            Clear(ClearType::All)
        ).unwrap();
    }

    let console_size = terminal::size().expect("Error: Cannot get console size.");
    for _ in 0..console_size.1 {
        for _ in 0..console_size.0 {
            print!(" ");
        }
        println!();
    }

    let rustris = Arc::new(Mutex::new(Rustris::new((10, 21))));

    {
        let mut rustris = rustris.lock().unwrap(); 
        rustris.game_data.control_mino = Some(rustris.get_next_mino());
    }

    let rustris_rc = Arc::clone(&rustris);
    let stdout_rc = Arc::clone(&stdout);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            {
                let mut rustris_rc = rustris_rc.lock().unwrap();
                if !rustris_rc.move_mino(1, 0) {
                    rustris_rc.place_control_mino(Block::Block);
                    rustris_rc.game_data.control_mino = Some(rustris_rc.get_next_mino());
                    rustris_rc.game_data.mino_pos = (0, 5);
                }
            }
            {
                let mut stdout = stdout_rc.lock().unwrap();
                let buf = rustris_rc.lock().unwrap().show();
                execute!(stdout, Print(buf)).unwrap()
            }
        }
    });

    loop {
        {
            let mut rustris = rustris.lock().unwrap();
            let mut stdout = stdout.lock().unwrap();
            let buf = rustris.show();
            execute!(stdout, Print(buf)).unwrap();
        }
        
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.move_mino(0, 1);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.move_mino(0, -1);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.move_mino(1, 0);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                while rustris.move_mino(1, 0) { }

                rustris.place_control_mino(Block::Block);
                rustris.game_data.control_mino = Some(rustris.get_next_mino());
                rustris.game_data.mino_pos = (0, 5);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.rotate_mino_to_right();
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.rotate_mino_to_left();
            }


            Event::Key(KeyEvent { 
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE
            }) => {
                break;
            }

            _ => ()
        }


        {
            let mut rustris = rustris.lock().unwrap();
            for x in 0..(rustris.game_data.field_size.1) {
                let mut air = false;
                for y in 0..(rustris.game_data.field_size.0) {
                    if let Block::Air = rustris.game_data.field[x][y] {
                        air = true;
                    }
                }
                
                if !air {
                    for y in 0..(rustris.game_data.field_size.0) {
                        rustris.game_data.field[x][y] = Block::Air;
                    }
    
                    for x2 in (0..(rustris.game_data.field_size.1)).rev() {
                        if x2 < x && x2 < 20 {
                            for y2 in 0..(rustris.game_data.field_size.0) {
                                rustris.game_data.field[x2 + 1][y2] = rustris.game_data.field[x2][y2];
                                rustris.game_data.field[x2][y2] = Block::Air;
                            }
                        }
                    }
                }
            }
        }
    }

    {
        let mut stdout = stdout.lock().unwrap();
        execute!(
            stdout, 
            cursor::MoveTo(0, 0),
            cursor::Show,
            Clear(ClearType::All)
        ).unwrap();
    }
}

#[cfg(test)]
mod tests {
    
}
