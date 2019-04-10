/// This file is derived from multiple files from https://github.com/unicode-rs/unicode-segmentation/tree/35a90f43319d87c8fc871d2b18cde9156f5834aa
/// The code is licensed under the Apache 2.0 license, and/or the MIT Licencse as described in the
/// license files included in this crate. This file has been modified from its original form.
use grapheme::GraphemeCat;

pub mod grapheme {
    use std::result::Result::{Err, Ok};

    pub use self::GraphemeCat::*;

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum GraphemeCat {
        GC_Any,
        GC_CR,
        GC_Control,
        GC_E_Base,
        GC_E_Base_GAZ,
        GC_E_Modifier,
        GC_Extend,
        GC_Glue_After_Zwj,
        GC_L,
        GC_LF,
        GC_LV,
        GC_LVT,
        GC_Prepend,
        GC_Regional_Indicator,
        GC_SpacingMark,
        GC_T,
        GC_V,
        GC_ZWJ,
    }

    #[perf_viz::record]
    pub fn grapheme_category(c: char) -> GraphemeCat {
        use std::cmp::Ordering::{Equal, Greater, Less};
        match GRAPHEME_CAT_TABLE.binary_search_by(|&(lo, hi, _)| {
            if lo <= c && c <= hi {
                Equal
            } else if hi < c {
                Less
            } else {
                Greater
            }
        }) {
            Ok(idx) => {
                let (_, _, cat) = GRAPHEME_CAT_TABLE[idx];
                cat
            }
            Err(_) => GC_Any,
        }
    }

