use std::fs;

use minijinja::{context, Environment, Value};
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

    let mut env = Environment::new();
    env.add_filter("to_seconds", |value: i64| -> i64 { value / 1000 });
    minijinja_contrib::add_to_environment(&mut env);

    env.add_template(template_name, template).unwrap();

    let template = env.get_template(template_name).unwrap();

    match data {
        Some(context) => template.render(context! { context => context }).unwrap(),
        None => template.render(context! {}).unwrap(),
    }
}
