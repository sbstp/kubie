use std::fmt::Display;

const SUB_COMMANDS: &'static [&'static str] = &[
    "ctx",
    "ns",
    "info",
    "exec",
    "lint",
    "edit",
    "edit-config",
    "update",
    "delete",
];

fn print_options<I, T, F>(options: I, filter: Option<F>)
where
    I: IntoIterator<Item = T>,
    T: Display,
    F: Display,
{
    if let Some(filter) = filter {
        let filter = filter.to_string();
        for x in options
            .into_iter()
            .map(|x| x.to_string())
            .filter(|x| x.starts_with(&filter))
        {
            println!("{}", x);
        }
    } else {
        for x in options {
            println!("{}", x);
        }
    }
}

pub fn get_completions(position: usize, line: String) {
    let words: Vec<_> = line.trim().split_whitespace().collect();

    match position {
        1 => {
            print_options(SUB_COMMANDS.iter(), words.get(position));
        }
        2 => match words[1] {
            "ctx" | "exec" | "edit" | "delete" => {
                print_options(&["a-ctx", "another-ctx", "hiya"], words.get(position));
            }
            _ => {}
        },
        _ => {}
    }
}
