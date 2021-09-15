// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

pub mod tokenizers;

/// The main struct for working with Gaze.
pub struct Gaze<'a, T> {
    tokenizers: &'a [&'a dyn Tokenizer<T>],
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GazeResult<T>
where
    T: Copy,
{
    Next,
    MatchAndTake(T),
    Match(T),
    NoMatch,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GazeResultEnd<T>
where
    T: Copy,
{
    Match(T),
    NoMatch,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GazeToken<'a, T> {
    pub span: &'a str,
    pub grapheme_offset: usize,
    //pub byte_offset: usize,
    //pub line: usize,
    //pub line_grapheme_offset: usize,
    pub token_type: T,
}

pub trait Tokenizer<T>
where
    T: Copy,
{
    fn attempt(&self, peek: &str, current_match: &str) -> GazeResult<T>;
    fn attempt_end(&self, current_match: &str) -> GazeResultEnd<T>;
}

impl<'a, T> Gaze<'a, T> {
    pub fn new(tokenizers: &'a [&dyn Tokenizer<T>]) -> Gaze<'a, T> {
        Gaze { tokenizers }
    }

    //// Runs a vector of tokenizers against the current state.
    //// Tokenizers are tested in the order given.
    //// Results are in the order matched.
    //// After each match the tokenizers are tested from the beginning with the new state.
    //// Anytime all of the tokenizers fail to match the function returns the current vector of GazeTokens,
    //// and the state of the Gaze instance remains where it is.
    //// If no tokenizers match at all an empty vector is returned.
    pub fn tokenize(&self, input: &'a str) -> (Vec<GazeToken<T>>, &str)
    where
        T: Copy + Debug,
    {
        let mut matches = Vec::new();
        let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        let mut graphemes_offset = 0usize;
        while graphemes_offset != graphemes.len() {
            let mut match_in_this_loop = false;
            for tokenizer in self.tokenizers {
                let start_of_this_loop = graphemes_offset;
                //loop for each tokenizer
                loop {
                    //loop for each grapheme against the current tokenizer from the current location
                    let peek = graphemes.get(graphemes_offset).copied();
                    match peek {
                        Some(peek) => {
                            let res = tokenizer
                                .attempt(peek, &input[start_of_this_loop..graphemes_offset]);
                            match res {
                                GazeResult::Next => {
                                    graphemes_offset += 1;
                                }
                                GazeResult::MatchAndTake(m) => {
                                    graphemes_offset += 1;
                                    match_in_this_loop = true;
                                    matches.push(GazeToken {
                                        span: &input[start_of_this_loop..graphemes_offset],
                                        grapheme_offset: start_of_this_loop,
                                        token_type: m,
                                    });
                                    break;
                                }
                                GazeResult::Match(m) => {
                                    match_in_this_loop = true;
                                    matches.push(GazeToken {
                                        span: &input[start_of_this_loop..graphemes_offset],
                                        grapheme_offset: start_of_this_loop,
                                        token_type: m,
                                    });
                                    break;
                                }
                                GazeResult::NoMatch => {
                                    graphemes_offset = start_of_this_loop;
                                    break;
                                }
                            }
                        }
                        None => {
                            let res =
                                tokenizer.attempt_end(&input[start_of_this_loop..graphemes_offset]);
                            match res {
                                GazeResultEnd::Match(m) => {
                                    match_in_this_loop = true;
                                    matches.push(GazeToken {
                                        span: &input[start_of_this_loop..graphemes_offset],
                                        grapheme_offset: start_of_this_loop,
                                        token_type: m,
                                    });
                                    break;
                                }
                                GazeResultEnd::NoMatch => {
                                    graphemes_offset = start_of_this_loop;
                                    break;
                                }
                            }
                        }
                    }
                }
                if match_in_this_loop {
                    break;
                } else {
                    graphemes_offset = start_of_this_loop;
                }
            }
            if !match_in_this_loop {
                break;
            }
        }
        (matches, &input[graphemes_offset..])
    }
}