    #[rustfmt::skip]
    const GRAPHEME_CAT_TABLE: &'static [(char, char, GraphemeCat)] = &[
        ('\u{0}', '\u{9}', GC_Control), ('\u{a}', '\u{a}', GC_LF), ('\u{b}', '\u{c}', GC_Control),
        ('\u{d}', '\u{d}', GC_CR), ('\u{e}', '\u{1f}', GC_Control), ('\u{7f}', '\u{9f}',
        GC_Control), ('\u{ad}', '\u{ad}', GC_Control), ('\u{300}', '\u{36f}', GC_Extend),
        ('\u{483}', '\u{489}', GC_Extend), ('\u{591}', '\u{5bd}', GC_Extend), ('\u{5bf}', '\u{5bf}',
        GC_Extend), ('\u{5c1}', '\u{5c2}', GC_Extend), ('\u{5c4}', '\u{5c5}', GC_Extend),
        ('\u{5c7}', '\u{5c7}', GC_Extend), ('\u{600}', '\u{605}', GC_Prepend), ('\u{610}',
        '\u{61a}', GC_Extend), ('\u{61c}', '\u{61c}', GC_Control), ('\u{64b}', '\u{65f}',
        GC_Extend), ('\u{670}', '\u{670}', GC_Extend), ('\u{6d6}', '\u{6dc}', GC_Extend),
        ('\u{6dd}', '\u{6dd}', GC_Prepend), ('\u{6df}', '\u{6e4}', GC_Extend), ('\u{6e7}',
        '\u{6e8}', GC_Extend), ('\u{6ea}', '\u{6ed}', GC_Extend), ('\u{70f}', '\u{70f}',
        GC_Prepend), ('\u{711}', '\u{711}', GC_Extend), ('\u{730}', '\u{74a}', GC_Extend),
        ('\u{7a6}', '\u{7b0}', GC_Extend), ('\u{7eb}', '\u{7f3}', GC_Extend), ('\u{816}', '\u{819}',
        GC_Extend), ('\u{81b}', '\u{823}', GC_Extend), ('\u{825}', '\u{827}', GC_Extend),
        ('\u{829}', '\u{82d}', GC_Extend), ('\u{859}', '\u{85b}', GC_Extend), ('\u{8d4}', '\u{8e1}',
        GC_Extend), ('\u{8e2}', '\u{8e2}', GC_Prepend), ('\u{8e3}', '\u{902}', GC_Extend),
        ('\u{903}', '\u{903}', GC_SpacingMark), ('\u{93a}', '\u{93a}', GC_Extend), ('\u{93b}',
        '\u{93b}', GC_SpacingMark), ('\u{93c}', '\u{93c}', GC_Extend), ('\u{93e}', '\u{940}',
        GC_SpacingMark), ('\u{941}', '\u{948}', GC_Extend), ('\u{949}', '\u{94c}', GC_SpacingMark),
        ('\u{94d}', '\u{94d}', GC_Extend), ('\u{94e}', '\u{94f}', GC_SpacingMark), ('\u{951}',
        '\u{957}', GC_Extend), ('\u{962}', '\u{963}', GC_Extend), ('\u{981}', '\u{981}', GC_Extend),
        ('\u{982}', '\u{983}', GC_SpacingMark), ('\u{9bc}', '\u{9bc}', GC_Extend), ('\u{9be}',
        '\u{9be}', GC_Extend), ('\u{9bf}', '\u{9c0}', GC_SpacingMark), ('\u{9c1}', '\u{9c4}',
        GC_Extend), ('\u{9c7}', '\u{9c8}', GC_SpacingMark), ('\u{9cb}', '\u{9cc}', GC_SpacingMark),
        ('\u{9cd}', '\u{9cd}', GC_Extend), ('\u{9d7}', '\u{9d7}', GC_Extend), ('\u{9e2}', '\u{9e3}',
        GC_Extend), ('\u{a01}', '\u{a02}', GC_Extend), ('\u{a03}', '\u{a03}', GC_SpacingMark),
        ('\u{a3c}', '\u{a3c}', GC_Extend), ('\u{a3e}', '\u{a40}', GC_SpacingMark), ('\u{a41}',
        '\u{a42}', GC_Extend), ('\u{a47}', '\u{a48}', GC_Extend), ('\u{a4b}', '\u{a4d}', GC_Extend),
        ('\u{a51}', '\u{a51}', GC_Extend), ('\u{a70}', '\u{a71}', GC_Extend), ('\u{a75}', '\u{a75}',
        GC_Extend), ('\u{a81}', '\u{a82}', GC_Extend), ('\u{a83}', '\u{a83}', GC_SpacingMark),
        ('\u{abc}', '\u{abc}', GC_Extend), ('\u{abe}', '\u{ac0}', GC_SpacingMark), ('\u{ac1}',
        '\u{ac5}', GC_Extend), ('\u{ac7}', '\u{ac8}', GC_Extend), ('\u{ac9}', '\u{ac9}',
        GC_SpacingMark), ('\u{acb}', '\u{acc}', GC_SpacingMark), ('\u{acd}', '\u{acd}', GC_Extend),
        ('\u{ae2}', '\u{ae3}', GC_Extend), ('\u{b01}', '\u{b01}', GC_Extend), ('\u{b02}', '\u{b03}',
        GC_SpacingMark), ('\u{b3c}', '\u{b3c}', GC_Extend), ('\u{b3e}', '\u{b3f}', GC_Extend),
        ('\u{b40}', '\u{b40}', GC_SpacingMark), ('\u{b41}', '\u{b44}', GC_Extend), ('\u{b47}',
        '\u{b48}', GC_SpacingMark), ('\u{b4b}', '\u{b4c}', GC_SpacingMark), ('\u{b4d}', '\u{b4d}',
        GC_Extend), ('\u{b56}', '\u{b57}', GC_Extend), ('\u{b62}', '\u{b63}', GC_Extend),
        ('\u{b82}', '\u{b82}', GC_Extend), ('\u{bbe}', '\u{bbe}', GC_Extend), ('\u{bbf}', '\u{bbf}',
        GC_SpacingMark), ('\u{bc0}', '\u{bc0}', GC_Extend), ('\u{bc1}', '\u{bc2}', GC_SpacingMark),
        ('\u{bc6}', '\u{bc8}', GC_SpacingMark), ('\u{bca}', '\u{bcc}', GC_SpacingMark), ('\u{bcd}',
        '\u{bcd}', GC_Extend), ('\u{bd7}', '\u{bd7}', GC_Extend), ('\u{c00}', '\u{c00}', GC_Extend),
        ('\u{c01}', '\u{c03}', GC_SpacingMark), ('\u{c3e}', '\u{c40}', GC_Extend), ('\u{c41}',
        '\u{c44}', GC_SpacingMark), ('\u{c46}', '\u{c48}', GC_Extend), ('\u{c4a}', '\u{c4d}',
        GC_Extend), ('\u{c55}', '\u{c56}', GC_Extend), ('\u{c62}', '\u{c63}', GC_Extend),
        ('\u{c81}', '\u{c81}', GC_Extend), ('\u{c82}', '\u{c83}', GC_SpacingMark), ('\u{cbc}',
        '\u{cbc}', GC_Extend), ('\u{cbe}', '\u{cbe}', GC_SpacingMark), ('\u{cbf}', '\u{cbf}',
        GC_Extend), ('\u{cc0}', '\u{cc1}', GC_SpacingMark), ('\u{cc2}', '\u{cc2}', GC_Extend),
        ('\u{cc3}', '\u{cc4}', GC_SpacingMark), ('\u{cc6}', '\u{cc6}', GC_Extend), ('\u{cc7}',
        '\u{cc8}', GC_SpacingMark), ('\u{cca}', '\u{ccb}', GC_SpacingMark), ('\u{ccc}', '\u{ccd}',
        GC_Extend), ('\u{cd5}', '\u{cd6}', GC_Extend), ('\u{ce2}', '\u{ce3}', GC_Extend),
        ('\u{d01}', '\u{d01}', GC_Extend), ('\u{d02}', '\u{d03}', GC_SpacingMark), ('\u{d3e}',
        '\u{d3e}', GC_Extend), ('\u{d3f}', '\u{d40}', GC_SpacingMark), ('\u{d41}', '\u{d44}',
        GC_Extend), ('\u{d46}', '\u{d48}', GC_SpacingMark), ('\u{d4a}', '\u{d4c}', GC_SpacingMark),
        ('\u{d4d}', '\u{d4d}', GC_Extend), ('\u{d4e}', '\u{d4e}', GC_Prepend), ('\u{d57}',
        '\u{d57}', GC_Extend), ('\u{d62}', '\u{d63}', GC_Extend), ('\u{d82}', '\u{d83}',
        GC_SpacingMark), ('\u{dca}', '\u{dca}', GC_Extend), ('\u{dcf}', '\u{dcf}', GC_Extend),
        ('\u{dd0}', '\u{dd1}', GC_SpacingMark), ('\u{dd2}', '\u{dd4}', GC_Extend), ('\u{dd6}',
        '\u{dd6}', GC_Extend), ('\u{dd8}', '\u{dde}', GC_SpacingMark), ('\u{ddf}', '\u{ddf}',
        GC_Extend), ('\u{df2}', '\u{df3}', GC_SpacingMark), ('\u{e31}', '\u{e31}', GC_Extend),
        ('\u{e33}', '\u{e33}', GC_SpacingMark), ('\u{e34}', '\u{e3a}', GC_Extend), ('\u{e47}',
        '\u{e4e}', GC_Extend), ('\u{eb1}', '\u{eb1}', GC_Extend), ('\u{eb3}', '\u{eb3}',
        GC_SpacingMark), ('\u{eb4}', '\u{eb9}', GC_Extend), ('\u{ebb}', '\u{ebc}', GC_Extend),
        ('\u{ec8}', '\u{ecd}', GC_Extend), ('\u{f18}', '\u{f19}', GC_Extend), ('\u{f35}', '\u{f35}',
        GC_Extend), ('\u{f37}', '\u{f37}', GC_Extend), ('\u{f39}', '\u{f39}', GC_Extend),
        ('\u{f3e}', '\u{f3f}', GC_SpacingMark), ('\u{f71}', '\u{f7e}', GC_Extend), ('\u{f7f}',
        '\u{f7f}', GC_SpacingMark), ('\u{f80}', '\u{f84}', GC_Extend), ('\u{f86}', '\u{f87}',
        GC_Extend), ('\u{f8d}', '\u{f97}', GC_Extend), ('\u{f99}', '\u{fbc}', GC_Extend),
        ('\u{fc6}', '\u{fc6}', GC_Extend), ('\u{102d}', '\u{1030}', GC_Extend), ('\u{1031}',
        '\u{1031}', GC_SpacingMark), ('\u{1032}', '\u{1037}', GC_Extend), ('\u{1039}', '\u{103a}',
        GC_Extend), ('\u{103b}', '\u{103c}', GC_SpacingMark), ('\u{103d}', '\u{103e}', GC_Extend),
        ('\u{1056}', '\u{1057}', GC_SpacingMark), ('\u{1058}', '\u{1059}', GC_Extend), ('\u{105e}',
        '\u{1060}', GC_Extend), ('\u{1071}', '\u{1074}', GC_Extend), ('\u{1082}', '\u{1082}',
        GC_Extend), ('\u{1084}', '\u{1084}', GC_SpacingMark), ('\u{1085}', '\u{1086}', GC_Extend),
        ('\u{108d}', '\u{108d}', GC_Extend), ('\u{109d}', '\u{109d}', GC_Extend), ('\u{1100}',
        '\u{115f}', GC_L), ('\u{1160}', '\u{11a7}', GC_V), ('\u{11a8}', '\u{11ff}', GC_T),
        ('\u{135d}', '\u{135f}', GC_Extend), ('\u{1712}', '\u{1714}', GC_Extend), ('\u{1732}',
        '\u{1734}', GC_Extend), ('\u{1752}', '\u{1753}', GC_Extend), ('\u{1772}', '\u{1773}',
        GC_Extend), ('\u{17b4}', '\u{17b5}', GC_Extend), ('\u{17b6}', '\u{17b6}', GC_SpacingMark),
        ('\u{17b7}', '\u{17bd}', GC_Extend), ('\u{17be}', '\u{17c5}', GC_SpacingMark), ('\u{17c6}',
        '\u{17c6}', GC_Extend), ('\u{17c7}', '\u{17c8}', GC_SpacingMark), ('\u{17c9}', '\u{17d3}',
        GC_Extend), ('\u{17dd}', '\u{17dd}', GC_Extend), ('\u{180b}', '\u{180d}', GC_Extend),
        ('\u{180e}', '\u{180e}', GC_Control), ('\u{1885}', '\u{1886}', GC_Extend), ('\u{18a9}',
        '\u{18a9}', GC_Extend), ('\u{1920}', '\u{1922}', GC_Extend), ('\u{1923}', '\u{1926}',
        GC_SpacingMark), ('\u{1927}', '\u{1928}', GC_Extend), ('\u{1929}', '\u{192b}',
        GC_SpacingMark), ('\u{1930}', '\u{1931}', GC_SpacingMark), ('\u{1932}', '\u{1932}',
        GC_Extend), ('\u{1933}', '\u{1938}', GC_SpacingMark), ('\u{1939}', '\u{193b}', GC_Extend),
        ('\u{1a17}', '\u{1a18}', GC_Extend), ('\u{1a19}', '\u{1a1a}', GC_SpacingMark), ('\u{1a1b}',
        '\u{1a1b}', GC_Extend), ('\u{1a55}', '\u{1a55}', GC_SpacingMark), ('\u{1a56}', '\u{1a56}',
        GC_Extend), ('\u{1a57}', '\u{1a57}', GC_SpacingMark), ('\u{1a58}', '\u{1a5e}', GC_Extend),
        ('\u{1a60}', '\u{1a60}', GC_Extend), ('\u{1a62}', '\u{1a62}', GC_Extend), ('\u{1a65}',
        '\u{1a6c}', GC_Extend), ('\u{1a6d}', '\u{1a72}', GC_SpacingMark), ('\u{1a73}', '\u{1a7c}',
        GC_Extend), ('\u{1a7f}', '\u{1a7f}', GC_Extend), ('\u{1ab0}', '\u{1abe}', GC_Extend),
        ('\u{1b00}', '\u{1b03}', GC_Extend), ('\u{1b04}', '\u{1b04}', GC_SpacingMark), ('\u{1b34}',
        '\u{1b34}', GC_Extend), ('\u{1b35}', '\u{1b35}', GC_SpacingMark), ('\u{1b36}', '\u{1b3a}',
        GC_Extend), ('\u{1b3b}', '\u{1b3b}', GC_SpacingMark), ('\u{1b3c}', '\u{1b3c}', GC_Extend),
        ('\u{1b3d}', '\u{1b41}', GC_SpacingMark), ('\u{1b42}', '\u{1b42}', GC_Extend), ('\u{1b43}',
        '\u{1b44}', GC_SpacingMark), ('\u{1b6b}', '\u{1b73}', GC_Extend), ('\u{1b80}', '\u{1b81}',
        GC_Extend), ('\u{1b82}', '\u{1b82}', GC_SpacingMark), ('\u{1ba1}', '\u{1ba1}',
        GC_SpacingMark), ('\u{1ba2}', '\u{1ba5}', GC_Extend), ('\u{1ba6}', '\u{1ba7}',
        GC_SpacingMark), ('\u{1ba8}', '\u{1ba9}', GC_Extend), ('\u{1baa}', '\u{1baa}',
        GC_SpacingMark), ('\u{1bab}', '\u{1bad}', GC_Extend), ('\u{1be6}', '\u{1be6}', GC_Extend),
        ('\u{1be7}', '\u{1be7}', GC_SpacingMark), ('\u{1be8}', '\u{1be9}', GC_Extend), ('\u{1bea}',
        '\u{1bec}', GC_SpacingMark), ('\u{1bed}', '\u{1bed}', GC_Extend), ('\u{1bee}', '\u{1bee}',
        GC_SpacingMark), ('\u{1bef}', '\u{1bf1}', GC_Extend), ('\u{1bf2}', '\u{1bf3}',
        GC_SpacingMark), ('\u{1c24}', '\u{1c2b}', GC_SpacingMark), ('\u{1c2c}', '\u{1c33}',
        GC_Extend), ('\u{1c34}', '\u{1c35}', GC_SpacingMark), ('\u{1c36}', '\u{1c37}', GC_Extend),
        ('\u{1cd0}', '\u{1cd2}', GC_Extend), ('\u{1cd4}', '\u{1ce0}', GC_Extend), ('\u{1ce1}',
        '\u{1ce1}', GC_SpacingMark), ('\u{1ce2}', '\u{1ce8}', GC_Extend), ('\u{1ced}', '\u{1ced}',
        GC_Extend), ('\u{1cf2}', '\u{1cf3}', GC_SpacingMark), ('\u{1cf4}', '\u{1cf4}', GC_Extend),
        ('\u{1cf8}', '\u{1cf9}', GC_Extend), ('\u{1dc0}', '\u{1df5}', GC_Extend), ('\u{1dfb}',
        '\u{1dff}', GC_Extend), ('\u{200b}', '\u{200b}', GC_Control), ('\u{200c}', '\u{200c}',
        GC_Extend), ('\u{200d}', '\u{200d}', GC_ZWJ), ('\u{200e}', '\u{200f}', GC_Control),
        ('\u{2028}', '\u{202e}', GC_Control), ('\u{2060}', '\u{206f}', GC_Control), ('\u{20d0}',
        '\u{20f0}', GC_Extend), ('\u{261d}', '\u{261d}', GC_E_Base), ('\u{26f9}', '\u{26f9}',
        GC_E_Base), ('\u{270a}', '\u{270d}', GC_E_Base), ('\u{2764}', '\u{2764}',
        GC_Glue_After_Zwj), ('\u{2cef}', '\u{2cf1}', GC_Extend), ('\u{2d7f}', '\u{2d7f}',
        GC_Extend), ('\u{2de0}', '\u{2dff}', GC_Extend), ('\u{302a}', '\u{302f}', GC_Extend),
        ('\u{3099}', '\u{309a}', GC_Extend), ('\u{a66f}', '\u{a672}', GC_Extend), ('\u{a674}',
        '\u{a67d}', GC_Extend), ('\u{a69e}', '\u{a69f}', GC_Extend), ('\u{a6f0}', '\u{a6f1}',
        GC_Extend), ('\u{a802}', '\u{a802}', GC_Extend), ('\u{a806}', '\u{a806}', GC_Extend),
        ('\u{a80b}', '\u{a80b}', GC_Extend), ('\u{a823}', '\u{a824}', GC_SpacingMark), ('\u{a825}',
        '\u{a826}', GC_Extend), ('\u{a827}', '\u{a827}', GC_SpacingMark), ('\u{a880}', '\u{a881}',
        GC_SpacingMark), ('\u{a8b4}', '\u{a8c3}', GC_SpacingMark), ('\u{a8c4}', '\u{a8c5}',
        GC_Extend), ('\u{a8e0}', '\u{a8f1}', GC_Extend), ('\u{a926}', '\u{a92d}', GC_Extend),
        ('\u{a947}', '\u{a951}', GC_Extend), ('\u{a952}', '\u{a953}', GC_SpacingMark), ('\u{a960}',
        '\u{a97c}', GC_L), ('\u{a980}', '\u{a982}', GC_Extend), ('\u{a983}', '\u{a983}',
        GC_SpacingMark), ('\u{a9b3}', '\u{a9b3}', GC_Extend), ('\u{a9b4}', '\u{a9b5}',
        GC_SpacingMark), ('\u{a9b6}', '\u{a9b9}', GC_Extend), ('\u{a9ba}', '\u{a9bb}',
        GC_SpacingMark), ('\u{a9bc}', '\u{a9bc}', GC_Extend), ('\u{a9bd}', '\u{a9c0}',
        GC_SpacingMark), ('\u{a9e5}', '\u{a9e5}', GC_Extend), ('\u{aa29}', '\u{aa2e}', GC_Extend),
        ('\u{aa2f}', '\u{aa30}', GC_SpacingMark), ('\u{aa31}', '\u{aa32}', GC_Extend), ('\u{aa33}',
        '\u{aa34}', GC_SpacingMark), ('\u{aa35}', '\u{aa36}', GC_Extend), ('\u{aa43}', '\u{aa43}',
        GC_Extend), ('\u{aa4c}', '\u{aa4c}', GC_Extend), ('\u{aa4d}', '\u{aa4d}', GC_SpacingMark),
        ('\u{aa7c}', '\u{aa7c}', GC_Extend), ('\u{aab0}', '\u{aab0}', GC_Extend), ('\u{aab2}',
        '\u{aab4}', GC_Extend), ('\u{aab7}', '\u{aab8}', GC_Extend), ('\u{aabe}', '\u{aabf}',
        GC_Extend), ('\u{aac1}', '\u{aac1}', GC_Extend), ('\u{aaeb}', '\u{aaeb}', GC_SpacingMark),
        ('\u{aaec}', '\u{aaed}', GC_Extend), ('\u{aaee}', '\u{aaef}', GC_SpacingMark), ('\u{aaf5}',
        '\u{aaf5}', GC_SpacingMark), ('\u{aaf6}', '\u{aaf6}', GC_Extend), ('\u{abe3}', '\u{abe4}',
        GC_SpacingMark), ('\u{abe5}', '\u{abe5}', GC_Extend), ('\u{abe6}', '\u{abe7}',
        GC_SpacingMark), ('\u{abe8}', '\u{abe8}', GC_Extend), ('\u{abe9}', '\u{abea}',
        GC_SpacingMark), ('\u{abec}', '\u{abec}', GC_SpacingMark), ('\u{abed}', '\u{abed}',
        GC_Extend), ('\u{ac00}', '\u{ac00}', GC_LV), ('\u{ac01}', '\u{ac1b}', GC_LVT), ('\u{ac1c}',
        '\u{ac1c}', GC_LV), ('\u{ac1d}', '\u{ac37}', GC_LVT), ('\u{ac38}', '\u{ac38}', GC_LV),
        ('\u{ac39}', '\u{ac53}', GC_LVT), ('\u{ac54}', '\u{ac54}', GC_LV), ('\u{ac55}', '\u{ac6f}',
        GC_LVT), ('\u{ac70}', '\u{ac70}', GC_LV), ('\u{ac71}', '\u{ac8b}', GC_LVT), ('\u{ac8c}',
        '\u{ac8c}', GC_LV), ('\u{ac8d}', '\u{aca7}', GC_LVT), ('\u{aca8}', '\u{aca8}', GC_LV),
        ('\u{aca9}', '\u{acc3}', GC_LVT), ('\u{acc4}', '\u{acc4}', GC_LV), ('\u{acc5}', '\u{acdf}',
        GC_LVT), ('\u{ace0}', '\u{ace0}', GC_LV), ('\u{ace1}', '\u{acfb}', GC_LVT), ('\u{acfc}',
        '\u{acfc}', GC_LV), ('\u{acfd}', '\u{ad17}', GC_LVT), ('\u{ad18}', '\u{ad18}', GC_LV),
        ('\u{ad19}', '\u{ad33}', GC_LVT), ('\u{ad34}', '\u{ad34}', GC_LV), ('\u{ad35}', '\u{ad4f}',
        GC_LVT), ('\u{ad50}', '\u{ad50}', GC_LV), ('\u{ad51}', '\u{ad6b}', GC_LVT), ('\u{ad6c}',
        '\u{ad6c}', GC_LV), ('\u{ad6d}', '\u{ad87}', GC_LVT), ('\u{ad88}', '\u{ad88}', GC_LV),
        ('\u{ad89}', '\u{ada3}', GC_LVT), ('\u{ada4}', '\u{ada4}', GC_LV), ('\u{ada5}', '\u{adbf}',
        GC_LVT), ('\u{adc0}', '\u{adc0}', GC_LV), ('\u{adc1}', '\u{addb}', GC_LVT), ('\u{addc}',
        '\u{addc}', GC_LV), ('\u{addd}', '\u{adf7}', GC_LVT), ('\u{adf8}', '\u{adf8}', GC_LV),
        ('\u{adf9}', '\u{ae13}', GC_LVT), ('\u{ae14}', '\u{ae14}', GC_LV), ('\u{ae15}', '\u{ae2f}',
        GC_LVT), ('\u{ae30}', '\u{ae30}', GC_LV), ('\u{ae31}', '\u{ae4b}', GC_LVT), ('\u{ae4c}',
        '\u{ae4c}', GC_LV), ('\u{ae4d}', '\u{ae67}', GC_LVT), ('\u{ae68}', '\u{ae68}', GC_LV),
        ('\u{ae69}', '\u{ae83}', GC_LVT), ('\u{ae84}', '\u{ae84}', GC_LV), ('\u{ae85}', '\u{ae9f}',
        GC_LVT), ('\u{aea0}', '\u{aea0}', GC_LV), ('\u{aea1}', '\u{aebb}', GC_LVT), ('\u{aebc}',
        '\u{aebc}', GC_LV), ('\u{aebd}', '\u{aed7}', GC_LVT), ('\u{aed8}', '\u{aed8}', GC_LV),
        ('\u{aed9}', '\u{aef3}', GC_LVT), ('\u{aef4}', '\u{aef4}', GC_LV), ('\u{aef5}', '\u{af0f}',
        GC_LVT), ('\u{af10}', '\u{af10}', GC_LV), ('\u{af11}', '\u{af2b}', GC_LVT), ('\u{af2c}',
        '\u{af2c}', GC_LV), ('\u{af2d}', '\u{af47}', GC_LVT), ('\u{af48}', '\u{af48}', GC_LV),
        ('\u{af49}', '\u{af63}', GC_LVT), ('\u{af64}', '\u{af64}', GC_LV), ('\u{af65}', '\u{af7f}',
        GC_LVT), ('\u{af80}', '\u{af80}', GC_LV), ('\u{af81}', '\u{af9b}', GC_LVT), ('\u{af9c}',
        '\u{af9c}', GC_LV), ('\u{af9d}', '\u{afb7}', GC_LVT), ('\u{afb8}', '\u{afb8}', GC_LV),
        ('\u{afb9}', '\u{afd3}', GC_LVT), ('\u{afd4}', '\u{afd4}', GC_LV), ('\u{afd5}', '\u{afef}',
        GC_LVT), ('\u{aff0}', '\u{aff0}', GC_LV), ('\u{aff1}', '\u{b00b}', GC_LVT), ('\u{b00c}',
        '\u{b00c}', GC_LV), ('\u{b00d}', '\u{b027}', GC_LVT), ('\u{b028}', '\u{b028}', GC_LV),
        ('\u{b029}', '\u{b043}', GC_LVT), ('\u{b044}', '\u{b044}', GC_LV), ('\u{b045}', '\u{b05f}',
        GC_LVT), ('\u{b060}', '\u{b060}', GC_LV), ('\u{b061}', '\u{b07b}', GC_LVT), ('\u{b07c}',
        '\u{b07c}', GC_LV), ('\u{b07d}', '\u{b097}', GC_LVT), ('\u{b098}', '\u{b098}', GC_LV),
        ('\u{b099}', '\u{b0b3}', GC_LVT), ('\u{b0b4}', '\u{b0b4}', GC_LV), ('\u{b0b5}', '\u{b0cf}',
        GC_LVT), ('\u{b0d0}', '\u{b0d0}', GC_LV), ('\u{b0d1}', '\u{b0eb}', GC_LVT), ('\u{b0ec}',
        '\u{b0ec}', GC_LV), ('\u{b0ed}', '\u{b107}', GC_LVT), ('\u{b108}', '\u{b108}', GC_LV),
        ('\u{b109}', '\u{b123}', GC_LVT), ('\u{b124}', '\u{b124}', GC_LV), ('\u{b125}', '\u{b13f}',
        GC_LVT), ('\u{b140}', '\u{b140}', GC_LV), ('\u{b141}', '\u{b15b}', GC_LVT), ('\u{b15c}',
        '\u{b15c}', GC_LV), ('\u{b15d}', '\u{b177}', GC_LVT), ('\u{b178}', '\u{b178}', GC_LV),
        ('\u{b179}', '\u{b193}', GC_LVT), ('\u{b194}', '\u{b194}', GC_LV), ('\u{b195}', '\u{b1af}',
        GC_LVT), ('\u{b1b0}', '\u{b1b0}', GC_LV), ('\u{b1b1}', '\u{b1cb}', GC_LVT), ('\u{b1cc}',
        '\u{b1cc}', GC_LV), ('\u{b1cd}', '\u{b1e7}', GC_LVT), ('\u{b1e8}', '\u{b1e8}', GC_LV),
        ('\u{b1e9}', '\u{b203}', GC_LVT), ('\u{b204}', '\u{b204}', GC_LV), ('\u{b205}', '\u{b21f}',
        GC_LVT), ('\u{b220}', '\u{b220}', GC_LV), ('\u{b221}', '\u{b23b}', GC_LVT), ('\u{b23c}',
        '\u{b23c}', GC_LV), ('\u{b23d}', '\u{b257}', GC_LVT), ('\u{b258}', '\u{b258}', GC_LV),
        ('\u{b259}', '\u{b273}', GC_LVT), ('\u{b274}', '\u{b274}', GC_LV), ('\u{b275}', '\u{b28f}',
        GC_LVT), ('\u{b290}', '\u{b290}', GC_LV), ('\u{b291}', '\u{b2ab}', GC_LVT), ('\u{b2ac}',
        '\u{b2ac}', GC_LV), ('\u{b2ad}', '\u{b2c7}', GC_LVT), ('\u{b2c8}', '\u{b2c8}', GC_LV),
        ('\u{b2c9}', '\u{b2e3}', GC_LVT), ('\u{b2e4}', '\u{b2e4}', GC_LV), ('\u{b2e5}', '\u{b2ff}',
        GC_LVT), ('\u{b300}', '\u{b300}', GC_LV), ('\u{b301}', '\u{b31b}', GC_LVT), ('\u{b31c}',
        '\u{b31c}', GC_LV), ('\u{b31d}', '\u{b337}', GC_LVT), ('\u{b338}', '\u{b338}', GC_LV),
        ('\u{b339}', '\u{b353}', GC_LVT), ('\u{b354}', '\u{b354}', GC_LV), ('\u{b355}', '\u{b36f}',
        GC_LVT), ('\u{b370}', '\u{b370}', GC_LV), ('\u{b371}', '\u{b38b}', GC_LVT), ('\u{b38c}',
        '\u{b38c}', GC_LV), ('\u{b38d}', '\u{b3a7}', GC_LVT), ('\u{b3a8}', '\u{b3a8}', GC_LV),
        ('\u{b3a9}', '\u{b3c3}', GC_LVT), ('\u{b3c4}', '\u{b3c4}', GC_LV), ('\u{b3c5}', '\u{b3df}',
        GC_LVT), ('\u{b3e0}', '\u{b3e0}', GC_LV), ('\u{b3e1}', '\u{b3fb}', GC_LVT), ('\u{b3fc}',
        '\u{b3fc}', GC_LV), ('\u{b3fd}', '\u{b417}', GC_LVT), ('\u{b418}', '\u{b418}', GC_LV),
        ('\u{b419}', '\u{b433}', GC_LVT), ('\u{b434}', '\u{b434}', GC_LV), ('\u{b435}', '\u{b44f}',
        GC_LVT), ('\u{b450}', '\u{b450}', GC_LV), ('\u{b451}', '\u{b46b}', GC_LVT), ('\u{b46c}',
        '\u{b46c}', GC_LV), ('\u{b46d}', '\u{b487}', GC_LVT), ('\u{b488}', '\u{b488}', GC_LV),
        ('\u{b489}', '\u{b4a3}', GC_LVT), ('\u{b4a4}', '\u{b4a4}', GC_LV), ('\u{b4a5}', '\u{b4bf}',
        GC_LVT), ('\u{b4c0}', '\u{b4c0}', GC_LV), ('\u{b4c1}', '\u{b4db}', GC_LVT), ('\u{b4dc}',
        '\u{b4dc}', GC_LV), ('\u{b4dd}', '\u{b4f7}', GC_LVT), ('\u{b4f8}', '\u{b4f8}', GC_LV),
        ('\u{b4f9}', '\u{b513}', GC_LVT), ('\u{b514}', '\u{b514}', GC_LV), ('\u{b515}', '\u{b52f}',
        GC_LVT), ('\u{b530}', '\u{b530}', GC_LV), ('\u{b531}', '\u{b54b}', GC_LVT), ('\u{b54c}',
        '\u{b54c}', GC_LV), ('\u{b54d}', '\u{b567}', GC_LVT), ('\u{b568}', '\u{b568}', GC_LV),
        ('\u{b569}', '\u{b583}', GC_LVT), ('\u{b584}', '\u{b584}', GC_LV), ('\u{b585}', '\u{b59f}',
        GC_LVT), ('\u{b5a0}', '\u{b5a0}', GC_LV), ('\u{b5a1}', '\u{b5bb}', GC_LVT), ('\u{b5bc}',
        '\u{b5bc}', GC_LV), ('\u{b5bd}', '\u{b5d7}', GC_LVT), ('\u{b5d8}', '\u{b5d8}', GC_LV),
        ('\u{b5d9}', '\u{b5f3}', GC_LVT), ('\u{b5f4}', '\u{b5f4}', GC_LV), ('\u{b5f5}', '\u{b60f}',
        GC_LVT), ('\u{b610}', '\u{b610}', GC_LV), ('\u{b611}', '\u{b62b}', GC_LVT), ('\u{b62c}',
        '\u{b62c}', GC_LV), ('\u{b62d}', '\u{b647}', GC_LVT), ('\u{b648}', '\u{b648}', GC_LV),
        ('\u{b649}', '\u{b663}', GC_LVT), ('\u{b664}', '\u{b664}', GC_LV), ('\u{b665}', '\u{b67f}',
        GC_LVT), ('\u{b680}', '\u{b680}', GC_LV), ('\u{b681}', '\u{b69b}', GC_LVT), ('\u{b69c}',
        '\u{b69c}', GC_LV), ('\u{b69d}', '\u{b6b7}', GC_LVT), ('\u{b6b8}', '\u{b6b8}', GC_LV),
        ('\u{b6b9}', '\u{b6d3}', GC_LVT), ('\u{b6d4}', '\u{b6d4}', GC_LV), ('\u{b6d5}', '\u{b6ef}',
        GC_LVT), ('\u{b6f0}', '\u{b6f0}', GC_LV), ('\u{b6f1}', '\u{b70b}', GC_LVT), ('\u{b70c}',
        '\u{b70c}', GC_LV), ('\u{b70d}', '\u{b727}', GC_LVT), ('\u{b728}', '\u{b728}', GC_LV),
        ('\u{b729}', '\u{b743}', GC_LVT), ('\u{b744}', '\u{b744}', GC_LV), ('\u{b745}', '\u{b75f}',
        GC_LVT), ('\u{b760}', '\u{b760}', GC_LV), ('\u{b761}', '\u{b77b}', GC_LVT), ('\u{b77c}',
        '\u{b77c}', GC_LV), ('\u{b77d}', '\u{b797}', GC_LVT), ('\u{b798}', '\u{b798}', GC_LV),
        ('\u{b799}', '\u{b7b3}', GC_LVT), ('\u{b7b4}', '\u{b7b4}', GC_LV), ('\u{b7b5}', '\u{b7cf}',
        GC_LVT), ('\u{b7d0}', '\u{b7d0}', GC_LV), ('\u{b7d1}', '\u{b7eb}', GC_LVT), ('\u{b7ec}',
        '\u{b7ec}', GC_LV), ('\u{b7ed}', '\u{b807}', GC_LVT), ('\u{b808}', '\u{b808}', GC_LV),
        ('\u{b809}', '\u{b823}', GC_LVT), ('\u{b824}', '\u{b824}', GC_LV), ('\u{b825}', '\u{b83f}',
        GC_LVT), ('\u{b840}', '\u{b840}', GC_LV), ('\u{b841}', '\u{b85b}', GC_LVT), ('\u{b85c}',
        '\u{b85c}', GC_LV), ('\u{b85d}', '\u{b877}', GC_LVT), ('\u{b878}', '\u{b878}', GC_LV),
        ('\u{b879}', '\u{b893}', GC_LVT), ('\u{b894}', '\u{b894}', GC_LV), ('\u{b895}', '\u{b8af}',
        GC_LVT), ('\u{b8b0}', '\u{b8b0}', GC_LV), ('\u{b8b1}', '\u{b8cb}', GC_LVT), ('\u{b8cc}',
        '\u{b8cc}', GC_LV), ('\u{b8cd}', '\u{b8e7}', GC_LVT), ('\u{b8e8}', '\u{b8e8}', GC_LV),
        ('\u{b8e9}', '\u{b903}', GC_LVT), ('\u{b904}', '\u{b904}', GC_LV), ('\u{b905}', '\u{b91f}',
        GC_LVT), ('\u{b920}', '\u{b920}', GC_LV), ('\u{b921}', '\u{b93b}', GC_LVT), ('\u{b93c}',
        '\u{b93c}', GC_LV), ('\u{b93d}', '\u{b957}', GC_LVT), ('\u{b958}', '\u{b958}', GC_LV),
        ('\u{b959}', '\u{b973}', GC_LVT), ('\u{b974}', '\u{b974}', GC_LV), ('\u{b975}', '\u{b98f}',
        GC_LVT), ('\u{b990}', '\u{b990}', GC_LV), ('\u{b991}', '\u{b9ab}', GC_LVT), ('\u{b9ac}',
        '\u{b9ac}', GC_LV), ('\u{b9ad}', '\u{b9c7}', GC_LVT), ('\u{b9c8}', '\u{b9c8}', GC_LV),
        ('\u{b9c9}', '\u{b9e3}', GC_LVT), ('\u{b9e4}', '\u{b9e4}', GC_LV), ('\u{b9e5}', '\u{b9ff}',
        GC_LVT), ('\u{ba00}', '\u{ba00}', GC_LV), ('\u{ba01}', '\u{ba1b}', GC_LVT), ('\u{ba1c}',
        '\u{ba1c}', GC_LV), ('\u{ba1d}', '\u{ba37}', GC_LVT), ('\u{ba38}', '\u{ba38}', GC_LV),
        ('\u{ba39}', '\u{ba53}', GC_LVT), ('\u{ba54}', '\u{ba54}', GC_LV), ('\u{ba55}', '\u{ba6f}',
        GC_LVT), ('\u{ba70}', '\u{ba70}', GC_LV), ('\u{ba71}', '\u{ba8b}', GC_LVT), ('\u{ba8c}',
        '\u{ba8c}', GC_LV), ('\u{ba8d}', '\u{baa7}', GC_LVT), ('\u{baa8}', '\u{baa8}', GC_LV),
        ('\u{baa9}', '\u{bac3}', GC_LVT), ('\u{bac4}', '\u{bac4}', GC_LV), ('\u{bac5}', '\u{badf}',
        GC_LVT), ('\u{bae0}', '\u{bae0}', GC_LV), ('\u{bae1}', '\u{bafb}', GC_LVT), ('\u{bafc}',
        '\u{bafc}', GC_LV), ('\u{bafd}', '\u{bb17}', GC_LVT), ('\u{bb18}', '\u{bb18}', GC_LV),
        ('\u{bb19}', '\u{bb33}', GC_LVT), ('\u{bb34}', '\u{bb34}', GC_LV), ('\u{bb35}', '\u{bb4f}',
        GC_LVT), ('\u{bb50}', '\u{bb50}', GC_LV), ('\u{bb51}', '\u{bb6b}', GC_LVT), ('\u{bb6c}',
        '\u{bb6c}', GC_LV), ('\u{bb6d}', '\u{bb87}', GC_LVT), ('\u{bb88}', '\u{bb88}', GC_LV),
        ('\u{bb89}', '\u{bba3}', GC_LVT), ('\u{bba4}', '\u{bba4}', GC_LV), ('\u{bba5}', '\u{bbbf}',
        GC_LVT), ('\u{bbc0}', '\u{bbc0}', GC_LV), ('\u{bbc1}', '\u{bbdb}', GC_LVT), ('\u{bbdc}',
        '\u{bbdc}', GC_LV), ('\u{bbdd}', '\u{bbf7}', GC_LVT), ('\u{bbf8}', '\u{bbf8}', GC_LV),
        ('\u{bbf9}', '\u{bc13}', GC_LVT), ('\u{bc14}', '\u{bc14}', GC_LV), ('\u{bc15}', '\u{bc2f}',
        GC_LVT), ('\u{bc30}', '\u{bc30}', GC_LV), ('\u{bc31}', '\u{bc4b}', GC_LVT), ('\u{bc4c}',
        '\u{bc4c}', GC_LV), ('\u{bc4d}', '\u{bc67}', GC_LVT), ('\u{bc68}', '\u{bc68}', GC_LV),
        ('\u{bc69}', '\u{bc83}', GC_LVT), ('\u{bc84}', '\u{bc84}', GC_LV), ('\u{bc85}', '\u{bc9f}',
        GC_LVT), ('\u{bca0}', '\u{bca0}', GC_LV), ('\u{bca1}', '\u{bcbb}', GC_LVT), ('\u{bcbc}',
        '\u{bcbc}', GC_LV), ('\u{bcbd}', '\u{bcd7}', GC_LVT), ('\u{bcd8}', '\u{bcd8}', GC_LV),
        ('\u{bcd9}', '\u{bcf3}', GC_LVT), ('\u{bcf4}', '\u{bcf4}', GC_LV), ('\u{bcf5}', '\u{bd0f}',
        GC_LVT), ('\u{bd10}', '\u{bd10}', GC_LV), ('\u{bd11}', '\u{bd2b}', GC_LVT), ('\u{bd2c}',
        '\u{bd2c}', GC_LV), ('\u{bd2d}', '\u{bd47}', GC_LVT), ('\u{bd48}', '\u{bd48}', GC_LV),
        ('\u{bd49}', '\u{bd63}', GC_LVT), ('\u{bd64}', '\u{bd64}', GC_LV), ('\u{bd65}', '\u{bd7f}',
        GC_LVT), ('\u{bd80}', '\u{bd80}', GC_LV), ('\u{bd81}', '\u{bd9b}', GC_LVT), ('\u{bd9c}',
        '\u{bd9c}', GC_LV), ('\u{bd9d}', '\u{bdb7}', GC_LVT), ('\u{bdb8}', '\u{bdb8}', GC_LV),
        ('\u{bdb9}', '\u{bdd3}', GC_LVT), ('\u{bdd4}', '\u{bdd4}', GC_LV), ('\u{bdd5}', '\u{bdef}',
        GC_LVT), ('\u{bdf0}', '\u{bdf0}', GC_LV), ('\u{bdf1}', '\u{be0b}', GC_LVT), ('\u{be0c}',
        '\u{be0c}', GC_LV), ('\u{be0d}', '\u{be27}', GC_LVT), ('\u{be28}', '\u{be28}', GC_LV),
        ('\u{be29}', '\u{be43}', GC_LVT), ('\u{be44}', '\u{be44}', GC_LV), ('\u{be45}', '\u{be5f}',
        GC_LVT), ('\u{be60}', '\u{be60}', GC_LV), ('\u{be61}', '\u{be7b}', GC_LVT), ('\u{be7c}',
        '\u{be7c}', GC_LV), ('\u{be7d}', '\u{be97}', GC_LVT), ('\u{be98}', '\u{be98}', GC_LV),
        ('\u{be99}', '\u{beb3}', GC_LVT), ('\u{beb4}', '\u{beb4}', GC_LV), ('\u{beb5}', '\u{becf}',
        GC_LVT), ('\u{bed0}', '\u{bed0}', GC_LV), ('\u{bed1}', '\u{beeb}', GC_LVT), ('\u{beec}',
        '\u{beec}', GC_LV), ('\u{beed}', '\u{bf07}', GC_LVT), ('\u{bf08}', '\u{bf08}', GC_LV),
        ('\u{bf09}', '\u{bf23}', GC_LVT), ('\u{bf24}', '\u{bf24}', GC_LV), ('\u{bf25}', '\u{bf3f}',
        GC_LVT), ('\u{bf40}', '\u{bf40}', GC_LV), ('\u{bf41}', '\u{bf5b}', GC_LVT), ('\u{bf5c}',
        '\u{bf5c}', GC_LV), ('\u{bf5d}', '\u{bf77}', GC_LVT), ('\u{bf78}', '\u{bf78}', GC_LV),
        ('\u{bf79}', '\u{bf93}', GC_LVT), ('\u{bf94}', '\u{bf94}', GC_LV), ('\u{bf95}', '\u{bfaf}',
        GC_LVT), ('\u{bfb0}', '\u{bfb0}', GC_LV), ('\u{bfb1}', '\u{bfcb}', GC_LVT), ('\u{bfcc}',
        '\u{bfcc}', GC_LV), ('\u{bfcd}', '\u{bfe7}', GC_LVT), ('\u{bfe8}', '\u{bfe8}', GC_LV),
        ('\u{bfe9}', '\u{c003}', GC_LVT), ('\u{c004}', '\u{c004}', GC_LV), ('\u{c005}', '\u{c01f}',
        GC_LVT), ('\u{c020}', '\u{c020}', GC_LV), ('\u{c021}', '\u{c03b}', GC_LVT), ('\u{c03c}',
        '\u{c03c}', GC_LV), ('\u{c03d}', '\u{c057}', GC_LVT), ('\u{c058}', '\u{c058}', GC_LV),
        ('\u{c059}', '\u{c073}', GC_LVT), ('\u{c074}', '\u{c074}', GC_LV), ('\u{c075}', '\u{c08f}',
        GC_LVT), ('\u{c090}', '\u{c090}', GC_LV), ('\u{c091}', '\u{c0ab}', GC_LVT), ('\u{c0ac}',
        '\u{c0ac}', GC_LV), ('\u{c0ad}', '\u{c0c7}', GC_LVT), ('\u{c0c8}', '\u{c0c8}', GC_LV),
        ('\u{c0c9}', '\u{c0e3}', GC_LVT), ('\u{c0e4}', '\u{c0e4}', GC_LV), ('\u{c0e5}', '\u{c0ff}',
        GC_LVT), ('\u{c100}', '\u{c100}', GC_LV), ('\u{c101}', '\u{c11b}', GC_LVT), ('\u{c11c}',
        '\u{c11c}', GC_LV), ('\u{c11d}', '\u{c137}', GC_LVT), ('\u{c138}', '\u{c138}', GC_LV),
        ('\u{c139}', '\u{c153}', GC_LVT), ('\u{c154}', '\u{c154}', GC_LV), ('\u{c155}', '\u{c16f}',
        GC_LVT), ('\u{c170}', '\u{c170}', GC_LV), ('\u{c171}', '\u{c18b}', GC_LVT), ('\u{c18c}',
        '\u{c18c}', GC_LV), ('\u{c18d}', '\u{c1a7}', GC_LVT), ('\u{c1a8}', '\u{c1a8}', GC_LV),
        ('\u{c1a9}', '\u{c1c3}', GC_LVT), ('\u{c1c4}', '\u{c1c4}', GC_LV), ('\u{c1c5}', '\u{c1df}',
        GC_LVT), ('\u{c1e0}', '\u{c1e0}', GC_LV), ('\u{c1e1}', '\u{c1fb}', GC_LVT), ('\u{c1fc}',
        '\u{c1fc}', GC_LV), ('\u{c1fd}', '\u{c217}', GC_LVT), ('\u{c218}', '\u{c218}', GC_LV),
        ('\u{c219}', '\u{c233}', GC_LVT), ('\u{c234}', '\u{c234}', GC_LV), ('\u{c235}', '\u{c24f}',
        GC_LVT), ('\u{c250}', '\u{c250}', GC_LV), ('\u{c251}', '\u{c26b}', GC_LVT), ('\u{c26c}',
        '\u{c26c}', GC_LV), ('\u{c26d}', '\u{c287}', GC_LVT), ('\u{c288}', '\u{c288}', GC_LV),
        ('\u{c289}', '\u{c2a3}', GC_LVT), ('\u{c2a4}', '\u{c2a4}', GC_LV), ('\u{c2a5}', '\u{c2bf}',
        GC_LVT), ('\u{c2c0}', '\u{c2c0}', GC_LV), ('\u{c2c1}', '\u{c2db}', GC_LVT), ('\u{c2dc}',
        '\u{c2dc}', GC_LV), ('\u{c2dd}', '\u{c2f7}', GC_LVT), ('\u{c2f8}', '\u{c2f8}', GC_LV),
        ('\u{c2f9}', '\u{c313}', GC_LVT), ('\u{c314}', '\u{c314}', GC_LV), ('\u{c315}', '\u{c32f}',
        GC_LVT), ('\u{c330}', '\u{c330}', GC_LV), ('\u{c331}', '\u{c34b}', GC_LVT), ('\u{c34c}',
        '\u{c34c}', GC_LV), ('\u{c34d}', '\u{c367}', GC_LVT), ('\u{c368}', '\u{c368}', GC_LV),
        ('\u{c369}', '\u{c383}', GC_LVT), ('\u{c384}', '\u{c384}', GC_LV), ('\u{c385}', '\u{c39f}',
        GC_LVT), ('\u{c3a0}', '\u{c3a0}', GC_LV), ('\u{c3a1}', '\u{c3bb}', GC_LVT), ('\u{c3bc}',
        '\u{c3bc}', GC_LV), ('\u{c3bd}', '\u{c3d7}', GC_LVT), ('\u{c3d8}', '\u{c3d8}', GC_LV),
        ('\u{c3d9}', '\u{c3f3}', GC_LVT), ('\u{c3f4}', '\u{c3f4}', GC_LV), ('\u{c3f5}', '\u{c40f}',
        GC_LVT), ('\u{c410}', '\u{c410}', GC_LV), ('\u{c411}', '\u{c42b}', GC_LVT), ('\u{c42c}',
        '\u{c42c}', GC_LV), ('\u{c42d}', '\u{c447}', GC_LVT), ('\u{c448}', '\u{c448}', GC_LV),
        ('\u{c449}', '\u{c463}', GC_LVT), ('\u{c464}', '\u{c464}', GC_LV), ('\u{c465}', '\u{c47f}',
        GC_LVT), ('\u{c480}', '\u{c480}', GC_LV), ('\u{c481}', '\u{c49b}', GC_LVT), ('\u{c49c}',
        '\u{c49c}', GC_LV), ('\u{c49d}', '\u{c4b7}', GC_LVT), ('\u{c4b8}', '\u{c4b8}', GC_LV),
        ('\u{c4b9}', '\u{c4d3}', GC_LVT), ('\u{c4d4}', '\u{c4d4}', GC_LV), ('\u{c4d5}', '\u{c4ef}',
        GC_LVT), ('\u{c4f0}', '\u{c4f0}', GC_LV), ('\u{c4f1}', '\u{c50b}', GC_LVT), ('\u{c50c}',
        '\u{c50c}', GC_LV), ('\u{c50d}', '\u{c527}', GC_LVT), ('\u{c528}', '\u{c528}', GC_LV),
        ('\u{c529}', '\u{c543}', GC_LVT), ('\u{c544}', '\u{c544}', GC_LV), ('\u{c545}', '\u{c55f}',
        GC_LVT), ('\u{c560}', '\u{c560}', GC_LV), ('\u{c561}', '\u{c57b}', GC_LVT), ('\u{c57c}',
        '\u{c57c}', GC_LV), ('\u{c57d}', '\u{c597}', GC_LVT), ('\u{c598}', '\u{c598}', GC_LV),
        ('\u{c599}', '\u{c5b3}', GC_LVT), ('\u{c5b4}', '\u{c5b4}', GC_LV), ('\u{c5b5}', '\u{c5cf}',
        GC_LVT), ('\u{c5d0}', '\u{c5d0}', GC_LV), ('\u{c5d1}', '\u{c5eb}', GC_LVT), ('\u{c5ec}',
        '\u{c5ec}', GC_LV), ('\u{c5ed}', '\u{c607}', GC_LVT), ('\u{c608}', '\u{c608}', GC_LV),
        ('\u{c609}', '\u{c623}', GC_LVT), ('\u{c624}', '\u{c624}', GC_LV), ('\u{c625}', '\u{c63f}',
        GC_LVT), ('\u{c640}', '\u{c640}', GC_LV), ('\u{c641}', '\u{c65b}', GC_LVT), ('\u{c65c}',
        '\u{c65c}', GC_LV), ('\u{c65d}', '\u{c677}', GC_LVT), ('\u{c678}', '\u{c678}', GC_LV),
        ('\u{c679}', '\u{c693}', GC_LVT), ('\u{c694}', '\u{c694}', GC_LV), ('\u{c695}', '\u{c6af}',
        GC_LVT), ('\u{c6b0}', '\u{c6b0}', GC_LV), ('\u{c6b1}', '\u{c6cb}', GC_LVT), ('\u{c6cc}',
        '\u{c6cc}', GC_LV), ('\u{c6cd}', '\u{c6e7}', GC_LVT), ('\u{c6e8}', '\u{c6e8}', GC_LV),
        ('\u{c6e9}', '\u{c703}', GC_LVT), ('\u{c704}', '\u{c704}', GC_LV), ('\u{c705}', '\u{c71f}',
        GC_LVT), ('\u{c720}', '\u{c720}', GC_LV), ('\u{c721}', '\u{c73b}', GC_LVT), ('\u{c73c}',
        '\u{c73c}', GC_LV), ('\u{c73d}', '\u{c757}', GC_LVT), ('\u{c758}', '\u{c758}', GC_LV),
        ('\u{c759}', '\u{c773}', GC_LVT), ('\u{c774}', '\u{c774}', GC_LV), ('\u{c775}', '\u{c78f}',
        GC_LVT), ('\u{c790}', '\u{c790}', GC_LV), ('\u{c791}', '\u{c7ab}', GC_LVT), ('\u{c7ac}',
        '\u{c7ac}', GC_LV), ('\u{c7ad}', '\u{c7c7}', GC_LVT), ('\u{c7c8}', '\u{c7c8}', GC_LV),
        ('\u{c7c9}', '\u{c7e3}', GC_LVT), ('\u{c7e4}', '\u{c7e4}', GC_LV), ('\u{c7e5}', '\u{c7ff}',
        GC_LVT), ('\u{c800}', '\u{c800}', GC_LV), ('\u{c801}', '\u{c81b}', GC_LVT), ('\u{c81c}',
        '\u{c81c}', GC_LV), ('\u{c81d}', '\u{c837}', GC_LVT), ('\u{c838}', '\u{c838}', GC_LV),
        ('\u{c839}', '\u{c853}', GC_LVT), ('\u{c854}', '\u{c854}', GC_LV), ('\u{c855}', '\u{c86f}',
        GC_LVT), ('\u{c870}', '\u{c870}', GC_LV), ('\u{c871}', '\u{c88b}', GC_LVT), ('\u{c88c}',
        '\u{c88c}', GC_LV), ('\u{c88d}', '\u{c8a7}', GC_LVT), ('\u{c8a8}', '\u{c8a8}', GC_LV),
        ('\u{c8a9}', '\u{c8c3}', GC_LVT), ('\u{c8c4}', '\u{c8c4}', GC_LV), ('\u{c8c5}', '\u{c8df}',
        GC_LVT), ('\u{c8e0}', '\u{c8e0}', GC_LV), ('\u{c8e1}', '\u{c8fb}', GC_LVT), ('\u{c8fc}',
        '\u{c8fc}', GC_LV), ('\u{c8fd}', '\u{c917}', GC_LVT), ('\u{c918}', '\u{c918}', GC_LV),
        ('\u{c919}', '\u{c933}', GC_LVT), ('\u{c934}', '\u{c934}', GC_LV), ('\u{c935}', '\u{c94f}',
        GC_LVT), ('\u{c950}', '\u{c950}', GC_LV), ('\u{c951}', '\u{c96b}', GC_LVT), ('\u{c96c}',
        '\u{c96c}', GC_LV), ('\u{c96d}', '\u{c987}', GC_LVT), ('\u{c988}', '\u{c988}', GC_LV),
        ('\u{c989}', '\u{c9a3}', GC_LVT), ('\u{c9a4}', '\u{c9a4}', GC_LV), ('\u{c9a5}', '\u{c9bf}',
        GC_LVT), ('\u{c9c0}', '\u{c9c0}', GC_LV), ('\u{c9c1}', '\u{c9db}', GC_LVT), ('\u{c9dc}',
        '\u{c9dc}', GC_LV), ('\u{c9dd}', '\u{c9f7}', GC_LVT), ('\u{c9f8}', '\u{c9f8}', GC_LV),
        ('\u{c9f9}', '\u{ca13}', GC_LVT), ('\u{ca14}', '\u{ca14}', GC_LV), ('\u{ca15}', '\u{ca2f}',
        GC_LVT), ('\u{ca30}', '\u{ca30}', GC_LV), ('\u{ca31}', '\u{ca4b}', GC_LVT), ('\u{ca4c}',
        '\u{ca4c}', GC_LV), ('\u{ca4d}', '\u{ca67}', GC_LVT), ('\u{ca68}', '\u{ca68}', GC_LV),
        ('\u{ca69}', '\u{ca83}', GC_LVT), ('\u{ca84}', '\u{ca84}', GC_LV), ('\u{ca85}', '\u{ca9f}',
        GC_LVT), ('\u{caa0}', '\u{caa0}', GC_LV), ('\u{caa1}', '\u{cabb}', GC_LVT), ('\u{cabc}',
        '\u{cabc}', GC_LV), ('\u{cabd}', '\u{cad7}', GC_LVT), ('\u{cad8}', '\u{cad8}', GC_LV),
        ('\u{cad9}', '\u{caf3}', GC_LVT), ('\u{caf4}', '\u{caf4}', GC_LV), ('\u{caf5}', '\u{cb0f}',
        GC_LVT), ('\u{cb10}', '\u{cb10}', GC_LV), ('\u{cb11}', '\u{cb2b}', GC_LVT), ('\u{cb2c}',
        '\u{cb2c}', GC_LV), ('\u{cb2d}', '\u{cb47}', GC_LVT), ('\u{cb48}', '\u{cb48}', GC_LV),
        ('\u{cb49}', '\u{cb63}', GC_LVT), ('\u{cb64}', '\u{cb64}', GC_LV), ('\u{cb65}', '\u{cb7f}',
        GC_LVT), ('\u{cb80}', '\u{cb80}', GC_LV), ('\u{cb81}', '\u{cb9b}', GC_LVT), ('\u{cb9c}',
        '\u{cb9c}', GC_LV), ('\u{cb9d}', '\u{cbb7}', GC_LVT), ('\u{cbb8}', '\u{cbb8}', GC_LV),
        ('\u{cbb9}', '\u{cbd3}', GC_LVT), ('\u{cbd4}', '\u{cbd4}', GC_LV), ('\u{cbd5}', '\u{cbef}',
        GC_LVT), ('\u{cbf0}', '\u{cbf0}', GC_LV), ('\u{cbf1}', '\u{cc0b}', GC_LVT), ('\u{cc0c}',
        '\u{cc0c}', GC_LV), ('\u{cc0d}', '\u{cc27}', GC_LVT), ('\u{cc28}', '\u{cc28}', GC_LV),
        ('\u{cc29}', '\u{cc43}', GC_LVT), ('\u{cc44}', '\u{cc44}', GC_LV), ('\u{cc45}', '\u{cc5f}',
        GC_LVT), ('\u{cc60}', '\u{cc60}', GC_LV), ('\u{cc61}', '\u{cc7b}', GC_LVT), ('\u{cc7c}',
        '\u{cc7c}', GC_LV), ('\u{cc7d}', '\u{cc97}', GC_LVT), ('\u{cc98}', '\u{cc98}', GC_LV),
        ('\u{cc99}', '\u{ccb3}', GC_LVT), ('\u{ccb4}', '\u{ccb4}', GC_LV), ('\u{ccb5}', '\u{cccf}',
        GC_LVT), ('\u{ccd0}', '\u{ccd0}', GC_LV), ('\u{ccd1}', '\u{cceb}', GC_LVT), ('\u{ccec}',
        '\u{ccec}', GC_LV), ('\u{cced}', '\u{cd07}', GC_LVT), ('\u{cd08}', '\u{cd08}', GC_LV),
        ('\u{cd09}', '\u{cd23}', GC_LVT), ('\u{cd24}', '\u{cd24}', GC_LV), ('\u{cd25}', '\u{cd3f}',
        GC_LVT), ('\u{cd40}', '\u{cd40}', GC_LV), ('\u{cd41}', '\u{cd5b}', GC_LVT), ('\u{cd5c}',
        '\u{cd5c}', GC_LV), ('\u{cd5d}', '\u{cd77}', GC_LVT), ('\u{cd78}', '\u{cd78}', GC_LV),
        ('\u{cd79}', '\u{cd93}', GC_LVT), ('\u{cd94}', '\u{cd94}', GC_LV), ('\u{cd95}', '\u{cdaf}',
        GC_LVT), ('\u{cdb0}', '\u{cdb0}', GC_LV), ('\u{cdb1}', '\u{cdcb}', GC_LVT), ('\u{cdcc}',
        '\u{cdcc}', GC_LV), ('\u{cdcd}', '\u{cde7}', GC_LVT), ('\u{cde8}', '\u{cde8}', GC_LV),
        ('\u{cde9}', '\u{ce03}', GC_LVT), ('\u{ce04}', '\u{ce04}', GC_LV), ('\u{ce05}', '\u{ce1f}',
        GC_LVT), ('\u{ce20}', '\u{ce20}', GC_LV), ('\u{ce21}', '\u{ce3b}', GC_LVT), ('\u{ce3c}',
        '\u{ce3c}', GC_LV), ('\u{ce3d}', '\u{ce57}', GC_LVT), ('\u{ce58}', '\u{ce58}', GC_LV),
        ('\u{ce59}', '\u{ce73}', GC_LVT), ('\u{ce74}', '\u{ce74}', GC_LV), ('\u{ce75}', '\u{ce8f}',
        GC_LVT), ('\u{ce90}', '\u{ce90}', GC_LV), ('\u{ce91}', '\u{ceab}', GC_LVT), ('\u{ceac}',
        '\u{ceac}', GC_LV), ('\u{cead}', '\u{cec7}', GC_LVT), ('\u{cec8}', '\u{cec8}', GC_LV),
        ('\u{cec9}', '\u{cee3}', GC_LVT), ('\u{cee4}', '\u{cee4}', GC_LV), ('\u{cee5}', '\u{ceff}',
        GC_LVT), ('\u{cf00}', '\u{cf00}', GC_LV), ('\u{cf01}', '\u{cf1b}', GC_LVT), ('\u{cf1c}',
        '\u{cf1c}', GC_LV), ('\u{cf1d}', '\u{cf37}', GC_LVT), ('\u{cf38}', '\u{cf38}', GC_LV),
        ('\u{cf39}', '\u{cf53}', GC_LVT), ('\u{cf54}', '\u{cf54}', GC_LV), ('\u{cf55}', '\u{cf6f}',
        GC_LVT), ('\u{cf70}', '\u{cf70}', GC_LV), ('\u{cf71}', '\u{cf8b}', GC_LVT), ('\u{cf8c}',
        '\u{cf8c}', GC_LV), ('\u{cf8d}', '\u{cfa7}', GC_LVT), ('\u{cfa8}', '\u{cfa8}', GC_LV),
        ('\u{cfa9}', '\u{cfc3}', GC_LVT), ('\u{cfc4}', '\u{cfc4}', GC_LV), ('\u{cfc5}', '\u{cfdf}',
        GC_LVT), ('\u{cfe0}', '\u{cfe0}', GC_LV), ('\u{cfe1}', '\u{cffb}', GC_LVT), ('\u{cffc}',
        '\u{cffc}', GC_LV), ('\u{cffd}', '\u{d017}', GC_LVT), ('\u{d018}', '\u{d018}', GC_LV),
        ('\u{d019}', '\u{d033}', GC_LVT), ('\u{d034}', '\u{d034}', GC_LV), ('\u{d035}', '\u{d04f}',
        GC_LVT), ('\u{d050}', '\u{d050}', GC_LV), ('\u{d051}', '\u{d06b}', GC_LVT), ('\u{d06c}',
        '\u{d06c}', GC_LV), ('\u{d06d}', '\u{d087}', GC_LVT), ('\u{d088}', '\u{d088}', GC_LV),
        ('\u{d089}', '\u{d0a3}', GC_LVT), ('\u{d0a4}', '\u{d0a4}', GC_LV), ('\u{d0a5}', '\u{d0bf}',
        GC_LVT), ('\u{d0c0}', '\u{d0c0}', GC_LV), ('\u{d0c1}', '\u{d0db}', GC_LVT), ('\u{d0dc}',
        '\u{d0dc}', GC_LV), ('\u{d0dd}', '\u{d0f7}', GC_LVT), ('\u{d0f8}', '\u{d0f8}', GC_LV),
        ('\u{d0f9}', '\u{d113}', GC_LVT), ('\u{d114}', '\u{d114}', GC_LV), ('\u{d115}', '\u{d12f}',
        GC_LVT), ('\u{d130}', '\u{d130}', GC_LV), ('\u{d131}', '\u{d14b}', GC_LVT), ('\u{d14c}',
        '\u{d14c}', GC_LV), ('\u{d14d}', '\u{d167}', GC_LVT), ('\u{d168}', '\u{d168}', GC_LV),
        ('\u{d169}', '\u{d183}', GC_LVT), ('\u{d184}', '\u{d184}', GC_LV), ('\u{d185}', '\u{d19f}',
        GC_LVT), ('\u{d1a0}', '\u{d1a0}', GC_LV), ('\u{d1a1}', '\u{d1bb}', GC_LVT), ('\u{d1bc}',
        '\u{d1bc}', GC_LV), ('\u{d1bd}', '\u{d1d7}', GC_LVT), ('\u{d1d8}', '\u{d1d8}', GC_LV),
        ('\u{d1d9}', '\u{d1f3}', GC_LVT), ('\u{d1f4}', '\u{d1f4}', GC_LV), ('\u{d1f5}', '\u{d20f}',
        GC_LVT), ('\u{d210}', '\u{d210}', GC_LV), ('\u{d211}', '\u{d22b}', GC_LVT), ('\u{d22c}',
        '\u{d22c}', GC_LV), ('\u{d22d}', '\u{d247}', GC_LVT), ('\u{d248}', '\u{d248}', GC_LV),
        ('\u{d249}', '\u{d263}', GC_LVT), ('\u{d264}', '\u{d264}', GC_LV), ('\u{d265}', '\u{d27f}',
        GC_LVT), ('\u{d280}', '\u{d280}', GC_LV), ('\u{d281}', '\u{d29b}', GC_LVT), ('\u{d29c}',
        '\u{d29c}', GC_LV), ('\u{d29d}', '\u{d2b7}', GC_LVT), ('\u{d2b8}', '\u{d2b8}', GC_LV),
        ('\u{d2b9}', '\u{d2d3}', GC_LVT), ('\u{d2d4}', '\u{d2d4}', GC_LV), ('\u{d2d5}', '\u{d2ef}',
        GC_LVT), ('\u{d2f0}', '\u{d2f0}', GC_LV), ('\u{d2f1}', '\u{d30b}', GC_LVT), ('\u{d30c}',
        '\u{d30c}', GC_LV), ('\u{d30d}', '\u{d327}', GC_LVT), ('\u{d328}', '\u{d328}', GC_LV),
        ('\u{d329}', '\u{d343}', GC_LVT), ('\u{d344}', '\u{d344}', GC_LV), ('\u{d345}', '\u{d35f}',
        GC_LVT), ('\u{d360}', '\u{d360}', GC_LV), ('\u{d361}', '\u{d37b}', GC_LVT), ('\u{d37c}',
        '\u{d37c}', GC_LV), ('\u{d37d}', '\u{d397}', GC_LVT), ('\u{d398}', '\u{d398}', GC_LV),
        ('\u{d399}', '\u{d3b3}', GC_LVT), ('\u{d3b4}', '\u{d3b4}', GC_LV), ('\u{d3b5}', '\u{d3cf}',
        GC_LVT), ('\u{d3d0}', '\u{d3d0}', GC_LV), ('\u{d3d1}', '\u{d3eb}', GC_LVT), ('\u{d3ec}',
        '\u{d3ec}', GC_LV), ('\u{d3ed}', '\u{d407}', GC_LVT), ('\u{d408}', '\u{d408}', GC_LV),
        ('\u{d409}', '\u{d423}', GC_LVT), ('\u{d424}', '\u{d424}', GC_LV), ('\u{d425}', '\u{d43f}',
        GC_LVT), ('\u{d440}', '\u{d440}', GC_LV), ('\u{d441}', '\u{d45b}', GC_LVT), ('\u{d45c}',
        '\u{d45c}', GC_LV), ('\u{d45d}', '\u{d477}', GC_LVT), ('\u{d478}', '\u{d478}', GC_LV),
        ('\u{d479}', '\u{d493}', GC_LVT), ('\u{d494}', '\u{d494}', GC_LV), ('\u{d495}', '\u{d4af}',
        GC_LVT), ('\u{d4b0}', '\u{d4b0}', GC_LV), ('\u{d4b1}', '\u{d4cb}', GC_LVT), ('\u{d4cc}',
        '\u{d4cc}', GC_LV), ('\u{d4cd}', '\u{d4e7}', GC_LVT), ('\u{d4e8}', '\u{d4e8}', GC_LV),
        ('\u{d4e9}', '\u{d503}', GC_LVT), ('\u{d504}', '\u{d504}', GC_LV), ('\u{d505}', '\u{d51f}',
        GC_LVT), ('\u{d520}', '\u{d520}', GC_LV), ('\u{d521}', '\u{d53b}', GC_LVT), ('\u{d53c}',
        '\u{d53c}', GC_LV), ('\u{d53d}', '\u{d557}', GC_LVT), ('\u{d558}', '\u{d558}', GC_LV),
        ('\u{d559}', '\u{d573}', GC_LVT), ('\u{d574}', '\u{d574}', GC_LV), ('\u{d575}', '\u{d58f}',
        GC_LVT), ('\u{d590}', '\u{d590}', GC_LV), ('\u{d591}', '\u{d5ab}', GC_LVT), ('\u{d5ac}',
        '\u{d5ac}', GC_LV), ('\u{d5ad}', '\u{d5c7}', GC_LVT), ('\u{d5c8}', '\u{d5c8}', GC_LV),
        ('\u{d5c9}', '\u{d5e3}', GC_LVT), ('\u{d5e4}', '\u{d5e4}', GC_LV), ('\u{d5e5}', '\u{d5ff}',
        GC_LVT), ('\u{d600}', '\u{d600}', GC_LV), ('\u{d601}', '\u{d61b}', GC_LVT), ('\u{d61c}',
        '\u{d61c}', GC_LV), ('\u{d61d}', '\u{d637}', GC_LVT), ('\u{d638}', '\u{d638}', GC_LV),
        ('\u{d639}', '\u{d653}', GC_LVT), ('\u{d654}', '\u{d654}', GC_LV), ('\u{d655}', '\u{d66f}',
        GC_LVT), ('\u{d670}', '\u{d670}', GC_LV), ('\u{d671}', '\u{d68b}', GC_LVT), ('\u{d68c}',
        '\u{d68c}', GC_LV), ('\u{d68d}', '\u{d6a7}', GC_LVT), ('\u{d6a8}', '\u{d6a8}', GC_LV),
        ('\u{d6a9}', '\u{d6c3}', GC_LVT), ('\u{d6c4}', '\u{d6c4}', GC_LV), ('\u{d6c5}', '\u{d6df}',
        GC_LVT), ('\u{d6e0}', '\u{d6e0}', GC_LV), ('\u{d6e1}', '\u{d6fb}', GC_LVT), ('\u{d6fc}',
        '\u{d6fc}', GC_LV), ('\u{d6fd}', '\u{d717}', GC_LVT), ('\u{d718}', '\u{d718}', GC_LV),
        ('\u{d719}', '\u{d733}', GC_LVT), ('\u{d734}', '\u{d734}', GC_LV), ('\u{d735}', '\u{d74f}',
        GC_LVT), ('\u{d750}', '\u{d750}', GC_LV), ('\u{d751}', '\u{d76b}', GC_LVT), ('\u{d76c}',
        '\u{d76c}', GC_LV), ('\u{d76d}', '\u{d787}', GC_LVT), ('\u{d788}', '\u{d788}', GC_LV),
        ('\u{d789}', '\u{d7a3}', GC_LVT), ('\u{d7b0}', '\u{d7c6}', GC_V), ('\u{d7cb}', '\u{d7fb}',
        GC_T), ('\u{fb1e}', '\u{fb1e}', GC_Extend), ('\u{fe00}', '\u{fe0f}', GC_Extend),
        ('\u{fe20}', '\u{fe2f}', GC_Extend), ('\u{feff}', '\u{feff}', GC_Control), ('\u{ff9e}',
        '\u{ff9f}', GC_Extend), ('\u{fff0}', '\u{fffb}', GC_Control), ('\u{101fd}', '\u{101fd}',
        GC_Extend), ('\u{102e0}', '\u{102e0}', GC_Extend), ('\u{10376}', '\u{1037a}', GC_Extend),
        ('\u{10a01}', '\u{10a03}', GC_Extend), ('\u{10a05}', '\u{10a06}', GC_Extend), ('\u{10a0c}',
        '\u{10a0f}', GC_Extend), ('\u{10a38}', '\u{10a3a}', GC_Extend), ('\u{10a3f}', '\u{10a3f}',
        GC_Extend), ('\u{10ae5}', '\u{10ae6}', GC_Extend), ('\u{11000}', '\u{11000}',
        GC_SpacingMark), ('\u{11001}', '\u{11001}', GC_Extend), ('\u{11002}', '\u{11002}',
        GC_SpacingMark), ('\u{11038}', '\u{11046}', GC_Extend), ('\u{1107f}', '\u{11081}',
        GC_Extend), ('\u{11082}', '\u{11082}', GC_SpacingMark), ('\u{110b0}', '\u{110b2}',
        GC_SpacingMark), ('\u{110b3}', '\u{110b6}', GC_Extend), ('\u{110b7}', '\u{110b8}',
        GC_SpacingMark), ('\u{110b9}', '\u{110ba}', GC_Extend), ('\u{110bd}', '\u{110bd}',
        GC_Prepend), ('\u{11100}', '\u{11102}', GC_Extend), ('\u{11127}', '\u{1112b}', GC_Extend),
        ('\u{1112c}', '\u{1112c}', GC_SpacingMark), ('\u{1112d}', '\u{11134}', GC_Extend),
        ('\u{11173}', '\u{11173}', GC_Extend), ('\u{11180}', '\u{11181}', GC_Extend), ('\u{11182}',
        '\u{11182}', GC_SpacingMark), ('\u{111b3}', '\u{111b5}', GC_SpacingMark), ('\u{111b6}',
        '\u{111be}', GC_Extend), ('\u{111bf}', '\u{111c0}', GC_SpacingMark), ('\u{111c2}',
        '\u{111c3}', GC_Prepend), ('\u{111ca}', '\u{111cc}', GC_Extend), ('\u{1122c}', '\u{1122e}',
        GC_SpacingMark), ('\u{1122f}', '\u{11231}', GC_Extend), ('\u{11232}', '\u{11233}',
        GC_SpacingMark), ('\u{11234}', '\u{11234}', GC_Extend), ('\u{11235}', '\u{11235}',
        GC_SpacingMark), ('\u{11236}', '\u{11237}', GC_Extend), ('\u{1123e}', '\u{1123e}',
        GC_Extend), ('\u{112df}', '\u{112df}', GC_Extend), ('\u{112e0}', '\u{112e2}',
        GC_SpacingMark), ('\u{112e3}', '\u{112ea}', GC_Extend), ('\u{11300}', '\u{11301}',
        GC_Extend), ('\u{11302}', '\u{11303}', GC_SpacingMark), ('\u{1133c}', '\u{1133c}',
        GC_Extend), ('\u{1133e}', '\u{1133e}', GC_Extend), ('\u{1133f}', '\u{1133f}',
        GC_SpacingMark), ('\u{11340}', '\u{11340}', GC_Extend), ('\u{11341}', '\u{11344}',
        GC_SpacingMark), ('\u{11347}', '\u{11348}', GC_SpacingMark), ('\u{1134b}', '\u{1134d}',
        GC_SpacingMark), ('\u{11357}', '\u{11357}', GC_Extend), ('\u{11362}', '\u{11363}',
        GC_SpacingMark), ('\u{11366}', '\u{1136c}', GC_Extend), ('\u{11370}', '\u{11374}',
        GC_Extend), ('\u{11435}', '\u{11437}', GC_SpacingMark), ('\u{11438}', '\u{1143f}',
        GC_Extend), ('\u{11440}', '\u{11441}', GC_SpacingMark), ('\u{11442}', '\u{11444}',
        GC_Extend), ('\u{11445}', '\u{11445}', GC_SpacingMark), ('\u{11446}', '\u{11446}',
        GC_Extend), ('\u{114b0}', '\u{114b0}', GC_Extend), ('\u{114b1}', '\u{114b2}',
        GC_SpacingMark), ('\u{114b3}', '\u{114b8}', GC_Extend), ('\u{114b9}', '\u{114b9}',
        GC_SpacingMark), ('\u{114ba}', '\u{114ba}', GC_Extend), ('\u{114bb}', '\u{114bc}',
        GC_SpacingMark), ('\u{114bd}', '\u{114bd}', GC_Extend), ('\u{114be}', '\u{114be}',
        GC_SpacingMark), ('\u{114bf}', '\u{114c0}', GC_Extend), ('\u{114c1}', '\u{114c1}',
        GC_SpacingMark), ('\u{114c2}', '\u{114c3}', GC_Extend), ('\u{115af}', '\u{115af}',
        GC_Extend), ('\u{115b0}', '\u{115b1}', GC_SpacingMark), ('\u{115b2}', '\u{115b5}',
        GC_Extend), ('\u{115b8}', '\u{115bb}', GC_SpacingMark), ('\u{115bc}', '\u{115bd}',
        GC_Extend), ('\u{115be}', '\u{115be}', GC_SpacingMark), ('\u{115bf}', '\u{115c0}',
        GC_Extend), ('\u{115dc}', '\u{115dd}', GC_Extend), ('\u{11630}', '\u{11632}',
        GC_SpacingMark), ('\u{11633}', '\u{1163a}', GC_Extend), ('\u{1163b}', '\u{1163c}',
        GC_SpacingMark), ('\u{1163d}', '\u{1163d}', GC_Extend), ('\u{1163e}', '\u{1163e}',
        GC_SpacingMark), ('\u{1163f}', '\u{11640}', GC_Extend), ('\u{116ab}', '\u{116ab}',
        GC_Extend), ('\u{116ac}', '\u{116ac}', GC_SpacingMark), ('\u{116ad}', '\u{116ad}',
        GC_Extend), ('\u{116ae}', '\u{116af}', GC_SpacingMark), ('\u{116b0}', '\u{116b5}',
        GC_Extend), ('\u{116b6}', '\u{116b6}', GC_SpacingMark), ('\u{116b7}', '\u{116b7}',
        GC_Extend), ('\u{1171d}', '\u{1171f}', GC_Extend), ('\u{11720}', '\u{11721}',
        GC_SpacingMark), ('\u{11722}', '\u{11725}', GC_Extend), ('\u{11726}', '\u{11726}',
        GC_SpacingMark), ('\u{11727}', '\u{1172b}', GC_Extend), ('\u{11c2f}', '\u{11c2f}',
        GC_SpacingMark), ('\u{11c30}', '\u{11c36}', GC_Extend), ('\u{11c38}', '\u{11c3d}',
        GC_Extend), ('\u{11c3e}', '\u{11c3e}', GC_SpacingMark), ('\u{11c3f}', '\u{11c3f}',
        GC_Extend), ('\u{11c92}', '\u{11ca7}', GC_Extend), ('\u{11ca9}', '\u{11ca9}',
        GC_SpacingMark), ('\u{11caa}', '\u{11cb0}', GC_Extend), ('\u{11cb1}', '\u{11cb1}',
        GC_SpacingMark), ('\u{11cb2}', '\u{11cb3}', GC_Extend), ('\u{11cb4}', '\u{11cb4}',
        GC_SpacingMark), ('\u{11cb5}', '\u{11cb6}', GC_Extend), ('\u{16af0}', '\u{16af4}',
        GC_Extend), ('\u{16b30}', '\u{16b36}', GC_Extend), ('\u{16f51}', '\u{16f7e}',
        GC_SpacingMark), ('\u{16f8f}', '\u{16f92}', GC_Extend), ('\u{1bc9d}', '\u{1bc9e}',
        GC_Extend), ('\u{1bca0}', '\u{1bca3}', GC_Control), ('\u{1d165}', '\u{1d165}', GC_Extend),
        ('\u{1d166}', '\u{1d166}', GC_SpacingMark), ('\u{1d167}', '\u{1d169}', GC_Extend),
        ('\u{1d16d}', '\u{1d16d}', GC_SpacingMark), ('\u{1d16e}', '\u{1d172}', GC_Extend),
        ('\u{1d173}', '\u{1d17a}', GC_Control), ('\u{1d17b}', '\u{1d182}', GC_Extend), ('\u{1d185}',
        '\u{1d18b}', GC_Extend), ('\u{1d1aa}', '\u{1d1ad}', GC_Extend), ('\u{1d242}', '\u{1d244}',
        GC_Extend), ('\u{1da00}', '\u{1da36}', GC_Extend), ('\u{1da3b}', '\u{1da6c}', GC_Extend),
        ('\u{1da75}', '\u{1da75}', GC_Extend), ('\u{1da84}', '\u{1da84}', GC_Extend), ('\u{1da9b}',
        '\u{1da9f}', GC_Extend), ('\u{1daa1}', '\u{1daaf}', GC_Extend), ('\u{1e000}', '\u{1e006}',
        GC_Extend), ('\u{1e008}', '\u{1e018}', GC_Extend), ('\u{1e01b}', '\u{1e021}', GC_Extend),
        ('\u{1e023}', '\u{1e024}', GC_Extend), ('\u{1e026}', '\u{1e02a}', GC_Extend), ('\u{1e8d0}',
        '\u{1e8d6}', GC_Extend), ('\u{1e944}', '\u{1e94a}', GC_Extend), ('\u{1f1e6}', '\u{1f1ff}',
        GC_Regional_Indicator), ('\u{1f385}', '\u{1f385}', GC_E_Base), ('\u{1f3c3}', '\u{1f3c4}',
        GC_E_Base), ('\u{1f3ca}', '\u{1f3cb}', GC_E_Base), ('\u{1f3fb}', '\u{1f3ff}',
        GC_E_Modifier), ('\u{1f442}', '\u{1f443}', GC_E_Base), ('\u{1f446}', '\u{1f450}',
        GC_E_Base), ('\u{1f466}', '\u{1f469}', GC_E_Base_GAZ), ('\u{1f46e}', '\u{1f46e}',
        GC_E_Base), ('\u{1f470}', '\u{1f478}', GC_E_Base), ('\u{1f47c}', '\u{1f47c}', GC_E_Base),
        ('\u{1f481}', '\u{1f483}', GC_E_Base), ('\u{1f485}', '\u{1f487}', GC_E_Base), ('\u{1f48b}',
        '\u{1f48b}', GC_Glue_After_Zwj), ('\u{1f4aa}', '\u{1f4aa}', GC_E_Base), ('\u{1f575}',
        '\u{1f575}', GC_E_Base), ('\u{1f57a}', '\u{1f57a}', GC_E_Base), ('\u{1f590}', '\u{1f590}',
        GC_E_Base), ('\u{1f595}', '\u{1f596}', GC_E_Base), ('\u{1f5e8}', '\u{1f5e8}',
        GC_Glue_After_Zwj), ('\u{1f645}', '\u{1f647}', GC_E_Base), ('\u{1f64b}', '\u{1f64f}',
        GC_E_Base), ('\u{1f6a3}', '\u{1f6a3}', GC_E_Base), ('\u{1f6b4}', '\u{1f6b6}', GC_E_Base),
        ('\u{1f6c0}', '\u{1f6c0}', GC_E_Base), ('\u{1f918}', '\u{1f91e}', GC_E_Base), ('\u{1f926}',
        '\u{1f926}', GC_E_Base), ('\u{1f930}', '\u{1f930}', GC_E_Base), ('\u{1f933}', '\u{1f939}',
        GC_E_Base), ('\u{1f93c}', '\u{1f93e}', GC_E_Base), ('\u{e0000}', '\u{e001f}', GC_Control),
        ('\u{e0020}', '\u{e007f}', GC_Extend), ('\u{e0080}', '\u{e00ff}', GC_Control), ('\u{e0100}',
        '\u{e01ef}', GC_Extend), ('\u{e01f0}', '\u{e0fff}', GC_Control)
    ];

}

