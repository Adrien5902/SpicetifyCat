use color_eyre::eyre::{Context, Result};
use std::fs;
use tree_sitter::Parser;

const DEBUG: bool = true;
macro_rules! log {
    ($($arg:tt)*) => {{
        if DEBUG == true {
            println!($($arg)*);
        }
    }};
}

fn compile_themes(css_output: &mut String) -> Result<()> {
    let path = "./themes";
    log!("Compiling themes");
    for res in fs::read_dir(path).with_context(|| path)? {
        let file = res?;
        let file_name = file.file_name();

        let theme_name = file_name.to_string_lossy().to_string();
        log!("\n  Compiling {theme_name}");
        let theme_path = file.path();

        let css_path = theme_path.join("user.css");
        if !css_path.exists() {
            log!("  No css for {theme_name} skipping");
            continue;
        }

        let mut css_code = fs::read_to_string(&css_path)
            .with_context(|| theme_name.clone())
            .with_context(|| css_path.to_str().unwrap().to_owned())?;
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_css::LANGUAGE.into())?;

        let mut positions = Vec::new();

        if let Some(tree) = parser.parse(&css_code, None) {
            for child in tree.root_node().named_children(&mut tree.walk()) {
                if child.grammar_name() != "rule_set" {
                    continue;
                }

                if let Some(selectors) = child.child(0)
                    && selectors.grammar_name() == "selectors"
                {
                    for selector in selectors.named_children(&mut tree.walk()) {
                        positions.push(selector.start_byte());
                    }
                }
            }

            let theme_predicate = format!("body.color_scheme_{theme_name} ");
            for pos in positions.into_iter().rev() {
                css_code.insert_str(pos, &theme_predicate);
            }

            css_output.push_str(&format!("\n/* BEGIN {theme_name} THEME CSS */\n"));
            css_output.push_str(&css_code);
            css_output.push_str(&format!("\n/* END {theme_name} THEME CSS */\n"));

            log!("  Compiled {theme_name} successfully");
        } else {
            log!("  Empty css file for {theme_name} skipping")
        }
    }

    Ok(())
}

fn push_global(css_output: &mut String) -> Result<()> {
    log!("Compiling global");
    let path = "global.css";
    let global_css = fs::read_to_string(path).with_context(|| path)?;
    css_output.push_str(&global_css);
    log!("Compiled global successfully\n");
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut css_output = String::new();
    push_global(&mut css_output)?;
    compile_themes(&mut css_output)?;

    fs::write("user.css", css_output)?;

    Ok(())
}
