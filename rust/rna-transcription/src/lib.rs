#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.char_indices() {
            match c {
                'C' | 'T' | 'A' | 'G' => continue,
                _ => return Err(i),
            }
        }
        return Ok(DNA {
            nucleotides: dna.to_string(),
        });
    }

    pub fn to_rna(self) -> RNA {
        RNA {
            nucleotides: self
                .nucleotides
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    d => d,
                }).collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.char_indices() {
            match c {
                'C' | 'U' | 'A' | 'G' => continue,
                _ => return Err(i),
            }
        }
        return Ok(RNA {
            nucleotides: rna.to_string(),
        });
    }
}
