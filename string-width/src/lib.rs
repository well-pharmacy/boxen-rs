/// Calculate the display width of a string, handling Unicode characters and ANSI escape sequences.
///
/// This function strips ANSI escape sequences and then calculates the visual width
/// of the remaining text, taking into account that some Unicode characters (like CJK)
/// take up more than one column in a terminal.
///
/// # Examples
///
/// ```
/// use string_width::string_width;
///
/// assert_eq!(string_width("a"), 1);
/// assert_eq!(string_width("古"), 2);
/// assert_eq!(string_width("\u{001B}[1m古\u{001B}[22m"), 2);
/// ```
pub fn string_width(s: &str) -> usize {
    // Strip ANSI escape sequences first
    let stripped = strip_ansi_escapes::strip(s);
    let clean_str = std::str::from_utf8(&stripped).unwrap_or("");

    // Calculate Unicode width, treating control characters as width 0
    clean_str
        .chars()
        .map(|c| {
            if c.is_control() {
                0
            } else {
                unicode_width::UnicodeWidthChar::width(c).unwrap_or(0)
            }
        })
        .sum()
}

/// Find the width of the widest line in a multi-line string.
/// 
/// This function splits the input string by newlines and returns the width
/// of the line that has the greatest display width.
/// 
/// # Examples
/// 
/// ```
/// use string_width::widest_line;
/// 
/// assert_eq!(widest_line("hello\nworld"), 5);
/// assert_eq!(widest_line("short\nlonger line\nhi"), 11);
/// assert_eq!(widest_line("古\n古古古"), 6);
/// ```
pub fn widest_line(s: &str) -> usize {
    s.lines()
        .map(string_width)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_characters() {
        assert_eq!(string_width("a"), 1);
        assert_eq!(string_width("hello"), 5);
        assert_eq!(string_width(""), 0);
    }

    #[test]
    fn test_unicode_characters() {
        // CJK characters should have width 2
        assert_eq!(string_width("古"), 2);
        assert_eq!(string_width("中文"), 4);
        assert_eq!(string_width("こんにちは"), 10);

        // Combining characters should have width 0
        assert_eq!(string_width("é"), 1); // e + combining acute accent
    }

    #[test]
    fn test_ansi_escape_sequences() {
        // Bold text with ANSI codes
        assert_eq!(string_width("\u{001B}[1m古\u{001B}[22m"), 2);
        assert_eq!(string_width("\u{001B}[31mhello\u{001B}[0m"), 5);

        // Color codes
        assert_eq!(string_width("\u{001B}[32mgreen\u{001B}[0m"), 5);

        // Multiple ANSI sequences
        assert_eq!(
            string_width("\u{001B}[1m\u{001B}[31mbold red\u{001B}[0m"),
            8
        );
    }

    #[test]
    fn test_mixed_content() {
        // Mix of ASCII, Unicode, and ANSI
        assert_eq!(string_width("hello古world"), 12); // 5 + 2 + 5
        assert_eq!(string_width("\u{001B}[1mhello古\u{001B}[0mworld"), 12);
    }

    #[test]
    fn test_control_characters() {
        // Tab and newline should have width 0 after ANSI stripping
        assert_eq!(string_width("hello\tworld"), 10);
        assert_eq!(string_width("hello\nworld"), 10);
    }

    #[test]
    fn test_zero_width_characters() {
        // Zero-width space
        assert_eq!(string_width("hello\u{200B}world"), 10);

        // Zero-width joiner (note: complex emoji sequences are counted as individual emojis)
        // This is a limitation - proper grapheme cluster handling would require additional dependencies
        assert_eq!(string_width("👨\u{200D}👩\u{200D}👧\u{200D}👦"), 8); // 4 emojis × 2 width each

        // Simple zero-width joiner test
        assert_eq!(string_width("a\u{200D}b"), 2); // ZWJ between letters has no visual effect
    }

    #[test]
    fn test_widest_line() {
        // Basic multi-line test
        assert_eq!(widest_line("hello\nworld"), 5);
        assert_eq!(widest_line("short\nlonger line\nhi"), 11);
        
        // Unicode characters
        assert_eq!(widest_line("古\n古古古"), 6);
        
        // Mixed content with ANSI codes
        assert_eq!(widest_line("hello\n\u{001B}[1mworld\u{001B}[0m\ntest"), 5);
        
        // Empty string and single line
        assert_eq!(widest_line(""), 0);
        assert_eq!(widest_line("single"), 6);
        
        // Lines with different types of content
        assert_eq!(widest_line("ascii\n古文字\n\u{001B}[32mcolored\u{001B}[0m"), 7);
    }
}
