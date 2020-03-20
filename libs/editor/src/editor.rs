use editor_types::{Cursor};
use macros::{d, u, SaturatingSub};
use platform_types::{screen_positioning::*, *};
use parsers::{Parsers};

use std::path::PathBuf;
use std::collections::VecDeque;
use text_buffer::{next_instance_of_selected, TextBuffer};

mod editor_view;
mod editor_buffers;
use editor_buffers::{
    EditorBuffers, 
    EditorBuffer, 
    SearchResults, 
    update_search_results, 
    ScrollableBuffer
};

#[derive(Debug, Default)]
struct ClipboardHistory {
    entries: VecDeque<String>,
    index: usize,
}

const AVERAGE_SELECTION_SIZE_ESTIMATE: usize = 32;

impl ClipboardHistory {
    fn cut(&mut self, buffer: &mut TextBuffer) -> Option<String> {
        self.push_and_join_into_option(buffer.cut_selections())
    }
    fn copy(&mut self, buffer: &TextBuffer) -> Option<String> {
        self.push_and_join_into_option(buffer.copy_selections())
    }

    fn push_and_join_into_option(&mut self, strings: Vec<String>) -> Option<String> {
        if strings.is_empty() {
            None
        } else {
            let mut output = String::with_capacity(strings.len() * AVERAGE_SELECTION_SIZE_ESTIMATE);

            let mut sep = "";
            for s in strings {
                output.push_str(sep);

                output.push_str(&s);

                self.push_if_does_not_match_top(s);

                sep = "\n";
            }

            Some(output)
        }
    }

    fn push_if_does_not_match_top(&mut self, to_push: String) {
        match self.entries.get(self.index).map(|s| s != &to_push) {
            None => {
                self.entries.push_back(to_push);
            }
            Some(true) => {
                self.entries.push_back(to_push);
                self.index += 1;
            }
            Some(false) => {}
        }
    }

