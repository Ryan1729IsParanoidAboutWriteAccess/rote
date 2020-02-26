use super::*;
use platform_types::pos;
use editor_types::{cur, vec1};
use arb_macros::{arb_enum};
use macros::{u};
use proptest::prelude::{proptest, Strategy};

#[derive(Clone, Copy, Debug)]
enum TabTweak {
    Move(BufferMove),
    SelectNext,
    SelectPrevious,
}

impl TabTweak {
    fn apply(self, buffers: &mut EditorBuffers) {
        u!{TabTweak}
        match self {
            Move(buffer_move) => buffers.move_buffer(buffer_move),
            SelectNext => buffers.select_next(),
            SelectPrevious => buffers.select_previous(),
        }
    }
}

mod arb {
    use super::*;
    use proptest::prelude::{Strategy};

    pub fn at_least_one() -> impl Strategy<Value = f32> {
        proptest::num::f32::POSITIVE.prop_map(|n| 
            if n >= 1.0 {
               n 
            } else {
                // NaN ends up here
                1.0
            }
        )
    }

    arb_enum!{
        pub fn tab_tweak() -> TabTweak
        {
            Move(_) => buffer_move(),
            SelectNext => Just(SelectNext),
            SelectPrevious => Just(SelectPrevious),
        }
    }
}

fn update_and_render_shows_the_cursor_when_pressing_home_on(text: &str, buffer_xywh: TextBoxXYWH, char_dim: CharDim) {
    let mut state: State = text.into();
    state.buffer_xywh = buffer_xywh;
    state.font_info = FontInfo {
        text_char_dim: char_dim,
        status_char_dim: char_dim,
        tab_char_dim: char_dim,
        find_replace_char_dim: char_dim,
    };
    

    dbg!(get_scrollable_buffer_mut!(state));

    update_and_render(&mut state, Input::MoveAllCursors(Move::ToBufferEnd));

    dbg!(get_scrollable_buffer_mut!(state));

    {
        let buffer = get_scrollable_buffer_mut!(state).unwrap();
        assert_eq!(
            buffer.text_buffer.borrow_cursors_vec()[0], cur!{pos!{l 0, o text.len()}},
            "*** Cursor Precondition failure! ***"
        );
        assert_ne!(buffer.scroll.x, 0.0, "*** Scroll Precondition failure! ***");
    }
    

    update_and_render(&mut state, Input::MoveAllCursors(Move::ToBufferStart));

    let buffer = get_scrollable_buffer_mut!(state).unwrap();
    assert_eq!(buffer.scroll.x, 0.0);
}

const CURSOR_SHOW_TEXT: &'static str = "            abcdefghijklmnopqrstuvwxyz::abcdefghijk::abcdefghijklmnopqrstuvwxyz";

#[test]
fn update_and_render_shows_the_cursor_when_pressing_home_in_this_case() {
    update_and_render_shows_the_cursor_when_pressing_home_on(
        CURSOR_SHOW_TEXT,
        tbxywh!(0.0, 0.0, 256.0, 192.0),
        CharDim { w: 4.0, h: 8.0 }
    );
}

macro_rules! max_one {
    ($n: expr) => {{
        let n = $n;
        if n >= 1.0 {
            n
        } else {
            // NaN ends up here
            1.0
        }
    }}
}

fn passes_preconditions(text: &str, buffer_xywh: TextBoxXYWH, char_dim: CharDim) -> bool {
    let mut state: State = text.into();
    state.buffer_xywh = buffer_xywh;
    state.font_info = FontInfo {
        text_char_dim: char_dim,
        status_char_dim: char_dim,
        tab_char_dim: char_dim,
        find_replace_char_dim: char_dim,
    };
    

    dbg!(get_scrollable_buffer_mut!(state));

    update_and_render(&mut state, Input::MoveAllCursors(Move::ToBufferEnd));

    dbg!(get_scrollable_buffer_mut!(state));

    let buffer = get_scrollable_buffer_mut!(state).unwrap();

    buffer.text_buffer.borrow_cursors_vec()[0] == cur!{pos!{l 0, o text.len()}}
    && buffer.scroll.x != 0.0
}

