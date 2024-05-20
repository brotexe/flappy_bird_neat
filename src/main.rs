//mod graphics;
//mod logic;

use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;

const GRAVITY_ACC: f32 = 0.1;
const JUMP_ACC: f32 = 1.0;

struct Bird {
    pos_y: f32,
    vec_y: f32,
    char: char,
}

impl Bird {
    fn update_char(&mut self) {
        if self.vec_y > 0.2 {
            self.char = '\\';
        } else if self.vec_y < -0.2 {
            self.char = '/';
        } else {
            self.char = '－';
        }
    }

    fn update_pos(&mut self, height_min: f32, height_max: f32) {
        self.pos_y += self.vec_y;
        self.pos_y = self.pos_y.min(height_max);
        self.pos_y = self.pos_y.max(height_min);
    }
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let (_width, height) = termion::terminal_size().unwrap();

    let mut bird = Bird {
        pos_y: height as f32 / 2.0,
        vec_y: 0.0,
        char: '-',
    };

    let max_fps = 20.0;
    let frame_time = Duration::from_secs_f32(1.0 / max_fps);

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    loop {
        bird.vec_y += GRAVITY_ACC;

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        } else if let Some(Ok(b' ')) = b {
            bird.vec_y = -JUMP_ACC;
        }

        bird.update_pos(0.0, height as f32);
        bird.update_char();

        // Clear screen
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();

        // Print bird
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(10, bird.pos_y.round() as u16),
            bird.char,
        )
        .unwrap();
        stdout.flush().unwrap();

        thread::sleep(frame_time);
    }
}
