fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "now".to_string();
    }
    let mut rs: String = String::new();

    let mut duration: Vec<u64> = Vec::new();
    duration.push(seconds / 31536000);
    duration.push(seconds % 31536000 / 86400);
    duration.push(seconds % 86400 / 3600);
    duration.push(seconds % 3600 / 60);
    duration.push(seconds % 60);

    let texts = ["year", "day", "hour", "minute", "second"];

    let count_non_zero: usize = duration.iter().filter(|&&x| x > 0).count();
    let mut done = 0;

    for i in 0..duration.len() {
        if duration[i] > 0 {
            rs.push_str(
                format!(
                    "{} {}{}{}",
                    duration[i],
                    texts[i],
                    if duration[i] > 1 { "s" } else { "" },
                    match count_non_zero - done {
                        0 | 1 => "",
                        2 => " and ",
                        _ => ", ",
                    }
                )
                .as_str(),
            );
            done += 1;
        }
    }
    rs
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
