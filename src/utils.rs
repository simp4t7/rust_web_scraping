use super::consts;

pub fn cleanup_text(input: String) -> String {
    input
        .chars()
        .take_while(|x| x.is_alphabetic() || consts::SPECIAL_CHARS.contains(x))
        .collect::<String>()
}
