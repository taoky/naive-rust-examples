#[derive(Clone, Debug)]
enum Language {
    English,
    Chinese,
    Japanese,
    Esperanto,
}

struct LangMeta {
    // enum 没有用到，但是还是留着
    // 使用以下的语法可以 suppress warning
    #[allow(dead_code)]
    lang: Language,
    name: &'static str,
    greeting: &'static str,
}

const METADATA: [LangMeta; 4] = [
    LangMeta {
        lang: Language::English,
        name: "English",
        greeting: "Hello, world!",
    },
    LangMeta {
        lang: Language::Chinese,
        name: "中文",
        greeting: "你好，世界！",
    },
    LangMeta {
        lang: Language::Japanese,
        name: "日本語",
        greeting: "こんにちは、世界！",
    },
    LangMeta {
        lang: Language::Esperanto,
        name: "Esperanto",
        greeting: "Saluton, mondo!",
    },
];

fn main() {
    for (key, value) in METADATA.iter().enumerate() {
        println!("{}: {}", key, value.name);
    }
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    let input = input.trim();
    let choice: usize = input.parse().expect("Not an integer");

    if choice < METADATA.len() {
        println!("{}", METADATA[choice].greeting);
    } else {
        println!("Invalid choice");
    }
}
