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
use std::env;

use colored::Colorize;

use crate::helper::StatusCodes;

fn main() -> StatusCodes {
    let args: Vec<String> = env::args().collect();

    let Some(args) = minigrep::parse_configs(&args) else {
        return StatusCodes::Failure(1, String::from("Usage Error"));
    };

    println!(
        "Searching for pattern: `{}`, in `{}`",
        args.query().bold().yellow(),
        args.path().bold().magenta(),
    );

    let contents = match minigrep::read_content(args.path()) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!(
                "{}",
                format!(
                    "Failed to read `{}`. Error: {}",
                    args.path(),
                    e.to_string().bold()
                )
                .red()
            );
            return StatusCodes::Failure(2, String::from("Couldn't read file contents"));
        }
    };

    println!("{} contains that following content:", args.path().blue());

    for line in contents.split("\n") {
        println!("\t{}", line);
    }
    StatusCodes::Success
}