#[derive(Clone)]
pub struct GraphemeIndices<'a> {
    start_offset: usize,
    iter: Graphemes<'a>,
}

impl<'a> Iterator for GraphemeIndices<'a> {
    type Item = (usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(usize, &'a str)> {
        self.iter
            .next()
            .map(|s| (s.as_ptr() as usize - self.start_offset, s))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// External iterator for a string's
/// [grapheme clusters](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries).
#[derive(Clone)]
pub struct Graphemes<'a> {
    string: &'a str,
    cursor: GraphemeCursor,
    cursor_back: GraphemeCursor,
}

impl<'a> Iterator for Graphemes<'a> {
    type Item = &'a str;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.cursor_back.offset - self.cursor.offset;
        (std::cmp::min(slen, 1), Some(slen))
    }

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let start = self.cursor.offset;
        if start == self.cursor_back.offset {
            return None;
        }
        let next = self.cursor.next_boundary(self.string);
        Some(&self.string[start..next])
    }
}

impl<'a> DoubleEndedIterator for Graphemes<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        let end = self.cursor_back.offset;
        if end == self.cursor.offset {
            return None;
        }
        let prev = self.cursor_back.prev_boundary(self.string, 0);
        Some(&self.string[prev..end])
    }
}

#[inline]
pub fn new_graphemes<'b>(s: &'b str) -> Graphemes<'b> {
    let len = s.len();
    Graphemes {
        string: s,
        cursor: GraphemeCursor::new(0, len),
        cursor_back: GraphemeCursor::new(len, len),
    }
}

