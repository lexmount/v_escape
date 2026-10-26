#![cfg(any(feature = "string", feature = "bytes", feature = "fmt"))]

fn assert_escaped(input: &str, expected: &str) {
    #[cfg(feature = "string")]
    {
        let mut output = String::from("prefix & ");
        v_xmlescape::escape_string(input, &mut output);
        assert_eq!(output, format!("prefix & {expected}"));
    }
    #[cfg(feature = "bytes")]
    {
        let mut output = b"prefix & ".to_vec();
        v_xmlescape::escape_bytes(input, &mut output);
        assert_eq!(output, format!("prefix & {expected}").as_bytes());
    }
    #[cfg(feature = "fmt")]
    assert_eq!(v_xmlescape::escape_fmt(input).to_string(), expected);
}

#[test]
fn xml_text_preserves_quotes_slashes_unicode_and_whitespace() {
    assert_escaped("\"café\" / '尾'\t\n\r 😀", "\"café\" / '尾'\t\n\r 😀");
    assert_escaped("", "");
}

#[test]
fn xml_text_escapes_markup_and_literal_entity_text() {
    assert_escaped("&<>", "&amp;&lt;&gt;");
    assert_escaped("<&amp;>&#x27;", "&lt;&amp;amp;&gt;&amp;#x27;");
    assert_escaped(
        "\"café\" / '尾' <tag>&amp;</tag>",
        "\"café\" / '尾' &lt;tag&gt;&amp;amp;&lt;/tag&gt;",
    );
}

#[test]
fn xml_text_preserves_every_other_ascii_byte() {
    let input: String = (0..=127u8)
        .filter(|byte| !b"&<>".contains(byte))
        .map(char::from)
        .collect();
    assert_escaped(&input, &input);
}

#[test]
fn xml_text_handles_utf8_and_escapes_across_simd_boundaries() {
    const INPUT: &str = "\"café\" / '尾' <&> 😀 &amp;";
    const EXPECTED: &str = "\"café\" / '尾' &lt;&amp;&gt; 😀 &amp;amp;";
    for prefix_len in 0..=64 {
        for suffix_len in [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65] {
            let prefix = "a".repeat(prefix_len);
            let suffix = "z".repeat(suffix_len);
            assert_escaped(
                &format!("{prefix}{INPUT}{suffix}"),
                &format!("{prefix}{EXPECTED}{suffix}"),
            );
        }
    }
}
