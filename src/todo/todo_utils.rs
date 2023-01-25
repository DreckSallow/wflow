use std::fmt::Display;

fn count_chars<T: ToString>(text: T) -> usize {
    text.to_string().chars().count()
}

fn fill_text(text: &str, n: i32) -> String {
    if n <= 0 {
        return text.to_string();
    }
    let mut filled = text.to_string();

    for _ in 0..n {
        filled.push(' ')
    }

    filled
}

pub fn table<T: Display + Clone>(mut body: Vec<Vec<T>>, headers: Vec<T>) -> Vec<Vec<String>> {
    let mut max_sizes: Vec<usize> = body[0].iter().map(|_| 0).collect();
    body.insert(0, headers);
    let mut table_content = Vec::new();

    for list in &body {
        let mut i = 0;
        for item in list {
            let length = count_chars(item);
            if length > max_sizes[i] {
                max_sizes[i] = length
            }
            i += 1;
        }
    }

    for list in &body {
        let mut new_list = Vec::with_capacity(list.len());

        let mut i = 0;

        for item in list {
            let item_str = &item.to_string();
            new_list.push(fill_text(
                &item_str,
                (max_sizes[i] - count_chars(item_str)) as i32,
            ));
            i += 1;
        }
        table_content.push(new_list);
    }

    table_content
}
