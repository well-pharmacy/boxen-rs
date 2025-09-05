use string_width::{string_width, widest_line};

fn main() {
    println!("String Width Demo");
    println!("=================");

    // Test cases from the JavaScript example
    let test_cases = vec![
        ("a", "ASCII character"),
        ("古", "CJK character (Chinese)"),
        ("\u{001B}[1m古\u{001B}[22m", "CJK with ANSI bold formatting"),
        ("hello", "ASCII string"),
        ("中文测试", "CJK string"),
        ("\u{001B}[31mRed Text\u{001B}[0m", "ASCII with ANSI color"),
        ("Mix古ed", "Mixed ASCII and CJK"),
        (
            "\u{001B}[1m\u{001B}[32mBold Green\u{001B}[0m",
            "Multiple ANSI codes",
        ),
        ("👨‍👩‍👧‍👦", "Emoji family (zero-width joiners)"),
        ("café", "ASCII with combining diacritics"),
    ];

    for (text, description) in test_cases {
        let width = string_width(text);
        println!("{:<40} => {}", description, width);
        println!("  Text: {:?}", text);
        println!("  Visual width: {}", width);
        println!();
    }
    
    println!("\nWidest Line Demo");
    println!("================");
    
    let multiline_tests = vec![
        ("hello\nworld", "Simple two lines"),
        ("short\nlonger line\nhi", "Three lines of different lengths"),
        ("古\n古古古", "Unicode characters"),
        ("Line 1\n\u{001B}[1mBold Line 2\u{001B}[0m\nLine 3", "ANSI formatted line"),
        ("Mix古ed\nASCII only\n中文测试", "Mixed content"),
    ];
    
    for (text, description) in multiline_tests {
        let width = widest_line(text);
        println!("{:<35} => {}", description, width);
        println!("  Lines: {:?}", text.lines().collect::<Vec<_>>());
        println!("  Widest line width: {}", width);
        println!();
    }
}
