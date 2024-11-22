use walkdir::WalkDir;
use pulldown_cmark::{html, Parser};
use tera::{Tera, Context};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

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

    // Create a mapping to hold file links
    let mut file_links: HashMap<PathBuf, String> = HashMap::new();

    // First pass: Traverse Markdown files to create the file mapping
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        if entry.path().extension().map_or(false, |ext| ext == "md") {
            let input_path = entry.path();
            let relative_path = input_path.strip_prefix(input_dir).unwrap();
            let output_path = Path::new(output_dir).join(relative_path).with_extension("html");

            // Store the output path relative to the output directory for later link generation
            file_links.insert(input_path.to_path_buf(), relative_path.to_string_lossy().to_string());
        }
    }

    // Second pass: Process the Markdown files and generate the HTML
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

            // Generate navigation links based on the file mapping
            let mut context = Context::new();
            let nav_links: Vec<String> = file_links.iter()
                .map(|(path, link)| {
                    // Construct the relative path for linking
                    let display_name = path.file_stem().unwrap().to_string_lossy().to_string();
                    format!("<a href=\"/{link}\">{}</a>", display_name)
                })
                .collect();
            
            context.insert("content", &html_output);
            context.insert("nav_links", &nav_links);

            // Render the page with Tera template
            let rendered = tera.render("base.html", &context).unwrap();

            // Create parent directories and write output file
            fs::create_dir_all(output_path.parent().unwrap()).unwrap();
            fs::write(output_path, rendered).unwrap();
        }
    }

    println!("Site generated in '{}'", output_dir);
}