#[inline]
pub fn new_grapheme_indices<'b>(s: &'b str) -> GraphemeIndices<'b> {
    GraphemeIndices {
        start_offset: s.as_ptr() as usize,
        iter: new_graphemes(s),
    }
}

// maybe unify with PairResult?
// An enum describing information about a potential boundary.
#[derive(PartialEq, Eq, Clone)]
enum GraphemeState {
    // No information is known.
    Unknown,
    // It is known to not be a boundary.
    NotBreak,
    // It is known to be a boundary.
    Break,
    // The codepoint after is a Regional Indicator Symbol, so a boundary iff
    // it is preceded by an even number of RIS codepoints. (GB12, GB13)
    Regional,
    // The codepoint after is in the E_Modifier category, so whether it's a boundary
    // depends on pre-context according to GB10.
    Emoji,
}

/// Cursor-based segmenter for grapheme clusters.
#[derive(Clone)]
pub struct GraphemeCursor {
    // Current cursor position.
    offset: usize,
    // Total length of the string.
    len: usize,
    // Information about the potential boundary at `offset`
    state: GraphemeState,
    // Category of codepoint immediately preceding cursor, if known.
    cat_before: Option<GraphemeCat>,
    // Category of codepoint immediately after cursor, if known.
    cat_after: Option<GraphemeCat>,
    // If set, at least one more codepoint immediately preceding this offset
    // is needed to resolve whether there's a boundary at `offset`.
    pre_context_offset: Option<usize>,
    // The number of RIS codepoints preceding `offset`. If `pre_context_offset`
    // is set, then counts the number of RIS between that and `offset`, otherwise
    // is an accurate count relative to the string.
    ris_count: Option<usize>,
}

