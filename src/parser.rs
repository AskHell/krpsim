// auto-generated: "lalrpop 0.17.2"
// sha256: 3f886b5e444bb8b7156de6ea5aeb19f328eb9873567ec4e0297e1f20bbd86d
use std::str::FromStr;
use std::collections::HashMap;
use crate::ast::{
    Inventory,
    Process,
    Simulation,
    convert
};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Simulation {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use std::collections::HashMap;
    use crate::ast::{
    Inventory,
    Process,
    Simulation,
    convert
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1((String, u32)),
        Variant2(::std::vec::Vec<(String, u32)>),
        Variant3(String),
        Variant4(::std::vec::Vec<String>),
        Variant5(::std::option::Option<(String, u32)>),
        Variant6(Inventory),
        Variant7((::std::vec::Vec<&'input str>, &'input str)),
        Variant8(::std::vec::Vec<&'input str>),
        Variant9(::std::option::Option<String>),
        Variant10(Vec<String>),
        Variant11(u32),
        Variant12((Vec<String>, bool)),
        Variant13(Process),
        Variant14(Simulation),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 9, 10, 0, 11,
        // State 1
        0, 0, 0, 0, 9, 10, 0, 11,
        // State 2
        -23, -23, -23, -23, 0, -23, -23, -23,
        // State 3
        0, 0, 0, 0, 0, 10, 0, 14,
        // State 4
        0, 0, 17, 0, 0, 10, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 10, 0, 0,
        // State 6
        0, 0, 0, 0, 9, 10, 0, 11,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 17, 0, 0, 10, 0, 0,
        // State 9
        -20, -20, -20, -20, 0, -20, -20, -20,
        // State 10
        0, -27, -27, -27, 0, -27, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -24, -24, -24, -24, 0, -24, -24, -24,
        // State 13
        0, -28, -28, -28, 0, -28, 0, 0,
        // State 14
        0, 0, 21, 0, 0, 10, 0, 0,
        // State 15
        26, 0, 0, 0, 0, 10, 27, 0,
        // State 16
        -25, 0, 0, 0, 0, -25, -25, 0,
        // State 17
        0, 0, 0, 0, 0, 10, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        26, 0, 0, 0, 0, 10, 0, 0,
        // State 20
        -26, 0, 0, 0, 0, -26, -26, 0,
        // State 21
        0, 0, 17, 0, 0, 10, 0, 0,
        // State 22
        32, 0, 0, 0, 0, 10, 33, 0,
        // State 23
        0, -11, 0, -11, -11, -11, 0, -11,
        // State 24
        0, 39, 0, 0, 0, 10, 0, 11,
        // State 25
        0, -37, 0, 0, 0, -37, 0, -37,
        // State 26
        0, -35, 0, -35, -35, -35, 0, -35,
        // State 27
        32, 0, 0, 0, 0, 10, 0, 0,
        // State 28
        0, 0, 0, 0, 0, -39, 0, 0,
        // State 29
        0, 39, 0, 0, 0, 10, 0, 11,
        // State 30
        26, 0, 0, 0, 0, 10, 0, 0,
        // State 31
        0, -38, 0, 0, 0, -38, 0, -38,
        // State 32
        0, -36, 0, -36, -36, -36, 0, -36,
        // State 33
        0, 39, 0, 0, 0, 10, 0, 11,
        // State 34
        0, 39, 0, 49, 0, 10, 0, 0,
        // State 35
        0, 0, -15, 0, 0, -15, 0, 0,
        // State 36
        0, 50, 0, 0, 0, 10, 0, 14,
        // State 37
        0, 0, 17, 0, 0, 10, 0, 0,
        // State 38
        0, 0, -18, 0, 0, -18, 0, 0,
        // State 39
        0, 39, 0, 0, 0, 10, 0, 11,
        // State 40
        0, 0, 0, 0, 0, -32, 0, 0,
        // State 41
        0, 39, 0, 49, 0, 10, 0, 0,
        // State 42
        0, 0, 17, 0, 0, 10, 0, 0,
        // State 43
        0, 39, 0, 49, 0, 10, 0, 0,
        // State 44
        0, 0, -17, 0, 0, -17, 0, 0,
        // State 45
        0, 0, -14, 0, 0, -14, 0, 0,
        // State 46
        0, 50, 0, 59, 0, 10, 0, 0,
        // State 47
        0, -4, 0, 0, 0, -4, 0, -4,
        // State 48
        0, -41, 0, 0, 0, -41, 0, -41,
        // State 49
        0, 0, -19, 0, 0, -19, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 10, 27, 0,
        // State 51
        0, 0, 0, 0, 0, -34, 0, 0,
        // State 52
        0, 39, 0, 49, 0, 10, 0, 0,
        // State 53
        0, 0, 0, 0, 0, -31, 0, 0,
        // State 54
        0, -9, 0, 0, 0, -9, 0, -9,
        // State 55
        0, 0, 0, 0, 0, 10, 27, 0,
        // State 56
        0, 0, -16, 0, 0, -16, 0, 0,
        // State 57
        0, -5, 0, 0, 0, -5, 0, -5,
        // State 58
        0, -42, 0, 0, 0, -42, 0, -42,
        // State 59
        0, 0, 0, 0, 0, 10, 33, 0,
        // State 60
        0, 0, 0, 0, 0, -33, 0, 0,
        // State 61
        0, -10, 0, 0, 0, -10, 0, -10,
        // State 62
        0, 0, 0, 0, -40, -40, 0, -40,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -23,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -45,
        // State 6
        0,
        // State 7
        -47,
        // State 8
        0,
        // State 9
        -20,
        // State 10
        0,
        // State 11
        -43,
        // State 12
        -24,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -46,
        // State 18
        -44,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -39,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -18,
        // State 39
        0,
        // State 40
        -32,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        -19,
        // State 50
        0,
        // State 51
        -34,
        // State 52
        0,
        // State 53
        -31,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        -33,
        // State 61
        0,
        // State 62
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 0, 5, 0, 0, 0, 0, 6, 7, 0, 8, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 0, 5, 0, 0, 0, 0, 6, 7, 0, 12, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 0, 5, 0, 0, 0, 0, 6, 7, 0, 19, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 3, 0, 23, 0, 0, 0, 0, 24, 25, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 28, 0, 0, 0, 29, 0, 30, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 34, 0, 0, 0, 35, 0, 0, 36, 3, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 40, 0, 0, 0, 41, 3, 0, 37, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 3, 0, 28, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 44, 0, 0, 45, 3, 0, 37, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 3, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 3, 0, 37, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 3, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 3, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 60, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 3, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 60, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"":""###,
            r###"";""###,
            r###""optimize""###,
            r###"r#"#[^\\n]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z_]+"#"###,
        ];
        __ACTION[(__state * 8)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Simulation;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 8 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 8 + (8 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 24 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(0, _) if true => Some(5),
            Token(1, _) if true => Some(6),
            Token(2, _) if true => Some(7),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 => match __token {
                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            1 => match __token {
                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            2 => match __token {
                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            3 => match __token {
                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            4 => match __token {
                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            5 => match __token {
                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            6 => match __token {
                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            7 => match __token {
                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 16,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 20,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            46 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct SimulationParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl SimulationParser {
        pub fn new() -> SimulationParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            SimulationParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Simulation, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Simulation,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            46 => {
                // __Simulation = Simulation => ActionFn(0);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 24 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (String, u32), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Vec<String>, bool), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (::std::vec::Vec<&'input str>, &'input str), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Inventory, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Process, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Simulation, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(String, u32)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(String, u32)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Associative> Separate) = Associative, Separate => ActionFn(25);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Associative> Separate)* =  => ActionFn(23);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action23::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Associative> Separate)* = (<Associative> Separate)+ => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Associative> Separate)+ = Associative, Separate => ActionFn(34);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action34::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Associative> Separate)+ = (<Associative> Separate)+, Associative, Separate => ActionFn(35);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action35::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Name> Separate) = Name, Separate => ActionFn(20);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action20::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Name> Separate)* =  => ActionFn(18);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action18::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Name> Separate)* = (<Name> Separate)+ => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Name> Separate)+ = Name, Separate => ActionFn(38);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action38::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Name> Separate)+ = (<Name> Separate)+, Name, Separate => ActionFn(39);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Associative = Name, CompleteBy, Num => ActionFn(8);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Associative? = Associative => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Associative? =  => ActionFn(22);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action22::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AssociativeList = Open, Associative, Close => ActionFn(42);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AssociativeList = Open, Close => ActionFn(43);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AssociativeList = Open, (<Associative> Separate)+, Associative, Close => ActionFn(44);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AssociativeList = Open, (<Associative> Separate)+, Close => ActionFn(45);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action45::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Close = ")" => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Close = Comment+, ")" => ActionFn(47);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment = r#"#[^\\n]*"# => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment* =  => ActionFn(26);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action26::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment* = Comment+ => ActionFn(27);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment+ = Comment => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment+ = Comment+, Comment => ActionFn(29);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CompleteBy = ":" => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // CompleteBy = Comment+, ":" => ActionFn(49);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Name = r#"[a-zA-Z_]+"# => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Name = Comment+, r#"[a-zA-Z_]+"# => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Name? = Name => ActionFn(16);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Name? =  => ActionFn(17);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action17::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NameList = Open, Name, Close => ActionFn(60);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NameList = Open, Close => ActionFn(61);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action61::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NameList = Open, (<Name> Separate)+, Name, Close => ActionFn(62);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action62::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (4, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NameList = Open, (<Name> Separate)+, Close => ActionFn(63);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action63::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+"# => ActionFn(52);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action52::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = Comment+, r#"[0-9]+"# => ActionFn(53);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action53::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Open = "(" => ActionFn(54);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Open = Comment+, "(" => ActionFn(55);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action55::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Optimize = "optimize", CompleteBy, NameList => ActionFn(4);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Process = Name, CompleteBy, AssociativeList, CompleteBy, AssociativeList, CompleteBy, Num => ActionFn(5);
        let __sym6 = __pop_Variant11(__symbols);
        let __sym5 = __pop_Variant7(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (7, 20)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separate = ";" => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Separate = Comment+, ";" => ActionFn(57);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Simulation = Associative, Simulation => ActionFn(1);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action1::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Simulation = Process, Simulation => ActionFn(2);
        let __sym1 = __pop_Variant14(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Simulation = Optimize => ActionFn(58);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Simulation = Optimize, Comment+ => ActionFn(59);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 22)
    }
}
pub use self::__parse__Simulation::SimulationParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use std::collections::HashMap;
    use crate::ast::{
    Inventory,
    Process,
    Simulation,
    convert
};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt<'f>(&self, formatter: &mut __fmt::Formatter<'f>) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^(\\#[\u{0}-\t\u{b}-\u{10ffff}]*)",
                "^([0-9]+)",
                "^([A-Z_a-z]+)",
                "^(\\()",
                "^(\\))",
                "^(:)",
                "^(;)",
                "^(optimize)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(\\#[\u{0}-\t\u{b}-\u{10ffff}]*)").unwrap(),
                __regex::Regex::new("^([0-9]+)").unwrap(),
                __regex::Regex::new("^([A-Z_a-z]+)").unwrap(),
                __regex::Regex::new("^(\\()").unwrap(),
                __regex::Regex::new("^(\\))").unwrap(),
                __regex::Regex::new("^(:)").unwrap(),
                __regex::Regex::new("^(;)").unwrap(),
                __regex::Regex::new("^(optimize)").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_start();
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
                    for __i in 0 .. 8 {
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
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Simulation, usize),
) -> Simulation
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, (String, u32), usize),
    (_, s, _): (usize, Simulation, usize),
) -> Simulation
{
    {
        let (name, quantity) = i;

        s.add_inventory(name, quantity)
    }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, p, _): (usize, Process, usize),
    (_, s, _): (usize, Simulation, usize),
) -> Simulation
{
    {
        s.add_process(p)
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, o, _): (usize, (Vec<String>, bool), usize),
    (_, _, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> Simulation
{
    {
        let s = Simulation::default();

        s.optimize(o)
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, l, _): (usize, Vec<String>, usize),
) -> (Vec<String>, bool)
{
    {
        let mut vec: Vec<String> = vec![];
        let mut time = false;

        for e in l.into_iter() {
            if e == "time" { time = true; }
            else { vec.push(e) }
        }
        (vec, time)
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, i, _): (usize, Inventory, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, o, _): (usize, Inventory, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, d, _): (usize, u32, usize),
) -> Process
{
    {
        Process::new(n, i, o, d)
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, l, _): (usize, ::std::vec::Vec<(String, u32)>, usize),
    (_, e, _): (usize, ::std::option::Option<(String, u32)>, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    {
        let mut vec = l;

        if let Some(e) = e { vec.push(e); }
        convert(vec)
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, l, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, ::std::option::Option<String>, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    {
        let mut vec = l;

        if let Some(e) = e { vec.push(e); }
        vec
    }
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, String, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    (_, b, _): (usize, u32, usize),
) -> (String, u32)
{
    {
        (a, b)
    }
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> u32
{
    u32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    (__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, u32), usize),
) -> ::std::option::Option<(String, u32)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(String, u32)>
{
    None
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(String, u32)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(String, u32)>, usize),
) -> ::std::vec::Vec<(String, u32)>
{
    v
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, u32), usize),
    (_, _, _): (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> (String, u32)
{
    (__0)
}

#[allow(unused_variables)]
fn __action26<
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
fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action29<
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
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (String, u32), usize),
) -> ::std::vec::Vec<(String, u32)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(String, u32)>, usize),
    (_, e, _): (usize, (String, u32), usize),
) -> ::std::vec::Vec<(String, u32)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, (String, u32), usize),
    __1: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> ::std::vec::Vec<(String, u32)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action25(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(String, u32)>, usize),
    __1: (usize, (String, u32), usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> ::std::vec::Vec<(String, u32)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action25(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::option::Option<(String, u32)>, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<(String, u32)>, usize),
    __2: (usize, ::std::option::Option<(String, u32)>, usize),
    __3: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action24(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action20(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action20(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::option::Option<String>, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<String>, usize),
    __2: (usize, ::std::option::Option<String>, usize),
    __3: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action19(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, (String, u32), usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action22(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<(String, u32)>, usize),
    __2: (usize, (String, u32), usize),
    __3: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action21(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<(String, u32)>, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Inventory
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action22(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> u32
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> u32
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> (::std::vec::Vec<&'input str>, &'input str)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action27(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, (Vec<String>, bool), usize),
) -> Simulation
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    __0: (usize, (Vec<String>, bool), usize),
    __1: (usize, ::std::vec::Vec<&'input str>, usize),
) -> Simulation
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action27(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, String, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<String>, usize),
    __2: (usize, String, usize),
    __3: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action16(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    __0: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
    __1: (usize, ::std::vec::Vec<String>, usize),
    __2: (usize, (::std::vec::Vec<&'input str>, &'input str), usize),
) -> Vec<String>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
