
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
        if checksum {
            let sum: usize = res.as_bytes().iter().map(|b| *b as usize).sum();
            res.push_str(&format!("\u{1}10={:03}\u{1}", sum % 255));
        }
        else {
            res.push('\u{1}');
        }
        res
    }
}
