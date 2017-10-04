use syntax::types;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__SourceExpression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use syntax::types;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_5c_27self_5c_27_22(&'input str),
        Term_22_5c_27strict_2ddynamic_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2deval_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dinline_5c_27_22(&'input str),
        Termr_23_22_28_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_2f_2f_29_3f_28_5c_5c_2a_5c_5c_2e_29_3f_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_28_5c_5c_2e_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_29_2a_28_3a_5c_5c_2a_7c_3a_5b0_2d9_5d_2b_29_3f_28_2f_5ba_2dzA_2dZ0_2d9_5c_5c_2d_5c_5c_2e___7e_5d_2a_29_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_22_23(&'input str),
        NtHostSource(types::Source<'input>),
        NtKeywordSource(types::Source<'input>),
        NtSchemeSource(types::Source<'input>),
        NtSourceExpression(types::Source<'input>),
        Nt____SourceExpression(types::Source<'input>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, 7, 8, 9, 10, 11, 12,
        // State 1
        -10, -10, -10, -10, -10, -10, -10,
        // State 2
        -9, -9, -9, -9, -9, -9, -9,
        // State 3
        -8, -8, -8, -8, -8, -8, -8,
        // State 4
        -11, -11, -11, -11, -11, -11, -11,
        // State 5
        -2, -2, -2, -2, -2, -2, -2,
        // State 6
        -5, -5, -5, -5, -5, -5, -5,
        // State 7
        -4, -4, -4, -4, -4, -4, -4,
        // State 8
        -6, -6, -6, -6, -6, -6, -6,
        // State 9
        -3, -3, -3, -3, -3, -3, -3,
        // State 10
        -1, -1, -1, -1, -1, -1, -1,
        // State 11
        -7, -7, -7, -7, -7, -7, -7,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -10,
        -9,
        -8,
        -11,
        -2,
        -5,
        -4,
        -6,
        -3,
        -1,
        -7,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 4, 5, 0,
        // State 1
        0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""\'self\'""###,
            r###""\'strict-dynamic\'""###,
            r###""\'unsafe-eval\'""###,
            r###""\'unsafe-hashed-attributes\'""###,
            r###""\'unsafe-inline\'""###,
            r###"r#"([a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*://)?(\\*\\.)?[a-zA-Z0-9\\-_]+(\\.[a-zA-Z0-9\\-_]+)*(:\\*|:[0-9]+)?(/[a-zA-Z0-9\\-\\._~]*)*"#"###,
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*:"#"###,
        ];
        __ACTION[(__state * 7)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_SourceExpression<
        'input,
    >(
        input: &'input str,
    ) -> Result<types::Source<'input>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (0, _) if true => 5,
                (1, _) if true => 6,
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
                let __action = __ACTION[__state * 7 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_5c_27self_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_2f_2f_29_3f_28_5c_5c_2a_5c_5c_2e_29_3f_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_28_5c_5c_2e_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_29_2a_28_3a_5c_5c_2a_7c_3a_5b0_2d9_5d_2b_29_3f_28_2f_5ba_2dzA_2dZ0_2d9_5c_5c_2d_5c_5c_2e___7e_5d_2a_29_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_22_23((__tok0)),
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
    ) -> Option<Result<types::Source<'input>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // HostSource = r#"([a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*://)?(\\*\\.)?[a-zA-Z0-9\\-_]+(\\.[a-zA-Z0-9\\-_]+)*(:\\*|:[0-9]+)?(/[a-zA-Z0-9\\-\\._~]*)*"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_28_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_2f_2f_29_3f_28_5c_5c_2a_5c_5c_2e_29_3f_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_28_5c_5c_2e_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_29_2a_28_3a_5c_5c_2a_7c_3a_5b0_2d9_5d_2b_29_3f_28_2f_5ba_2dzA_2dZ0_2d9_5c_5c_2d_5c_5c_2e___7e_5d_2a_29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtHostSource(__nt), __end));
                0
            }
            2 => {
                // KeywordSource = "\'self\'" => ActionFn(6);
                let __sym0 = __pop_Term_22_5c_27self_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                1
            }
            3 => {
                // KeywordSource = "\'unsafe-inline\'" => ActionFn(7);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dinline_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                1
            }
            4 => {
                // KeywordSource = "\'unsafe-eval\'" => ActionFn(8);
                let __sym0 = __pop_Term_22_5c_27unsafe_2deval_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                1
            }
            5 => {
                // KeywordSource = "\'strict-dynamic\'" => ActionFn(9);
                let __sym0 = __pop_Term_22_5c_27strict_2ddynamic_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                1
            }
            6 => {
                // KeywordSource = "\'unsafe-hashed-attributes\'" => ActionFn(10);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                1
            }
            7 => {
                // SchemeSource = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*:"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSchemeSource(__nt), __end));
                2
            }
            8 => {
                // SourceExpression = SchemeSource => ActionFn(1);
                let __sym0 = __pop_NtSchemeSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                3
            }
            9 => {
                // SourceExpression = KeywordSource => ActionFn(2);
                let __sym0 = __pop_NtKeywordSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                3
            }
            10 => {
                // SourceExpression = HostSource => ActionFn(3);
                let __sym0 = __pop_NtHostSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                3
            }
            11 => {
                // __SourceExpression = SourceExpression => ActionFn(0);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 5 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_5c_27self_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27self_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27strict_2ddynamic_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27unsafe_2deval_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_27unsafe_2dinline_5c_27_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_2f_2f_29_3f_28_5c_5c_2a_5c_5c_2e_29_3f_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_28_5c_5c_2e_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_29_2a_28_3a_5c_5c_2a_7c_3a_5b0_2d9_5d_2b_29_3f_28_2f_5ba_2dzA_2dZ0_2d9_5c_5c_2d_5c_5c_2e___7e_5d_2a_29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_2f_2f_29_3f_28_5c_5c_2a_5c_5c_2e_29_3f_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_28_5c_5c_2e_5ba_2dzA_2dZ0_2d9_5c_5c_2d___5d_2b_29_2a_28_3a_5c_5c_2a_7c_3a_5b0_2d9_5d_2b_29_3f_28_2f_5ba_2dzA_2dZ0_2d9_5c_5c_2d_5c_5c_2e___7e_5d_2a_29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_3a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtHostSource<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtHostSource(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtKeywordSource<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtKeywordSource(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSchemeSource<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSchemeSource(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____SourceExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, types::Source<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____SourceExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__SourceExpression::parse_SourceExpression;
mod __intern_token {
    #![allow(unused_imports)]
    use syntax::types;
    use std::str::FromStr;
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
                "^((?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*(?u:://))?((?u:\\*\\.))?(?u:[-0-9A-Z_-_a-z])+((?u:\\.)(?u:[-0-9A-Z_-_a-z])+)*((?u::\\*)|(?u::)(?u:[0-9])+)?((?u:/)(?u:[-\\.-\\.0-9A-Z_-_a-z\\~-\\~])*)*",
                "^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*(?u::)",
                "^(?u:\'self\')",
                "^(?u:\'strict\\-dynamic\')",
                "^(?u:\'unsafe\\-eval\')",
                "^(?u:\'unsafe\\-hashed\\-attributes\')",
                "^(?u:\'unsafe\\-inline\')",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*(?u:://))?((?u:\\*\\.))?(?u:[-0-9A-Z_-_a-z])+((?u:\\.)(?u:[-0-9A-Z_-_a-z])+)*((?u::\\*)|(?u::)(?u:[0-9])+)?((?u:/)(?u:[-\\.-\\.0-9A-Z_-_a-z\\~-\\~])*)*").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*(?u::)").unwrap(),
                __regex::Regex::new("^(?u:\'self\')").unwrap(),
                __regex::Regex::new("^(?u:\'strict\\-dynamic\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-eval\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-hashed\\-attributes\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-inline\')").unwrap(),
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
                    for __i in 0 .. 7 {
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
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::Scheme(&__0[..__0.len()-1])
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::Host(__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::Self_
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeInline
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeEval
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::StrictDynamic
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeHashedAttributes
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
