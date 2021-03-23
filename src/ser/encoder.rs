
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

    pub fn finish(self) -> String {
        let mut res = self.inner.join("\u{1}");
        res.push('\u{1}');
        res
    }
}
