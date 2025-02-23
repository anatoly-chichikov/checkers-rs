use checkers_rs::markdown::parser::MarkdownRenderer;
use crossterm::style::{Color, ResetColor, SetForegroundColor};

fn strip_color_codes(s: &str) -> String {
    // First remove the reset codes
    let without_reset = s.replace(&format!("{}", ResetColor), "");

    // Then remove all color codes
    without_reset
        .replace(&format!("{}", SetForegroundColor(Color::Magenta)), "")
        .replace(&format!("{}", SetForegroundColor(Color::White)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Cyan)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Yellow)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Green)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Blue)), "")
}

#[test]
fn test_headers() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "# Header 1\n## Header 2\n### Header 3";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert!(plain.contains("HEADER 1"));
    assert!(plain.contains("  HEADER 2"));
    assert!(plain.contains("    HEADER 3"));
    assert!(!plain.contains('#'));
    Ok(())
}

#[test]
fn test_text_emphasis() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "Normal **bold** normal *italic* normal **bold *italic* bold**";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert!(plain.contains("Normal bold normal italic normal bold italic bold"));
    assert!(!plain.contains('*'));

    assert!(result.contains(&format!("{}", SetForegroundColor(Color::Green))));
    assert!(result.contains(&format!("{}", SetForegroundColor(Color::Cyan))));
    Ok(())
}

#[test]
fn test_nested_emphasis() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "**bold *italic in bold* still bold**";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert_eq!(plain, "bold italic in bold still bold");
    assert!(!plain.contains('*'));
    Ok(())
}

#[test]
fn test_code_blocks() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "Inline `code` and:\n```\nmultiline\ncode block\n```";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert!(plain.contains("Inline code and:"));
    assert!(plain.contains("\nmultiline\ncode block\n"));
    assert!(!plain.contains('`'));

    assert!(result.contains(&format!("{}", SetForegroundColor(Color::Green))));
    assert!(result.contains(&format!("{}", SetForegroundColor(Color::Yellow))));
    Ok(())
}

#[test]
fn test_links() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "[Link text](https://example.com) and [another](http://test.com)";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert!(plain.contains("Link text"));
    assert!(plain.contains("another"));
    assert!(!plain.contains('['));
    assert!(!plain.contains(']'));
    assert!(!plain.contains("https://example.com"));
    assert!(!plain.contains("http://test.com"));

    assert!(result.contains(&format!("{}", SetForegroundColor(Color::Blue))));
    Ok(())
}

#[test]
fn test_mixed_formatting() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "# Main Title\n\n**Bold text** with `code` and *italic*\n\n```\ncode block\n```\n\n[Link](url)";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert!(plain.contains("MAIN TITLE"));
    assert!(plain.contains("Bold text with code and italic"));
    assert!(plain.contains("\ncode block\n"));
    assert!(plain.contains("Link"));
    assert!(!plain.contains("url"));
    Ok(())
}

#[test]
fn test_empty_input() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let result = renderer.render("")?;
    assert_eq!(strip_color_codes(&result), "");
    Ok(())
}

#[test]
fn test_newlines() -> std::io::Result<()> {
    let mut renderer = MarkdownRenderer::new();
    let markdown = "Line 1\n\nLine 2\nLine 3";
    let result = renderer.render(markdown)?;
    let plain = strip_color_codes(&result);

    assert_eq!(plain, "Line 1\n\nLine 2\nLine 3");
    Ok(())
}
