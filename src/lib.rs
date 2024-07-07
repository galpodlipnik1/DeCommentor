use utils::str_to_quote_style;

mod config;
mod utils;
mod processor;
mod constants;

pub fn run(root: String) {
    let config = config::read_into_config(root.as_str()).unwrap_or_else(|e| {
        println!("{:?}", e);
        std::process::exit(1);
    });
    println!("Config: {:?}", config);

    let path = match config.path {
        Some(path) => path,
        None => ".".to_string(),
    };

    let files = utils::walk_dir(&path, config.ignore_files.unwrap_or_default()).unwrap_or_else(|e| {
        println!("{:?}", e);
        std::process::exit(1);
    });

    if files.is_empty() {
        println!("No files found in the directory: {}", root);
        return;
    }

    for file in &files {
        let mut content = processor::read_file(&file).unwrap_or_else(|| {
            println!("Error reading file: {}", file.path);
            std::process::exit(1);
        });
        let new_file = file;

        if let Some(true) = config.remove_comments {
            let (new_content, mut new_file) = processor::remove_comments(content, new_file.clone()).unwrap_or_else(|e| {
                println!("Error removing comments: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;
        }

        if let Some(true) = config.remove_empty_lines {
            let (new_content, mut new_file) = processor::remove_empty_lines(content, new_file.clone()).unwrap_or_else(|e| {
                println!("Error removing empty lines: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;

        }

        if let Some(true) = config.remove_trailing_spaces {
            let (new_content, mut new_file) = processor::remove_trailing_spaces(content, new_file.clone()).unwrap_or_else(|e| {
                println!("Error removing trailing spaces: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;
        }

        if let Some(true) = config.bracket_spacing {
            let (new_content, mut new_file) = processor::fix_bracket_spacing(content, new_file.clone()).unwrap_or_else(|e| {
                println!("Error fixing bracket spacing: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;
        }

        if let Some(ref quote_style) = config.quote_style {
            let quote_style_enum = str_to_quote_style(quote_style.clone().to_lowercase()).expect("Invalid quote style");
            let (new_content, mut new_file) = processor::fix_quote_style(content, new_file.clone(), quote_style_enum).unwrap_or_else(|e| {
                println!("Error changing quote style: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;
        }

        if let Some(indent_size) = config.indent_size {
            let (new_content, mut new_file) = processor::fix_indentation(content, indent_size, new_file.clone()).unwrap_or_else(|e| {
                println!("Error fixing indentation: {:?}", e);
                std::process::exit(1);
            });
            content = new_content;
            new_file = new_file;
        }

        println!("Completed processing file: {}, size: {}, was changed: {}", new_file.name, new_file.size, new_file.is_modified);

        utils::write_to_file(file.path.clone().as_str(), content.as_str());
    }
    
}