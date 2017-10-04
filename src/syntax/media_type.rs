extern crate lalrpop_util as __lalrpop_util;

mod __parse__MediaType {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(&'input str),
        NtMediaType(&'input str),
        Nt____MediaType(&'input str),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3,
        // State 1
        -2,
        // State 2
        -1,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -2,
        -1,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0,
        // State 1
        0, 0,
        // State 2
        0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"#"###,
        ];
        __ACTION[(__state * 1)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_MediaType<
        'input,
    >(
        input: &'input str,
    ) -> Result<&'input str, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (0, _) if true => 0,
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
                let __action = __ACTION[__state * 1 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23((__tok0)),
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
    ) -> Option<Result<&'input str,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // MediaType = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"# => ActionFn(1);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                0
            }
            2 => {
                // __MediaType = MediaType => ActionFn(0);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 2 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____MediaType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____MediaType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__MediaType::parse_MediaType;
mod __intern_token {
    #![allow(unused_imports)]
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
                "^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+(?u:/)(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+(?u:/)(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+").unwrap(),
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
                    for __i in 0 .. 1 {
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
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
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
