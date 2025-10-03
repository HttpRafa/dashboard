use maud::{DOCTYPE, Markup, html};

pub struct HtmlComponent;

impl HtmlComponent {
    pub fn build(title: &str, body: Markup) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    link href="/style.css" rel="stylesheet";
                    link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" rel="stylesheet";
                    script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
                    title {
                        (title)
                    }
                }

                body class="bg-zinc-800" {
                    (body)
                }
            }
        }
    }
}
