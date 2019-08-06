use super::*;
use proptest::num::f32;
use proptest::prelude::*;
pub fn usual() -> f32::Any {
    f32::POSITIVE | f32::NEGATIVE | f32::NORMAL | f32::ZERO
}

pub fn scrollable_screen(spec: f32::Any) -> impl Strategy<Value = ScrollableScreen> {
    (spec, spec, spec, spec).prop_map(|(x, y, w, h)| ScrollableScreen {
        scroll: ScrollXY { x, y },
        wh: ScreenSpaceWH { w, h },
    })
}

pub fn char_dim(spec: f32::Any) -> impl Strategy<Value = CharDim> {
    (spec, spec).prop_map(|(w, h)| CharDim { w, h })
}

pub fn text_xy(spec: f32::Any) -> impl Strategy<Value = TextSpaceXY> {
    (spec, spec).prop_map(|(x, y)| TextSpaceXY { x, y })
}

#[allow(dead_code)]
pub fn screen_xy(spec: f32::Any) -> impl Strategy<Value = ScreenSpaceXY> {
    (spec, spec).prop_map(|(x, y)| ScreenSpaceXY { x, y })
}

#[allow(dead_code)]
pub fn wh(spec: f32::Any) -> impl Strategy<Value = ScreenSpaceWH> {
    (spec, spec).prop_map(|(w, h)| ScreenSpaceWH { w, h })
}

pub fn plausible_scrollable_screen() -> impl Strategy<Value = ScrollableScreen> {
    let u = usual();
    let len = f32::POSITIVE | f32::NORMAL | f32::ZERO;
    (u, u, len, len).prop_map(|(x, y, w, h)| ScrollableScreen {
        scroll: ScrollXY { x, y },
        wh: ScreenSpaceWH { w, h },
    })
}
