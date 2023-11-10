// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//use gaze::tokenizers::{TakeString, TakeWhile};
use gaze::{
    nibblers::{TakeFirstNblr, TakeNblr},
    Gaze, Nibbler,
};

fn is_text(s: &str) -> bool {
    s.ge("a") && s.le("z")
}

#[derive(Clone, PartialEq, Debug)]
enum Test {
    A,
    B,
    C,
}

fn take_2<I>(gaze: &mut Gaze<I>) -> Option<Vec<I>>
where
    I: Clone,
{
    let x = gaze.next();
    match x {
        None => None,
        Some(xx) => {
            let y = gaze.next();
            match y {
                None => None,
                Some(yy) => Some(vec![xx, yy]),
            }
        }
    }
}

// #[test]
// fn basic_gaze_test() {
//     let v = vec![Test::A, Test::B, Test::C];
//     let mut gaze = Gaze::from_vec(v);
//     let res = gaze.attempt(&take_2);
//     assert!(!gaze.is_complete());
//     assert_eq!(res, Some(vec![Test::A, Test::B]));
// }

// #[test]
// fn basic_gaze_test_str() {
//     let mut gaze: Gaze<&str> = Gaze::<&str>::from_str("abc");
//     let res = gaze.attempt(&take_2);
//     assert!(!gaze.is_complete());
//     assert_eq!(gaze.next(), Some("c"));
//     assert_eq!(res, Some(vec!["a", "b"]));
// }

// #[test]
// fn take_string_no_input() {
//     let mut gaze: Gaze<&str> = Gaze::<&str>::from_str("hello, world");
//     let hello_step = take_string("hello");
//     let no_match_step = take_string("to_match");
//     let rest_step = take_string(", world");

//     let res = gaze.attempt(&hello_step);
//     assert_eq!(res, Some("hello"));
//     assert!(!gaze.is_complete());

//     let res = gaze.attempt(&no_match_step);
//     assert_eq!(res, None);
//     assert!(!gaze.is_complete());

//     let res = gaze.attempt(&rest_step);
//     assert_eq!(res, Some(", world"));
//     assert!(gaze.is_complete());
// }

// #[test]
// fn take_while_basic_test() {
//     let mut gaze: Gaze<&str> = Gaze::<&str>::from_str("hello, world");
//     let res = gaze.attempt(&take_while_str(&is_text));
//     assert_eq!(res, Some(String::from("hello")));
//     assert_eq!(gaze.is_complete(), false);
//     let res = gaze.attempt(&take_string(", world"));
//     assert_eq!(res, Some(", world"));
//     assert_eq!(gaze.is_complete(), true);
// }

// #[test]
// fn take_while_str_closure() {
//     let mut gaze: Gaze<&str> = Gaze::<&str>::from_str("hello, world");
//     let res = gaze.attempt(&take_while_str(&is_text));
//     assert_eq!(res, Some(String::from("hello")));
//     assert_eq!(gaze.is_complete(), false);
//     let res = gaze.attempt(&take_string(", world"));
//     assert_eq!(res, Some(", world"));
//     assert_eq!(gaze.is_complete(), true);
// }

#[test]
fn take_nibbler() {
    let mut gaze: Gaze<Test> = Gaze::from_vec(vec![Test::A, Test::B]);
    let mut nibbler = TakeNblr { to_match: Test::A };
    let res: Result<Option<Test>, ()> = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::A)));
    assert_eq!(gaze.peek(), Some(Test::B));
}

#[test]
fn take_first() {
    let mut gaze = Gaze::from_vec(vec![Test::A, Test::B, Test::C]);
    let mut nibbler = TakeFirstNblr::<Test, Test, ()>(vec![
        Box::new(TakeNblr { to_match: Test::C }),
        Box::new(TakeNblr { to_match: Test::B }),
        Box::new(TakeNblr { to_match: Test::A }),
    ]);
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::A)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::B)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::C)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(None));
}

