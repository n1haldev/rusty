use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
        "article": "JSON in rust!",
        "author": "Nihal T M",
        "paragraph": [
            {
                "name": "Starting of the paragraph"
            },
            {
                "name": "Body of the paragraph"
            },
            {
                "name": "End of the paragraph"
            }
        ]
    }
    "#;

    let parsed: Article = read_json_typed(json);
    println!("\n\nThe name of the first paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
