use std::usize;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, n) in dna.chars().into_iter().enumerate() {
            if n == 'A' || n == 'C' || n == 'G' || n == 'T' {
                continue;
            }
            return Err(i);
        }
        return Ok(Dna(String::from(dna)));
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(""),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, n) in rna.chars().into_iter().enumerate() {
            if n == 'A' || n == 'C' || n == 'G' || n == 'U' {
                continue;
            }
            return Err(i);
        }
        return Ok(Rna(String::from(rna)));
    }
}
