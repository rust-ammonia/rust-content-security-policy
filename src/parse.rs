use types;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Directive {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_5c_27self_5c_27_22(&'input str),
        Term_22_5c_27strict_2ddynamic_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2deval_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dinline_5c_27_22(&'input str),
        Term_22_3a_22(&'input str),
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
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtKeywordSource(types::Source<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtMediaTypeList(::std::vec::Vec<&'input str>),
        NtScheme(&'input str),
        NtScheme_2a(::std::vec::Vec<&'input str>),
        NtScheme_2b(::std::vec::Vec<&'input str>),
        NtSchemeSource(types::Source<'input>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpression(types::Source<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0, 0,
        // State 1
        -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 2
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 3
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 4
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 5
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 6
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 7
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 8
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 9
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 10
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 11
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 14
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 15
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 16
        0, 24, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 17
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 20
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 21
        0, 0, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 22
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 23
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 24
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 25
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 26
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 27
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 28
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 29
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 30
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 31
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 32
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 33
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 34
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 35
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 36
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 37
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 38
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 39
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43,
        // State 41
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 42
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 43
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0,
        // State 45
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 46
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 47
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 48
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 49
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 50
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 51
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -42,
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
        -37,
        0,
        -36,
        -38,
        -40,
        -13,
        -41,
        -21,
        -24,
        -23,
        -25,
        -22,
        -30,
        -1,
        -2,
        -3,
        -4,
        -5,
        -6,
        -7,
        -8,
        -9,
        -27,
        -29,
        -14,
        -26,
        -33,
        -16,
        -10,
        -11,
        -12,
        -35,
        -39,
        -28,
        -34,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 23, 0, 0, 0, 0,
        // State 3
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 31, 0, 0, 0, 0,
        // State 4
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 32, 0, 0, 0, 0,
        // State 5
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 33, 0, 0, 0, 0,
        // State 6
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 34, 0, 0, 0, 0,
        // State 7
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 35, 0, 0, 0, 0,
        // State 8
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 36, 0, 0, 0, 0,
        // State 9
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 37, 0, 0, 0, 0,
        // State 10
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 38, 0, 0, 0, 0,
        // State 11
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 39, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 40, 41, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 44, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 46, 0, 0, 0, 0,
        // State 15
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 47, 0, 0, 0, 0,
        // State 16
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 21, 22, 48, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 18, 0, 0, 0, 19, 0, 0, 20, 50, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###""\'self\'""###,
            r###""\'strict-dynamic\'""###,
            r###""\'unsafe-eval\'""###,
            r###""\'unsafe-hashed-attributes\'""###,
            r###""\'unsafe-inline\'""###,
            r###"":""###,
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
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"#"###,
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"#"###,
        ];
        __ACTION[(__state * 26)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (17, _) if true => 15,
                (18, _) if true => 16,
                (19, _) if true => 17,
                (20, _) if true => 18,
                (21, _) if true => 19,
                (22, _) if true => 20,
                (23, _) if true => 21,
                (24, _) if true => 22,
                (25, _) if true => 23,
                (0, _) if true => 24,
                (1, _) if true => 25,
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
                let __action = __ACTION[__state * 26 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_5c_27self_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23((__tok0)),
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
                // Directive = "child-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(20);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaTypeList => ActionFn(21);
                let __sym1 = __pop_NtMediaTypeList(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(44);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", Scheme+ => ActionFn(45);
                let __sym1 = __pop_NtScheme_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(4);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(5);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(6);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(7);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // KeywordSource = "\'self\'" => ActionFn(28);
                let __sym0 = __pop_Term_22_5c_27self_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            22 => {
                // KeywordSource = "\'unsafe-inline\'" => ActionFn(29);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dinline_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            23 => {
                // KeywordSource = "\'unsafe-eval\'" => ActionFn(30);
                let __sym0 = __pop_Term_22_5c_27unsafe_2deval_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            24 => {
                // KeywordSource = "\'strict-dynamic\'" => ActionFn(31);
                let __sym0 = __pop_Term_22_5c_27strict_2ddynamic_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            25 => {
                // KeywordSource = "\'unsafe-hashed-attributes\'" => ActionFn(32);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            26 => {
                // MediaType = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                3
            }
            27 => {
                // MediaType+ = MediaType => ActionFn(36);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            28 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(37);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            29 => {
                // MediaTypeList = MediaType+ => ActionFn(34);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaTypeList(__nt), __end));
                5
            }
            30 => {
                // Scheme = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme(__nt), __end));
                6
            }
            31 => {
                // Scheme* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            32 => {
                // Scheme* = Scheme+ => ActionFn(41);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            33 => {
                // Scheme+ = Scheme => ActionFn(42);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            34 => {
                // Scheme+ = Scheme+, Scheme => ActionFn(43);
                let __sym1 = __pop_NtScheme(__symbols);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            35 => {
                // SchemeSource = Scheme, ":" => ActionFn(27);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSchemeSource(__nt), __end));
                9
            }
            36 => {
                // SourceExpression = SchemeSource => ActionFn(25);
                let __sym0 = __pop_NtSchemeSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            37 => {
                // SourceExpression = KeywordSource => ActionFn(26);
                let __sym0 = __pop_NtKeywordSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            38 => {
                // SourceExpression+ = SourceExpression => ActionFn(38);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            39 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(39);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            40 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(23);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            41 => {
                // SourceExpressionList = "\'none\'" => ActionFn(24);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            42 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            43 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____DirectiveSet(__nt), __end));
                14
            }
            44 => {
                // __SourceExpression = SourceExpression => ActionFn(3);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpression(__nt), __end));
                15
            }
            45 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpressionList(__nt), __end));
                16
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 17 + __nonterminal] - 1;
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
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_NtMediaTypeList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaTypeList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2b(__v), __r) => (__l, __v, __r),
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
}
pub use self::__parse__Directive::parse_Directive;

mod __parse__DirectiveSet {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_5c_27self_5c_27_22(&'input str),
        Term_22_5c_27strict_2ddynamic_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2deval_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dinline_5c_27_22(&'input str),
        Term_22_3a_22(&'input str),
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
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtKeywordSource(types::Source<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtMediaTypeList(::std::vec::Vec<&'input str>),
        NtScheme(&'input str),
        NtScheme_2a(::std::vec::Vec<&'input str>),
        NtScheme_2b(::std::vec::Vec<&'input str>),
        NtSchemeSource(types::Source<'input>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpression(types::Source<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 3
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 4
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 5
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 6
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 7
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 8
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 9
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 10
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 11
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 12
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 13
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 16
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 17
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 18
        0, 27, 28, 29, 30, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 19
        4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0, 0,
        // State 20
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 23
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 24
        0, 0, 28, 29, 30, 31, 32, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 25
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 26
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 27
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 28
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 29
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 30
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 31
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 32
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 33
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 34
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 35
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 36
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 37
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 38
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 39
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 40
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 41
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 42
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46,
        // State 44
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 45
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 46
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0,
        // State 48
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 49
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 50
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 51
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 52
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 53
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 54
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 55
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -18,
        -43,
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
        -37,
        0,
        -36,
        -38,
        -40,
        -13,
        -41,
        -21,
        -24,
        -23,
        -25,
        -22,
        -30,
        -1,
        -2,
        -3,
        -4,
        -5,
        -6,
        -7,
        -8,
        -9,
        -27,
        -29,
        -14,
        -26,
        -33,
        -16,
        -10,
        -11,
        -12,
        -20,
        -35,
        -39,
        -28,
        -34,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 26, 0, 0, 0, 0,
        // State 5
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 34, 0, 0, 0, 0,
        // State 6
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 35, 0, 0, 0, 0,
        // State 7
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 36, 0, 0, 0, 0,
        // State 8
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 37, 0, 0, 0, 0,
        // State 9
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 38, 0, 0, 0, 0,
        // State 10
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 39, 0, 0, 0, 0,
        // State 11
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 40, 0, 0, 0, 0,
        // State 12
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 41, 0, 0, 0, 0,
        // State 13
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 42, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 43, 44, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 47, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 49, 0, 0, 0, 0,
        // State 17
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 50, 0, 0, 0, 0,
        // State 18
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 24, 25, 51, 0, 0, 0, 0,
        // State 19
        2, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 21, 0, 0, 0, 22, 0, 0, 23, 54, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###""\'self\'""###,
            r###""\'strict-dynamic\'""###,
            r###""\'unsafe-eval\'""###,
            r###""\'unsafe-hashed-attributes\'""###,
            r###""\'unsafe-inline\'""###,
            r###"":""###,
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
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"#"###,
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"#"###,
        ];
        __ACTION[(__state * 26)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (17, _) if true => 15,
                (18, _) if true => 16,
                (19, _) if true => 17,
                (20, _) if true => 18,
                (21, _) if true => 19,
                (22, _) if true => 20,
                (23, _) if true => 21,
                (24, _) if true => 22,
                (25, _) if true => 23,
                (0, _) if true => 24,
                (1, _) if true => 25,
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
                let __action = __ACTION[__state * 26 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_5c_27self_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23((__tok0)),
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
                // Directive = "child-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(20);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaTypeList => ActionFn(21);
                let __sym1 = __pop_NtMediaTypeList(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(44);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", Scheme+ => ActionFn(45);
                let __sym1 = __pop_NtScheme_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(4);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(5);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(6);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(7);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // KeywordSource = "\'self\'" => ActionFn(28);
                let __sym0 = __pop_Term_22_5c_27self_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            22 => {
                // KeywordSource = "\'unsafe-inline\'" => ActionFn(29);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dinline_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            23 => {
                // KeywordSource = "\'unsafe-eval\'" => ActionFn(30);
                let __sym0 = __pop_Term_22_5c_27unsafe_2deval_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            24 => {
                // KeywordSource = "\'strict-dynamic\'" => ActionFn(31);
                let __sym0 = __pop_Term_22_5c_27strict_2ddynamic_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            25 => {
                // KeywordSource = "\'unsafe-hashed-attributes\'" => ActionFn(32);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            26 => {
                // MediaType = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                3
            }
            27 => {
                // MediaType+ = MediaType => ActionFn(36);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            28 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(37);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            29 => {
                // MediaTypeList = MediaType+ => ActionFn(34);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaTypeList(__nt), __end));
                5
            }
            30 => {
                // Scheme = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme(__nt), __end));
                6
            }
            31 => {
                // Scheme* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            32 => {
                // Scheme* = Scheme+ => ActionFn(41);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            33 => {
                // Scheme+ = Scheme => ActionFn(42);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            34 => {
                // Scheme+ = Scheme+, Scheme => ActionFn(43);
                let __sym1 = __pop_NtScheme(__symbols);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            35 => {
                // SchemeSource = Scheme, ":" => ActionFn(27);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSchemeSource(__nt), __end));
                9
            }
            36 => {
                // SourceExpression = SchemeSource => ActionFn(25);
                let __sym0 = __pop_NtSchemeSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            37 => {
                // SourceExpression = KeywordSource => ActionFn(26);
                let __sym0 = __pop_NtKeywordSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            38 => {
                // SourceExpression+ = SourceExpression => ActionFn(38);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            39 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(39);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            40 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(23);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            41 => {
                // SourceExpressionList = "\'none\'" => ActionFn(24);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            42 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Directive(__nt), __end));
                13
            }
            43 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            44 => {
                // __SourceExpression = SourceExpression => ActionFn(3);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpression(__nt), __end));
                15
            }
            45 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpressionList(__nt), __end));
                16
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 17 + __nonterminal] - 1;
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
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_NtMediaTypeList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaTypeList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2b(__v), __r) => (__l, __v, __r),
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
}
pub use self::__parse__DirectiveSet::parse_DirectiveSet;

mod __parse__SourceExpression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_5c_27self_5c_27_22(&'input str),
        Term_22_5c_27strict_2ddynamic_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2deval_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dinline_5c_27_22(&'input str),
        Term_22_3a_22(&'input str),
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
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtKeywordSource(types::Source<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtMediaTypeList(::std::vec::Vec<&'input str>),
        NtScheme(&'input str),
        NtScheme_2a(::std::vec::Vec<&'input str>),
        NtScheme_2b(::std::vec::Vec<&'input str>),
        NtSchemeSource(types::Source<'input>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpression(types::Source<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 1
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 4
        -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44,
        // State 5
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 6
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 7
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 8
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 9
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 10
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 11
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -37,
        0,
        -36,
        -44,
        -21,
        -24,
        -23,
        -25,
        -22,
        -30,
        -35,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 3, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###""\'self\'""###,
            r###""\'strict-dynamic\'""###,
            r###""\'unsafe-eval\'""###,
            r###""\'unsafe-hashed-attributes\'""###,
            r###""\'unsafe-inline\'""###,
            r###"":""###,
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
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"#"###,
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"#"###,
        ];
        __ACTION[(__state * 26)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (17, _) if true => 15,
                (18, _) if true => 16,
                (19, _) if true => 17,
                (20, _) if true => 18,
                (21, _) if true => 19,
                (22, _) if true => 20,
                (23, _) if true => 21,
                (24, _) if true => 22,
                (25, _) if true => 23,
                (0, _) if true => 24,
                (1, _) if true => 25,
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
                let __action = __ACTION[__state * 26 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_5c_27self_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23((__tok0)),
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
                // Directive = "child-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(20);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaTypeList => ActionFn(21);
                let __sym1 = __pop_NtMediaTypeList(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(44);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", Scheme+ => ActionFn(45);
                let __sym1 = __pop_NtScheme_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(4);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(5);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(6);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(7);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // KeywordSource = "\'self\'" => ActionFn(28);
                let __sym0 = __pop_Term_22_5c_27self_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            22 => {
                // KeywordSource = "\'unsafe-inline\'" => ActionFn(29);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dinline_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            23 => {
                // KeywordSource = "\'unsafe-eval\'" => ActionFn(30);
                let __sym0 = __pop_Term_22_5c_27unsafe_2deval_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            24 => {
                // KeywordSource = "\'strict-dynamic\'" => ActionFn(31);
                let __sym0 = __pop_Term_22_5c_27strict_2ddynamic_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            25 => {
                // KeywordSource = "\'unsafe-hashed-attributes\'" => ActionFn(32);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            26 => {
                // MediaType = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                3
            }
            27 => {
                // MediaType+ = MediaType => ActionFn(36);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            28 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(37);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            29 => {
                // MediaTypeList = MediaType+ => ActionFn(34);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaTypeList(__nt), __end));
                5
            }
            30 => {
                // Scheme = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme(__nt), __end));
                6
            }
            31 => {
                // Scheme* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            32 => {
                // Scheme* = Scheme+ => ActionFn(41);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            33 => {
                // Scheme+ = Scheme => ActionFn(42);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            34 => {
                // Scheme+ = Scheme+, Scheme => ActionFn(43);
                let __sym1 = __pop_NtScheme(__symbols);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            35 => {
                // SchemeSource = Scheme, ":" => ActionFn(27);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSchemeSource(__nt), __end));
                9
            }
            36 => {
                // SourceExpression = SchemeSource => ActionFn(25);
                let __sym0 = __pop_NtSchemeSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            37 => {
                // SourceExpression = KeywordSource => ActionFn(26);
                let __sym0 = __pop_NtKeywordSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            38 => {
                // SourceExpression+ = SourceExpression => ActionFn(38);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            39 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(39);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            40 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(23);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            41 => {
                // SourceExpressionList = "\'none\'" => ActionFn(24);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            42 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Directive(__nt), __end));
                13
            }
            43 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____DirectiveSet(__nt), __end));
                14
            }
            44 => {
                // __SourceExpression = SourceExpression => ActionFn(3);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            45 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpressionList(__nt), __end));
                16
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 17 + __nonterminal] - 1;
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
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_NtMediaTypeList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaTypeList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2b(__v), __r) => (__l, __v, __r),
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
}
pub use self::__parse__SourceExpression::parse_SourceExpression;

mod __parse__SourceExpressionList {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use types;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_22(&'input str),
        Term_22_5c_27none_5c_27_22(&'input str),
        Term_22_5c_27self_5c_27_22(&'input str),
        Term_22_5c_27strict_2ddynamic_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2deval_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(&'input str),
        Term_22_5c_27unsafe_2dinline_5c_27_22(&'input str),
        Term_22_3a_22(&'input str),
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
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(&'input str),
        NtDirective(types::Directive<'input>),
        NtDirectiveSet(types::DirectiveSet<'input>),
        NtKeywordSource(types::Source<'input>),
        NtMediaType(&'input str),
        NtMediaType_2b(::std::vec::Vec<&'input str>),
        NtMediaTypeList(::std::vec::Vec<&'input str>),
        NtScheme(&'input str),
        NtScheme_2a(::std::vec::Vec<&'input str>),
        NtScheme_2b(::std::vec::Vec<&'input str>),
        NtSchemeSource(types::Source<'input>),
        NtSourceExpression(types::Source<'input>),
        NtSourceExpression_2b(::std::vec::Vec<types::Source<'input>>),
        NtSourceExpressionList(Vec<types::Source<'input>>),
        Nt____Directive(types::Directive<'input>),
        Nt____DirectiveSet(types::DirectiveSet<'input>),
        Nt____SourceExpression(types::Source<'input>),
        Nt____SourceExpressionList(Vec<types::Source<'input>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 8, 9, 10, 11, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0,
        // State 1
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 4
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 5
        0, 0, 9, 10, 11, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0,
        // State 6
        -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45,
        // State 7
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 8
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 9
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 10
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 11
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 12
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 13
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 14
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 15
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -37,
        0,
        -36,
        -38,
        -40,
        -45,
        -41,
        -21,
        -24,
        -23,
        -25,
        -22,
        -30,
        -35,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 3, 0, 0, 4, 5, 6, 7, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 2, 0, 0, 0, 3, 0, 0, 4, 16, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""""###,
            r###""\'none\'""###,
            r###""\'self\'""###,
            r###""\'strict-dynamic\'""###,
            r###""\'unsafe-eval\'""###,
            r###""\'unsafe-hashed-attributes\'""###,
            r###""\'unsafe-inline\'""###,
            r###"":""###,
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
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"#"###,
            r###"r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"#"###,
        ];
        __ACTION[(__state * 26)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (13, _) if true => 11,
                (14, _) if true => 12,
                (15, _) if true => 13,
                (16, _) if true => 14,
                (17, _) if true => 15,
                (18, _) if true => 16,
                (19, _) if true => 17,
                (20, _) if true => 18,
                (21, _) if true => 19,
                (22, _) if true => 20,
                (23, _) if true => 21,
                (24, _) if true => 22,
                (25, _) if true => 23,
                (0, _) if true => 24,
                (1, _) if true => 25,
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
                let __action = __ACTION[__state * 26 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_5c_27none_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_5c_27self_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5c_27strict_2ddynamic_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5c_27unsafe_2deval_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5c_27unsafe_2dinline_5c_27_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22base_2duri_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22child_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22connect_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22default_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22font_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22frame_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22img_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22manifest_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::Term_22media_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::Term_22object_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::Term_22plugin_2dtypes_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::Term_22sandbox_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::Term_22script_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22style_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (25, __tok0) => __Symbol::Term_22worker_2dsrc_22((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23((__tok0)),
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
                // Directive = "child-src", SourceExpressionList => ActionFn(8);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22child_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            2 => {
                // Directive = "connect-src", SourceExpressionList => ActionFn(9);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22connect_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            3 => {
                // Directive = "default-src", SourceExpressionList => ActionFn(10);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22default_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            4 => {
                // Directive = "font-src", SourceExpressionList => ActionFn(11);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22font_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            5 => {
                // Directive = "frame-src", SourceExpressionList => ActionFn(12);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22frame_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            6 => {
                // Directive = "img-src", SourceExpressionList => ActionFn(13);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22img_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            7 => {
                // Directive = "manifest-src", SourceExpressionList => ActionFn(14);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22manifest_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            8 => {
                // Directive = "media-src", SourceExpressionList => ActionFn(15);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22media_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            9 => {
                // Directive = "object-src", SourceExpressionList => ActionFn(16);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22object_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            10 => {
                // Directive = "script-src", SourceExpressionList => ActionFn(17);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22script_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            11 => {
                // Directive = "style-src", SourceExpressionList => ActionFn(18);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22style_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            12 => {
                // Directive = "worker-src", SourceExpressionList => ActionFn(19);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22worker_2dsrc_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            13 => {
                // Directive = "base-uri", SourceExpressionList => ActionFn(20);
                let __sym1 = __pop_NtSourceExpressionList(__symbols);
                let __sym0 = __pop_Term_22base_2duri_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            14 => {
                // Directive = "plugin-types", MediaTypeList => ActionFn(21);
                let __sym1 = __pop_NtMediaTypeList(__symbols);
                let __sym0 = __pop_Term_22plugin_2dtypes_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            15 => {
                // Directive = "sandbox" => ActionFn(44);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            16 => {
                // Directive = "sandbox", Scheme+ => ActionFn(45);
                let __sym1 = __pop_NtScheme_2b(__symbols);
                let __sym0 = __pop_Term_22sandbox_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirective(__nt), __end));
                0
            }
            17 => {
                // DirectiveSet = "" => ActionFn(4);
                let __sym0 = __pop_Term_22_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            18 => {
                // DirectiveSet = Directive => ActionFn(5);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            19 => {
                // DirectiveSet = Directive, ";" => ActionFn(6);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            20 => {
                // DirectiveSet = Directive, ";", DirectiveSet => ActionFn(7);
                let __sym2 = __pop_NtDirectiveSet(__symbols);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtDirectiveSet(__nt), __end));
                1
            }
            21 => {
                // KeywordSource = "\'self\'" => ActionFn(28);
                let __sym0 = __pop_Term_22_5c_27self_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            22 => {
                // KeywordSource = "\'unsafe-inline\'" => ActionFn(29);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dinline_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            23 => {
                // KeywordSource = "\'unsafe-eval\'" => ActionFn(30);
                let __sym0 = __pop_Term_22_5c_27unsafe_2deval_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            24 => {
                // KeywordSource = "\'strict-dynamic\'" => ActionFn(31);
                let __sym0 = __pop_Term_22_5c_27strict_2ddynamic_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            25 => {
                // KeywordSource = "\'unsafe-hashed-attributes\'" => ActionFn(32);
                let __sym0 = __pop_Term_22_5c_27unsafe_2dhashed_2dattributes_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtKeywordSource(__nt), __end));
                2
            }
            26 => {
                // MediaType = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+/[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)+"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_2f_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType(__nt), __end));
                3
            }
            27 => {
                // MediaType+ = MediaType => ActionFn(36);
                let __sym0 = __pop_NtMediaType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            28 => {
                // MediaType+ = MediaType+, MediaType => ActionFn(37);
                let __sym1 = __pop_NtMediaType(__symbols);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMediaType_2b(__nt), __end));
                4
            }
            29 => {
                // MediaTypeList = MediaType+ => ActionFn(34);
                let __sym0 = __pop_NtMediaType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMediaTypeList(__nt), __end));
                5
            }
            30 => {
                // Scheme = r#"[a-zA-Z]([a-zA-Z]|[0-9]|\\+|\\-|\\.)*"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme(__nt), __end));
                6
            }
            31 => {
                // Scheme* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            32 => {
                // Scheme* = Scheme+ => ActionFn(41);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2a(__nt), __end));
                7
            }
            33 => {
                // Scheme+ = Scheme => ActionFn(42);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            34 => {
                // Scheme+ = Scheme+, Scheme => ActionFn(43);
                let __sym1 = __pop_NtScheme(__symbols);
                let __sym0 = __pop_NtScheme_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtScheme_2b(__nt), __end));
                8
            }
            35 => {
                // SchemeSource = Scheme, ":" => ActionFn(27);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtScheme(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSchemeSource(__nt), __end));
                9
            }
            36 => {
                // SourceExpression = SchemeSource => ActionFn(25);
                let __sym0 = __pop_NtSchemeSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            37 => {
                // SourceExpression = KeywordSource => ActionFn(26);
                let __sym0 = __pop_NtKeywordSource(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression(__nt), __end));
                10
            }
            38 => {
                // SourceExpression+ = SourceExpression => ActionFn(38);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            39 => {
                // SourceExpression+ = SourceExpression+, SourceExpression => ActionFn(39);
                let __sym1 = __pop_NtSourceExpression(__symbols);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtSourceExpression_2b(__nt), __end));
                11
            }
            40 => {
                // SourceExpressionList = SourceExpression+ => ActionFn(23);
                let __sym0 = __pop_NtSourceExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            41 => {
                // SourceExpressionList = "\'none\'" => ActionFn(24);
                let __sym0 = __pop_Term_22_5c_27none_5c_27_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSourceExpressionList(__nt), __end));
                12
            }
            42 => {
                // __Directive = Directive => ActionFn(1);
                let __sym0 = __pop_NtDirective(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Directive(__nt), __end));
                13
            }
            43 => {
                // __DirectiveSet = DirectiveSet => ActionFn(0);
                let __sym0 = __pop_NtDirectiveSet(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____DirectiveSet(__nt), __end));
                14
            }
            44 => {
                // __SourceExpression = SourceExpression => ActionFn(3);
                let __sym0 = __pop_NtSourceExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____SourceExpression(__nt), __end));
                15
            }
            45 => {
                // __SourceExpressionList = SourceExpressionList => ActionFn(2);
                let __sym0 = __pop_NtSourceExpressionList(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 17 + __nonterminal] - 1;
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
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_28_5ba_2dzA_2dZ_5d_7c_5b0_2d9_5d_7c_5c_5c_2b_7c_5c_5c_2d_7c_5c_5c_2e_29_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
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
    fn __pop_NtMediaTypeList<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMediaTypeList(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtScheme_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtScheme_2b(__v), __r) => (__l, __v, __r),
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
}
pub use self::__parse__SourceExpressionList::parse_SourceExpressionList;
mod __intern_token {
    #![allow(unused_imports)]
    use types;
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
                "^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*",
                "^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+(?u:/)(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+",
                "^",
                "^(?u:\'none\')",
                "^(?u:\'self\')",
                "^(?u:\'strict\\-dynamic\')",
                "^(?u:\'unsafe\\-eval\')",
                "^(?u:\'unsafe\\-hashed\\-attributes\')",
                "^(?u:\'unsafe\\-inline\')",
                "^(?u::)",
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
                __regex::Regex::new("^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))*").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+(?u:/)(?u:[A-Za-z])((?u:[A-Za-z])|(?u:[0-9])|(?u:\\+)|(?u:\\-)|(?u:\\.))+").unwrap(),
                __regex::Regex::new("^").unwrap(),
                __regex::Regex::new("^(?u:\'none\')").unwrap(),
                __regex::Regex::new("^(?u:\'self\')").unwrap(),
                __regex::Regex::new("^(?u:\'strict\\-dynamic\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-eval\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-hashed\\-attributes\')").unwrap(),
                __regex::Regex::new("^(?u:\'unsafe\\-inline\')").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
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
                    for __i in 0 .. 26 {
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
) -> types::DirectiveSet<'input>
{
    types::DirectiveSet::new()
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Directive<'input>, usize),
) -> types::DirectiveSet<'input>
{
    types::DirectiveSet::with(__0)
}

#[allow(unused_variables)]
fn __action6<
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
fn __action7<
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
fn __action8<
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
fn __action9<
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
fn __action10<
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
fn __action11<
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
fn __action12<
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
fn __action13<
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
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
fn __action18<
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
fn __action19<
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
fn __action20<
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
fn __action21<
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
fn __action22<
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
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<types::Source<'input>>, usize),
) -> Vec<types::Source<'input>>
{
    (__0)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<types::Source<'input>>
{
    Vec::new()
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> types::Source<'input>
{
    (__0)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::Scheme(__0)
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::Self_
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeInline
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeEval
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::StrictDynamic
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> types::Source<'input>
{
    types::Source::UnsafeHashedAttributes
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    (__0)
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action37<
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
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, types::Source<'input>, usize),
) -> ::std::vec::Vec<types::Source<'input>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action39<
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
fn __action40<
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
fn __action41<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action43<
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
fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> types::Directive<'input>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
) -> types::Directive<'input>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
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
