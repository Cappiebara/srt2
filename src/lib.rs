mod parser;
mod subtitle;

pub use crate::parser::parse_subs;
pub use crate::subtitle::Subtitle;

#[test]
fn hegelian_dialectics() {
    let srt = r#"1
00:00:00,000 --> 00:00:04,000
How do I put this basically enough?

2
00:00:04,000 --> 00:00:10,000
It's a philosophical theory, the kind you might encounter if you took time to read some books.

3
00:00:10,000 --> 00:00:16,000
The fundamental premise is to envision history as a sequence of "dialectical" conflicts.

4
00:00:16,000 --> 00:00:26,000
Each dialectic begins with a proposition, a thesis, which inherently contains or creates its opposite, an antithesis.

5
00:00:26,000 --> 00:00:29,000
Thesis and antithesis.

6
00:00:29,000 --> 00:00:31,000
The conflict is inevitable.

7
00:00:31,000 --> 00:00:34,000
But the resolution of the conflict yields something new.

8
00:00:34,000 --> 00:00:36,000
A synthesis.

9
00:00:36,000 --> 00:00:38,000
Eliminating the flaws in each.

10
00:00:38,000 --> 00:00:42,000
Leaving behind common elements and ideas."#;

    let timestamp = |h: u32, m: u32, s: u32, ms: u32| {
        (ms & 0x3FF) | ((s & 0x3F) << 10) | ((m & 0x3F) << 16) | ((h & 0xF) << 22)
    };

    let expected = vec![
        Subtitle::new(1, timestamp(0, 0, 0, 0), timestamp(0, 0, 4, 0), "How do I put this basically enough?".to_string()),
        Subtitle::new(2, timestamp(0, 0, 4, 0), timestamp(0, 0, 10, 0), "It's a philosophical theory, the kind you might encounter if you took time to read some books.".to_string()),
        Subtitle::new(3, timestamp(0, 0, 10, 0), timestamp(0, 0, 16, 0), "The fundamental premise is to envision history as a sequence of \"dialectical\" conflicts.".to_string()),
        Subtitle::new(4, timestamp(0, 0, 16, 0), timestamp(0, 0, 26, 0), "Each dialectic begins with a proposition, a thesis, which inherently contains or creates its opposite, an antithesis.".to_string()),
        Subtitle::new(5, timestamp(0, 0, 26, 0), timestamp(0, 0, 29, 0), "Thesis and antithesis.".to_string()),
        Subtitle::new(6, timestamp(0, 0, 29, 0), timestamp(0, 0, 31, 0), "The conflict is inevitable.".to_string()),
        Subtitle::new(7, timestamp(0, 0, 31, 0), timestamp(0, 0, 34, 0), "But the resolution of the conflict yields something new.".to_string()),
        Subtitle::new(8, timestamp(0, 0, 34, 0), timestamp(0, 0, 36, 0), "A synthesis.".to_string()),
        Subtitle::new(9, timestamp(0, 0, 36, 0), timestamp(0, 0, 38, 0), "Eliminating the flaws in each.".to_string()),
        Subtitle::new(10, timestamp(0, 0, 38, 0), timestamp(0, 0, 42, 0), "Leaving behind common elements and ideas.".to_string()),
    ];

    assert_eq!(parse_subs(srt, Vec::new()), expected);
}
