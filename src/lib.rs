// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fmt::Debug};
use unicode_segmentation::UnicodeSegmentation;

pub mod steps;

/// The main struct for working with Gaze.
pub struct Gaze<'a, T> {
    tokenizers: &'a [&'a dyn Tokenizer<T>],
    // input: &'a str,
    // graphemes: Vec<&'a str>,
    // grapheme_offset: usize,
    // offset: usize, //byte offset?
    // line: usize,             //line number
    // location_in_line: usize, //grapheme offset of current line
    // save_point: Option<usize>,
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
    fn attempt(&self, peek: Option<&str>, current_match: &str) -> GazeResult<T>;
}

impl<'a, T> Gaze<'a, T> {
    pub fn new(tokenizers: &'a [&dyn Tokenizer<T>]) -> Gaze<'a, T> {
        //        let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        Gaze {
            tokenizers,
            // input,
            // graphemes,
            // grapheme_offset: 0,
            // offset: 0,
            // line: 0,
            // location_in_line: 0,
            // save_point: None,
        }
    }

    //// Runs a vector of tokenizers against the current state.
    //// Tokenizers are tested in the order given.
    //// Results are in the order matched.
    //// After each match the tokenizers are tested from the beginning with the new state.
    //// Anytime all of the tokenizers fail to match the function returns the current vector of GazeTokens,
    //// and the state of the Gaze instance remains where it is.
    //// If no tokenizers match at all an empty vector is returned.
    pub fn tokenize(&self, input: &'a str) -> Vec<GazeToken<T>>
    where
        T: Copy + Debug,
    {
        let mut matches = Vec::new();
        let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        let mut graphemes_offset = 0usize;
        while graphemes_offset != graphemes.len() {
            println!("a");
            //loop for all of the input text
            let mut match_in_this_loop = false;
            for tokenizer in self.tokenizers {
                println!("b");
                let mut tokenizer_offset = 0usize;
                let start_of_this_loop = graphemes_offset;
                //loop for each tokenizer
                loop {
                    //loop for each grapheme against the current tokenizer from the current location
                    let peek = graphemes.get(graphemes_offset).map(|c| *c);
                    println!("IN GAZE: {} {}", start_of_this_loop, graphemes_offset);
                    let res = tokenizer.attempt(peek, &input[start_of_this_loop..graphemes_offset]);
                    println!("RES: {:?}", res);
                    if graphemes_offset > 3 {
                        return vec![];
                    }
                    match res {
                        GazeResult::Next => {
                            graphemes_offset += 1;
                            tokenizer_offset += 1;
                        }
                        GazeResult::MatchAndTake(m) => {
                            graphemes_offset += 1;
                            match_in_this_loop = true;
                            matches.push(GazeToken {
                                span: &input[start_of_this_loop..graphemes_offset],
                                grapheme_offset: graphemes_offset,
                                token_type: m,
                            });
                            break;
                        }
                        GazeResult::Match(m) => {
                            match_in_this_loop = true;
                            matches.push(GazeToken {
                                span: &input[start_of_this_loop..graphemes_offset],
                                grapheme_offset: graphemes_offset,
                                token_type: m,
                            });
                            break;
                        }
                        GazeResult::NoMatch => {
                            graphemes_offset = start_of_this_loop;
                            break;
                        } // Ok(m) => {
                          //     matches.push(GazeToken {
                          //         //TODO fix values
                          //         span: &self.input[start_of_this_loop..self.grapheme_offset],
                          //         grapheme_offset: start_of_this_loop,
                          //         //line: 0,
                          //         //line_offset: 0,
                          //         token_type: m.0,
                          //     });
                          //     match_in_this_loop = true;
                          //     break;
                          // }
                          // Err(_) => {
                          //     self.grapheme_offset = start_of_this_loop;
                          // }
                    }
                }
                if match_in_this_loop {
                    println!("in match");
                    break;
                } else {
                    println!("in no match");
                    graphemes_offset = start_of_this_loop;
                    //break;
                }
            }
        }
        matches
    }
}
