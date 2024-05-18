use maud::{html, Markup};

pub fn error_message_fragment(message: &str, details: Option<&str>) -> Markup {
    html! {
        div .error {
            h2 {
                (message)
            }
            @if let Some(description) = details  {
                span {
                    (description)
                }
            }
        }
    }
}
