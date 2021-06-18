#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        return HighScores { scores: scores };
    }

    pub fn scores(&self) -> &[u32] {
        return self.scores;
    }

    pub fn latest(&self) -> Option<u32> {
        return self.scores.last().copied();
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut v = self.scores.to_vec();
        v.sort();
        return v.last().copied();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort();
        let mut a = v[v.len() - 3..v.len()].to_vec();
        a.reverse();
        return a;
    }
}
