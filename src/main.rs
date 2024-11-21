use walkdir::WalkDir;
use pulldown_cmark::{html, Parser};
use tera::{Tera, Context};
use std::fs;
use std::path::Path;

fn main() {
    let input_dir = "knowledge_base";
    let output_dir = "output";

    // Create output directory
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir).unwrap();
    }
    fs::create_dir(output_dir).unwrap();

    // Initialize Tera for templating
    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error: {}", e);
            return;
        }
    };

    // Traverse Markdown files
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        if entry.path().extension().map_or(false, |ext| ext == "md") {
            let input_path = entry.path();
            let relative_path = input_path.strip_prefix(input_dir).unwrap();
            let output_path = Path::new(output_dir).join(relative_path).with_extension("html");

            // Read Markdown file
            let markdown = fs::read_to_string(input_path).unwrap();

            // Convert Markdown to HTML
            let parser = Parser::new(&markdown);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            // Render with Tera template
            let mut context = Context::new();
            context.insert("content", &html_output);
            let rendered = tera.render("base.html", &context).unwrap();

            // Create parent directories and write output file
            fs::create_dir_all(output_path.parent().unwrap()).unwrap();
            fs::write(output_path, rendered).unwrap();
        }
    }

    println!("Site generated in '{}'", output_dir);
}
