fn main() {
    for right in 1..10 {
        let out = format!(
            r#"#[allow(unused_macros)]\nmacro_rules! partial_{} {{ ($inner: expr, $($arg: expr),+ ) => ( box |{}| {{ $inner($($arg),+ , {}) }} ) }}"#,
            right,
            (0..right).into_iter().map(|x| format!("_{}", x)).collect::<Vec<_>>().join(", "),
            (0..right).into_iter().map(|x| format!("_{}", x)).collect::<Vec<_>>().join(", ")
        );

        println!("{}", out);
    }
}

/*

// fn(A, B) -> fn(C) -> {eval fn(A, B, C)}
macro_rules! partial_1_1 { ($inner: expr) => ( box |_0| { box |_1| { $inner(_0, _1) } }; ) }
macro_rules! partial_1_2 { ($inner: expr) => ( box |_0| { box |_1, _2| { $inner(_0, _1, _2) } }; ) }
macro_rules! partial_1_3 { ($inner: expr) => ( box |_0| { box |_1, _2, _3| { $inner(_0, _1, _2, _3) } }; ) }
macro_rules! partial_1_4 { ($inner: expr) => ( box |_0| { box |_1, _2, _3, _4| { $inner(_0, _1, _2, _3, _4) } }; ) }
macro_rules! partial_1_5 { ($inner: expr) => ( box |_0| { box |_1, _2, _3, _4, _5| { $inner(_0, _1, _2, _3, _4, _5) } }; ) }
macro_rules! partial_2_1 { ($inner: expr) => ( box |_0, _1| { box |_2| { $inner(_0, _1, _2) } }; ) }
macro_rules! partial_2_2 { ($inner: expr) => ( box |_0, _1| { box |_2, _3| { $inner(_0, _1, _2, _3) } }; ) }
macro_rules! partial_2_3 { ($inner: expr) => ( box |_0, _1| { box |_2, _3, _4| { $inner(_0, _1, _2, _3, _4) } }; ) }
macro_rules! partial_2_4 { ($inner: expr) => ( box |_0, _1| { box |_2, _3, _4, _5| { $inner(_0, _1, _2, _3, _4, _5) } }; ) }
macro_rules! partial_2_5 { ($inner: expr) => ( box |_0, _1| { box |_2, _3, _4, _5, _6| { $inner(_0, _1, _2, _3, _4, _5, _6) } }; ) }
macro_rules! partial_3_1 { ($inner: expr) => ( box |_0, _1, _2| { box |_3| { $inner(_0, _1, _2, _3) } }; ) }
macro_rules! partial_3_2 { ($inner: expr) => ( box |_0, _1, _2| { box |_3, _4| { $inner(_0, _1, _2, _3, _4) } }; ) }
macro_rules! partial_3_3 { ($inner: expr) => ( box |_0, _1, _2| { box |_3, _4, _5| { $inner(_0, _1, _2, _3, _4, _5) } }; ) }
macro_rules! partial_3_4 { ($inner: expr) => ( box |_0, _1, _2| { box |_3, _4, _5, _6| { $inner(_0, _1, _2, _3, _4, _5, _6) } }; ) }
macro_rules! partial_3_5 { ($inner: expr) => ( box |_0, _1, _2| { box |_3, _4, _5, _6, _7| { $inner(_0, _1, _2, _3, _4, _5, _6, _7) } }; ) }
macro_rules! partial_4_1 { ($inner: expr) => ( box |_0, _1, _2, _3| { box |_4| { $inner(_0, _1, _2, _3, _4) } }; ) }
macro_rules! partial_4_2 { ($inner: expr) => ( box |_0, _1, _2, _3| { box |_4, _5| { $inner(_0, _1, _2, _3, _4, _5) } }; ) }
macro_rules! partial_4_3 { ($inner: expr) => ( box |_0, _1, _2, _3| { box |_4, _5, _6| { $inner(_0, _1, _2, _3, _4, _5, _6) } }; ) }
macro_rules! partial_4_4 { ($inner: expr) => ( box |_0, _1, _2, _3| { box |_4, _5, _6, _7| { $inner(_0, _1, _2, _3, _4, _5, _6, _7) } }; ) }
macro_rules! partial_4_5 { ($inner: expr) => ( box |_0, _1, _2, _3| { box |_4, _5, _6, _7, _8| { $inner(_0, _1, _2, _3, _4, _5, _6, _7, _8) } }; ) }
macro_rules! partial_5_1 { ($inner: expr) => ( box |_0, _1, _2, _3, _4| { box |_5| { $inner(_0, _1, _2, _3, _4, _5) } }; ) }
macro_rules! partial_5_2 { ($inner: expr) => ( box |_0, _1, _2, _3, _4| { box |_5, _6| { $inner(_0, _1, _2, _3, _4, _5, _6) } }; ) }
macro_rules! partial_5_3 { ($inner: expr) => ( box |_0, _1, _2, _3, _4| { box |_5, _6, _7| { $inner(_0, _1, _2, _3, _4, _5, _6, _7) } }; ) }
macro_rules! partial_5_4 { ($inner: expr) => ( box |_0, _1, _2, _3, _4| { box |_5, _6, _7, _8| { $inner(_0, _1, _2, _3, _4, _5, _6, _7, _8) } }; ) }
macro_rules! partial_5_5 { ($inner: expr) => ( box |_0, _1, _2, _3, _4| { box |_5, _6, _7, _8, _9| { $inner(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9) } }; ) }
*/