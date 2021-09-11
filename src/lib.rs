// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use unicode_segmentation::UnicodeSegmentation;

pub mod steps;

/// The main struct for working with Gaze.
pub struct Gaze<'a> {
    input: &'a str,
    graphemes: Vec<&'a str>,
    grapheme_offset: usize,
    offset: usize, //byte offset?
                   // line: usize,             //line number
                   // location_in_line: usize, //grapheme offset of current line
                   // save_point: Option<usize>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Match<T>(T)
where
    T: Copy;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NoMatch;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GazeToken<'a, T> {
    pub span: &'a str,
    pub grapheme_offset: usize,
    //pub line: usize,
    //pub line_offset: usize,
    pub token_type: T,
}

pub trait Tokenizer<T>
where
    T: Copy,
{
    fn attempt(&self, gaze: &mut Gaze) -> Result<Match<T>, NoMatch>;
}

impl<'a> Iterator for Gaze<'a> {
    type Item = &'a str;

    /// Increases the current Parser location 1 space and returns the next char.
    /// Returns None if there is no more text.
    fn next(&mut self) -> Option<&'a str> {
        if self.graphemes.len() <= self.grapheme_offset {
            None
        } else {
            let next = self.graphemes[self.grapheme_offset];
            self.grapheme_offset += 1;
            Some(next)
        }
    }
}

impl Gaze<'_> {
    pub fn new(input: &str) -> Gaze {
        let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        Gaze {
            input,
            graphemes,
            grapheme_offset: 0,
            offset: 0,
            // line: 0,
            // location_in_line: 0,
            // save_point: None,
        }
    }

    /// Returns the next char, but doesn't affect the current Parser location.
    /// Returns None if there is no more text.
    pub fn peek(&self) -> Option<&str> {
        if self.is_complete() {
            None
        } else {
            Some(self.graphemes[self.grapheme_offset])
        }
    }

    /// Runs a vector of tokenizers against the current state.
    /// Tokenizers are tested in the order given.
    /// Results are in the order matched.
    /// After each match the tokenizers are tested from the beginning with the new state.
    /// Anytime all of the tokenizers fail to match the function returns the current vector of GazeTokens,
    /// and the state of the Gaze instance remains where it is.
    /// If no tokenizers match at all an empty vector is returned.
    pub fn run<T>(&mut self, tokenizers: &Vec<&dyn Tokenizer<T>>) -> Vec<GazeToken<T>>
    where
        T: Copy,
    {
        let mut matches = Vec::new();
        loop {
            let mut match_in_this_loop = false;
            let start_of_this_loop = self.grapheme_offset;
            for tokenizer in tokenizers {
                let res = tokenizer.attempt(self);
                match res {
                    Ok(m) => {
                        matches.push(GazeToken {
                            //TODO fix values
                            span: &self.input[start_of_this_loop..self.grapheme_offset],
                            grapheme_offset: start_of_this_loop,
                            //line: 0,
                            //line_offset: 0,
                            token_type: m.0,
                        });
                        match_in_this_loop = true;
                        break;
                    }
                    Err(_) => {
                        self.grapheme_offset = start_of_this_loop;
                    }
                }
            }
            if match_in_this_loop {
                continue;
            } else {
                self.grapheme_offset = start_of_this_loop;
                break;
            }
        }
        matches
    }

    pub fn current_offset(&self) -> usize {
        self.grapheme_offset
    }

    pub fn is_complete(&self) -> bool {
        self.graphemes.len() <= self.grapheme_offset
    }
}
