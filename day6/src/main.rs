use std::fs;

/// Read lines from a file

fn find_packet_start(file: &str) -> usize {
    let signal_string = fs::read_to_string(file).unwrap();
    let signal = signal_string.as_str();

    let signal_length = signal.len();
    let mut start_index: usize = 0;
    let mut end_index: usize = start_index + 4;
    while end_index < signal_length {
        let char_quad = &signal[start_index..end_index];

        if characters_are_unique(char_quad) {
            //println!("found unique set: {}", char_quad);
            return end_index;
        }
        start_index += 1;
        end_index += 1;
    }

    return 0;
}
fn find_message_start(file: &str) -> usize {
    let signal_string = fs::read_to_string(file).unwrap();
    let signal = signal_string.as_str();

    let signal_length = signal.len();
    let mut start_index: usize = 0;
    let mut end_index: usize = start_index + 14;
    while end_index < signal_length {
        let char_quad = &signal[start_index..end_index];

        if characters_are_unique(char_quad) {
            //println!("found unique set: {}", char_quad);
            return end_index;
        }
        start_index += 1;
        end_index += 1;
    }

    return 0;
}

fn characters_are_unique(char_quad: &str) -> bool {
    for c1 in char_quad.chars() {
        let mut found_count: i32 = 0;
        for c2 in char_quad.chars() {
            if c1 == c2 {
                found_count += 1;
            }
            if found_count > 1 {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let packet_start = find_packet_start("./day6.txt");
    println!("Found packet start at position: {}", packet_start);
    let message_start = find_message_start("./day6.txt");
    println!("Found message start at position: {}", message_start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_packet_start_1() {
        let packet_start = find_packet_start("./tests/day6-1.txt");
        assert_eq!(packet_start, 7)
    }

    #[test]
    fn test_characters_are_not_unique() {
        let char_quad = "abad";
        assert_eq!(characters_are_unique(char_quad), false)
    }
    #[test]
    fn test_characters_are_unique() {
        let char_quad = "abec";
        assert_eq!(characters_are_unique(char_quad), true)
    }
}
