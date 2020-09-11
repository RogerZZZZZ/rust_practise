pub fn build_proverb(list: &[&str]) -> String {
    let mut lines: Vec<String> = Vec::new();
    if list.len() > 0 {
        lines.extend(
            (0..list.len() - 1)
                .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
        );
        lines.push(format!("And all for the want of a {}.", list[0]));
    }
    lines.join("\n")
}
