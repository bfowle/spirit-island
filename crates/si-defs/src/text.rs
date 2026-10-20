//! The engine's text markup (`{Dahan}`, `{BR}`, `{Bullet}` …) to plain words.

pub fn plain(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('{') {
        out.push_str(&rest[..i]);
        let Some(j) = rest[i..].find('}') else {
            out.push_str(&rest[i..]);
            return out;
        };
        let tag = &rest[i + 1..i + j];
        out.push_str(match tag {
            "BR" => "\n",
            "Bullet" => "- ",
            "EndBullet" => "",
            "Fast" => "Fast",
            "Slow" => "Slow",
            "Or" => "or",
            "1Range" => "Range 1",
            "+1Range" => "+1 Range",
            t if t.ends_with("Left") => &t[..t.len() - 4],
            t => t,
        });
        rest = &rest[i + j + 1..];
    }
    out.push_str(rest);
    out.replace("<nobr>", "").replace("</nobr>", "")
}

#[cfg(test)]
mod tests {
    #[test]
    fn markup_becomes_words() {
        assert_eq!(super::plain("Push up to 4 {Dahan}. {Or} If Invaders are present, 2 {Fear}."), "Push up to 4 Dahan. or If Invaders are present, 2 Fear.");
        assert_eq!(super::plain("a{BR}{Bullet}b{EndBullet}"), "a\n- b");
    }
}
