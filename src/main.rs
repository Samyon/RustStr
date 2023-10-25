fn main() {
    // transform("some string  123 ffd", 5);
    simple_tests();
}

#[derive(Clone)]
struct WordStr {
    word: String,
    space: String,
    current: bool,
}

impl WordStr {
    pub fn new(word: String) -> Self {
        WordStr {
            word,
            space: "".to_string(),
            current: false,
        }
    }

    pub fn to_res_str(&self) -> String {
        format!("{}{}", &self.word, &self.space)
    }
}


fn fill(line_width: u32, words: &mut Vec<WordStr>) -> String {
    let mut res_str: String;
    let mut current_words: Vec<WordStr> = (*words)
        .iter()
        .filter(|x| (**x).current == true)
        .cloned()
        .collect();
    let count = current_words.len();

    let mut i: i32 = -1;
    loop {
        i = i + 1;
        if (count == 0) || (count == 1) || (i as usize == count - 1) {
            i = 0;
        }
        current_words[i as usize].space.push(' ');

        res_str = current_words.iter().filter(|x| (**x).current == true).
            map(|x| (*x).to_res_str()).collect::<String>();
        if res_str.len() == line_width as usize {
            break;
        }
    }

    (*words)
        .iter_mut()
        .filter(|x| (**x).current == true)
        .for_each(|x| (*x).current = false);

    res_str
}

pub fn transform(input: &str, line_width: u32) -> String {
    let mut words: Vec<WordStr> = Vec::new();
    let mut res_str: String;
    let mut result: String = "".to_string();
    for word in input.split(" ") {
        if word != "" {
            words.push(WordStr::new(word.to_string()));
        }
    }

    let mut i: i32 = -1;
    loop {
        i = i + 1;
        if (i as usize) == words.len() {
            break;
        }
        words[i as usize].current = true;
        res_str = words.iter().filter(|x| (**x).current == true).map(|x| (*x).to_res_str()).collect::<String>();

        if res_str.chars().count() == line_width as usize {
            result += &res_str;
            if (i as usize) < words.len() - 1 {
                result += "\n";
            }
            words
                .iter_mut()
                .filter(|x| (**x).current == true)
                .for_each(|x| (*x).current = false);
            continue;
        }

        if res_str.chars().count() > line_width as usize {
            words[i as usize].current = false;
            i -= 1;
            words[i as usize].space.pop();
            result += &fill(line_width, &mut words);
            if (i as usize) < words.len() - 1 {
                result += "\n";
            }
            continue;
        }

        if i as usize == words.len() - 1 && res_str.len() < line_width as usize {
            result += &fill(line_width, &mut words);
        }

        words[i as usize].space.push(' ');
    }
    result
}

fn simple_tests() {
    let test_cases = [
        ("", 5, ""),
        ("test", 5, "test "),
        ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
         "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
        ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
    ];

    let mut res: bool = true;
    for &(input, line_width, expected) in &test_cases {
        let i_did = transform(input, line_width);
        println!("i_did: '{i_did}'");
        println!("expected: '{expected}'");
        if expected != i_did
        {
            println!("NotOK----------------------");
            res = false;
        } else { println!("OK================="); }
    }
    if res == true
    {
        println!("=====All OK===========");
    } else { println!("-----------Fail-----------"); }
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
            ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }
}


