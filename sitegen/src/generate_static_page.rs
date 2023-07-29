use std::{fs, path::Path};

pub fn generate_static_page() -> String {
    let title = "TESTING TITLE";
    let md = fs::read_to_string(Path::new("test_markdown_files/basic.md")).unwrap();
    markdown::Options::gfm();
    wrap_content_in_core_html(&markdown::to_html(&md), &title)
}

fn wrap_content_in_core_html(content: &str, title: &str) -> String {

    let html_arr = [
        // self explanatory
        "<!DOCTYPE html>
        <html>
        <head>
        <meta charset=\"UTF-8\">",

        // page title
        &["<title>", "</title>"].join(title),

        // CDN fetch for hightlight.js
        r#"
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.8.0/build/styles/default.min.css">
        <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.8.0/build/highlight.min.js"></script>
        <script>hljs.highlightAll();</script>
        "#,

        // close off the head and start the body
        "</head>
        <body>",

        // The content of the page, generated via mardown parsing
        content,

        // close off the body and html
        "</body>
        </html>"
    ];

    html_arr.concat()
}