#[test]
fn update_and_render_shows_the_cursor_when_pressing_home() {
    use proptest::test_runner::{TestRunner, TestCaseError};
    let mut runner = TestRunner::default();

    runner.run(&(
        arb::at_least_one(),
        arb::at_least_one(),
        arb::at_least_one(),
        arb::at_least_one(),
    ), |(box_w, box_h, w, h)| {

        let w_max = max_one!(box_w / 2.0);
        let w = if w > w_max {
            w_max
        } else {
            // NaN ends up here
            w
        };

        let h_max = max_one!(box_h / 2.0);
        let h = if h > h_max {
            h_max
        } else {
            // NaN ends up here
            h
        };

        if passes_preconditions(
            CURSOR_SHOW_TEXT,
            tbxywh!(0.0, 0.0, box_w, box_h),
            CharDim { w, h }
        ) {
            update_and_render_shows_the_cursor_when_pressing_home_on(
                CURSOR_SHOW_TEXT,
                tbxywh!(0.0, 0.0, box_w, box_h),
                CharDim { w, h }
            );
            Ok(())
        } else {
            Err(TestCaseError::Reject("failed preconditions".into()))
        }
    }).unwrap();
}

/* this was from before I decided to change how the screen gets auto-scrolled along the x axis.
#[test]
fn update_and_render_shows_the_cursor_when_pressing_home_in_this_realistic_case() {
    update_and_render_shows_the_cursor_when_pressing_home_on(
        CURSOR_SHOW_TEXT,
        tbxywh!(0.0, 0.0, 1920.0, 1080.0),
        CharDim { w: 16.0, h: 32.0 }
    );
}

#[test]
fn update_and_render_shows_the_cursor_when_pressing_home_in_this_reduced_realistic_case() {
    let text = CURSOR_SHOW_TEXT;
    let buffer_xywh = tbxywh!(0.0, 0.0, 1920.0, 1080.0);
    let char_dim = CharDim { w: 16.0, h: 32.0 };

    let mut state: State = text.into();
    state.buffer_xywh = buffer_xywh;
    state.font_info = FontInfo {
        text_char_dim: char_dim,
        status_char_dim: char_dim,
        tab_char_dim: char_dim,
        find_replace_char_dim: char_dim,
    };
    

    dbg!(get_scrollable_buffer_mut!(state));

    update_and_render(&mut state, Input::MoveAllCursors(Move::ToBufferEnd));

    dbg!(get_scrollable_buffer_mut!(state));

    {
        let buffer = get_scrollable_buffer_mut!(state).unwrap();
        assert_eq!(
            buffer.text_buffer.borrow_cursors_vec()[0], cur!{pos!{l 0, o text.len()}},
            "*** Cursor Precondition failure! ***"
        );
        assert_ne!(buffer.scroll.x, 0.0, "*** Scroll Precondition failure! ***");
    }

    let buffer = get_scrollable_buffer_mut!(state).unwrap();

    buffer.text_buffer.move_all_cursors(Move::ToBufferStart);

    let result = try_to_show_cursors_on(buffer, buffer_xywh, char_dim);
    
    dbg!(result);

    assert_eq!(buffer.scroll.x, 0.0);
}
*/

