use std::usize;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(invalid) = dna.find(|n| n != 'A' && n != 'C' && n != 'G' && n != 'T') {
            return Err(invalid);
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
        if let Some(invalid) = rna.find(|n| n != 'A' && n != 'C' && n != 'G' && n != 'U') {
            return Err(invalid);
        }
        return Ok(Rna(String::from(rna)));
    }
}
