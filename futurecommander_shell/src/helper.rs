/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::PathBuf;
use std::vec::IntoIter;

use futurecommander_filesystem::{
    ReadableFileSystem,
    Entry,
    Container
};

use crate::{
    tools::get_parent_or_root
};

use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::{
    error::ReadlineError,
    Helper,
    Context,
    hint::Hinter,
    highlight::{Highlighter, MatchingBracketHighlighter},
    completion::{Completer, Pair},
    validate::{self, MatchingBracketValidator, Validator}
};

static WHITE_PROMPT: &str = "\x1b[1;97m>>\x1b[0m ";
static RED_PROMPT: &str = "\x1b[1;91m>>\x1b[0m ";

const fn available_commands() -> [&'static str; 18] {
    [
        "exit",
        "cd",
        "debug_status",
        "debug_container",
        "debug_add_state",
        "debug_sub_state",
        "debug_transaction",
        "pwd",
        "reset",
        "ls",
        "cp",
        "mv",
        "rm",
        "mkdir",
        "touch",
        "tree",
        "apply",
        "history"
    ]
}

pub struct VirtualHelper<'a>  {
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    container: &'a Container,
    cwd: PathBuf
}

impl  <'a>VirtualHelper<'a>  {
    pub fn new(container: &'a Container, cwd: PathBuf) -> VirtualHelper  {
        VirtualHelper{
            validator: MatchingBracketValidator::new(),
            highlighter: MatchingBracketHighlighter::new(),
            container,
            cwd
        }
    }

    pub fn score(given: &str, expected: &str) -> usize {
        let mut score = 0;
        for (i, c) in expected.chars().enumerate() {
            if given.len() < i {
                return score;
            }

            if given.chars().nth(i) == Some(c) {
                score += 1;
            }
        }

        score
    }

    pub fn score_to_pairs(scores: IntoIter<(usize, String)>, pos: usize) -> Vec<Pair> {
        scores.map(|(_score, command)| {
            let mut replacement = command.to_string();
            if pos <= replacement.len() {
                replacement.replace_range(..pos, "");
            }
            Pair { display: command, replacement }
        }).collect()
    }

    pub fn score_and_pos_to_pairs(scores: IntoIter<(usize, String, usize)>) -> Vec<Pair> {
        scores.map(|(_score, command, pos)| {
            let mut replacement = command.to_string();
            if pos <= replacement.len() {
                replacement.replace_range(..pos, "");
            }
            Pair { display: command, replacement }
        }).collect()
    }

    pub fn command_candidates(&self, given: &str, pos: usize) -> Vec<Pair> {
        let mut scores = Vec::new();
        let mut max_score : usize = 0;

        for command in available_commands().iter() {
            let score = Self::score(given, command);
            if score > max_score {
                max_score = score;
            }

            if pos <= command.len() {
                scores.push((score, command.to_string()));
            }
        }

        let most_matchings_count  = scores.iter()
            .filter(| &(score, _)|  score == &max_score )
            .count();

        if most_matchings_count > 1 {
            return Self::score_to_pairs(scores.into_iter(), pos);
        }

        Self::score_to_pairs(
            scores.into_iter()
                .filter(| &(score, _)|  score == max_score )
                .collect::<Vec<(usize, String)>>()
                .into_iter(),
            pos
        )
    }

    pub fn path_candidates(&self, given: &str) -> Vec<Pair> {
        let mut max_score : usize = 0;
        let mut scores = Vec::new();
        let given_path = self.cwd.join(given);
        let parent = get_parent_or_root(given_path.as_path());

        match self.container.read_dir(given_path.as_path()) {
            Ok(collection) => {
                for entry in collection.iter() {
                    let path = entry.path().strip_prefix(&self.cwd).unwrap_or_else(|_| entry.path());
                    let path_str = path.as_os_str().to_str().unwrap();
                    let score = Self::score(given, path_str);
                    if score > max_score {
                        max_score = score;
                    }

                    scores.push((score, path_str.to_string(), given.len()));
                }
            },
            Err(_) =>
                if let Ok(collection) = self.container.read_dir(parent.as_path()) {
                    for entry in collection.iter() {
                        let path = entry.path().strip_prefix(&self.cwd).unwrap_or_else(|_| entry.path());
                        let path_str = path.as_os_str().to_str().unwrap();
                        let score = Self::score(given, path_str);
                        if score > max_score {
                            max_score = score;
                        }

                        scores.push((score, path_str.to_string(), given.len()));
                    }
                }
        }


        let most_matchings_count  = scores.iter()
            .filter(| &(score, _, _)|  score == &max_score )
            .count();

        if most_matchings_count > 0 {
            return Self::score_and_pos_to_pairs(
                scores.into_iter()
                    .filter(| &(score, _, pos)|  score >= pos )
                    .collect::<Vec<(usize, String, usize)>>()
                    .into_iter()
            )
        }

        Self::score_and_pos_to_pairs(
            scores.into_iter()
                .filter(| &(score, _, _)|  score == max_score )
                .collect::<Vec<(usize, String, usize)>>()
                .into_iter()
        )
    }
}

impl <'a> Completer for VirtualHelper<'a>  {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let input : Vec<&str> = line.split(' ').collect();

        if input.len() == 1 {
            return Ok((pos, self.command_candidates(line.trim(), pos)));
        }

        Ok((pos, self.path_candidates(input.last().unwrap())))
    }
}

impl <'a> Hinter for VirtualHelper<'a>  {
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> { None }
}

impl <'a> Highlighter for VirtualHelper<'a>  {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, _prompt: &'p str, _default: bool) -> Cow<'b, str> {
        if self.container.is_empty() {
            Borrowed(WHITE_PROMPT)
        } else {
            Borrowed(RED_PROMPT)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl  <'a>Helper for VirtualHelper<'a>  {}


impl <'a>Validator for VirtualHelper<'a> {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}
