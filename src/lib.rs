// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

pub mod tokenizers;

pub type Tokenizer<T> = dyn Fn(Option<&str>, &str) -> GazeResult<T>;

pub fn gaze<'a, T>(input: &'a str, tokenizers: &Vec<Box<Tokenizer<T>>>) -> (Vec<GazeToken<'a, T>>, &'a str)
where
    T: Copy,
{
    let mut matches = Vec::new();
    let graphemes = input.graphemes(true).collect::<Vec<&str>>();
    let mut graphemes_offset = 0usize;
    while graphemes_offset != graphemes.len() {
        let mut match_in_this_loop = false;
        for tokenizer in tokenizers {
            let start_of_this_loop = graphemes_offset;
            //loop for each tokenizer
            loop {
                //loop for each grapheme against the current tokenizer from the current location
                let peek = graphemes.get(graphemes_offset).copied();

                let res = tokenizer(peek, &input[start_of_this_loop..graphemes_offset]);
                match res {
                    GazeResult::Next => match peek {
                        Some(_) => graphemes_offset += 1,
                        None => {
                            graphemes_offset = start_of_this_loop;
                            break;
                        }
                    },
                    GazeResult::MatchAndTake(m) => match peek {
                        Some(_) => {
                            graphemes_offset += 1;
                            match_in_this_loop = true;
                            matches.push(GazeToken {
                                span: &input[start_of_this_loop..graphemes_offset],
                                grapheme_offset: start_of_this_loop,
                                token_type: m,
                            });
                            break;
                        }
                        None => {
                            graphemes_offset = start_of_this_loop;
                            break;
                        }
                    },
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
    //pub byte_offset: usize,
    //pub line: usize,
    //pub line_grapheme_offset: usize,
    pub token_type: T,
}
