use std::time::Instant;

fn three_vowels_string(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn three_vowels_str(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn test() {
    let test_word_string = "Curious".to_string();
    let test_word_str = "Curious";
    let iterations = 1e8 as u32;

    // Benchmark the `&String` version
    let start = Instant::now();
    for _ in 0..iterations {
        three_vowels_string(&test_word_string);
    }
    let duration_string = start.elapsed();
    println!("&String version took: {:?}", duration_string);

    // Benchmark the `&str` version
    let start = Instant::now();
    for _ in 0..iterations {
        three_vowels_str(test_word_str);
    }
    let duration_str = start.elapsed();
    println!("&str version took: {:?}", duration_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_vowels() {
        main();
    }
}