use syntax::media_type;
use syntax::source_expression;
use syntax::types;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Directive {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use syntax::media_type;
    use syntax::source_expression;
    use syntax::types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22base_2duri_22(&'input str),
        Term_22child_2dsrc_22(&'input str),
        Term_22connect_2dsrc_22(&'input str),
        Term_22default_2dsrc_22(&'input str),
        Term_22font_2dsrc_22(&'input str),
        Term_22frame_2dsrc_22(&'input str),
        Term_22img_2dsrc_22(&'input str),
        Term_22manifest_2dsrc_22(&'input str),
        Term_22media_2dsrc_22(&'input str),
        Term_22object_2dsrc_22(&'input str),
        Term_22plugin_2dtypes_22(&'input str),
        Term_22sandbox_22(&'input str),
        Term_22script_2dsrc_22(&'input str),
        Term_22style_2dsrc_22(&'input str),
        Term_22worker_2dsrc_22(&'input str),
        Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(::std::vec::Vec<&'input str>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(::std::vec::Vec<&'input str>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0,
        // State 1
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 2
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 3
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 4
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 5
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 6
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 7
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 8
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 9
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 10
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 11
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36,
        // State 14
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 15
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 16
        0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 17
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 19
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 20
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 21
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 22
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 23
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 24
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 25
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 26
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 27
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 28
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 29
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 30
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 31
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34,
        // State 33
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42,
        // State 35
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 36
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 37
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 38
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 39
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 40
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 41
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -29,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -15,
        0,
        0,
        0,
        -25,
        -27,
        -13,
        -28,
        -24,
        -1,
        -2,
        -3,
        -4,
        -5,
        -6,
        -7,
        -8,
        -9,
        -22,
        -14,
        -21,
        -16,
        -34,
        -10,
        -11,
        -12,
        -26,
        -23,
        -35,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 18, 19, 20, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 18, 19, 23, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 18, 19, 24, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 18, 19, 25, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 18, 19, 26, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 18, 19, 27, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 18, 19, 28, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 18, 19, 29, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 18, 19, 30, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 18, 19, 31, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 32, 33, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35,
        // State 14
        0, 0, 0, 0, 18, 19, 37, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 18, 19, 38, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 18, 19, 39, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###"";""###,
            r###""base-uri""###,
            r###""child-src""###,
            r###""connect-src""###,
            r###""default-src""###,
            r###""font-src""###,
            r###""frame-src""###,
            r###""img-src""###,
            r###""manifest-src""###,
            r###""media-src""###,
            r###""object-src""###,
            r###""plugin-types""###,
            r###""sandbox""###,
            r###""script-src""###,
            r###""style-src""###,
            r###""worker-src""###,
            r###"r#"[^\\s;]+"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Directive<
        'input,
    >(
        input: &'input str,
    ) -> Result<types::Directive<'input>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (1, _) if true => 0,
                (2, _) if true => 1,
                (3, _) if true => 2,
                (4, _) if true => 3,
                (5, _) if true => 4,
                (6, _) if true => 5,
                (7, _) if true => 6,
                (8, _) if true => 7,
                (9, _) if true => 8,
                (10, _) if true => 9,
                (11, _) if true => 10,
                (12, _) if true => 11,
                (13, _) if true => 12,
                (14, _) if true => 13,
                (15, _) if true => 14,
                (16, _) if true => 15,
                (17, _) if true => 16,
                (18, _) if true => 17,
                (0, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<types::Directive<'input>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Directive = "child-src", SourceExpressionList => ActionFn(7);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaType+ => ActionFn(20);
                let __sym1 = __pop_NtMediaType_2b(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(34);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", r#"[^\\s;]+"#+ => ActionFn(35);
                let __sym1 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(3);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(4);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(5);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(6);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // MediaType = r#"[^\\s;]+"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action25::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                2
            }
            22 => {
                // MediaType+ = MediaType => ActionFn(30);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            23 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(31);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            24 => {
                // SourceExpression = r#"[^\\s;]+"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action24::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                4
            }
            25 => {
                // SourceExpression+ = SourceExpression => ActionFn(26);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            26 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(27);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            27 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(22);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            28 => {
                // SourceExpressionList = "\'none\'" => ActionFn(23);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            29 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            30 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____DirectiveSet(__nt), __end));
                8
            }
            31 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpressionList(__nt), __end));
                9
            }
            32 => {
                // r#"[^\\s;]+"#* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            33 => {
                // r#"[^\\s;]+"#* = r#"[^\\s;]+"#+ => ActionFn(29);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            34 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"# => ActionFn(32);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            35 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"#+, r#"[^\\s;]+"# => ActionFn(33);
                let __sym1 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 12 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27none_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27none_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22base_2duri_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22base_2duri_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22child_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22child_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22connect_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22connect_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22default_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22default_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22font_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22font_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22frame_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22frame_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22img_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22img_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22manifest_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22manifest_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22media_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22media_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22object_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22object_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22plugin_2dtypes_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22plugin_2dtypes_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22sandbox_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22sandbox_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22script_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22script_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22style_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22style_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22worker_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22worker_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirective<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirective(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Directive<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Directive(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____DirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____DirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Directive::parse_Directive;

mod __parse__DirectiveSet {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use syntax::media_type;
    use syntax::source_expression;
    use syntax::types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22base_2duri_22(&'input str),
        Term_22child_2dsrc_22(&'input str),
        Term_22connect_2dsrc_22(&'input str),
        Term_22default_2dsrc_22(&'input str),
        Term_22font_2dsrc_22(&'input str),
        Term_22frame_2dsrc_22(&'input str),
        Term_22img_2dsrc_22(&'input str),
        Term_22manifest_2dsrc_22(&'input str),
        Term_22media_2dsrc_22(&'input str),
        Term_22object_2dsrc_22(&'input str),
        Term_22plugin_2dtypes_22(&'input str),
        Term_22sandbox_22(&'input str),
        Term_22script_2dsrc_22(&'input str),
        Term_22style_2dsrc_22(&'input str),
        Term_22worker_2dsrc_22(&'input str),
        Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(::std::vec::Vec<&'input str>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(::std::vec::Vec<&'input str>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        4, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0,
        // State 1
        0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 3
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 4
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 5
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 6
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 7
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 8
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 9
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 10
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 11
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 12
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 13
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 15
        0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39,
        // State 16
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 17
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 18
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 19
        4, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0,
        // State 20
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 21
        0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 22
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 23
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 24
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 25
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 26
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 27
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 28
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 29
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 30
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 31
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 32
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 33
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 34
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 35
        0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 36
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 37
        0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46,
        // State 38
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 39
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 40
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 41
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 42
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 43
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 44
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 45
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -18,
        -30,
        -17,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -15,
        0,
        0,
        0,
        -19,
        -25,
        -27,
        -13,
        -28,
        -24,
        -1,
        -2,
        -3,
        -4,
        -5,
        -6,
        -7,
        -8,
        -9,
        -22,
        -14,
        -21,
        -16,
        -34,
        -10,
        -11,
        -12,
        -20,
        -26,
        -23,
        -35,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 21, 22, 23, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 21, 22, 26, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 21, 22, 27, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 21, 22, 28, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 21, 22, 29, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 21, 22, 30, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 21, 22, 31, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 21, 22, 32, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 21, 22, 33, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 21, 22, 34, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 35, 36, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38,
        // State 16
        0, 0, 0, 0, 21, 22, 40, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 21, 22, 41, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 21, 22, 42, 0, 0, 0, 0, 0,
        // State 19
        2, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###"";""###,
            r###""base-uri""###,
            r###""child-src""###,
            r###""connect-src""###,
            r###""default-src""###,
            r###""font-src""###,
            r###""frame-src""###,
            r###""img-src""###,
            r###""manifest-src""###,
            r###""media-src""###,
            r###""object-src""###,
            r###""plugin-types""###,
            r###""sandbox""###,
            r###""script-src""###,
            r###""style-src""###,
            r###""worker-src""###,
            r###"r#"[^\\s;]+"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_DirectiveSet<
        'input,
    >(
        input: &'input str,
    ) -> Result<types::DirectiveSet<'input>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (1, _) if true => 0,
                (2, _) if true => 1,
                (3, _) if true => 2,
                (4, _) if true => 3,
                (5, _) if true => 4,
                (6, _) if true => 5,
                (7, _) if true => 6,
                (8, _) if true => 7,
                (9, _) if true => 8,
                (10, _) if true => 9,
                (11, _) if true => 10,
                (12, _) if true => 11,
                (13, _) if true => 12,
                (14, _) if true => 13,
                (15, _) if true => 14,
                (16, _) if true => 15,
                (17, _) if true => 16,
                (18, _) if true => 17,
                (0, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<types::DirectiveSet<'input>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Directive = "child-src", SourceExpressionList => ActionFn(7);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaType+ => ActionFn(20);
                let __sym1 = __pop_NtMediaType_2b(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(34);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", r#"[^\\s;]+"#+ => ActionFn(35);
                let __sym1 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(3);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(4);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(5);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(6);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // MediaType = r#"[^\\s;]+"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action25::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                2
            }
            22 => {
                // MediaType+ = MediaType => ActionFn(30);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            23 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(31);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            24 => {
                // SourceExpression = r#"[^\\s;]+"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action24::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                4
            }
            25 => {
                // SourceExpression+ = SourceExpression => ActionFn(26);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            26 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(27);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            27 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(22);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            28 => {
                // SourceExpressionList = "\'none\'" => ActionFn(23);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            29 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Directive(__nt), __end));
                7
            }
            30 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            31 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpressionList(__nt), __end));
                9
            }
            32 => {
                // r#"[^\\s;]+"#* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            33 => {
                // r#"[^\\s;]+"#* = r#"[^\\s;]+"#+ => ActionFn(29);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            34 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"# => ActionFn(32);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            35 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"#+, r#"[^\\s;]+"# => ActionFn(33);
                let __sym1 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 12 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27none_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27none_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22base_2duri_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22base_2duri_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22child_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22child_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22connect_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22connect_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22default_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22default_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22font_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22font_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22frame_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22frame_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22img_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22img_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22manifest_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22manifest_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22media_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22media_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22object_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22object_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22plugin_2dtypes_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22plugin_2dtypes_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22sandbox_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22sandbox_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22script_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22script_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22style_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22style_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22worker_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22worker_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirective<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirective(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Directive<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Directive(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____DirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____DirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__DirectiveSet::parse_DirectiveSet;

mod __parse__SourceExpressionList {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use syntax::media_type;
    use syntax::source_expression;
    use syntax::types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22base_2duri_22(&'input str),
        Term_22child_2dsrc_22(&'input str),
        Term_22connect_2dsrc_22(&'input str),
        Term_22default_2dsrc_22(&'input str),
        Term_22font_2dsrc_22(&'input str),
        Term_22frame_2dsrc_22(&'input str),
        Term_22img_2dsrc_22(&'input str),
        Term_22manifest_2dsrc_22(&'input str),
        Term_22media_2dsrc_22(&'input str),
        Term_22object_2dsrc_22(&'input str),
        Term_22plugin_2dtypes_22(&'input str),
        Term_22sandbox_22(&'input str),
        Term_22script_2dsrc_22(&'input str),
        Term_22style_2dsrc_22(&'input str),
        Term_22worker_2dsrc_22(&'input str),
        Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(::std::vec::Vec<&'input str>),
        Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(::std::vec::Vec<&'input str>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        // State 1
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        // State 3
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 4
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 5
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 6
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -25,
        -27,
        -31,
        -28,
        -24,
        -26,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 2, 3, 4, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###"";""###,
            r###""base-uri""###,
            r###""child-src""###,
            r###""connect-src""###,
            r###""default-src""###,
            r###""font-src""###,
            r###""frame-src""###,
            r###""img-src""###,
            r###""manifest-src""###,
            r###""media-src""###,
            r###""object-src""###,
            r###""plugin-types""###,
            r###""sandbox""###,
            r###""script-src""###,
            r###""style-src""###,
            r###""worker-src""###,
            r###"r#"[^\\s;]+"#"###,
        ];
        __ACTION[(__state * 19)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_SourceExpressionList<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<types::Source<'input>>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (1, _) if true => 0,
                (2, _) if true => 1,
                (3, _) if true => 2,
                (4, _) if true => 3,
                (5, _) if true => 4,
                (6, _) if true => 5,
                (7, _) if true => 6,
                (8, _) if true => 7,
                (9, _) if true => 8,
                (10, _) if true => 9,
                (11, _) if true => 10,
                (12, _) if true => 11,
                (13, _) if true => 12,
                (14, _) if true => 13,
                (15, _) if true => 14,
                (16, _) if true => 15,
                (17, _) if true => 16,
                (18, _) if true => 17,
                (0, _) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 19 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<types::Source<'input>>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Directive = "child-src", SourceExpressionList => ActionFn(7);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaType+ => ActionFn(20);
                let __sym1 = __pop_NtMediaType_2b(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(34);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", r#"[^\\s;]+"#+ => ActionFn(35);
                let __sym1 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(3);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(4);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(5);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(6);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // MediaType = r#"[^\\s;]+"# => ActionFn(25);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action25::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                2
            }
            22 => {
                // MediaType+ = MediaType => ActionFn(30);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            23 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(31);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                3
            }
            24 => {
                // SourceExpression = r#"[^\\s;]+"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action24::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                4
            }
            25 => {
                // SourceExpression+ = SourceExpression => ActionFn(26);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            26 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(27);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                5
            }
            27 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(22);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            28 => {
                // SourceExpressionList = "\'none\'" => ActionFn(23);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                6
            }
            29 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Directive(__nt), __end));
                7
            }
            30 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____DirectiveSet(__nt), __end));
                8
            }
            31 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            32 => {
                // r#"[^\\s;]+"#* =  => ActionFn(28);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action28::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            33 => {
                // r#"[^\\s;]+"#* = r#"[^\\s;]+"#+ => ActionFn(29);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__nt), __end));
                10
            }
            34 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"# => ActionFn(32);
                let __sym0 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            35 => {
                // r#"[^\\s;]+"#+ = r#"[^\\s;]+"#+, r#"[^\\s;]+"# => ActionFn(33);
                let __sym1 = __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__symbols);
                let __sym0 = __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__nt), __end));
                11
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 12 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27none_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27none_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22base_2duri_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22base_2duri_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22child_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22child_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22connect_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22connect_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22default_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22default_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22font_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22font_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22frame_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22frame_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22img_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22img_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22manifest_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22manifest_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22media_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22media_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22object_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22object_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22plugin_2dtypes_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22plugin_2dtypes_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22sandbox_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22sandbox_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22script_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22script_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22style_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22style_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22worker_2dsrc_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22worker_2dsrc_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirective<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirective(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMediaType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpression_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpression_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Directive<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Directive<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Directive(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____DirectiveSet<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::DirectiveSet<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____DirectiveSet(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____SourceExpressionList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<types::Source<'input>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SourceExpressionList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntr_23_22_5b_5e_5c_5cs_3b_5d_2b_22_23_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__SourceExpressionList::parse_SourceExpressionList;
mod __intern_token {
    #![allow(unused_imports)]
    use syntax::media_type;
    use syntax::source_expression;
    use syntax::types;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+",
                "^",
                "^(?u:\'none\')",
                "^(?u:;)",
                "^(?u:base\\-uri)",
                "^(?u:child\\-src)",
                "^(?u:connect\\-src)",
                "^(?u:default\\-src)",
                "^(?u:font\\-src)",
                "^(?u:frame\\-src)",
                "^(?u:img\\-src)",
                "^(?u:manifest\\-src)",
                "^(?u:media\\-src)",
                "^(?u:object\\-src)",
                "^(?u:plugin\\-types)",
                "^(?u:sandbox)",
                "^(?u:script\\-src)",
                "^(?u:style\\-src)",
                "^(?u:worker\\-src)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+").unwrap(),
                __regex::Regex::new("^").unwrap(),
                __regex::Regex::new("^(?u:\'none\')").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:base\\-uri)").unwrap(),
                __regex::Regex::new("^(?u:child\\-src)").unwrap(),
                __regex::Regex::new("^(?u:connect\\-src)").unwrap(),
                __regex::Regex::new("^(?u:default\\-src)").unwrap(),
                __regex::Regex::new("^(?u:font\\-src)").unwrap(),
                __regex::Regex::new("^(?u:frame\\-src)").unwrap(),
                __regex::Regex::new("^(?u:img\\-src)").unwrap(),
                __regex::Regex::new("^(?u:manifest\\-src)").unwrap(),
                __regex::Regex::new("^(?u:media\\-src)").unwrap(),
                __regex::Regex::new("^(?u:object\\-src)").unwrap(),
                __regex::Regex::new("^(?u:plugin\\-types)").unwrap(),
                __regex::Regex::new("^(?u:sandbox)").unwrap(),
                __regex::Regex::new("^(?u:script\\-src)").unwrap(),
                __regex::Regex::new("^(?u:style\\-src)").unwrap(),
                __regex::Regex::new("^(?u:worker\\-src)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 19 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::DirectiveSet<'input>, usize),
) -> types::DirectiveSet<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Directive<'input>, usize),
) -> types::Directive<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> Vec<types::Source<'input>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::DirectiveSet<'input>
{
    types::DirectiveSet::new()
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Directive<'input>, usize),
) -> types::DirectiveSet<'input>
{
    types::DirectiveSet::with(__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Directive<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> types::DirectiveSet<'input>
{
    types::DirectiveSet::with(__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, d, _): (usize, types::Directive<'input>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, types::DirectiveSet<'input>, usize),
) -> types::DirectiveSet<'input>
{
    {let mut s = s; s.add_directive(d); s}
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ChildSrc(__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ConnectSrc(__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::DefaultSrc(__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::FontSrc(__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::FrameSrc(__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ImgSrc(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ManifestSrc(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::MediaSrc(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ObjectSrc(__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::ScriptSrc(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::StyleSrc(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::WorkerSrc(__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<types::Source<'input>>, usize),
) -> types::Directive<'input>
{
    types::Directive::BaseUri(__0)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> types::Directive<'input>
{
    types::Directive::PluginTypes(__0)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> types::Directive<'input>
{
    types::Directive::Sandbox
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<types::Source<'input>>, usize),
) -> Vec<types::Source<'input>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<types::Source<'input>>
{
    Vec::new()
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<types::Source<'input>,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>
{
    source_expression::parse_SourceExpression(__0)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<&'input str,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>
{
    media_type::parse_MediaType(__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> ::std::vec::Vec<types::Source<'input>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<types::Source<'input>>, usize),
    (_, e, _): (usize, types::Source<'input>, usize),
) -> ::std::vec::Vec<types::Source<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> types::Directive<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action28(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
) -> types::Directive<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
