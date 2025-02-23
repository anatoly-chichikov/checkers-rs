use checkers_rs::markdown_renderer::MarkdownRenderer;
use crossterm::style::{Color, ResetColor, SetForegroundColor};

fn strip_color_codes(s: &str) -> String {
    println!("Before stripping: {:?}", s);

    // First remove the reset codes
    let without_reset = s.replace(&format!("{}", ResetColor), "");

    // Then remove all color codes
    let result = without_reset
        .replace(&format!("{}", SetForegroundColor(Color::Magenta)), "")
        .replace(&format!("{}", SetForegroundColor(Color::White)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Cyan)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Yellow)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Green)), "")
        .replace(&format!("{}", SetForegroundColor(Color::Blue)), "")
        .replace(
            &format!(
                "{}",
                SetForegroundColor(Color::Rgb {
                    r: 173,
                    g: 255,
                    b: 47
                })
            ),
            "",
        );

    println!("After stripping: {:?}", result);
    result
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