    fn paste(&mut self, buffer: &mut TextBuffer, possible_string: Option<String>) {
        if let Some(s) = possible_string {
            self.push_if_does_not_match_top(s)
        }

        if let Some(s) = self.entries.get(self.index) {
            buffer.insert_string(s.to_owned());
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    // TODO side by side visible buffers
    // visible_buffers: VisibleBuffers,
    buffers: EditorBuffers,
    buffer_xywh: TextBoxXYWH,
    current_buffer_kind: BufferIdKind,
    menu_mode: MenuMode,
    file_switcher: ScrollableBuffer,
    file_switcher_results: FileSwitcherResults,
    find: ScrollableBuffer,
    find_xywh: TextBoxXYWH,
    replace: ScrollableBuffer,
    replace_xywh: TextBoxXYWH,
    go_to_position: ScrollableBuffer,
    go_to_position_xywh: TextBoxXYWH,
    font_info: FontInfo,
    clipboard_history: ClipboardHistory,
    parsers: parsers::Parsers,
}

// this macro helps the borrow checker figure out that borrows are valid.
macro_rules! get_scrollable_buffer_mut {
    /* -> Option<&mut ScrollableBuffer> */
    ($state: expr) => {{
        get_scrollable_buffer_mut!($state, $state.current_buffer_kind)
    }};
    ($state: expr, $kind: expr) => {{
        u!{BufferIdKind}
        match $kind {
            None => Option::None,
            Text => Some(&mut $state.buffers.get_current_buffer_mut().scrollable),
            Find => Some(&mut $state.find),
            Replace => Some(&mut $state.replace),
            FileSwitcher => Some(&mut $state.file_switcher),
            GoToPosition => Some(&mut $state.go_to_position),
        }
    }}
}

macro_rules! set_indexed_id {
    ($name: ident, $variant: ident) => {
        impl State {
            #[allow(dead_code)]
            fn $name(&mut self, i: g_i::Index) {
                self.set_id(b_id!(BufferIdKind::$variant, i));
            }
        }
    };
}

set_indexed_id! {
    set_none_id,
    None
}
set_indexed_id! {
    set_text_id,
    Text
}
set_indexed_id! {
    set_find_id,
    Find
}
set_indexed_id! {
    set_replace_id,
    Replace
}
set_indexed_id! {
    set_file_switcher_id,
    FileSwitcher
}
set_indexed_id! {
    set_go_to_position_id,
    GoToPosition
}

impl State {
    pub fn new() -> State {
        d!()
    }

    fn adjust_buffer_selection(&mut self, adjustment: SelectionAdjustment) {
        self.buffers.adjust_selection(adjustment);
    }

    fn close_buffer(&mut self, index: g_i::Index) {
        self.buffers.close_buffer(index);
    }

    fn get_id(&self) -> BufferId {
        b_id!(self.current_buffer_kind, self.buffers.current_index())
    }

    fn set_id(&mut self, id: BufferId) {
        self.current_buffer_kind = id.kind;

        if let Some(buffer) = get_scrollable_buffer_mut!(self) {
            // These need to be cleared so that the `platform_types::View` that is passed down
            // can be examined to detemine if the user wants to navigate away from the given
            // buffer. We do this with each buffer, even though a client might only care about
            // buffers of a given menu kind, since a different client might care about different
            // ones, including plain `Text` buffers.
            buffer.reset_cursor_states();
        }

        self.buffers.set_current_index(id.index);
    }

    fn add_or_select_buffer(&mut self, name: BufferName, str: String) {
        self.buffers.add_or_select_buffer(name, str);

        self.current_buffer_kind = BufferIdKind::Text;
    }

    fn next_scratch_buffer_number(&self) -> u32 {
        let mut output = 0;
        for b in self.buffers.iter() {
            match b.name {
                BufferName::Path(_) => continue,
                BufferName::Scratch(n) => {
                    output = std::cmp::max(output, n);
                }
            }
        }
        output.wrapping_add(1)
    }

    fn get_current_char_dim(&self) -> CharDim {
        Self::char_dim_for_buffer_kind(&self.font_info, self.current_buffer_kind)
    }

    fn char_dim_for_buffer_kind(font_info: &FontInfo, kind: BufferIdKind) -> CharDim {
        u!{BufferIdKind}
        match kind {
            Text => font_info.text_char_dim,
            // None uses the same char_dim as the menus since it represents keyboard navigation in
            // the menus.
            None | Find | Replace | FileSwitcher | GoToPosition => font_info.find_replace_char_dim,
        }
    }

    fn try_to_show_cursors_on(&mut self, kind: BufferIdKind) -> Option<()> {
        u!{BufferIdKind}

        let buffer = get_scrollable_buffer_mut!(self, kind)?;
        let xywh = match kind {
            None => return Option::None,
            Text => self.buffer_xywh,
            Find => self.find_xywh,
            Replace => self.replace_xywh,
            FileSwitcher => self.find_xywh, // TODO customize
            GoToPosition => self.go_to_position_xywh,
        };

        let char_dim = Self::char_dim_for_buffer_kind(&self.font_info, kind);

        let attempt_result = buffer.try_to_show_cursors_on(xywh, char_dim);

        match attempt_result {
            VisibilityAttemptResult::Succeeded => Some(()),
            _ => Option::None,
        }
    }

    fn set_menu_mode(&mut self, mode: MenuMode) {
        let current_index = self.buffers.current_index();

        self.menu_mode = mode;
        match mode {
            MenuMode::GoToPosition => {
                self.set_go_to_position_id(current_index);
            }
            MenuMode::FileSwitcher => {
                self.set_file_switcher_id(current_index);
            }
            MenuMode::FindReplace(_) => {
                self.set_find_id(current_index);
            }
            MenuMode::Hidden => {
                self.set_text_id(current_index);
            }
        }
    }

    fn find_replace_mode(&mut self) -> Option<FindReplaceMode> {
        match self.menu_mode {
            MenuMode::FindReplace(mode) => {
                Some(mode)
            }
            MenuMode::GoToPosition | MenuMode::FileSwitcher | MenuMode::Hidden => {
                None
            }
        }
    }

    fn opened_paths(&self) -> Vec<&PathBuf> {
        let mut opened_paths: Vec<&PathBuf> = Vec::with_capacity(self.buffers.len().into());

        for buffer in self.buffers.iter() {
            match &buffer.name {
                BufferName::Path(p) => {
                    opened_paths.push(p);
                }
                BufferName::Scratch(_) => {}
            };
        }
        opened_paths
    }
}

pub fn new() -> State {
    d!()
}

impl From<String> for State {
    fn from(s: String) -> Self {
        let mut output: Self = d!();

        output.buffers = EditorBuffers::new(EditorBuffer::new(d!(), s));

        output
    }
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let mut output: Self = d!();

        output.buffers = EditorBuffers::new(EditorBuffer::new(d!(), s));

        output
    }
}

fn parse_for_go_to_position(input: &str) -> Result<Position, std::num::ParseIntError> {
   input.parse::<Position>().map(|p| {
        // The largest use of jumping to a position is to jump to line numbers emitted 
        // by an error message. Most line numbers like this start at one, so we need 
        // to counteract that.
        pos!{
            l p.line.saturating_sub(1),
            o p.offset.saturating_sub(1).0
        }
   })
}

macro_rules! set_if_present {
    ($source:ident => $target:ident.$field:ident) => {
        if let Some($field) = $source.$field {
            $target.$field = $field;
        }
    };
}

//#[check_or_no_panic::check_or_no_panic]
pub fn update_and_render(state: &mut State, input: Input) -> UpdateAndRenderOutput {
    perf_viz::record_guard!("update_and_render");

    macro_rules! try_to_show_cursors {
        () => {
            try_to_show_cursors!(state.current_buffer_kind);
        };
        ($kind: expr) => {
            state.try_to_show_cursors_on($kind);
            // TODO trigger error popup based on result?
        };
    }

    macro_rules! buffer_view_sync {
        () => {
            match state.menu_mode {
                MenuMode::Hidden | MenuMode::FindReplace(_) => {            
                    update_search_results(
                        &state.find.text_buffer,
                        state.buffers.get_current_buffer_mut()
                    );
                }
                MenuMode::FileSwitcher => {
                    let needle_string: String =
                        state.file_switcher.text_buffer.borrow_rope().into();
                    let needle_str: &str = &needle_string;
                    state.file_switcher_results =
                        paths::find_in(state.opened_paths().iter().map(|p| p.as_path()), needle_str);
                }
                MenuMode::GoToPosition => {}
            }
            try_to_show_cursors!();
        };
    }

    let mut edited_buffer_index: Option<g_i::Index> = Option::None;
    macro_rules! post_edit_sync {
        () => {
            buffer_view_sync!();
            edited_buffer_index = Some(state.buffers.current_index());
        };
    }

    macro_rules! editor_buffer_call {
        (sync $buffer: ident . $($method_call:tt)*) => {
            editor_buffer_call!($buffer . $($method_call)*)
        };
        ($buffer: ident . $($method_call:tt)*) => {
            editor_buffer_call!($buffer {$buffer.$($method_call)*})
        };
        (sync $buffer: ident $tokens:block) => {{
            editor_buffer_call!($buffer $tokens)
        }};
        ($buffer: ident $tokens:block) => {{
            let $buffer = state.buffers.get_current_buffer_mut();
            $tokens;
        }}
    }

    macro_rules! buffer_call {
        (sync $buffer: ident . $($method_call:tt)*) => {
            let output = buffer_call!($buffer . $($method_call)*);
            post_edit_sync!();
            output
        };
        ($buffer: ident . $($method_call:tt)*) => {
            buffer_call!($buffer {$buffer.$($method_call)*})
        };
        (sync $buffer: ident $tokens:block) => {{
            buffer_call!($buffer $tokens);
            post_edit_sync!();
        }};
        ($buffer: ident $tokens:block) => {{
            if let Some($buffer) = get_scrollable_buffer_mut!(state) {
                $tokens;
            }
        }}
    }
    macro_rules! text_buffer_call {
        (sync $buffer: ident . $($method_call:tt)*) => {
            text_buffer_call!($buffer . $($method_call)*)
        };
        ($buffer: ident . $($method_call:tt)*) => {
            text_buffer_call!($buffer {$buffer.$($method_call)*})
        };
        (sync $buffer: ident $tokens:block) => {{
            text_buffer_call!($buffer $tokens)
        }};
        ($buffer: ident $tokens:block) => {{
            if let Some($buffer) = get_scrollable_buffer_mut!(state).map(|b| &mut b.text_buffer) {
                $tokens;
            }
        }}
    }

    if cfg!(feature = "extra-prints")
    {
        if_changed::dbg!(&input);
    }

    let mut cmd = Cmd::NoCmd;

    macro_rules! close_menu_if_any {
        () => {
            state.set_menu_mode(MenuMode::Hidden);
        };
    }

    u!{Input}
    match input {
        Input::None => {}
        Quit => {}
        CloseMenuIfAny => {
            close_menu_if_any!();
        }
        Insert(c) => buffer_call!(sync b{
            b.text_buffer.insert(c);
        }),
        Delete => buffer_call!(sync b {
            b.text_buffer.delete();
        }),
        DeleteLines => buffer_call!(sync b {
            b.text_buffer.delete_lines();
        }),
        Redo => buffer_call!(sync b {
            b.text_buffer.redo();
        }),
        Undo => buffer_call!(sync b {
            b.text_buffer.undo();
        }),
        MoveAllCursors(r#move) => {
            buffer_call!(b{
                b.text_buffer.move_all_cursors(r#move);
                try_to_show_cursors!();
            });
        }
        ExtendSelectionForAllCursors(r#move) => {
            buffer_call!(b{
                b.text_buffer.extend_selection_for_all_cursors(r#move);
                try_to_show_cursors!();
            });
        }
        SelectAll => {
            text_buffer_call!(b.select_all());
            // We don't need to make sure a cursor is visible here since the user
            // will understand where the cursor is.
        }
        ScrollVertically(amount) => buffer_call!(b{
            b.scroll.y -= amount;
        }),
        ScrollHorizontally(amount) => buffer_call!(b{
            b.scroll.x += amount;
        }),
        ResetScroll => buffer_call!(b{
            b.scroll = d!();
        }),
        SetSizeDependents(sds) => {
            set_if_present!(sds => state.font_info);
            set_if_present!(sds => state.buffer_xywh);
            set_if_present!(sds => state.find_xywh);
            set_if_present!(sds => state.replace_xywh);
            set_if_present!(sds => state.go_to_position_xywh);
        }
        SetCursor(xy, replace_or_add) => {
            let char_dim = state.get_current_char_dim();
            buffer_call!(b{
                let position = text_space_to_position(
                    text_box_to_text(xy, b.scroll),
                    char_dim,
                    PositionRound::Up,
                );

                b.text_buffer.set_cursor(position, replace_or_add);
            });
        }
        DragCursors(xy) => {
            let char_dim = state.get_current_char_dim();
            buffer_call!(b{
                let position = text_space_to_position(
                    text_box_to_text(xy, b.scroll),
                    char_dim,
                    PositionRound::Up,
                );
                // In practice we currently expect this to be sent only immeadately after an
                // `Input::SetCursors` input, so there will be only one cursor. But it seems like
                // we might as well just do it to all the cursors
                b.text_buffer.drag_cursors(position);
            })
        }
        SelectCharTypeGrouping(xy, replace_or_add) => {
            let char_dim = state.get_current_char_dim();
            buffer_call!(b{
                // We want different rounding for selections so that if we trigger a selection on the
                // right side of a character, we select that character rather than the next character.
                let position = text_space_to_position(
                    text_box_to_text(xy, b.scroll),
                    char_dim,
                    PositionRound::TowardsZero,
                );

                b.text_buffer.select_char_type_grouping(position, replace_or_add)
            })
        }
        ExtendSelectionWithSearch => {
            buffer_call!(b{
                let cursor = b.text_buffer.borrow_cursors().first().clone();
                match cursor.get_highlight_position() {
                    Option::None => {
                        b.text_buffer.select_char_type_grouping(cursor.get_position(), ReplaceOrAdd::Add);
                    }
                    Some(_) => {
                        if let Some(pair) = next_instance_of_selected(b.text_buffer.borrow_rope(), &cursor) {
                            b.text_buffer.set_cursor(pair, ReplaceOrAdd::Add);
                        }
                    }
                }
            });
        }
        SetBufferPath(buffer_index, path) => {
            state.buffers.set_path(buffer_index, path);
        }
        Cut => buffer_call!(sync b {
            if let Some(s) = state.clipboard_history.cut(&mut b.text_buffer) {
                cmd = Cmd::SetClipboard(s);
            }
        }),
        Copy => buffer_call!(b {
            if let Some(s) = state.clipboard_history.copy(&mut b.text_buffer) {
                cmd = Cmd::SetClipboard(s);
            }
            try_to_show_cursors!();
        }),
        Paste(op_s) => buffer_call!(sync b {
            state.clipboard_history.paste(&mut b.text_buffer, op_s);
        }),
        InsertNumbersAtCursors => buffer_call!(sync b {
            b.text_buffer.insert_at_each_cursor(|i| i.to_string());
        }),
        NewScratchBuffer(data_op) => {
            state.buffers.push_and_select_new(EditorBuffer::new(
                BufferName::Scratch(state.next_scratch_buffer_number()),
                data_op.unwrap_or_default(),
            ));
            state.current_buffer_kind = BufferIdKind::Text;
        }
        TabIn => {
            text_buffer_call!(sync b.tab_in());
        }
        TabOut => {
            text_buffer_call!(sync b.tab_out());
        }
        AddOrSelectBuffer(name, str) => {
            state.add_or_select_buffer(name, str);
            buffer_view_sync!();
        }
        AdjustBufferSelection(adjustment) => {
            state.adjust_buffer_selection(adjustment);
        }
        SelectBuffer(id) => {
            state.set_id(id);
            match id.kind {
                BufferIdKind::Text => {
                    close_menu_if_any!();
                }
                _ => {}
            }
        }
        OpenOrSelectBuffer(path) => {
            if let Some(id) = state
                .buffers
                .iter_with_indexes()
                .find(|(_, b)| match &b.name {
                    BufferName::Path(p) => *p == path,
                    BufferName::Scratch(_) => false,
                })
                .map(|(i, _)| b_id!(BufferIdKind::Text, i))
            {
                state.set_id(id);
                close_menu_if_any!();
            } else {
                cmd = Cmd::LoadFile(path);
            }
        }
        CloseBuffer(index) => {
            state.close_buffer(index);
        }
        SetMenuMode(mode) => {
            if mode == MenuMode::Hidden {
                state.set_menu_mode(mode);
            } else {
                let mut selections = d!();

                text_buffer_call!(b {
                    selections = b.copy_selections();
                });

                let mut selection: Option<String> = if selections.len() == 1 {
                    Some(selections.swap_remove(0))
                } else {
                    Option::None
                };

                state.set_menu_mode(mode);

                selection = match (selection, mode) {
                    (Option::None, _) => {Option::None}
                    (Some(selection), MenuMode::GoToPosition) => {
                        if dbg!(parse_for_go_to_position(&selection)).is_err() {
                            Option::None
                        } else {
                            Some(selection)
                        }
                    }
                    (s, _) => {s}
                };

                
                text_buffer_call!(sync b{
                    if let Some(selection) = selection {
                        // For the small menu buffers I'd rather keep all the history
                        // even if the history order is confusing, since the text 
                        // entered is usually small ... kind of like a shell prompt.
                        const UPPER_LOOP_BOUND: usize = 1024;
                        // Use a bound like this just in case, since we do actually 
                        // proiritize a sufficently quick response over a perfect history
                        for _ in 0..UPPER_LOOP_BOUND {
                            if b.redo().is_none() {
                                break;
                            }
                        }
    
                        b.select_all();
                        b.insert_string(selection);
                    }

                    b.select_all();
                    // We don't need to make sure a cursor is visible here since the user
                    // will understand where the cursor is.
                });
            }
        }
        NextLanguage => {
            editor_buffer_call!(b {
                b.parser_kind = Some(
                    dbg!(b.get_parser_kind().next().unwrap_or_default())
                );
            });
        }
        SubmitForm => match state.current_buffer_kind {
            BufferIdKind::None | BufferIdKind::Text => {}
            BufferIdKind::Find => match state.find_replace_mode() {
                Option::None => {
                    debug_assert!(false, "state.find_replace_mode() returned None");
                }
                Some(FindReplaceMode::CurrentFile) => {
                    let haystack = state.buffers.get_current_buffer_mut();
                    let needle = &state.find.text_buffer;
                    let needle_string: String = needle.into();
                    if needle_string == haystack.search_results.needle {
                        // advance to next search result
                        if needle_string.len() > 0 {
                            let search_results = &mut haystack.search_results;
                            let len = search_results.ranges.len();
                            search_results.current_range += 1;
                            if search_results.current_range >= len {
                                search_results.current_range = 0;
                            }

                            if let Some(pair) = haystack
                                .search_results
                                .ranges
                                .get(haystack.search_results.current_range)
                            {
                                let c: Cursor = pair.into();
                                haystack
                                    .scrollable
                                    .text_buffer
                                    .set_cursor(c, ReplaceOrAdd::Replace);
                                try_to_show_cursors!(BufferIdKind::Text);
                            }
                        }
                    } else {
                        update_search_results(needle, haystack);
                    }
                    try_to_show_cursors!();
                    
                }
            },
            BufferIdKind::Replace => {
                let i = state.buffers.current_index();
                dbg!("TODO BufferIdKind::Replace {}", i);
            }
            BufferIdKind::GoToPosition => {
                text_buffer_call!(b{
                    let input: String = b.into();
                    if let Ok(position) = parse_for_go_to_position(&input) {
                        state.buffers.get_current_buffer_mut().scrollable.text_buffer.set_cursor(position, ReplaceOrAdd::Replace);
                        state.set_menu_mode(MenuMode::Hidden);
                        try_to_show_cursors!();
                    }
                    // TODO: show Err case
                });
            }
            BufferIdKind::FileSwitcher => {
                post_edit_sync!();
            }
        },
    }
    let mut view = d!();

    editor_view::render(state, &mut view);
    (view, cmd)
}

#[cfg(test)]
mod tests;