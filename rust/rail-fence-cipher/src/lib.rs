pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut out = String::new();
        for i in 0..self.rails {
            for (j, c) in text.chars().enumerate() {
                if j % (2 * self.rails - 2) == i
                    || j % (2 * self.rails - 2) == 2 * self.rails - 2 - i
                {
                    out.push(c);
                }
            }
        }
        out
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut out: Vec<char> = vec![Default::default(); cipher.len()];
        let mut it = cipher.chars();
        for i in 0..self.rails {
            for j in 0..cipher.len() {
                if j % (2 * self.rails - 2) == i
                    || j % (2 * self.rails - 2) == 2 * self.rails - 2 - i
                {
                    out[j] = it.next().unwrap();
                }
            }
        }
        out.into_iter().collect()
    }
}
