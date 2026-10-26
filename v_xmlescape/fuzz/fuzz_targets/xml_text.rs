#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let mut expected = String::new();
    for character in data.chars() {
        match character {
            '&' => expected.push_str("&amp;"),
            '<' => expected.push_str("&lt;"),
            '>' => expected.push_str("&gt;"),
            _ => expected.push(character),
        }
    }

    let mut output = String::new();
    v_xmlescape::escape_string(data, &mut output);
    assert_eq!(output, expected);
    assert_eq!(v_xmlescape::escape_fmt(data).to_string(), expected);

    let mut bytes = Vec::new();
    v_xmlescape::escape_bytes(data, &mut bytes);
    assert_eq!(bytes, expected.as_bytes());
});
