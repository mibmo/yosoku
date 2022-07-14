use crossterm::{
    cursor::{MoveLeft, MoveRight},
    event::{self, KeyCode},
    queue,
    style::{Attribute, Color, Print, PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};
use unicode_segmentation::UnicodeSegmentation;
//use yosoku::;

type EResult<T> = Result<T, Box<dyn std::error::Error>>;

struct TermReader;

impl Iterator for TermReader {
    type Item = event::KeyCode;

    fn next(&mut self) -> Option<Self::Item> {
        match event::read().ok()? {
            event::Event::Key(key) => Some(key.code),
            _ => None,
        }
    }
}

fn interactive_input() -> EResult<String> {
    fn predict_next(/*predictor: &Predictor,*/ input: &str) -> Option<String> {
        let words = input.unicode_words();
        let last_word = words.last().unwrap_or("ERROR");

        // mock prediction. @TODO: use actual predictor
        match last_word {
            "never" => Some("gonna"),
            "gonna" => Some("give"),
            "give" => Some("you"),
            "you" => Some("up"),
            _ => None,
        }
        .map(|s| s.to_owned())
    }

    fn show_prediction(stdout: &mut Stdout, input: &mut String, next_word: &str) -> EResult<()> {
        let styled = next_word.with(Color::DarkGrey).attribute(Attribute::Italic);

        let mut move_len = next_word.len() as u16;

        if input.chars().last() != Some(' ') {
            move_len += 1;
            queue!(stdout, MoveRight(1))?;
        }

        queue!(stdout, PrintStyledContent(styled), MoveLeft(move_len))?;

        Ok(())
    }

    let raw_was_enabled = terminal::is_raw_mode_enabled()?;
    terminal::enable_raw_mode()?;

    let mut stdout = stdout();
    let mut input = String::new();

    for key in TermReader {
        let mut autocomplete = false;

        match key {
            KeyCode::Esc => break,
            KeyCode::Enter => return Ok(input),
            KeyCode::Tab => {
                autocomplete = true;
            }
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Char(ch) => {
                input.push(ch);
            }
            _ => {}
        };

        queue!(
            stdout,
            MoveLeft(u16::MAX),
            Clear(ClearType::CurrentLine),
            Print(&input)
        )?;

        // could be done with a loop; code just has to run twice
        if let Some(next_word) = predict_next(&input) {
            if autocomplete {
                if input.chars().last() != Some(' ') {
                    input.push(' ');
                    queue!(stdout, MoveRight(1))?;
                }

                input.push_str(&next_word);
                queue!(stdout, Print(&next_word))?;

                // show next completion too
                if let Some(next_word) = predict_next(&input) {
                    show_prediction(&mut stdout, &mut input, &next_word)?;
                }
            } else {
                show_prediction(&mut stdout, &mut input, &next_word)?;
            }
        }

        stdout.flush()?;
    }

    // @TODO: this doesn't seem to work correctly; terminal is still borked upon exit
    // (even if run unconditionally in main! maybe an issue with crossterm?)
    if !raw_was_enabled {
        terminal::disable_raw_mode()?;
    }

    return Ok(String::new());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = interactive_input()?;
    println!("Output: {text:?}");

    Ok(())
}
