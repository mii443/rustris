mod game_data;
mod mino;
mod block;
mod rustris;
mod mino_rotation;
mod game_status;
mod super_rotation;
use std::{io::stdout, process::exit, sync::{Arc, Mutex}, thread, time::Duration};

use crossterm::{cursor, event::{Event, KeyCode, KeyEvent, KeyModifiers, read}, execute, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{self, Clear, ClearType, enable_raw_mode}};
use game_data::GameData;
use game_status::GameStatus;
use mino_rotation::MinoRotation;

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

    let rustris = Arc::new(Mutex::new(
        Rustris::new(
            GameData::new((10, 21), true)
        )
    ));

    {
        let mut select = 0;
        let mut rustris = rustris.lock().unwrap(); 

        loop {
            let mut stdout = stdout.lock().unwrap();

            execute!(
                stdout, 
                cursor::MoveTo(0, 0),
                cursor::Hide,
                Clear(ClearType::All)
            ).unwrap();

            if select == 0 {
                print!("> ");
            }
            println!("Play");

            if select == 1 {
                print!("> ");
            }
            println!("Ghost {}", 
                if rustris.game_data.show_ghost {
                    "ON"
                } else {
                    "OFF"
                }
            );

            if select == 2 {
                print!("> ");
            }
            println!("Quit");

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE
                }) => {
                    if select > 0 {
                        select -= 1;
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE
                }) => {
                    if select < 2 {
                        select += 1;
                    }
                }
    
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE
                }) => {
                    match select {
                        0 => {
                            break;
                        }
                        1 => {
                            rustris.game_data.show_ghost = !rustris.game_data.show_ghost;
                        }
                        2 => {
                            std::process::exit(0);
                        }
                        _ => ()
                    }
                }

                _ => ()
            }
        }
    }

    {
        let mut stdout = stdout.lock().unwrap();
        execute!(
            stdout, 
            cursor::MoveTo(0, 0),
            cursor::Hide,
            Clear(ClearType::All)
        ).unwrap();
    }

    {
        let mut rustris = rustris.lock().unwrap(); 
        rustris.game_data.control_mino = Some(rustris.get_next_mino());
    }

    let rustris_rc = Arc::clone(&rustris);
    let stdout_rc = Arc::clone(&stdout);
    let exit_flag = Arc::new(Mutex::new(false));
    let exit_flag_rc = Arc::clone(&exit_flag);
    let control_count = Arc::new(Mutex::new(0));
    let control_count_rc = Arc::clone(&control_count);
    let frame_thread = thread::spawn(move || {
        let mut ground_flag = false;
        let mut before_control_count = 0;
        loop {
            if *exit_flag_rc.lock().unwrap() {
                break;
            }
            thread::sleep(Duration::from_secs_f32(0.5));
            {
                let mut rustris_rc = rustris_rc.lock().unwrap();
                if !rustris_rc.move_mino(1, 0) {
                    if ground_flag {
                        let control_count = *control_count_rc.lock().unwrap();
                        if before_control_count < control_count && control_count < 15 {
                            before_control_count = control_count.clone();
                            ground_flag = false;
                        } else {
                            rustris_rc.place_control_mino(Block::Block);
                            rustris_rc.next_mino();
                            ground_flag = false;
                            *control_count_rc.lock().unwrap() = 0;
                            before_control_count = 0;
                        }
                    } else {
                        ground_flag = true;
                    }
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
                *control_count.lock().unwrap() += 1;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.move_mino(0, -1);
                *control_count.lock().unwrap() += 1;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.move_mino(1, 0);
                *control_count.lock().unwrap() += 1;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                while rustris.move_mino(1, 0) { }

                rustris.place_control_mino(Block::Block);
                rustris.next_mino();
                *control_count.lock().unwrap() += 1;
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                if let Some(holding) = rustris.game_data.hold_mino.clone() {
                    let control_tmp = rustris.game_data.control_mino.clone();
                    rustris.game_data.control_mino = Some(holding);
                    rustris.game_data.hold_mino = control_tmp;
                    rustris.game_data.mino_pos = (0, 3);
                    rustris.game_data.mino_rotation = MinoRotation::Up;
                } else {
                    rustris.game_data.hold_mino = rustris.game_data.control_mino.clone();
                    rustris.next_mino();
                }
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::NONE
            }) => {
                let mut rustris = rustris.lock().unwrap();
                rustris.rotate_mino_to_right();
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
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

        {
            let mut rustris = rustris.lock().unwrap();
            if let GameStatus::Gameover = rustris.game_status {
                let mut stdout = stdout.lock().unwrap();
                execute!(
                    stdout, 
                    cursor::MoveTo(0, 0),
                    cursor::Show,
                    Clear(ClearType::All),
                    SetBackgroundColor(Color::Red),
                ).unwrap();
                
                let console_size = terminal::size().expect("Error: Cannot get console size.");

                execute!(
                    stdout,
                    cursor::MoveTo(console_size.0 / 2, console_size.1 / 2),
                    Print("Game Over")
                ).unwrap();

                thread::sleep(Duration::from_secs(3));

                execute!(
                    stdout, 
                    cursor::MoveTo(0, 0),
                    cursor::Show,
                    Clear(ClearType::All),
                    ResetColor
                ).unwrap();
                break;
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

    *exit_flag.lock().unwrap() = true;
    frame_thread.join();
    main();
}

#[cfg(test)]
mod tests {
    
}
