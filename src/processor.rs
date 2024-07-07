use crate::utils::File;
use std::{fs::File as FsFile, io::{ErrorKind, Read}};
use crate::constants::COMMENT_REGEX;

#[derive(PartialEq)]
pub enum QuoteStyle {
    Single,
    Double
}


pub fn read_file(file: &File) -> Option<String> {        
    let mut file = match FsFile::open(&file.path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                println!("Permission denied when opening file: {}", file.path);
            } else {
                println!("Error opening file: {}", e);
            }
            return None;
        },
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            None
        },
    }
}

#[allow(dead_code)]
pub fn fix_indentation(content: String, indent_size: u8, file: File) -> Result<(String, File), std::io::Error> {
    let indent = " ".repeat(indent_size as usize);
    let mut is_modified = false;
    let lines: Vec<&str> = content.lines().collect();
    let mut new_content = String::new();
    let mut current_indent_level = 0;

    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim_start();
        let original_indent_level = line.len() - trimmed_line.len();
        let new_indent_level = original_indent_level / indent_size as usize;
    
        // Adjust current_indent_level based on the line content
        if trimmed_line.starts_with('}') || trimmed_line.starts_with(']') || trimmed_line.starts_with(')') {
            // Only decrease indent if the new_indent_level is less than the current_indent_level
            if new_indent_level < current_indent_level {
                current_indent_level = new_indent_level;
            }
        } else if new_indent_level > current_indent_level {
            current_indent_level = new_indent_level;
        }
    
        let new_indent = indent.repeat(current_indent_level);
    
        // Format the line with the new indentation
        let formatted_line = if index == lines.len() - 1 {
            format!("{}{}", new_indent, trimmed_line) // Do not add a new line at the end of the last line
        } else {
            format!("{}{}\n", new_indent, trimmed_line)
        };
    
        // Check if the line was modified
        if formatted_line.trim_end() != *line {
            is_modified = true;
        }
    
        new_content.push_str(&formatted_line);
    }

    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

pub fn remove_comments(content: String, file: File) -> Result<(String, File), std::io::Error> {
    let mut new_content = String::new();
    let mut is_modified = false;
    let lines = content.split('\n');
    for line in lines {
        if !COMMENT_REGEX.is_match(line.trim()) {
            new_content.push_str(line);
            new_content.push('\n'); 
        } else {
            is_modified = true;
        }
    }

    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

pub fn remove_empty_lines(content: String, file: File) -> Result<(String, File), std::io::Error> {
    let new_content = content.lines()
                             .filter(|line| !line.trim().is_empty())
                             .collect::<Vec<&str>>()
                             .join("\n");
    let is_modified = new_content != content;
    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

pub fn remove_trailing_spaces(content: String, file: File) -> Result<(String, File), std::io::Error> {
    let mut new_content = String::new();
    let mut is_modified = false;
    let lines = content.split('\n');
    for line in lines {
        let trimmed_line = line.trim_end();
        if trimmed_line != line {
            is_modified = true;
        }
        new_content.push_str(trimmed_line);
        new_content.push('\n');
    }

    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

pub fn fix_quote_style(content: String, file: File, quote_style: QuoteStyle) -> Result<(String, File), std::io::Error> {
    let mut new_content = String::new();
    let mut is_modified = false;
    let lines = content.split('\n');
    for line in lines {
        let mut new_line = String::new();
        let mut inside_quote = false;
        for c in line.chars() {
            match c {
                '\'' if quote_style == QuoteStyle::Double => {
                    if !inside_quote {
                        new_line.push('"'); 
                        is_modified = true;
                        inside_quote = true; 
                    } else {
                        new_line.push(c); 
                        inside_quote = false; 
                    }
                }
                '"' if quote_style == QuoteStyle::Single => {
                    if !inside_quote {
                        new_line.push('\''); 
                        is_modified = true;
                        inside_quote = true; 
                    } else {
                        new_line.push(c); 
                        inside_quote = false; 
                    }
                }
                '"' | '\'' => {
                    new_line.push(c);
                    if (c == '"' && quote_style == QuoteStyle::Single) || (c == '\'' && quote_style == QuoteStyle::Double) {
                        inside_quote = !inside_quote;
                    }
                }
                _ => new_line.push(c),
            }
        }
        new_content.push_str(&new_line);
        new_content.push('\n');
    }

    
    if !new_content.is_empty() {
        new_content.pop();
    }

    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

pub fn fix_bracket_spacing(content: String, file: File) -> Result<(String, File), std::io::Error> {
    let mut new_content = String::new();
    let mut is_modified = false;
    let lines = content.split('\n');

    for line in lines {
        let mut new_line = String::new();
        let mut inside_bracket = false;
        for c in line.chars() {
            match c {
                '{' if !inside_bracket => {
                    new_line.push_str("{ ");
                    is_modified = true;
                    inside_bracket = true;
                }
                '}' if inside_bracket => {
                    new_line.push_str(" }");
                    is_modified = true;
                    inside_bracket = false;
                }
                _ => new_line.push(c),
            }
        }
        new_content.push_str(&new_line);
        new_content.push('\n');
    }

    let new_file = File::new(file.name, file.path, file.size, file.extension, is_modified);

    Ok((new_content, new_file))
}

