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

#[derive(Debug)]
pub struct Args<'a> {
    name: &'a str, // Name of the binary
    query: &'a str,
    path: &'a str,
}

impl<'a> Args<'a> {
    pub fn new(name: &'a str, query: &'a str, path: &'a str) -> Self {
        Self { name, query, path }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn query(&self) -> &str {
        self.query
    }
    pub fn path(&self) -> &str {
        self.path
    }
}
