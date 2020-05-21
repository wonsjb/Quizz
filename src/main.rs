
use std::{thread, time};
use console::{Key, Term};
use std::io;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;

mod moving_words;
mod moving_word;
mod quizz;

use moving_words::MovingWords;
use quizz::Quizz;

fn spawn_stdin_channel() -> Receiver<Key> {
    let (tx, rx) = mpsc::channel::<Key>();
    thread::spawn(move ||  {
        let term = Term::stdout();
        loop {
            match term.read_key() {
                Ok(key) => tx.send(key).expect("Could not set key"),
                Err(_) => ()
            }
        }
    });
    rx
}

fn main() -> io::Result<()> {
    let terminal = Term::stdout();
    terminal.clear_screen()?;
    terminal.hide_cursor()?;
    
    let sleep_time = time::Duration::from_millis(150);
    let quizz = Quizz::parse("quizz/geek_quizz.yaml");
    let keys = spawn_stdin_channel();

    let mut words = MovingWords::new(quizz.get_texts());

    loop {
        words.moving.advance(&terminal);
        terminal.clear_screen()?;
        words.moving.print(&terminal)?;
        terminal.move_cursor_to(0, 0)?;
        terminal.flush()?;
        thread::sleep(sleep_time);
        match keys.try_recv() {
            Ok(key) => match key {
                Key::Escape => break,
                Key::ArrowLeft => words.prev(),
                Key::ArrowRight => words.next(),
                Key::Char('q') => break,
                Key::Char('Q') => break,
                _ => ()
            },
            Err(_) => ()
        }
    }

    terminal.show_cursor()?;
    terminal.clear_screen()?;
    terminal.move_cursor_to(0, 0)?;
    terminal.flush()?;

    Ok(())
}