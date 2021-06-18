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
        return self.scores.into_iter().max().cloned();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut to_sort = self.scores.to_vec();
        to_sort.sort();
        return to_sort.iter().rev().take(3).cloned().collect();
    }
}
