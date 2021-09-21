// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;
use unicode_segmentation::UnicodeSegmentation;

pub mod tokenizers;

pub type Tokenizer<I, T> = dyn Fn(Option<I>, &Vec<I>) -> GazeResult<T>;

pub struct Gaze<I> {
    offset: usize,
    input: Vec<I>,
}

impl<I> Gaze<I>
where
    I: Copy,
{
    pub fn from_str(text: &str) -> Gaze<&str> {
        let input = text.graphemes(true).collect::<Vec<&str>>();
        Gaze { input, offset: 0 }
    }

    pub fn from_vec(input: Vec<I>) -> Gaze<I> {
        Gaze { input, offset: 0 }
    }

    pub fn is_complete(&self) -> bool {
        self.offset >= self.input.len()
    }

    pub fn peek(&self) -> Option<I>
    where
        I: Copy,
    {
        if self.is_complete() {
            None
        } else {
            Some(self.input[self.offset])
        }
    }

    pub fn next(&mut self) -> Option<I> {
        if self.is_complete() {
            None
        } else {
            self.offset += 1;
            Some(self.input[self.offset])
        }
    }

    pub fn attempt<T>(&mut self, tokenizer: &Tokenizer<I, T>) -> Option<GazeToken<T>>
    where
        I: Copy,
        T: Copy,
    {
        //            let mut matches: Vec<&I> = Vec::new();
        //     // let graphemes = input.graphemes(true).collect::<Vec<&str>>();
        //     // let mut graphemes_offset = 0usize;
        if !self.is_complete() {
            //graphemes_offset != graphemes.len() {
            let mut match_in_this_loop = false; //I can probably delete this?
                                                //         //for tokenizer in tokenizers {
            let start_of_this_loop = self.offset;
            //             //loop for each tokenizer
            loop {
                //                 //loop for each grapheme against the current tokenizer from the current location
                let peek = self.input.get(self.offset).copied();

                let res = tokenizer(peek, &self.input[start_of_this_loop..self.offset].to_vec());
                match res {
                    GazeResult::Next => match peek {
                        Some(_) => self.offset += 1,
                        None => {
                            self.offset = start_of_this_loop;
                            return None;
                        }
                    },
                    GazeResult::MatchAndTake(m) => match peek {
                        Some(_) => {
                            self.offset += 1;
                            match_in_this_loop = true;
                            return Some(GazeToken {
                                //span: &self.input[start_of_this_loop..self.offset],
                                offset: start_of_this_loop,
                                token_type: m,
                            });
                        }
                        None => {
                            self.offset = start_of_this_loop;
                            return None;
                        }
                    },
                    GazeResult::Match(m) => {
                        match_in_this_loop = true;
                        return Some(GazeToken {
                            //span: &input[start_of_this_loop..graphemes_offset],
                            offset: start_of_this_loop,
                            token_type: m,
                        });
                    }
                    GazeResult::NoMatch => {
                        self.offset = start_of_this_loop;
                        return None;
                    }
                }
            }
        } else {
            return None;
        }
        //             }
        //             if match_in_this_loop {
        //                 break;
        //             } else {
        //                 graphemes_offset = start_of_this_loop;
        //             }
        //         //}
        //         if !match_in_this_loop {
        //             break;
        //         }
        //     }
        //     (matches, &input[graphemes_offset..])
    }
}

// pub fn gaze<'a, T>(
//     input: &'a str,
//     tokenizers: &Vec<Box<Tokenizer<T>>>,
// ) -> (Vec<GazeToken<'a, T>>, &'a str)
// where
//     T: Copy,
// {
//     let mut matches = Vec::new();
//     let graphemes = input.graphemes(true).collect::<Vec<&str>>();
//     let mut graphemes_offset = 0usize;
//     while graphemes_offset != graphemes.len() {
//         let mut match_in_this_loop = false;
//         for tokenizer in tokenizers {
//             let start_of_this_loop = graphemes_offset;
//             //loop for each tokenizer
//             loop {
//                 //loop for each grapheme against the current tokenizer from the current location
//                 let peek = graphemes.get(graphemes_offset).copied();

//                 let res = tokenizer(peek, &input[start_of_this_loop..graphemes_offset]);
//                 match res {
//                     GazeResult::Next => match peek {
//                         Some(_) => graphemes_offset += 1,
//                         None => {
//                             graphemes_offset = start_of_this_loop;
//                             break;
//                         }
//                     },
//                     GazeResult::MatchAndTake(m) => match peek {
//                         Some(_) => {
//                             graphemes_offset += 1;
//                             match_in_this_loop = true;
//                             matches.push(GazeToken {
//                                 span: &input[start_of_this_loop..graphemes_offset],
//                                 grapheme_offset: start_of_this_loop,
//                                 token_type: m,
//                             });
//                             break;
//                         }
//                         None => {
//                             graphemes_offset = start_of_this_loop;
//                             break;
//                         }
//                     },
//                     GazeResult::Match(m) => {
//                         match_in_this_loop = true;
//                         matches.push(GazeToken {
//                             span: &input[start_of_this_loop..graphemes_offset],
//                             grapheme_offset: start_of_this_loop,
//                             token_type: m,
//                         });
//                         break;
//                     }
//                     GazeResult::NoMatch => {
//                         graphemes_offset = start_of_this_loop;
//                         break;
//                     }
//                 }
//             }
//             if match_in_this_loop {
//                 break;
//             } else {
//                 graphemes_offset = start_of_this_loop;
//             }
//         }
//         if !match_in_this_loop {
//             break;
//         }
//     }
//     (matches, &input[graphemes_offset..])
// }

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
pub struct GazeToken<T> {
    //<'a, I, T> {
    //pub span: &'a Vec<I>,
    pub offset: usize,
    //pub byte_offset: usize,
    //pub line: usize,
    //pub line_grapheme_offset: usize,
    pub token_type: T,
}
