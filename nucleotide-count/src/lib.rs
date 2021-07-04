use std::collections::HashMap;

fn validate_nucleotide(n: char) -> Result<bool, char> {
    match n {
        'A' | 'C' | 'G' | 'T' => Ok(true),
        _ => Err(n),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate_nucleotide(nucleotide)?;

    let mut count: usize = 0;
    for n in dna.chars() {
        validate_nucleotide(n)?;
        if n == nucleotide {
            count += 1;
        }
    }
    return Ok(count);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut count = vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)];

    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => count[0].1 += 1,
            'C' => count[1].1 += 1,
            'G' => count[2].1 += 1,
            'T' => count[3].1 += 1,
            _ => return Err(nucleotide),
        }
    }

    Ok(count.into_iter().collect())
}
