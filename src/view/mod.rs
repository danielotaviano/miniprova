use std::fs;

use minijinja::render;
use serde::Serialize;

pub fn get_template(path: &str) -> Result<&'static str, String> {
    let def_path = format!("src/view/{path}.jinja");

    let result = fs::read(def_path);

    if let Err(s) = result {
        return Err(s.to_string());
    }

    let content = result.unwrap();

    let raw_html = String::from_utf8(content);

    let static_content = match raw_html {
        Ok(html) => html,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let static_content: &'static str = Box::leak(static_content.into_boxed_str());
    Ok(static_content)
}

pub fn render_template<T: Serialize>(template_name: &str, data: Option<T>) -> String {
    let template = get_template(template_name)
        .unwrap_or_else(|_| panic!("Failed to get template: {}", template_name));

    match data {
        Some(context) => render!(template, context),
        None => render!(template),
    }
}
