// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::tokenizers::TakeString;
use gaze::{Gaze, GazeToken, Tokenizer};

#[test]
fn handle_empty_string_matcher() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let t1 = TakeString::new("this", TokenType::This);
    let t2 = TakeString::new(" ", TokenType::WS);
    let t3 = TakeString::new("is", TokenType::Is);
    let t4 = TakeString::new("some", TokenType::Some);
    let t5 = TakeString::new("text", TokenType::Text);
    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[&t1, &t2, &t3, &t4, &t5];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("");
    assert_eq!(res, (vec![], ""));
}

#[test]
fn handle_zero_text_zero_tokenizers() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("");
    assert_eq!(res, (vec![], ""));
}

#[test]
fn handle_zero_tokenizers() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("wtf");
    assert_eq!(res, (vec![], "wtf"));
}

#[test]
fn handle_zero_matches() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let t1 = TakeString::new("this", TokenType::This);
    let t2 = TakeString::new(" ", TokenType::WS);
    let t3 = TakeString::new("is", TokenType::Is);
    let t4 = TakeString::new("some", TokenType::Some);
    let t5 = TakeString::new("text", TokenType::Text);
    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[&t1, &t2, &t3, &t4, &t5];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("wtf");
    assert_eq!(res, (vec![], "wtf"));
}

#[test]
fn handle_partial_matches() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let t1 = TakeString::new("this", TokenType::This);
    let t2 = TakeString::new(" ", TokenType::WS);
    let t3 = TakeString::new("is", TokenType::Is);
    let t4 = TakeString::new("some", TokenType::Some);
    let t5 = TakeString::new("text", TokenType::Text);
    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[&t1, &t2, &t3, &t4, &t5];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("this is some wtf");
    assert_eq!(
        res,
        (
            vec![
                GazeToken {
                    span: "this",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 0,
                    token_type: TokenType::This
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 4,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: "is",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 5,
                    token_type: TokenType::Is
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 7,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: "some",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 8,
                    token_type: TokenType::Some
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 12,
                    token_type: TokenType::WS
                },
            ],
            "wtf"
        )
    );
}

#[test]
fn handle_string_matcher() {
    #[derive(PartialEq, Debug, Clone, Copy)]
    enum TokenType {
        This,
        WS,
        Is,
        Some,
        Text,
    }

    let t1 = TakeString::new("this", TokenType::This);
    let t2 = TakeString::new(" ", TokenType::WS);
    let t3 = TakeString::new("is", TokenType::Is);
    let t4 = TakeString::new("some", TokenType::Some);
    let t5 = TakeString::new("text", TokenType::Text);
    let tokenizers: &[&dyn Tokenizer<TokenType>] = &[&t1, &t2, &t3, &t4, &t5];
    let gaze = Gaze::new(tokenizers);

    let res = gaze.tokenize("this is some text  ");
    assert_eq!(
        res,
        (
            vec![
                GazeToken {
                    span: "this",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 0,
                    token_type: TokenType::This
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 4,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: "is",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 5,
                    token_type: TokenType::Is
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 7,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: "some",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 8,
                    token_type: TokenType::Some
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 12,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: "text",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 13,
                    token_type: TokenType::Text
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 17,
                    token_type: TokenType::WS
                },
                GazeToken {
                    span: " ",
                    // line: 0,
                    // line_offset: 0,
                    grapheme_offset: 18,
                    token_type: TokenType::WS
                },
            ],
            ""
        )
    );
}

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