// An enum describing the result from lookup of a pair of categories.
#[derive(PartialEq, Eq)]
enum PairResult {
    NotBreak, // definitely not a break
    Break,    // definitely a break
    Regional, // a break if preceded by an even number of RIS
    Emoji,    // a break if preceded by emoji base and (Extend)*
}

fn check_pair(before: GraphemeCat, after: GraphemeCat) -> PairResult {
    use self::PairResult::*;
    use grapheme::GraphemeCat::*;
    match (before, after) {
        (GC_CR, GC_LF) => NotBreak,                                 // GB3
        (GC_Control, _) => Break,                                   // GB4
        (GC_CR, _) => Break,                                        // GB4
        (GC_LF, _) => Break,                                        // GB4
        (_, GC_Control) => Break,                                   // GB5
        (_, GC_CR) => Break,                                        // GB5
        (_, GC_LF) => Break,                                        // GB5
        (GC_L, GC_L) => NotBreak,                                   // GB6
        (GC_L, GC_V) => NotBreak,                                   // GB6
        (GC_L, GC_LV) => NotBreak,                                  // GB6
        (GC_L, GC_LVT) => NotBreak,                                 // GB6
        (GC_LV, GC_V) => NotBreak,                                  // GB7
        (GC_LV, GC_T) => NotBreak,                                  // GB7
        (GC_V, GC_V) => NotBreak,                                   // GB7
        (GC_V, GC_T) => NotBreak,                                   // GB7
        (GC_LVT, GC_T) => NotBreak,                                 // GB8
        (GC_T, GC_T) => NotBreak,                                   // GB8
        (_, GC_Extend) => NotBreak,                                 // GB9
        (_, GC_ZWJ) => NotBreak,                                    // GB9
        (_, GC_SpacingMark) => NotBreak,                            // GB9a
        (GC_Prepend, _) => NotBreak,                                // GB9b
        (GC_E_Base, GC_E_Modifier) => NotBreak,                     // GB10
        (GC_E_Base_GAZ, GC_E_Modifier) => NotBreak,                 // GB10
        (GC_Extend, GC_E_Modifier) => Emoji,                        // GB10
        (GC_ZWJ, GC_Glue_After_Zwj) => NotBreak,                    // GB11
        (GC_ZWJ, GC_E_Base_GAZ) => NotBreak,                        // GB11
        (GC_Regional_Indicator, GC_Regional_Indicator) => Regional, // GB12, GB13
        (_, _) => Break,                                            // GB999
    }
}

