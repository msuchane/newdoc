/*
newdoc: Generate pre-populated documentation modules formatted with AsciiDoc.
Copyright (C) 2022  Marek Suchánek  <msuchane@redhat.com>

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

//! # `cmd_line.rs`
//!
//! This module defines the command-line arguments and behavior of `newdoc`.

use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/*
#[command(group(
    ArgGroup::new("required")
                .args([
                    "assembly",
                    "concept",
                    "procedure",
                    "reference",
                    "snippet",
                    "validate",
                ])
                .required(true)
                .multiple(true)
))]
*/
pub struct Cli {
    /// Generate the file without any comments
    #[bpaf(short('C'), long)]
    pub no_comments: bool,

    /// Generate the file without any example, placeholder content
    #[bpaf(short('E'), long, long("expert-mode"))]
    pub no_examples: bool,

    /// Do not use module type prefixes (such as `proc_`) in file names
    #[bpaf(short('P'), long, long("no-prefixes"))]
    pub no_file_prefixes: bool,

    /// Add use module type prefixes (such as `proc_`) in AsciiDoc anchors
    #[bpaf(short('A'), long)]
    pub anchor_prefixes: bool,

    /// Save the generated files in this directory
    #[bpaf(short('T'), long, argument("DIRECTORY"))]
    pub target_dir: Option<PathBuf>,

    #[bpaf(external, fallback(Verbosity::Default))]
    pub verbosity: Verbosity,

    #[bpaf(
        external,
        group_help("Generate or validate files"),
        guard(at_least_one_file, SOME_FILES)
    )]
    pub action: Action,
}

#[derive(Clone, Debug, Bpaf)]
pub struct Action {
    /// Create an assembly file
    #[bpaf(short, long, argument("TITLE"), many)]
    pub assembly: Vec<String>,

    /// Create a concept module
    #[bpaf(short, long, argument("TITLE"), many)]
    pub concept: Vec<String>,

    /// Create a procedure module
    #[bpaf(short, long, argument("TITLE"), many)]
    pub procedure: Vec<String>,

    /// Create a reference module
    #[bpaf(short, long, argument("TITLE"), many)]
    pub reference: Vec<String>,

    /// Create a snippet file
    #[bpaf(short, long, argument("TITLE"), many)]
    pub snippet: Vec<String>,

    /// Create an assembly that includes the other specified modules
    #[bpaf(short, long, argument("TITLE"))]
    pub include_in: Option<String>,
    /// Validate (lint) an existing module or assembly file
    #[bpaf(short('l'), long, argument("FILE"))]
    pub validate: Vec<PathBuf>,
}

#[derive(Clone, Copy, Debug, Bpaf)]
pub enum Verbosity {
    /// Display additional, debug messages
    #[bpaf(short, long)]
    Verbose,
    /// Hide info-level messages. Display only warnings and errors
    #[bpaf(short, long)]
    Quiet,
    #[bpaf(hide)]
    Default,
}

/// Check that the current command generates or validates at least one file.
fn at_least_one_file(action: &Action) -> bool {
    !action.assembly.is_empty()
        || !action.concept.is_empty()
        || !action.procedure.is_empty()
        || !action.reference.is_empty()
        || !action.validate.is_empty()
        || action.include_in.is_some()
}

/// The error message if the command does not generate or validate files.
const SOME_FILES: &str = "Specify at least one file to generate or validate.";

/// Get command-line arguments as the `Cli` struct.
#[must_use]
pub fn get_args() -> Cli {
    let usage_prefix = "Usage: newdoc {usage}";
    cli().usage(usage_prefix).run()
}
