use crossterm::{
    cursor::{self, Hide},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};

use crate::interface::input::{read_input, GameInput};

fn clean_section_text(text: &str) -> String {
    let mut result = text.trim();

    // Remove all possible prefixes
    for prefix in &[
        "**Did You Know?**",
        "Did You Know?",
        "**Tip of the Day**",
        "Tip of the Day",
        "**Challenge**",
        "Challenge",
        "###",
    ] {
        if let Some(stripped) = result.strip_prefix(prefix) {
            result = stripped.trim();
        }
    }

    result = result.trim_start_matches('\n');
    result.trim().to_string()
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}

fn get_terminal_center_x(content_width: usize) -> u16 {
    let (cols, _) = terminal::size().unwrap_or((80, 24));
    if cols as usize > content_width {
        ((cols as usize - content_width) / 2) as u16
    } else {
        0
    }
}

fn print_centered_line(stdout: &mut io::Stdout, text: &str, width: usize) -> io::Result<()> {
    let padding = get_terminal_center_x(width);
    for _ in 0..padding {
        write!(stdout, " ")?;
    }
    writeln!(stdout, "{}", text)?;
    Ok(())
}

fn print_ascii_art_header(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.queue(SetForegroundColor(Color::Red))?;

    writeln!(stdout)?;
    print_centered_line(stdout, "╔═╗╦ ╦╔═╗╔═╗╦╔═╔═╗╦═╗╔═╗", 24)?;
    print_centered_line(stdout, "║  ╠═╣║╣ ║  ╠╩╗║╣ ╠╦╝╚═╗", 24)?;
    print_centered_line(stdout, "╚═╝╩ ╩╚═╝╚═╝╩ ╩╚═╝╩╚═╚═╝", 24)?;
    print_centered_line(stdout, "░░░░░░░░░░░░░░░░░░░░░░░░", 24)?;

    stdout.queue(ResetColor)?;
    writeln!(stdout)?;
    Ok(())
}

fn print_did_you_know_section(stdout: &mut io::Stdout, text: &str) -> io::Result<()> {
    const BOX_WIDTH: usize = 65;
    const CONTENT_WIDTH: usize = 61;

    stdout.queue(SetForegroundColor(Color::Blue))?;

    print_centered_line(
        stdout,
        "┌─────────────────────── Did You Know? ─────────────────────────┐",
        BOX_WIDTH,
    )?;

    let lines = wrap_text(text, CONTENT_WIDTH);

    for line in lines {
        let padding = get_terminal_center_x(BOX_WIDTH);
        for _ in 0..padding {
            write!(stdout, " ")?;
        }
        write!(stdout, "│ ")?;
        write!(stdout, "{:<width$}", line, width = CONTENT_WIDTH)?;
        writeln!(stdout, " │")?;
    }

    print_centered_line(
        stdout,
        "└───────────────────────────────────────────────────────────────┘",
        BOX_WIDTH,
    )?;

    stdout.queue(ResetColor)?;
    writeln!(stdout)?;
    Ok(())
}

fn print_tip_section(stdout: &mut io::Stdout, text: &str) -> io::Result<()> {
    const CONTENT_WIDTH: usize = 65;

    print_centered_line(stdout, "💡 Tip of the Day", CONTENT_WIDTH)?;

    stdout.queue(SetForegroundColor(Color::Blue))?;
    print_centered_line(stdout, "════════════════", CONTENT_WIDTH)?;
    stdout.queue(ResetColor)?;

    let lines = wrap_text(text, CONTENT_WIDTH);
    for line in lines {
        print_centered_line(stdout, &line, CONTENT_WIDTH)?;
    }

    writeln!(stdout)?;
    Ok(())
}

fn print_challenge_section(stdout: &mut io::Stdout, text: &str) -> io::Result<()> {
    const CONTENT_WIDTH: usize = 65;

    print_centered_line(stdout, "🎯 Today's Challenge", CONTENT_WIDTH)?;

    stdout.queue(SetForegroundColor(Color::Blue))?;
    print_centered_line(stdout, "═══════════════════", CONTENT_WIDTH)?;
    stdout.queue(ResetColor)?;

    let lines = wrap_text(text, CONTENT_WIDTH);
    for line in lines {
        print_centered_line(stdout, &line, CONTENT_WIDTH)?;
    }

    writeln!(stdout)?;
    Ok(())
}

pub fn wait_for_input() -> io::Result<bool> {
    loop {
        if let Some(input) = read_input()? {
            match input {
                GameInput::Select => return Ok(true), // Enter was pressed
                GameInput::Quit => return Ok(false),  // Q/Esc was pressed
                _ => {}                               // Ignore cursor movements
            }
        }
    }
}

pub fn display_welcome_screen(message: &str) -> io::Result<bool> {
    let mut stdout = stdout();

    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(Hide)?;

    print_ascii_art_header(&mut stdout)?;

    let sections: Vec<&str> = message.split("\n\n").collect();

    let (did_you_know, tip, challenge) = if sections.len() >= 3 {
        let did_you_know = clean_section_text(sections[0]);
        let tip = clean_section_text(sections[1]);
        let challenge = clean_section_text(sections[2]);
        (did_you_know, tip, challenge)
    } else {
        (
            "The game of checkers has been played for thousands of years!".to_string(),
            "Always try to control the center of the board.".to_string(),
            "Try to win a game without losing any pieces!".to_string(),
        )
    };

    print_did_you_know_section(&mut stdout, &did_you_know)?;
    print_tip_section(&mut stdout, &tip)?;
    print_challenge_section(&mut stdout, &challenge)?;

    stdout.queue(SetForegroundColor(Color::DarkGrey))?;
    print_centered_line(&mut stdout, "Press ENTER to begin or Q/ESC to quit...", 65)?;
    stdout.queue(ResetColor)?;

    stdout.flush()?;

    // Enable raw mode to capture key presses without echo
    terminal::enable_raw_mode()?;
    let result = wait_for_input();
    terminal::disable_raw_mode()?;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_short() {
        let text = "This is a short line.";
        let wrapped = wrap_text(text, 30);
        assert_eq!(wrapped, vec!["This is a short line."]);
    }

    #[test]
    fn test_wrap_text_long() {
        let text = "This is a very long line that needs to be wrapped because it exceeds the maximum width.";
        let wrapped = wrap_text(text, 30);
        assert_eq!(wrapped.len(), 3);
        assert!(wrapped[0].len() <= 30);
        assert!(wrapped[1].len() <= 30);
        assert!(wrapped[2].len() <= 30);
    }

    #[test]
    fn test_wrap_text_with_newlines() {
        let text = "First paragraph.\n\nSecond paragraph.";
        let wrapped = wrap_text(text, 30);
        assert_eq!(wrapped.len(), 3);
        assert_eq!(wrapped[0], "First paragraph.");
        assert_eq!(wrapped[1], "");
        assert_eq!(wrapped[2], "Second paragraph.");
    }
}
