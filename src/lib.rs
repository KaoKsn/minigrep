/*
    minigrep (v0.1.0) is a free software that searches for a specified pattern in a file.
    Copyright (C) <2026>  <KaoKsn>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod helper;

#[cfg(test)]
mod tests;

use std::fs;
use std::io;

use crate::helper::StatusCodes;
use colored::Colorize;

/// Stores the components of the arguments as a struct.
#[derive(Debug)]
pub struct Args<'a> {
    name: &'a str, // Name of the binary
    query: &'a str,
    path: &'a str,
}

/// Implement some common required associated functions on Args
impl<'a> Args<'a> {
    /// Create a new instance of Args type.
    pub fn new(name: &'a str, query: &'a str, path: &'a str) -> Self {
        Self { name, query, path }
    }
    /// getter for attribute `name` which holds the name of the binary.
    pub fn name(&self) -> &str {
        self.name
    }
    /// getter for attribute `query` which holds pattern to search for.
    pub fn query(&self) -> &str {
        self.query
    }
    /// getter for attribute `path` which holds the path to the target file.
    pub fn path(&self) -> &str {
        self.path
    }
}

/// Read the contents of a file named 'filename'.
pub fn read_content(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

/// Parse the configuration options passed through the command line and return a struct which
/// contains references to the values parsed.
/// Returns Option\<Args\> to allow the caller to handle errors.
pub fn parse_configs(args: &[String]) -> Result<Args<'_>, StatusCodes> {
    // Args contains references that are valid for lifetime 'a
    if args.len() < 3 {
        let usage_message = helper::usage();
        return Err(StatusCodes::Failure(1, usage_message));
    } else if args.len() > 3 {
        eprintln!(
            "{}",
            "Extraneous configurations were found. Discarding them."
                .bold()
                .yellow()
        )
    }
    Ok(Args::new(&args[0], &args[1], &args[2]))
}

pub fn run(args: Args) -> Result<(), StatusCodes> {
    let contents = match read_content(args.path()) {
        Ok(contents) => contents,
        Err(e) => {
            let message = format!(
                "Failed to read `{}`. Error: {}",
                args.path(),
                e.to_string().bold().red()
            );
            return Err(StatusCodes::Failure(2, message));
        }
    };

    // Print the line number and colored string.
    for (line_no, line) in search(args.query(), &contents).iter().enumerate() {
        match highlight(args.query(), line) {
            Some(value) => println!("{}: {value}", (line_no + 1).to_string().green()),
            None => continue,
        }
    }
    Ok(())
}

/// Search for `query` in the content provided and return the lines that contain
/// them as a vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // References borrowed by matches must be valid until contents exists.
    // It doesn't depend on the query.
    let mut matches: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

/// Given a line and query, return a colored string which highlights(in yellow) query in the line.
/// If query doesn't exists in the line, return None.
pub fn highlight<'a>(query: &'a str, line: &'a str) -> Option<String> {
    let Some(start) = line.find(query) else {
        return None;
    };
    let end = start + query.len();
    Some(format!(
        "{}{}{}",
        &line[0..start],
        line[start..end].bright_yellow(),
        &line[end..]
    ))
}
