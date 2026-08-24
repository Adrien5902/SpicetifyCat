use color_eyre::eyre::{Context, Result};
use std::fs;

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

        let css_code = fs::read_to_string(&css_path)
            .with_context(|| theme_name.clone())
            .with_context(|| css_path.to_str().unwrap().to_owned())?;

        let theme_predicate_selector = format!("body.color_scheme_{theme_name}");
        css_output.push_str(&format!(
            "\n/* BEGIN {theme_name} THEME CSS */\n{theme_predicate_selector} {}\n",
            "{"
        ));
        css_output.push_str(&ident(&css_code, "    "));
        css_output.push_str(&format!("\n{}\n/* END {theme_name} THEME CSS */\n", "}"));

        log!("  Compiled {theme_name} successfully");
    }

    Ok(())
}

fn ident(s: &str, ident: &str) -> String {
    s.lines()
        .map(|line| {
            if line.is_empty() {
                String::from("\n")
            } else {
                format!("\n{ident}{line}")
            }
        })
        .collect()
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