macro_rules! decide {
    ($self:expr, $is_break:expr) => {
        $self.state = if $is_break {
            GraphemeState::Break
        } else {
            GraphemeState::NotBreak
        };
    };
    ($self:expr, $is_break:expr, return) => {{
        decide!($self, $is_break);
        $is_break
    }};
}

impl GraphemeCursor {
    pub fn new(offset: usize, len: usize) -> GraphemeCursor {
        GraphemeCursor {
            offset,
            len,
            state: GraphemeState::Break,
            cat_before: None,
            cat_after: None,
            pre_context_offset: None,
            ris_count: None,
        }
    }

    #[perf_viz::record]
    fn handle_regional(&mut self, chunk: &str) {
        use grapheme as gr;
        let mut ris_count = self.ris_count.unwrap_or(0);
        for ch in chunk.chars().rev() {
            if gr::grapheme_category(ch) != gr::GC_Regional_Indicator {
                self.ris_count = Some(ris_count);
                decide!(self, (ris_count % 2) == 0);
                return;
            }
            ris_count += 1;
        }
        self.ris_count = Some(ris_count);
        decide!(self, (ris_count % 2) == 0);
    }

    #[perf_viz::record]
    fn handle_emoji(&mut self, chunk: &str) {
        use grapheme as gr;
        for ch in chunk.chars().rev() {
            match gr::grapheme_category(ch) {
                gr::GC_Extend => (),
                gr::GC_E_Base | gr::GC_E_Base_GAZ => {
                    decide!(self, false);
                    return;
                }
                _ => {
                    decide!(self, true);
                    return;
                }
            }
        }
        decide!(self, true);
    }

