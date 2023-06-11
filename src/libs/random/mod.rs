use rand::distributions::Alphanumeric;
use rand::Rng;

#[allow(dead_code)]
pub fn generate_alphanumeric_string(len: Option<i32>) -> String {
    const DEFAULT_LEN: i32 = 12;

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len.unwrap_or(DEFAULT_LEN) as usize)
        .map(char::from)
        .collect::<String>()
}

#[allow(dead_code)]
pub fn generate_int_from_range(min: Option<i32>, max: Option<i32>) -> i32 {
    const DEFAULT_MIN: i32 = 0;
    const DEFAULT_MAX: i32 = 1_000_000;

    rand::thread_rng().gen_range(min.unwrap_or(DEFAULT_MIN)..=max.unwrap_or(DEFAULT_MAX))
}
