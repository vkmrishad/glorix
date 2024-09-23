use nu_ansi_term::{Color, Style};
use pyo3::prelude::*;

/// Parses a style and color from a tag like "bold yellow" or "italic red".
fn parse_style_and_color(tag: &str) -> Style {
    let mut style = Style::new();

    for part in tag.split_whitespace() {
        match part {
            "bold" | "b" => style = style.bold(),
            "dim" | "d" => style = style.dimmed(),
            "italic" | "i" => style = style.italic(),
            "underline" | "u" => style = style.underline(),
            "strike" | "s" => style = style.strikethrough(),
            "reverse" | "r" => style = style.reverse(),
            "blink" => style = style.blink(),
            "reset" => style = Style::new(), // Resets all styles
            _ => {
                // Attempt to parse a color dynamically

                if part.starts_with("on_") {
                    // Handle background color
                    if let Some(bg_color) = parse_dynamic_color(part.trim_start_matches("on_")) {
                        style = style.on(bg_color);
                    }
                } else {
                    // Handle foreground color
                    if let Some(color) = parse_dynamic_color(part) {
                        style = style.fg(color);
                    }
                }
            }
        }
    }

    style
}

/// Parses a color string into a Color enum.
fn parse_dynamic_color(color_str: &str) -> Option<Color> {
    match color_str.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        _ => None, // Return None for unknown colors
    }
}

fn apply_styles(input: &str) -> String {
    let mut formatted_string = String::new();
    let mut current_styles: Vec<Style> = Vec::new(); // Stack of applied styles and strikethrough flag
    let mut i = 0;

    while i < input.len() {
        if input[i..].starts_with("[/") {
            // Handle closing tag (e.g., [/bold yellow])
            if let Some(close_tag_end) = input[i..].find(']') {
                current_styles.pop(); // Remove the last applied style when closing a tag
                i += close_tag_end + 1;
                continue;
            }
        } else if input[i..].starts_with('[') {
            //Handle opening tag (e.g., [bold yellow], [red on red])
            if let Some(tag_end) = input[i..].find(']') {
                let tag = &input[i + 1..i + tag_end]; // Extract the tag
                let parts: Vec<&str> = tag.split_whitespace().collect();

                if tag.contains("on") {
                    current_styles.push(parse_style_and_color(parts[0]));
                    if parts.len() > 1 {
                        let remaining_style = parts[1..].join("_");
                        current_styles.push(parse_style_and_color(&remaining_style));
                    }

                } else {
                    let style = parse_style_and_color(tag);
                    current_styles.push(style); // Push the new style and strikethrough onto the stack
                }
                i += tag_end + 1;
                continue;
            }
        } else {
            // Append characters with the current style
            let mut next_tag = input.len(); // Find the next tag start
            if let Some(tag_start) = input[i..].find('[') {
                next_tag = i + tag_start;
            }

            // Get the current character slice
            let text_slice = &input[i..next_tag];

            // Apply all current styles in the stack
            let mut styled_text = text_slice.to_string();
            for style in &current_styles {
                styled_text = style.paint(&styled_text).to_string();
            }

            formatted_string.push_str(&styled_text);
            i = next_tag;
        }
    }

    formatted_string
}

/// Prints a string with dynamic formatting (e.g., [bold], [italic], [red], etc.)
#[pyfunction]
fn print(string: String) -> PyResult<()> {
    let formatted_string = apply_styles(&string);
    println!("{}", formatted_string); // Print the result with formatting
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn glorix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print, m)?)?;
    Ok(())
}