    #[perf_viz::record]
    pub fn is_boundary(&mut self, chunk: &str, chunk_start: usize) -> bool {
        use grapheme as gr;
        if self.state == GraphemeState::Break {
            return true;
        }
        if self.state == GraphemeState::NotBreak {
            return false;
        }
        let offset_in_chunk = self.offset - chunk_start;
        if self.cat_after.is_none() {
            let ch = chunk[offset_in_chunk..].chars().next().unwrap();
            self.cat_after = Some(gr::grapheme_category(ch));
        }
        if self.offset == chunk_start {
            match self.cat_after.unwrap() {
                gr::GC_Regional_Indicator => self.state = GraphemeState::Regional,
                gr::GC_E_Modifier => self.state = GraphemeState::Emoji,
                _ => {}
            }
        }
        if self.cat_before.is_none() {
            let ch = chunk[..offset_in_chunk].chars().rev().next().unwrap();
            self.cat_before = Some(gr::grapheme_category(ch));
        }
        match check_pair(self.cat_before.unwrap(), self.cat_after.unwrap()) {
            PairResult::NotBreak => return decide!(self, false, return),
            PairResult::Break => return decide!(self, true, return),
            PairResult::Regional => {
                if let Some(ris_count) = self.ris_count {
                    return decide!(self, (ris_count % 2) == 0, return);
                }
                self.handle_regional(&chunk[..offset_in_chunk]);
            }
            PairResult::Emoji => {
                self.handle_emoji(&chunk[..offset_in_chunk]);
            }
        }
        if self.state == GraphemeState::Break {
            true
        } else if self.state == GraphemeState::NotBreak {
            false
        } else {
            unreachable!("inconsistent state and we were unwrapping before");
        }
    }

