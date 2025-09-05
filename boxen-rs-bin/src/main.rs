use string_width::string_width;

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
}
