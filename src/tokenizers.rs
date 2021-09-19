// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use unicode_segmentation::UnicodeSegmentation;

use crate::{GazeResult, Tokenizer};

// pub fn take_string<T>(to_match: &str, token: T, peek: Option<&str>, current_match: &str) -> GazeResult<T> {
//     todo!()
// }

pub fn take_string<'a, T: 'a>(to_match: &'a str, token: T) -> impl Fn(&str) -> GazeResult<T> + 'a where T: Copy {
    let graphemes = to_match.graphemes(true).collect::<Vec<&str>>();
    let input_length = to_match.len();
    move |i: &str| -> GazeResult<T> {
        println!("{:?} {}", graphemes, input_length);
        if i == to_match {
            GazeResult::Match(token)
        } else {
            GazeResult::NoMatch
        }

//     fn attempt(&self, peek: &str, current_match: &str) -> GazeResult<T> {
//         if self.graphemes[current_match.len()] == peek {
//             if current_match.len() == self.input_length - 1 {
//                 //TODO doesn't handle unicode correctly
//                 GazeResult::MatchAndTake(self.token)
//             } else {
//                 GazeResult::Next
//             }
//         } else {
//             GazeResult::NoMatch
//         }
//     }

//     fn attempt_end(&self, _: &str) -> GazeResultEnd<T> {
//         GazeResultEnd::NoMatch
//     }


    }
}

// pub struct TakeString<'a, T> {
//     graphemes: Vec<&'a str>,
//     token: T,
//     input_length: usize,
// }

// impl<T> TakeString<'_, T>
// where
//     T: Copy,
// {
//     pub fn new(value: &str, token: T) -> TakeString<T> {
//         let graphemes = value.graphemes(true).collect::<Vec<&str>>();
//         TakeString {
//             graphemes,
//             token,
//             input_length: value.len(),
//         }
//     }
// }

// impl<T> Tokenizer<T> for TakeString<'_, T>
// where
//     T: Copy,
// {
//     fn attempt(&self, peek: &str, current_match: &str) -> GazeResult<T> {
//         if self.graphemes[current_match.len()] == peek {
//             if current_match.len() == self.input_length - 1 {
//                 //TODO doesn't handle unicode correctly
//                 GazeResult::MatchAndTake(self.token)
//             } else {
//                 GazeResult::Next
//             }
//         } else {
//             GazeResult::NoMatch
//         }
//     }

//     fn attempt_end(&self, _: &str) -> GazeResultEnd<T> {
//         GazeResultEnd::NoMatch
//     }
// }

// /// A Step that ignores all chars passed in.
// pub struct IgnoreAll<'a>(pub HashSet<&'a str>);

// impl Tokenizer<()> for IgnoreAll<'_> {
//     fn attempt(&self, gaze: &mut Gaze) -> Result<(), GazeErr> {
//         //TODO this will need to be rewritten once handle Unicode better
//         loop {
//             let next_value = gaze.peek();
//             match next_value {
//                 None => {
//                     return Ok(());
//                 }
//                 Some(c) => {
//                     if self.0.contains(c) {
//                         gaze.next();
//                     } else {
//                         return Ok(());
//                     }
//                 }
//             }
//         }
//     }
// }

//// A Step that takes values from the String until the predicate fails.
// pub struct TakeWhile<'a, T>(pub &'a dyn Fn(&str) -> bool, pub T);

// impl<T> Tokenizer<T> for TakeWhile<'_, T>
// where
//     T: Copy,
// {
//     fn attempt(&self, peek: &str, current_match: &str) -> GazeResult<T> {
//         //TODO this will need to be rewritten once handle Unicode better
//         if self.0(peek) {
//             GazeResult::Next
//         } else if current_match.is_empty() {
//             GazeResult::NoMatch
//         } else {
//             GazeResult::Match(self.1)
//         }
//     }

//     fn attempt_end(&self, current_match: &str) -> GazeResultEnd<T> {
//         if current_match.is_empty() {
//             GazeResultEnd::NoMatch
//         } else {
//             GazeResultEnd::Match(self.1)
//         }
//     }
// }

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
