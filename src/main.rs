use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::raw::IntoRawMode;

struct Bird {
    y: f32,
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let (_width, height) = termion::terminal_size().unwrap();

    let mut bird = Bird {
        y: height as f32 / 2.0,
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
        write!(
            stdout,
            "{}O",
            termion::cursor::Goto(10, bird.y.round() as u16)
        )
        .unwrap();

        if bird.y < height as f32 {
            bird.y += 0.3;
        }

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        } else if let Some(Ok(b' ')) = b {
            if bird.y - 10.0 > 0.0 {
                bird.y -= 10.0;
            }
        }

        stdout.flush().unwrap();

        thread::sleep(frame_time);
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();
    }
}