    #[perf_viz::record]
    pub fn next_boundary(&mut self, chunk: &str) -> usize {
        use grapheme as gr;
        let mut iter = chunk[self.offset..].chars();
        let mut ch = iter.next().unwrap();
        loop {
            self.offset += ch.len_utf8();
            self.state = GraphemeState::Unknown;
            self.cat_before = self.cat_after.take();
            if self.cat_before.is_none() {
                self.cat_before = Some(gr::grapheme_category(ch));
            }
            if self.cat_before.unwrap() == GraphemeCat::GC_Regional_Indicator {
                self.ris_count = self.ris_count.map(|c| c + 1);
            } else {
                self.ris_count = Some(0);
            }
            if let Some(next_ch) = iter.next() {
                ch = next_ch;
                self.cat_after = Some(gr::grapheme_category(ch));
            } else if self.offset == self.len {
                decide!(self, true);
            } else {
                unreachable!("we were unwrapping before");
            }
            if self.is_boundary(chunk, 0) {
                return self.offset;
            }
        }
    }

    #[perf_viz::record]
    pub fn prev_boundary(&mut self, chunk: &str, chunk_start: usize) -> usize {
        use grapheme as gr;
        let mut iter = chunk[..self.offset - chunk_start].chars().rev();
        let mut ch = iter.next().unwrap();
        loop {
            self.offset -= ch.len_utf8();
            self.cat_after = self.cat_before.take();
            self.state = GraphemeState::Unknown;
            if let Some(ris_count) = self.ris_count {
                self.ris_count = if ris_count > 0 {
                    Some(ris_count - 1)
                } else {
                    None
                };
            }
            if let Some(prev_ch) = iter.next() {
                ch = prev_ch;
                self.cat_before = Some(gr::grapheme_category(ch));
            } else if self.offset == 0 {
                decide!(self, true);
            } else {
                unreachable!("we were unwrapping before");
            }

            if self.is_boundary(chunk, chunk_start) {
                return self.offset;
            }
        }
    }
}

pub trait UnicodeSegmentation {
    fn graphemes<'a>(&'a self) -> Graphemes<'a>;

    fn grapheme_indices<'a>(&'a self) -> GraphemeIndices<'a>;
}

impl UnicodeSegmentation for str {
    #[inline]
    fn graphemes(&self) -> Graphemes {
        new_graphemes(self)
    }

    #[inline]
    fn grapheme_indices(&self) -> GraphemeIndices {
        new_grapheme_indices(self)
    }
}
