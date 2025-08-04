fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut current = low;
    while current <= high {
        *total += current;
        current += step;
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let mut words: Vec<String> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    for word in text.split_whitespace() {
        let mut found = false;

        for i in 0..words.len() {
            if words[i] == word {
                counts[i] += 1;
                found = true;
                break;
            }
        }

        if !found {
            words.push(word.to_string());
            counts.push(1);
        }
    }

    let mut max_index = 0;
    let mut max_count = 0;

    for i in 0..counts.len() {
        if counts[i] > max_count {
            max_count = counts[i];
            max_index = i;
        }
    }

    (words[max_index].clone(), max_count)
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