#[test]
fn update_and_render_shows_the_cursor_when_pressing_home_in_this_further_reduced_realistic_case() {
    let text = CURSOR_SHOW_TEXT;
    let xywh = tbxywh!(0.0, 0.0, 1920.0, 1080.0);
    let char_dim = CharDim { w: 16.0, h: 32.0 };    

    let mut buffer = ScrollableBuffer {
        text_buffer: {
            let mut t: TextBuffer = text.into();
            t.set_cursor(
                cur!{pos!{l 0, o text.len()}},
                ReplaceOrAdd::Replace
            );
            t
        },
        scroll: ScrollXY {
            x: 320.0,
            y: 0.0,
        },
    };

    //
    // update_and_render inlined
    buffer.text_buffer.move_all_cursors(Move::ToBufferStart);

    let scroll = &mut buffer.scroll;
    let cursors = buffer.text_buffer.borrow_cursors_vec();

    // We try first with this smaller xywh to make the cursor appear
    // in the center more often.
    let mut small_xywh = xywh.clone();
    small_xywh.xy.x += small_xywh.wh.w / 4.0;
    small_xywh.wh.w /= 2.0;
    small_xywh.xy.y += small_xywh.wh.h / 4.0;
    small_xywh.wh.h /= 2.0;

    let mut attempt_result;
    attempt_result = attempt_to_make_sure_at_least_one_cursor_is_visible(
        scroll,
        small_xywh,
        char_dim,
        cursors,
    );

    assert_eq!(scroll.x, 0.0);

    dbg!(attempt_result);

    if attempt_result != VisibilityAttemptResult::Succeeded {
        dbg!();
        attempt_result = attempt_to_make_sure_at_least_one_cursor_is_visible(
            scroll,
            xywh,
            char_dim,
            cursors,
        );
    }
    //
    //

    dbg!(attempt_result);

    assert_eq!(buffer.scroll.x, 0.0);
}

#[test]
fn attempt_to_make_sure_at_least_one_cursor_is_visible_reports_correctly_in_this_case() {
    let mut scroll = ScrollXY {
            x: 320.0,
            y: 0.0,
        };

    let xywh = tbxywh!(480.0, 270.0, 960.0, 540.0);

    let attempt_result = attempt_to_make_sure_at_least_one_cursor_is_visible(
        &mut scroll,
        xywh,
        CharDim { w: 16.0, h: 32.0 },
        &vec1![cur!{}],
    );

    if scroll.x != 320.0 {
        assert_eq!(attempt_result, VisibilityAttemptResult::Succeeded, "false negative");
    } else {
        assert_ne!(attempt_result, VisibilityAttemptResult::Succeeded, "false positive");
    }
}

#[test]
fn attempt_to_make_xy_visible_reports_correctly_in_this_case() {
    let mut scroll = ScrollXY {
            x: 320.0,
            y: 0.0,
        };

    let xywh = tbxywh!(480.0, 270.0, 960.0, 540.0);

    let attempt_result = attempt_to_make_xy_visible(
        &mut scroll,
        xywh,
        CharDim { w: 16.0, h: 32.0 }.into(),
        TextSpaceXY {
            x: 0.0,
            y: 0.0,
        },
    );

    if scroll.x != 320.0 {
        assert_eq!(attempt_result, VisibilityAttemptResult::Succeeded, "false negative x = {}", scroll.x);
    } else {
        assert_ne!(attempt_result, VisibilityAttemptResult::Succeeded, "false positive x = {}", scroll.x);
    }
}

fn no_tab_tweak_causes_getting_the_current_buffer_to_return_none_on(
    mut state: State,
    tweaks: Vec<TabTweak>,
) {
    // precondition
    assert!(state.get_current_buffer().is_some(), "precondition failed");

    for tweak in tweaks {
        tweak.apply(&mut state);

        assert!(state.get_current_buffer().is_some(), "{:?} caused get_current_buffer to return None", tweak);
    }
}

proptest!{
    #[test]
    fn no_tab_tweak_causes_getting_the_current_buffer_to_return_none(
        state in arb::editor_state(),
        tweaks in arb::tab_tweaks(16),
    ) {
        no_tab_tweak_causes_getting_the_current_buffer_to_return_none_on(
            state,
            tweaks,
        )
    }
}