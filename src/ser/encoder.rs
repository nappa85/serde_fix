
pub struct Encoder {
    inner: Vec<String>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            inner: Vec::new(),
        }
    }

    pub fn append_pair(&mut self, name: &str, value: &str) -> &mut Self {
        self.inner.push(format!("{}={}", name, value));
        self
    }

    pub fn finish(self, checksum: bool) -> String {
        let mut res = self.inner.join("\u{1}");
        res.push('\u{1}');
        if checksum {
            // set BodyLenght (9)
            if let Some(pos) = res.find("\u{1}9=0\u{1}") {
                res.replace_range(pos..(pos + 5), &format!("\u{1}9={}\u{1}", res.len() - (pos + 5)));
            }
            let sum: usize = res.as_bytes().iter().map(|b| *b as usize).sum();
            res.push_str(&format!("10={:03}\u{1}", sum % crate::CHECKSUM_MOD));
        }
        res
    }
}
