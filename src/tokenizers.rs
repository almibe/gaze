// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

use crate::GazeResult;

pub fn take_string<'a, T: 'a>(
    to_match: &'a str,
    token: T,
) -> impl Fn(Option<&str>, &str) -> GazeResult<T> + 'a
where
    T: Copy,
{
    let graphemes = to_match.graphemes(true).collect::<Vec<&str>>();
    let input_length = to_match.len();
    move |peek: Option<&str>, current_match: &str| -> GazeResult<T> {
        match peek {
            Some(peek) => {
                if graphemes[current_match.len()] == peek {
                    if current_match.len() == input_length - 1 {
                        //TODO doesn't handle unicode correctly
                        GazeResult::MatchAndTake(token)
                    } else {
                        GazeResult::Next
                    }
                } else {
                    GazeResult::NoMatch
                }
            }
            None => GazeResult::NoMatch,
        }
    }
}

pub fn take_while<'a, T: 'a>(
    matcher: &'a dyn Fn(&str) -> bool,
    token: T,
) -> impl Fn(Option<&str>, &str) -> GazeResult<T> + 'a
where
    T: Copy,
{
    move |peek: Option<&str>, current_match: &str| -> GazeResult<T> {
        match peek {
            Some(peek) => {
                if matcher(peek) {
                    GazeResult::Next
                } else if current_match.is_empty() {
                    GazeResult::NoMatch
                } else {
                    GazeResult::Match(token)
                }
            }
            None => {
                if current_match.is_empty() {
                    GazeResult::NoMatch
                } else {
                    GazeResult::Match(token)
                }
            }
        }
    }
}

// /// A Step that takes values from the String until the predicate passes.
// pub struct TakeUntil<'a>(pub &'a dyn Fn(&str) -> bool);

// impl Tokenizer<String> for TakeUntil<'_> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
//         //TODO this will need to be rewritten once handle Unicode better
//         //TODO also this should share code with TakeWhile
//         let mut res = String::new();
//         loop {
//             let next_value = gaze.peek();
//             match next_value {
//                 None => {
//                     return Ok(res);
//                 }
//                 Some(c) => {
//                     if !self.0(c) {
//                         res += c;
//                         gaze.next();
//                     } else {
//                         return Ok(res);
//                     }
//                 }
//             }
//         }
//     }
// }

// pub struct TakeFirst<'a, T>(pub Box<[&'a dyn Tokenizer<T>]>);

// impl<T> Tokenizer<T> for TakeFirst<'_, T> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<T, GazeErr> {
//         for step in &*self.0 {
//             let res = gaze.run(*step);
//             match res {
//                 Ok(_) => return res,
//                 Err(_) => continue,
//             }
//         }
//         Err(GazeErr::NoMatch)
//     }
// }

// pub struct TakeAll<'a, T>(pub Box<[&'a dyn Tokenizer<T>]>);

// impl<T> Tokenizer<Box<[T]>> for TakeAll<'_, T> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<Box<[T]>, GazeErr> {
//         let mut res: Vec<T> = Vec::new();
//         for step in &*self.0 {
//             let r = gaze.run(*step);
//             match r {
//                 Ok(r) => res.push(r),
//                 Err(_) => return Err(GazeErr::NoMatch),
//             }
//         }
//         Ok(res.into_boxed_slice())
//     }
// }
