use string_width::string_width;

/// Alignment options for text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Options for text alignment
#[derive(Debug, Clone)]
pub struct AlignOptions {
    /// The alignment type (left, center, right)
    pub align: Alignment,
    /// The string to split lines on (default: "\n")
    pub split: String,
    /// The padding character to use (default: " ")
    pub pad: char,
}

impl Default for AlignOptions {
    fn default() -> Self {
        Self {
            align: Alignment::Center,
            split: "\n".to_string(),
            pad: ' ',
        }
    }
}

impl AlignOptions {
    /// Create new options with specified alignment
    pub fn new(align: Alignment) -> Self {
        Self {
            align,
            ..Default::default()
        }
    }

    /// Set the split string
    pub fn split<S: Into<String>>(mut self, split: S) -> Self {
        self.split = split.into();
        self
    }

    /// Set the padding character
    pub fn pad(mut self, pad: char) -> Self {
        self.pad = pad;
        self
    }
}

/// Align text with support for ANSI escape sequences
/// 
/// # Examples
/// 
/// ```
/// use ansi_align::{ansi_align, Alignment, AlignOptions};
/// 
/// // Center align (default)
/// let result = ansi_align("hello\nworld", None);
/// 
/// // Right align
/// let result = ansi_align("hello\nworld", Some(AlignOptions::new(Alignment::Right)));
/// 
/// // Left align with custom padding
/// let opts = AlignOptions::new(Alignment::Left).pad('.');
/// let result = ansi_align("hello\nworld", Some(opts));
/// ```
pub fn ansi_align(text: &str, opts: Option<AlignOptions>) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let opts = opts.unwrap_or_default();
    
    // Short-circuit left alignment as no-op
    if opts.align == Alignment::Left {
        return text.to_string();
    }

    let lines: Vec<&str> = text.split(&opts.split).collect();
    
    // Calculate width for each line and find maximum
    let line_data: Vec<(String, usize)> = lines
        .iter()
        .map(|line| {
            let line_str = line.to_string();
            let width = string_width(&line_str);
            (line_str, width)
        })
        .collect();
    
    let max_width = line_data.iter().map(|(_, width)| *width).max().unwrap_or(0);
    
    // Apply alignment to each line
    let aligned_lines: Vec<String> = line_data
        .into_iter()
        .map(|(line_str, width)| {
            let padding_needed = match opts.align {
                Alignment::Left => 0, // Already handled above
                Alignment::Center => (max_width - width) / 2,
                Alignment::Right => max_width - width,
            };
            
            let padding: String = opts.pad.to_string().repeat(padding_needed);
            format!("{}{}", padding, line_str)
        })
        .collect();
    
    aligned_lines.join(&opts.split)
}

/// Align text to the left (no-op, returns original text)
pub fn left(text: &str) -> String {
    ansi_align(text, Some(AlignOptions::new(Alignment::Left)))
}

/// Align text to the center
pub fn center(text: &str) -> String {
    ansi_align(text, Some(AlignOptions::new(Alignment::Center)))
}

/// Align text to the right
pub fn right(text: &str) -> String {
    ansi_align(text, Some(AlignOptions::new(Alignment::Right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_alignment() {
        let text = "hello\nworld";
        let result = left(text);
        assert_eq!(result, text); // Left alignment should be no-op
    }

    #[test]
    fn test_center_alignment() {
        let text = "hi\nhello";
        let result = center(text);
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines[0], " hi");    // 1 space padding for "hi"
        assert_eq!(lines[1], "hello");  // No padding for "hello"
    }

    #[test]
    fn test_right_alignment() {
        let text = "hi\nhello";
        let result = right(text);
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines[0], "   hi");  // 3 spaces padding for "hi"
        assert_eq!(lines[1], "hello");  // No padding for "hello"
    }

    #[test]
    fn test_unicode_characters() {
        let text = "古\n古古古";
        let result = center(text);
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines[0], "  古");    // 2 spaces padding (CJK char is width 2)
        assert_eq!(lines[1], "古古古");  // No padding
    }

    #[test]
    fn test_ansi_escape_sequences() {
        let text = "hello\n\u{001B}[1mworld\u{001B}[0m";
        let result = center(text);
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines[0], "hello");
        assert_eq!(lines[1], "\u{001B}[1mworld\u{001B}[0m"); // ANSI codes preserved
    }

    #[test]
    fn test_custom_options() {
        let text = "a|bb";
        let opts = AlignOptions::new(Alignment::Right)
            .split("|")
            .pad('.');
        let result = ansi_align(text, Some(opts));
        assert_eq!(result, ".a|bb");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(ansi_align("", None), "");
        assert_eq!(left(""), "");
        assert_eq!(center(""), "");
        assert_eq!(right(""), "");
    }

    #[test]
    fn test_single_line() {
        let text = "hello";
        assert_eq!(left(text), "hello");
        assert_eq!(center(text), "hello");
        assert_eq!(right(text), "hello");
    }
}
