use maud::{html, Markup, DOCTYPE};

pub fn page_template(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title {
                    (title)
                }
            }
            body {
                (content)
            }
        }
    }
}
