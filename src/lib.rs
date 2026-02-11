use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);

    let mut html = String::new();
    let mut in_code_block = false;
    let mut pending_link: Option<String> = None;
    let mut pending_image: Option<(String, String)> = None;

    for event in parser {
        if pending_image.is_some() {
            match event {
                Event::End(TagEnd::Image) => {
                    let (src, alt) = pending_image.take().expect("pending image exists");
                    html.push_str(&format!("<img src=\"{}\" alt=\"{}\" />", escape_html(&src), escape_html(&alt)));
                }
                Event::Text(text) | Event::Code(text) => {
                    if let Some((_, alt)) = pending_image.as_mut() {
                        alt.push_str(&text);
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if let Some((_, alt)) = pending_image.as_mut() {
                        alt.push(' ');
                    }
                }
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    html.push_str("<p>");
                }
                Tag::Heading { level, .. } => {
                    let level_num = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    html.push_str(&format!("<h{}>", level_num));
                }
                Tag::BlockQuote(_) => {
                    html.push_str("<blockquote>");
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    match kind {
                        CodeBlockKind::Fenced(lang) => {
                            let lang = lang.trim();
                            if lang.is_empty() {
                                html.push_str("<pre><code>");
                            } else {
                                html.push_str(&format!("<pre><code class=\"language-{}\">", escape_html(lang)));
                            }
                        }
                        CodeBlockKind::Indented => {
                            html.push_str("<pre><code>");
                        }
                    }
                }
                Tag::List(_) => {
                    html.push_str("<ul>");
                }
                Tag::Item => {
                    html.push_str("<li>");
                }
                Tag::Emphasis => {
                    html.push_str("<em>");
                }
                Tag::Strong => {
                    html.push_str("<strong>");
                }
                Tag::Strikethrough => {
                    html.push_str("<del>");
                }
                Tag::Link { dest_url, .. } => {
                    pending_link = Some(dest_url.to_string());
                    html.push_str("<a href=\"");
                }
                Tag::Image { dest_url, .. } => {
                    pending_image = Some((dest_url.to_string(), String::new()));
                }
                Tag::Table(_) => {
                    html.push_str("<table>");
                }
                Tag::TableHead => {
                    html.push_str("<thead>");
                }
                Tag::TableRow => {
                    html.push_str("<tr>");
                }
                Tag::TableCell => {
                    html.push_str("<td>");
                }
                Tag::FootnoteDefinition(name) => {
                    html.push_str(&format!("<div id=\"fn-{}\">", escape_html(&name)));
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => {
                    html.push_str("</p>\n");
                }
                TagEnd::Heading(level) => {
                    let level_num = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    html.push_str(&format!("</h{}>\n", level_num));
                }
                TagEnd::BlockQuote(_) => {
                    html.push_str("</blockquote>\n");
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    html.push_str("</code></pre>\n");
                }
                TagEnd::List(_) => {
                    html.push_str("</ul>\n");
                }
                TagEnd::Item => {
                    html.push_str("</li>\n");
                }
                TagEnd::Emphasis => {
                    html.push_str("</em>");
                }
                TagEnd::Strong => {
                    html.push_str("</strong>");
                }
                TagEnd::Strikethrough => {
                    html.push_str("</del>");
                }
                TagEnd::Link => {
                    html.push_str("\">");
                    if let Some(link) = pending_link.take() {
                        html.push_str(&escape_html(&link));
                    }
                    html.push_str("</a>");
                }
                TagEnd::Image => {}
                TagEnd::Table => {
                    html.push_str("</table>\n");
                }
                TagEnd::TableHead => {
                    html.push_str("</thead>");
                }
                TagEnd::TableRow => {
                    html.push_str("</tr>\n");
                }
                TagEnd::TableCell => {
                    html.push_str("</td>");
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    html.push_str(&escape_html(&text));
                } else {
                    html.push_str(&escape_html(&text));
                }
            }
            Event::Code(text) => {
                html.push_str(&format!("<code>{}</code>", escape_html(&text)));
            }
            Event::Html(text) | Event::InlineHtml(text) => {
                html.push_str(&text);
            }
            Event::SoftBreak => {
                html.push(' ');
            }
            Event::HardBreak => {
                html.push_str("<br>\n");
            }
            Event::Rule => {
                html.push_str("<hr>\n");
            }
            Event::TaskListMarker(done) => {
                if done {
                    html.push_str("<input type=\"checkbox\" checked disabled> ");
                } else {
                    html.push_str("<input type=\"checkbox\" disabled> ");
                }
            }
            Event::FootnoteReference(name) => {
                html.push_str(&format!("<a href=\"#fn-{}\">[^{}]</a>", escape_html(&name), escape_html(&name)));
            }
            Event::InlineMath(text) | Event::DisplayMath(text) => {
                html.push_str(&format!("${}$", escape_html(&text)));
            }
        }
    }

    html
}

fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = "# Hello\n\nThis is **bold** text.";
        let output = parse_markdown(input);
        assert!(output.contains("<h1>"));
        assert!(output.contains("Hello"));
        assert!(output.contains("<strong>"));
        assert!(output.contains("bold"));
    }

    #[test]
    fn test_parse_code_block() {
        let input = "```rust\nfn main() {}\n```";
        let output = parse_markdown(input);
        assert!(output.contains("<pre>"));
        assert!(output.contains("<code"));
        assert!(output.contains("language-rust"));
    }
}