#[test]
fn take_first_and_map() {
    let mut gaze = Gaze::from_vec(vec![Test::A, Test::B, Test::C]);
    let mut nibbler = TakeFirstNblr::<Test, Test, ()>(vec![
        Box::new(TakeNblr { to_match: Test::C }),
        Box::new(TakeNblr { to_match: Test::B }),
        Box::new(TakeNblr { to_match: Test::A }),
    ]);
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::A)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::B)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(Some(Test::C)));
    let res = gaze.attempt(&mut nibbler);
    assert_eq!(res, Ok(None));
}

// #[test]
// fn handle_empty_string_matcher() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {
//         This,
//         WS,
//         Is,
//         Some,
//         Text,
//     }

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[
//         &TakeString::new("this", TokenType::This),
//         &TakeString::new(" ", TokenType::WS),
//         &TakeString::new("is", TokenType::Is),
//         &TakeString::new("some", TokenType::Some),
//         &TakeString::new("text", TokenType::Text),
//     ];
//     let gaze = Gaze::new(tokenizers.to_vec());

//     let res = gaze.tokenize("");
//     assert_eq!(res, (vec![], ""));
// }

// #[test]
// fn handle_zero_text_zero_tokenizers() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {}

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("");
//     assert_eq!(res, (vec![], ""));
// }

// #[test]
// fn handle_zero_tokenizers() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {}

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("wtf");
//     assert_eq!(res, (vec![], "wtf"));
// }

// #[test]
// fn handle_zero_matches() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {
//         This,
//         WS,
//         Is,
//         Some,
//         Text,
//     }

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[
//         &TakeString::new("this", TokenType::This),
//         &TakeString::new(" ", TokenType::WS),
//         &TakeString::new("is", TokenType::Is),
//         &TakeString::new("some", TokenType::Some),
//         &TakeString::new("text", TokenType::Text),
//     ];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("wtf");
//     assert_eq!(res, (vec![], "wtf"));
// }

// #[test]
// fn handle_partial_matches() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {
//         This,
//         WS,
//         Is,
//         Some,
//         Text,
//     }

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[
//         &TakeString::new("this", TokenType::This),
//         &TakeString::new(" ", TokenType::WS),
//         &TakeString::new("is", TokenType::Is),
//         &TakeString::new("some", TokenType::Some),
//         &TakeString::new("text", TokenType::Text),
//     ];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("this is some wtf");
//     assert_eq!(
//         res,
//         (
//             vec![
//                 GazeToken {
//                     span: "this",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 0,
//                     token_type: TokenType::This
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 4,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "is",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 5,
//                     token_type: TokenType::Is
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 7,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "some",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 8,
//                     token_type: TokenType::Some
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 12,
//                     token_type: TokenType::WS
//                 },
//             ],
//             "wtf"
//         )
//     );
// }

// #[test]
// fn handle_string_matcher() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {
//         This,
//         WS,
//         Is,
//         Some,
//         Text,
//     }

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[
//         &TakeString::new("this", TokenType::This),
//         &TakeString::new(" ", TokenType::WS),
//         &TakeString::new("is", TokenType::Is),
//         &TakeString::new("some", TokenType::Some),
//         &TakeString::new("text", TokenType::Text),
//     ];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("this is some text  ");
//     assert_eq!(
//         res,
//         (
//             vec![
//                 GazeToken {
//                     span: "this",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 0,
//                     token_type: TokenType::This
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 4,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "is",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 5,
//                     token_type: TokenType::Is
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 7,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "some",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 8,
//                     token_type: TokenType::Some
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 12,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "text",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 13,
//                     token_type: TokenType::Text
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 17,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: " ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 18,
//                     token_type: TokenType::WS
//                 },
//             ],
//             ""
//         )
//     );
// }

// #[test]
// fn take_while() {
//     #[derive(PartialEq, Debug, Clone, Copy)]
//     enum TokenType {
//         WS,
//         Text,
//         Digit,
//     }

//     fn is_digit(s: &str) -> bool {
//         s.ge("0") && s.le("9")
//     }

//     fn is_ws(s: &str) -> bool {
//         s.eq(" ")
//     }

//     let tokenizers: &[&dyn Tokenizer<TokenType>] = &[
//         &TakeWhile(&|s: &str| { is_text(s) }, TokenType::Text ),
//         &TakeWhile(&|s: &str| { is_digit(s) }, TokenType::Digit ),
//         &TakeWhile(&|s: &str| { is_ws(s) }, TokenType::WS ),
//     ];
//     let gaze = Gaze::new(tokenizers);

//     let res = gaze.tokenize("234242     3dsflasjfkj    !!   ");
//     assert_eq!(
//         res,
//         (
//             vec![
//                 GazeToken {
//                     span: "234242",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 0,
//                     token_type: TokenType::Digit
//                 },
//                 GazeToken {
//                     span: "     ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 6,
//                     token_type: TokenType::WS
//                 },
//                 GazeToken {
//                     span: "3",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 11,
//                     token_type: TokenType::Digit
//                 },
//                 GazeToken {
//                     span: "dsflasjfkj",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 12,
//                     token_type: TokenType::Text
//                 },
//                 GazeToken {
//                     span: "    ",
//                     // line: 0,
//                     // line_offset: 0,
//                     grapheme_offset: 22,
//                     token_type: TokenType::WS
//                 },
//             ],
//             "!!   "
//         )
//     );
// }

// #[test]
// fn handle_ignore_all() {
//     let mut gaze = Gaze::new("   \t  this \tis some text \t ");
//     let mut hs = HashSet::new();
//     hs.insert(" ");
//     hs.insert("\t");
//     let ignore_all = IgnoreAll(hs);
//     let res = gaze.run(&ignore_all);
//     assert_eq!(res, Ok(()));
//     assert_eq!(gaze.peek(), Some("t".into()));
//     assert_eq!(gaze.is_complete(), false);
//     let res = gaze.run(&ignore_all);
//     assert_eq!(res, Ok(()));
//     assert_eq!(gaze.is_complete(), false);
//     let res = gaze.run(&TakeString::new("this \tis some text"));
//     assert_eq!(res, Ok("this \tis some text".into()));
//     assert_eq!(gaze.is_complete(), false);
//     let res = gaze.run(&ignore_all);
//     assert_eq!(res, Ok(()));
//     assert_eq!(gaze.is_complete(), true);
//     let res = gaze.run(&ignore_all);
//     assert_eq!(res, Ok(()));
//     assert_eq!(gaze.is_complete(), true);
// }

// #[test]
// fn nested_steps() {
//     struct Internal();

//     impl Tokenizer<String> for Internal {
//         fn attempt(&self, gaze: &mut Gaze) -> Result<String, GazeErr> {
//             gaze.run(&TakeString::new("a"))?;
//             gaze.run(&TakeString::new("b"))?;
//             gaze.run(&TakeString::new("c"))?;
//             Ok("abc".into())
//         }
//     }

//     let step = Internal();

//     let mut gaze_pass = Gaze::new("abc");
//     assert_eq!(gaze_pass.run(&step), Ok("abc".into()));
//     assert_eq!(gaze_pass.is_complete(), true);

//     let mut gaze_fail = Gaze::new("abd");
//     assert_eq!(gaze_fail.run(&step), Err(GazeErr::NoMatch));
//     assert_eq!(gaze_fail.is_complete(), false);
//     assert_eq!(gaze_fail.current_offset(), 0);
// }

// #[test]
// fn take_first() {
//     let mut gaze = Gaze::new("abbc");

//     let a = TakeString::new("a");
//     let b = TakeString::new("b");
//     let c = TakeString::new("c");
//     let take_first = TakeFirst(Box::new([&c, &b, &a]));

//     let res = gaze.run(&take_first);
//     assert_eq!(res, Ok("a".into()));
// }

// #[test]
// fn take_all() {
//     let mut gaze = Gaze::new("abbc");

//     let a = TakeString::new("a");
//     let b = TakeString::new("b");
//     let c = TakeString::new("c");
//     let take_all = TakeAll(Box::new([&a, &b, &b, &c]));

//     let res = gaze.run(&take_all);
//     let expect = vec!["a".into(), "b".into(), "b".into(), "c".into()].into_boxed_slice();
//     assert_eq!(res, Ok(expect));
// }
