pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    for (index, _keyword) in list.into_iter().enumerate() {
        if index != list.len() - 1 {
            proverb.push_str(
                format!(
                    "For want of a {} the {} was lost.\n",
                    _keyword,
                    list[index + 1]
                )
                .as_str(),
            )
        } else {
            proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str())
        }
    }

    proverb
}
