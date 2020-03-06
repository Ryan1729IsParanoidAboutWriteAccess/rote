// This was originally based on example code for https://github.com/alexheretic/glyph-brush
// the code is licensed under the Apache 2.0 license, as described in the license file in the
// opengl folder, to the extent that the code remains as it was
// (at commit 90e7c7c331e9f991e11de6404b2ca073c0a09e61)

use glutin::{dpi::LogicalPosition, Api, GlProfile, GlRequest};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;
use wimp_render::{get_find_replace_info, FindReplaceInfo, get_go_to_position_info, GoToPositionInfo};
use wimp_types::{ui, ui::{PhysicalButtonState, Navigation}, transform_status, BufferStatus, BufferStatusMap, BufferStatusTransition, RunState};
use file_chooser;
use macros::d;
use platform_types::{screen_positioning::screen_to_text_box, *};
use shared::{Res};


mod clipboard_layer {
    pub use clipboard::ClipboardProvider;
    use shared::Res;
    /// This enum exists so we can do dynamic dispatch on `ClipboardProvider` instances even though
    /// the trait requires `Sized`. The reason  we want to do that, is so that if we try to run this
    /// on a platform where `clipboard::ClipboardContext::new` retirns an `Err` we can continue
    /// operation, just without system clipboard support.
    pub enum Clipboard {
        System(clipboard::ClipboardContext),
        Fallback(clipboard::nop_clipboard::NopClipboardContext),
    }

    impl clipboard::ClipboardProvider for Clipboard {
        fn new() -> Res<Self> {
            let result: Result<
                clipboard::ClipboardContext,
                clipboard::nop_clipboard::NopClipboardContext,
            > = clipboard::ClipboardContext::new().map_err(|err| {
                eprintln!("System clipboard not supported. {}", err);
                // `NopClipboardContext::new` always returns an `Ok`
                clipboard::nop_clipboard::NopClipboardContext::new().unwrap()
            });

            let output = match result {
                Ok(ctx) => Clipboard::System(ctx),
                Err(ctx) => Clipboard::Fallback(ctx),
            };

            // `get_clipboard` currently relies on this neer returning `Err`.
            Ok(output)
        }
        fn get_contents(&mut self) -> Res<String> {
            match self {
                Clipboard::System(ctx) => ctx.get_contents(),
                Clipboard::Fallback(ctx) => ctx.get_contents(),
            }
        }
        fn set_contents(&mut self, s: String) -> Res<()> {
            match self {
                Clipboard::System(ctx) => ctx.set_contents(s),
                Clipboard::Fallback(ctx) => ctx.set_contents(s),
            }
        }
    }

    pub fn get_clipboard() -> Clipboard {
        // As you can see in the implementation of the `new` method, it always returns `Ok`
        Clipboard::new().unwrap()
    }
}
use clipboard_layer::{get_clipboard, Clipboard, ClipboardProvider};

#[perf_viz::record]
pub fn run(update_and_render: UpdateAndRender) -> Res<()> {
    const EVENTS_PER_FRAME: usize = 16;

    if cfg!(target_os = "linux") {
        use std::env;
        // winit wayland is currently still wip
        if env::var("WINIT_UNIX_BACKEND").is_err() {
            env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
        // disables vsync sometimes on x11
        if env::var("vblank_mode").is_err() {
            env::set_var("vblank_mode", "0");
        }
    }

    let title = "rote";

    let mut args = std::env::args();
    //exe name
    args.next();

    let mut data_dir = None;
    let mut hidpi_factor_override = None;

    const VERSION: &'static str = "--version";
    const HELP: &'static str = "--help";
    const DATA_DIR_OVERRIDE: &'static str = "--data-dir-override";
    const HIDPI_OVERRIDE: &'static str = "--hidpi-override";

    while let Some(s) = args.next() {
        let s: &str = &s;
        match s {
            HELP => {
                let accepted_args = [VERSION, HELP, DATA_DIR_OVERRIDE, HIDPI_OVERRIDE];
                println!("accepted args: ");
                for arg in accepted_args.iter() {
                    print!("    {}", arg);
                    if *arg == DATA_DIR_OVERRIDE {
                        print!(" <data directory path>");
                    }
                    if *arg == HIDPI_OVERRIDE {
                        print!(" <hidpi factor (positive floating point number)>");
                    }
                    println!()
                }
                std::process::exit(0)
            }
            VERSION => {
                println!("{} version {}", title, env!("CARGO_PKG_VERSION"));
                std::process::exit(0)
            }
            DATA_DIR_OVERRIDE => {
                data_dir = Some(args.next().ok_or_else(|| {
                    format!(
                        "{0} needs an argument. For example: {0} ./data",
                        DATA_DIR_OVERRIDE
                    )
                })?)
                .map(PathBuf::from);
            }
            HIDPI_OVERRIDE => {
                hidpi_factor_override = Some(args.next().ok_or_else(|| {
                    format!(
                        "{0} needs an argument. For example: {0} 1.5",
                        HIDPI_OVERRIDE
                    )
                })?)
                .and_then(|s| {
                    use std::str::FromStr;
                    f64::from_str(&s).ok()
                });
            }
            _ => {
                eprintln!("unknown arg {:?}", s);
                std::process::exit(1)
            }
        }
    }

    let data_dir = data_dir
        .or_else(|| {
            directories::ProjectDirs::from("com", "ryanwiedemann", title)
                .map(|proj_dirs| proj_dirs.data_dir().to_owned())
        })
        .ok_or("Could not find app data dir")?;

    match std::fs::metadata(&data_dir) {
        Ok(meta) => {
            if meta.is_dir() {
                Ok(())
            } else {
                Err("data_dir existed but was not a directory!".to_owned())
            }
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                std::fs::create_dir_all(&data_dir)
                    .map_err(|e| e.to_string())
                    .and_then(|_| {
                        std::fs::metadata(&data_dir)
                            .map_err(|e| e.to_string())
                            .and_then(|meta| {
                                if meta.is_dir() {
                                    Ok(())
                                } else {
                                    Err("data_dir was created but was not a directory!".to_owned())
                                }
                            })
                    })
            } else {
                Err(err.to_string())
            }
        }
    }?;

    let edited_files_dir_buf = data_dir.join("edited_files_v1/");
    let edited_files_index_path_buf = data_dir.join("edited_files_v1_index.txt");

    let mut clipboard: Clipboard = get_clipboard();

    #[derive(Clone, Debug)]
    enum CustomEvent {
        OpenFile(PathBuf),
        SaveNewFile(PathBuf, g_i::Index),
        SendBuffersToBeSaved,
        EditedBufferError(String),
    }
    unsafe impl Send for CustomEvent {}
    unsafe impl Sync for CustomEvent {}

    use glutin::event_loop::EventLoop;
    let events: EventLoop<CustomEvent> = glutin::event_loop::EventLoop::with_user_event();
    let event_proxy = events.create_proxy();

    let glutin_context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        //As of now we only need 3.3 for GL_TIME_ELAPSED. Otherwise we could use 3.2.
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_srgb(true)
        .with_depth_buffer(24)
        .build_windowed(
            glutin::window::WindowBuilder::new()
                .with_inner_size(
                    glutin::dpi::Size::Logical(glutin::dpi::LogicalSize::new(1024.0, 576.0))
                 )
                .with_title(title),
            &events,
        )?;
    let glutin_context = unsafe { glutin_context.make_current().map_err(|(_, e)| e)? };

    let mut current_hidpi_factor = 1.0;
    macro_rules! get_hidpi_factor {
        () => {
            hidpi_factor_override.unwrap_or(current_hidpi_factor)
        }
    }

    dbg!(glutin_context.get_pixel_format());

    let (mut gl_state, char_dims) = gl_layer::init(
        get_hidpi_factor!() as f32,
        &wimp_render::TEXT_SIZES,
        wimp_render::TEXT_BACKGROUND_COLOUR,
        |symbol| glutin_context.get_proc_address(symbol) as _,
    )?;

    let font_info = wimp_render::get_font_info(&char_dims);

    const TARGET_RATE: f64 = 128.0; //250.0);

    let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(TARGET_RATE);

    let mut running = true;
    let mut dimensions = glutin_context
        .window()
        .inner_size();

    macro_rules! screen_wh {
        () => {
            ScreenSpaceWH {
                w: dimensions.width as f32,
                h: dimensions.height as f32,
            }
        };
    }

    macro_rules! get_non_font_size_dependents {
        ($mode: expr) => {{
            let wh = screen_wh!();
            let FindReplaceInfo {
                find_text_xywh,
                replace_text_xywh,
                ..
            } = get_find_replace_info(font_info, wh);
            let GoToPositionInfo {
                input_text_xywh,
                ..
            } = get_go_to_position_info(font_info, wh);
            SizeDependents {
                buffer_xywh: wimp_render::get_edit_buffer_xywh(
                    $mode,
                    font_info,
                    screen_wh!()
                )
                .into(),
                find_xywh: find_text_xywh.into(),
                replace_xywh: replace_text_xywh.into(),
                go_to_position_xywh: input_text_xywh.into(),
                font_info: font_info.into(),
            }
        }};
    }

    use std::sync::mpsc::{Sender, channel};

    // into the edited files thread
    let (edited_files_in_sink, edited_files_in_source) = channel();
    // out of the edited files thread
    let (edited_files_out_sink, edited_files_out_source) = channel();

    enum EditedFilesThread {
        Quit,
        Buffers(g_i::State, Vec<edited_storage::BufferInfo>),
    }

    let mut edited_files_join_handle = Some({
        let edited_files_dir = edited_files_dir_buf.clone();
        let edited_files_index_path = edited_files_index_path_buf.clone();
        let proxy = event_proxy.clone();

        std::thread::Builder::new()
            .name("edited_files".to_string())
            .spawn(move || {
                loop {
                    macro_rules! handle_message {
                        ($message: expr) => {{
                            use EditedFilesThread::*;
                            match $message {
                                Quit => return,
                                Buffers(index_state, buffers) => {
                                    match edited_storage::store_buffers(
                                        &edited_files_dir,
                                        &edited_files_index_path,
                                        buffers,
                                        index_state,
                                    ) {
                                        Ok(transitions) => {
                                            for transition in transitions {
                                                let _hope_it_gets_there =
                                                    edited_files_out_sink.send(transition);
                                            }
                                        }
                                        Err(e) => {
                                            use std::error::Error;
                                            let _hope_it_gets_there =
                                                proxy.send_event(CustomEvent::EditedBufferError(
                                                    e.description().to_owned(),
                                                ));
                                        }
                                    }
                                }
                            }
                        }};
                    }

                    // 20 * 50 = 1_000, 60_000 ms = 1 minute
                    // so this waits roughly a minute plus waiting time for messages
                    const QUIT_CHECK_COUNT: u32 = 20; // * 60;
                    for _ in 0..QUIT_CHECK_COUNT {
                        std::thread::sleep(Duration::from_millis(50));
                        if let Ok(message) = edited_files_in_source.try_recv() {
                            handle_message!(message);
                        }
                    }

                    let _hope_it_gets_there = proxy.send_event(CustomEvent::SendBuffersToBeSaved);

                    if let Ok(message) = edited_files_in_source.recv() {
                        handle_message!(message);
                    }
                }
            })
            .expect("Could not start edited_files thread!")
    });

    // into the editor thread
    let (editor_in_sink, editor_in_source) = channel();
    // out of the editor thread
    let (editor_out_sink, editor_out_source) = channel();

    let mut editor_join_handle = Some(
        std::thread::Builder::new()
            .name("editor".to_string())
            .spawn(move || {
                while let Ok(input) = editor_in_source.recv() {
                    let was_quit = Input::Quit == input;
                    let pair = update_and_render(input);
                    let _hope_it_gets_there = editor_out_sink.send(pair);
                    if was_quit {
                        return;
                    }
                }
            })
            .expect("Could not start editor thread!"),
    );

    let previous_tabs =
            edited_storage::load_previous_tabs(&edited_files_dir_buf, &edited_files_index_path_buf);

    let mut r_s = {
        let (view, c) = update_and_render(Input::SetSizeDependents(
            get_non_font_size_dependents!(d!())
        ));

        let mut cmds = VecDeque::with_capacity(EVENTS_PER_FRAME);
        cmds.push_back(c);

        let mut ui: ui::State = d!();
        ui.window_is_focused = true;

        let buffer_status_map = BufferStatusMap::with_capacity((previous_tabs.len() + 1) * 2);

        RunState {
            view,
            cmds,
            ui,
            buffer_status_map,
            editor_in_sink,
        }
    };

    // If you didn't click on the same symbol, counting that as a double click seems like it
    // would be annoying.
    let mouse_epsilon_radius: f32 = {
        let (w, h) = (font_info.text_char_dim.w, font_info.text_char_dim.h);

        (if w < h { w } else { h }) / 2.0
    };

    let (mut last_click_x, mut last_click_y) = (std::f32::NAN, std::f32::NAN);

    let mut dt = Duration::from_nanos(((1.0 / TARGET_RATE) * 1_000_000_000.0) as u64);

    macro_rules! mouse_within_radius {
        () => {{
            let mouse_pos = &r_s.ui.mouse_pos;
            (last_click_x - mouse_pos.x).abs() <= mouse_epsilon_radius
                && (last_click_y - mouse_pos.y).abs() <= mouse_epsilon_radius
        }};
    }

    {
        macro_rules! call_u_and_r {
            ($input:expr) => {{
                call_u_and_r!(r_s, $input)
            }};
            ($vars: ident, $input:expr) => {{
                $vars.ui.note_interaction();
                let _hope_it_gets_there = $vars.editor_in_sink.send($input);
            }};
        }

        for (i, (name, data)) in previous_tabs.into_iter().enumerate() {
            call_u_and_r!(Input::AddOrSelectBuffer(name, data));

            let index_state = r_s.view.index_state;

            // if we bothered saving them before, they were clearly edited.
            r_s.buffer_status_map.insert(
                index_state,
                index_state.new_index(g_i::IndexPart::or_max(i)),
                BufferStatus::EditedAndSaved,
            );
        }

        use std::collections::BTreeMap;
        let mut commands: BTreeMap<_, (_, fn(&mut RunState))> = std::collections::BTreeMap::new();

        macro_rules! register_command {
            ($modifiers: expr, $main_key: ident, $label: literal, $(_)? $code: block) => {
                register_command!($modifiers, $main_key, $label, _unused_identifier $code)
            };
            ($modifiers: expr, $main_key: ident, $label: literal, $vars: ident $code: block) => {{
                fn cmd_fn($vars: &mut RunState) {
                    $code
                }
                let key = ($modifiers, VirtualKeyCode::$main_key);  
                debug_assert!(commands.get(&key).is_none());
                commands.insert(key, ($label, cmd_fn));
            }}
        }

        macro_rules! register_commands {
            ($([$($tokens: tt)*])+) => {
                $(
                    register_command!{
                        $($tokens)*
                    }
                )+
            }
        }

        use glutin::event::*;

        const LOGO: ModifiersState = ModifiersState::LOGO;
        const ALT: ModifiersState = ModifiersState::ALT;
        const CTRL: ModifiersState = ModifiersState::CTRL;
        const SHIFT: ModifiersState = ModifiersState::SHIFT;

        register_commands!{
            [CTRL, Home, "Move cursors to start.", state {
                call_u_and_r!(state, Input::MoveAllCursors(Move::ToBufferStart))
            }]
            [CTRL, End, "Move cursors to end.", state {
                call_u_and_r!(state, Input::MoveAllCursors(Move::ToBufferEnd))
            }]
        }

        events.run(move |event, _, control_flow| {
            // eventually we'll likely want to tell the editor, and have it decide whether/how
            // to display it to the user.
            macro_rules! handle_platform_error {
                ($err: expr) => {
                    let error = format!("{},{}: {}", file!(), line!(), $err);
                    eprintln!("{}", error);
                    call_u_and_r!(Input::NewScratchBuffer(Some(error)));
                };
            }

            macro_rules! save_to_disk {
                ($path: expr, $str: expr, $buffer_index: expr) => {
                    let index = $buffer_index;
                    match std::fs::write($path, $str) {
                        Ok(_) => {
                            let view = &r_s.view;
                            let buffer_status_map = &mut r_s.buffer_status_map;
                            buffer_status_map.insert(
                                view.index_state,
                                index,
                                transform_status(
                                    buffer_status_map
                                        .get(view.index_state, index)
                                        .unwrap_or_default(),
                                    BufferStatusTransition::Save
                                )
                            );
                            call_u_and_r!(Input::SetBufferPath(index, $path.to_path_buf()));
                        }
                        Err(err) => {
                            handle_platform_error!(err);
                        }
                    }
                };
            }

            macro_rules! load_file {
                ($path: expr) => {{
                    let p = $path;
                    match std::fs::read_to_string(&p) {
                        Ok(s) => {
                            call_u_and_r!(Input::AddOrSelectBuffer(BufferName::Path(p), s));
                        }
                        Err(err) => {
                            handle_platform_error!(err);
                        }
                    }
                }};
            }

            match event {
                Event::WindowEvent { event, .. } => {
                    macro_rules! quit {
                        () => {{
                            perf_viz::end_record!("main loop");
                            call_u_and_r!(Input::Quit);
                            let _hope_it_gets_there = edited_files_in_sink.send(EditedFilesThread::Quit);
                            running = false;

                            // If we got here, we assume that we've sent a Quit input to the editor thread so it will stop.
                            match editor_join_handle.take() {
                                Some(j_h) => j_h.join().expect("Could not join editor thread!"),
                                None => {}
                            };

                            match edited_files_join_handle.take() {
                                Some(j_h) => j_h.join().expect("Could not join edited_files thread!"),
                                None => {}
                            };

                            perf_viz::output!();

                            let _ = gl_layer::cleanup(&gl_state);

                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                        }};
                    }

                    macro_rules! file_chooser_call {
                        ($func: ident, $path: ident in $event: expr) => {
                            let proxy =
                                std::sync::Arc::new(std::sync::Mutex::new(event_proxy.clone()));
                            let proxy = proxy.clone();
                            file_chooser::$func(move |$path: PathBuf| {
                                let _bye = proxy
                                    .lock()
                                    .expect("file_chooser thread private mutex locked!?")
                                    .send_event($event);
                            })
                        };
                    }

                    macro_rules! text_box_xy {
                        () => {{
                            let view = &r_s.view;
                            let xy = wimp_render::get_current_buffer_rect(
                                view.current_buffer_id,
                                view.menu.get_mode(),
                                font_info,
                                screen_wh!(),
                            )
                            .xy;

                            screen_to_text_box(r_s.ui.mouse_pos, xy)
                        }};
                    }

                    macro_rules! switch_menu_mode {
                        ($mode: expr) => {
                            let mode = $mode;

                            call_u_and_r!(Input::SetMenuMode(mode));

                            call_u_and_r!(Input::SetSizeDependents(SizeDependents {
                                buffer_xywh: wimp_render::get_edit_buffer_xywh(
                                    mode,
                                    font_info,
                                    screen_wh!()
                                )
                                .into(),
                                find_xywh: None,
                                replace_xywh: None,
                                go_to_position_xywh: None,
                                font_info: None,
                            }));
                        };
                    }

                    if cfg!(feature = "print-raw-input") {
                        match &event {
                            &WindowEvent::KeyboardInput { ref input, .. } => {
                                println!(
                                    "{:?}",
                                    (
                                        input.virtual_keycode.unwrap_or(VirtualKeyCode::WebStop),
                                        input.state
                                    )
                                );
                            }
                            _ => {}
                        }
                    }
                    
                    // The plan is to merge this case into the match below once all the commands have been registered.
                    #[allow(deprecated)]
                    match event {
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } => {
                            if let Some((label, command)) = commands.get(&(modifiers, keypress)) {
                                dbg!(label);
                                command(&mut r_s);
                            }
                        }
                        _ => {}
                    }

                    // As of this writing, issues on https://github.com/rust-windowing/winit ,
                    // specifically #1124 and #883, suggest that the it is up in the air as to
                    // whether the modifiers field on some of the matches below will actually
                    // be eventually removed or not. So, in the meantime, I choose the path 
                    // that is the least work right now, since it seems unlikely for the amount
                    // of work it will be later to grow significantly. Time will tell.
                    #[allow(deprecated)]
                    match event {
                        WindowEvent::CloseRequested => quit!(),
                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            ..
                        } => {
                            current_hidpi_factor = scale_factor;
                        }
                        WindowEvent::Resized(size) => {
                            let hidpi_factor = get_hidpi_factor!();
                            glutin_context.resize(size);
                            dimensions = size;
                            call_u_and_r!(Input::SetSizeDependents(
                                get_non_font_size_dependents!(
                                    r_s.view.menu.get_mode()
                                )
                            ));
                            gl_layer::set_dimensions(
                                &mut gl_state,
                                hidpi_factor as _,
                                (dimensions.width as _, dimensions.height as _),
                            );
                        }
                        WindowEvent::Focused(is_focused) => {
                            dbg!("set to ", is_focused);
                            r_s.ui.window_is_focused = is_focused;
                        }
                        WindowEvent::ReceivedCharacter(mut c) => {
                            if c != '\u{1}'     // "start of heading" (sent with Ctrl-a)
                             && c != '\u{3}'    // "end of text" (sent with Ctrl-c)
                             && c != '\u{4}'    // "end of transmission" (sent with Ctrl-d)
                             && c != '\u{6}'    // "acknowledge" (sent with Ctrl-f)
                             && c != '\u{7}'    // bell (sent with Ctrl-g)
                             && c != '\u{8}'    // backspace (sent with Ctrl-h)
                             && c != '\u{9}'    // horizontal tab (sent with Ctrl-i)
                             && c != '\u{f}'    // "shift in" AKA use black ink apparently, (sent with Ctrl-o)
                             && c != '\u{10}'   // "data link escape" AKA interprt the following as raw data, (sent with Ctrl-p)
                             && c != '\u{13}'   // "device control 3" (sent with Ctrl-s)
                             && c != '\u{14}'   // "device control 4" (sent with Ctrl-t)
                             && c != '\u{16}'   // "synchronous idle" (sent with Ctrl-v)
                             && c != '\u{17}'   // "end of transmission block" (sent with Ctrl-w)
                             && c != '\u{18}'   // "cancel" (sent with Ctrl-x)
                             && c != '\u{19}'   // "end of medium" (sent with Ctrl-y)
                             && c != '\u{1a}'   // "substitute" (sent with Ctrl-z)
                             && c != '\u{1b}'   // escape
                             && c != '\u{7f}'   // delete
                            {
                                if c == '\r' {
                                    c = '\n';
                                }

                                if c == '\n' {
                                    use BufferIdKind::*;
                                    match r_s.view.current_buffer_id.kind {
                                        None => {
                                            r_s.ui.navigation = Navigation::Interact;
                                        }
                                        Text => {
                                            call_u_and_r!(Input::Insert(c));
                                        }
                                        Find | Replace | FileSwitcher | GoToPosition => {
                                            call_u_and_r!(Input::SubmitForm);
                                        }
                                    }
                                } else {
                                    call_u_and_r!(Input::Insert(c));
                                }
                            }
                        }
                        WindowEvent::MouseWheel {
                            delta: MouseScrollDelta::LineDelta(_, y),
                            modifiers,
                            ..
                        } if modifiers.is_empty() => {
                            let ui = &mut r_s.ui;
                            let scroll_y = y * wimp_render::SCROLL_MULTIPLIER;
                            if wimp_render::inside_tab_area(ui.mouse_pos, font_info) {
                                ui.tab_scroll += scroll_y;
                            } else {
                                call_u_and_r!(Input::ScrollVertically(scroll_y));
                            }
                        }
                        WindowEvent::MouseWheel {
                            delta: MouseScrollDelta::LineDelta(_, y),
                            modifiers,
                            ..
                        } if modifiers == SHIFT => {
                            let ui = &mut r_s.ui;
                            let scroll_y = y * wimp_render::SCROLL_MULTIPLIER;
                            if wimp_render::inside_tab_area(ui.mouse_pos, font_info) {
                                ui.tab_scroll += scroll_y;
                            } else {
                                call_u_and_r!(Input::ScrollHorizontally(scroll_y));
                            }
                        }
                        WindowEvent::CursorMoved {
                            position,
                            modifiers,
                            ..
                        } => {
                            let ui = &mut r_s.ui;
                            let LogicalPosition::<f32> { x, y } = position.to_logical(get_hidpi_factor!());
                            ui.mouse_pos = ScreenSpaceXY {
                                x,
                                y,
                            };

                            match modifiers {
                                m if m.is_empty() => {
                                    let cursor_icon = if wimp_render::should_show_text_cursor(
                                        ui.mouse_pos,
                                        r_s.view.menu.get_mode(),
                                        font_info,
                                        screen_wh!(),
                                    ) {
                                        glutin::window::CursorIcon::Text
                                    } else {
                                        d!()
                                    };

                                    glutin_context.window().set_cursor_icon(cursor_icon);

                                    if ui.left_mouse_state.is_pressed() && !mouse_within_radius!() {
                                        call_u_and_r!(Input::DragCursors(text_box_xy!()));
                                    }
                                }
                                _ => {}
                            }
                        }
                        WindowEvent::MouseInput {
                            button: MouseButton::Left,
                            state: ElementState::Pressed,
                            modifiers,
                            ..
                        } // allow things like Shift-Alt-Click
                        if (!modifiers).intersects(!CTRL) => {
                            r_s.ui.left_mouse_state = PhysicalButtonState::PressedThisFrame;

                            let replace_or_add = if modifiers.ctrl() {
                                ReplaceOrAdd::Add
                            } else {
                                ReplaceOrAdd::Replace
                            };

                            let input = if mouse_within_radius!() {
                                Input::SelectCharTypeGrouping(text_box_xy!(), replace_or_add)
                            } else {
                                Input::SetCursor(text_box_xy!(), replace_or_add)
                            };

                            call_u_and_r!(input);
                        }
                        WindowEvent::MouseInput {
                            button: MouseButton::Left,
                            state: ElementState::Released,
                            ..
                        } => {
                            let ui = &mut r_s.ui;
                            ui.left_mouse_state = PhysicalButtonState::ReleasedThisFrame;
                            last_click_x = ui.mouse_pos.x;
                            last_click_y = ui.mouse_pos.y;
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } 
                        if modifiers == CTRL => match keypress {
                            VirtualKeyCode::Key0 => {
                                let ui = &mut r_s.ui;
                                if wimp_render::inside_tab_area(ui.mouse_pos, font_info) {
                                    let width = dimensions.width;
                                    let height = dimensions.height;

                                    wimp_render::make_active_tab_visible(
                                        ui,
                                        &r_s.view,
                                        &font_info,
                                        (width as _, height as _),
                                    );
                                } else {
                                    call_u_and_r!(Input::ResetScroll);
                                }
                            }
                            VirtualKeyCode::Left => {
                                call_u_and_r!(Input::MoveAllCursors(
                                    Move::ToPreviousLikelyEditLocation
                                ));
                            }
                            VirtualKeyCode::Right => {
                                call_u_and_r!(Input::MoveAllCursors(
                                    Move::ToNextLikelyEditLocation
                                ));
                            }
                            VirtualKeyCode::A => {
                                call_u_and_r!(Input::SelectAll);
                            }
                            VirtualKeyCode::C => {
                                call_u_and_r!(Input::Copy);
                            }
                            VirtualKeyCode::D => {
                                call_u_and_r!(Input::ExtendSelectionWithSearch);
                            }
                            VirtualKeyCode::F => {
                                switch_menu_mode!(MenuMode::FindReplace(FindReplaceMode::CurrentFile));
                            }
                            VirtualKeyCode::G => {
                                switch_menu_mode!(MenuMode::GoToPosition);
                            }
                            VirtualKeyCode::O => {
                                file_chooser_call!(single, p in CustomEvent::OpenFile(p));
                            }
                            VirtualKeyCode::P => {
                                switch_menu_mode!(MenuMode::FileSwitcher);
                            }
                            VirtualKeyCode::S => {
                                if let Some((i, buffer)) = r_s.view.get_visible_index_and_buffer() {
                                    match buffer.name {
                                        BufferName::Scratch(_) => {
                                            file_chooser_call!(
                                                save,
                                                p in CustomEvent::SaveNewFile(p, i)
                                            );
                                        }
                                        BufferName::Path(ref p) => {
                                            save_to_disk!(p, &buffer.data.chars, i);
                                        }
                                    }
                                }
                            }
                            VirtualKeyCode::T => {
                                call_u_and_r!(Input::NewScratchBuffer(None));
                            }
                            VirtualKeyCode::V => {
                                call_u_and_r!(Input::Paste(clipboard.get_contents().ok()));
                            }
                            VirtualKeyCode::W => match r_s.view.current_buffer_id {
                                BufferId {
                                    kind: BufferIdKind::Text,
                                    index,
                                    ..
                                } => {
                                    call_u_and_r!(Input::CloseBuffer(index));
                                }
                                _ => {
                                    call_u_and_r!(Input::CloseMenuIfAny);
                                }
                            },
                            VirtualKeyCode::X => {
                                call_u_and_r!(Input::Cut);
                            }
                            VirtualKeyCode::Y => {
                                call_u_and_r!(Input::Redo);
                            }
                            VirtualKeyCode::Z => {
                                call_u_and_r!(Input::Undo);
                            }
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Next
                                ));
                            }
                            _ => (),
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if 
                        modifiers == ALT | CTRL => match keypress {
                            VirtualKeyCode::Key0 => {
                                call_u_and_r!(Input::InsertNumbersAtCursors);
                            }
                            VirtualKeyCode::L => {
                                call_u_and_r!(Input::NextLanguage);
                            }
                            _ => (),
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if modifiers == CTRL | SHIFT => match keypress {
                            VirtualKeyCode::Home => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(
                                    Move::ToBufferStart
                                ));
                            }
                            VirtualKeyCode::End => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(
                                    Move::ToBufferEnd
                                ));
                            }
                            VirtualKeyCode::Left => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(
                                    Move::ToPreviousLikelyEditLocation
                                ));
                            }
                            VirtualKeyCode::Right => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(
                                    Move::ToNextLikelyEditLocation
                                ));
                            }
                            VirtualKeyCode::S => {
                                if let Some(i) = r_s.view.visible_buffer {
                                    file_chooser_call!(
                                        save,
                                        p in CustomEvent::SaveNewFile(p, i)
                                    );
                                }
                            }
                            VirtualKeyCode::Z => {
                                call_u_and_r!(Input::Redo);
                            }
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Previous
                                ));
                            }
                            _ => (),
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if modifiers.is_empty() => match if cfg!(debug_assertions) {dbg!(keypress)} else {keypress} {
                            VirtualKeyCode::Escape => {
                                call_u_and_r!(Input::SetSizeDependents(SizeDependents {
                                    buffer_xywh: wimp_render::get_edit_buffer_xywh(
                                        d!(),
                                        font_info,
                                        screen_wh!()
                                    )
                                    .into(),
                                    find_xywh: None,
                                    replace_xywh: None,
                                    go_to_position_xywh: None,
                                    font_info: None,
                                }));
                                call_u_and_r!(Input::CloseMenuIfAny);
                            }
                            VirtualKeyCode::F1 => {
                                call_u_and_r!(Input::DeleteLines);
                            }
                            VirtualKeyCode::Back => {
                                call_u_and_r!(Input::Delete);
                            }
                            VirtualKeyCode::Up => {
                                call_u_and_r!(Input::MoveAllCursors(Move::Up));
                                r_s.ui.navigation = Navigation::Up;
                            }
                            VirtualKeyCode::Down => {
                                call_u_and_r!(Input::MoveAllCursors(Move::Down));
                                r_s.ui.navigation = Navigation::Down;
                            }
                            VirtualKeyCode::Left => {
                                call_u_and_r!(Input::MoveAllCursors(Move::Left));
                            }
                            VirtualKeyCode::Right => {
                                call_u_and_r!(Input::MoveAllCursors(Move::Right));
                            }
                            VirtualKeyCode::Home => {
                                call_u_and_r!(Input::MoveAllCursors(Move::ToLineStart));
                            }
                            VirtualKeyCode::End => {
                                call_u_and_r!(Input::MoveAllCursors(Move::ToLineEnd));
                            }
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::TabIn);
                            }
                            _ => (),
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if modifiers == SHIFT => match keypress {
                            VirtualKeyCode::Up => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(Move::Up));
                                r_s.ui.navigation = Navigation::Up;
                            }
                            VirtualKeyCode::Down => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(Move::Down));
                                r_s.ui.navigation = Navigation::Down;
                            }
                            VirtualKeyCode::Left => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(Move::Left));
                            }
                            VirtualKeyCode::Right => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(Move::Right));
                            }
                            VirtualKeyCode::Home => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(
                                    Move::ToLineStart
                                ));
                            }
                            VirtualKeyCode::End => {
                                call_u_and_r!(Input::ExtendSelectionForAllCursors(Move::ToLineEnd));
                            }
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::TabOut);
                            }
                            VirtualKeyCode::Return => {
                                call_u_and_r!(Input::Insert('\n'));
                            }
                            _ => (),
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if modifiers == LOGO | CTRL => match keypress {
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Move(SelectionMove::Right)
                                ));
                            }
                            VirtualKeyCode::Home => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Move(SelectionMove::ToStart)
                                ));
                            }
                            VirtualKeyCode::End => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Move(SelectionMove::ToEnd)
                                ));
                            }
                            _ => {}
                        },
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(keypress),
                                    modifiers,
                                    ..
                                },
                            ..
                        } if modifiers == LOGO | CTRL | SHIFT => match keypress {
                            VirtualKeyCode::Tab => {
                                call_u_and_r!(Input::AdjustBufferSelection(
                                    SelectionAdjustment::Move(SelectionMove::Left)
                                ));
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                Event::MainEventsCleared if running => {
                    for _ in 0..EVENTS_PER_FRAME {
                        match editor_out_source.try_recv() {
                            Ok((v, c)) => {
                                r_s.view = v;
                                if let Some(index) = r_s.view.edited_buffer_index {
                                    r_s.buffer_status_map.insert(
                                        r_s.view.index_state,
                                        index,
                                        BufferStatus::EditedAndUnSaved
                                    );
                                }

                                r_s.cmds.push_back(c);
                            }
                            _ => break,
                        };
                    }

                    for _ in 0..EVENTS_PER_FRAME {
                        match edited_files_out_source.try_recv() {
                            Ok((index, transition)) => {
                                let view = &r_s.view;
                                let buffer_status_map = &mut r_s.buffer_status_map;
                                buffer_status_map.insert(
                                    view.index_state,
                                    index,
                                    transform_status(
                                        buffer_status_map
                                            .get(view.index_state, index)
                                            .unwrap_or_default(),
                                        transition
                                    )
                                );
                            }
                            _ => break,
                        };
                    }

                    // Queue a RedrawRequested event so we draw the updated view quickly.
                    glutin_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    r_s.ui.frame_init();
                    if_changed::dbg!(&r_s.ui.keyboard);

                    let (text_and_rects, input) =
                        wimp_render::view(
                            &mut r_s,
                            &font_info,
                            screen_wh!(),
                            dt,
                        );
                    let width = dimensions.width;
                    let height = dimensions.height;

                    gl_layer::render(&mut gl_state, text_and_rects, width as _, height as _)
                        .expect("gl_layer::render didn't work");

                    glutin_context
                        .swap_buffers()
                        .expect("swap_buffers didn't work!");

                    for _ in 0..EVENTS_PER_FRAME {
                        if let Some(cmd) = r_s.cmds.pop_front() {
                            match cmd {
                                Cmd::SetClipboard(s) => {
                                    if let Err(err) = clipboard.set_contents(s) {
                                        handle_platform_error!(err);
                                    }
                                }
                                Cmd::LoadFile(path) => load_file!(path),
                                Cmd::NoCmd => {}
                            }
                        } else {
                            break;
                        }
                    }

                    if let Some(input) = input {
                        call_u_and_r!(input);
                    }

                    if let Some(rate) = loop_helper.report_rate() {
                        glutin_context.window().set_title(&format!(
                            "{}{} {:.0} FPS {:?} click {:?}",
                            title,
                            if cfg!(debug_assertions) {
                                " DEBUG"
                            } else {
                                ""
                            },
                            rate,
                            (r_s.ui.mouse_pos.x, r_s.ui.mouse_pos.y),
                            (last_click_x, last_click_y),
                        ));
                    }

                    r_s.ui.frame_end();
                    perf_viz::start_record!("sleepin'");
                    loop_helper.loop_sleep();
                    perf_viz::end_record!("sleepin'");

                    perf_viz::end_record!("main loop");
                    perf_viz::start_record!("main loop");

                    // We want to track the time that the message loop takes too!
                    dt = loop_helper.loop_start();
                }
                Event::UserEvent(e) => match e {
                    CustomEvent::OpenFile(p) => load_file!(p),
                    CustomEvent::SaveNewFile(ref p, index) => {
                        let view = &r_s.view;
                        // The fact we need to store the index and retreive it later, potentially
                        // across multiple updates, is why this thread needs to know about the
                        // generational indices.
                        if let Some(b) = index
                            .get(view.index_state)
                            .and_then(|i| view.buffers.get(i))
                        {
                            save_to_disk!(p, &b.data.chars, index);
                        }
                    }
                    CustomEvent::SendBuffersToBeSaved => {
                        let view = &r_s.view;
                        let _hope_it_gets_there = edited_files_in_sink.send(
                            EditedFilesThread::Buffers(
                                view.index_state,
                                view.buffers.iter().enumerate().map(|(i, b)|
                                    (
                                        b.to_owned(), 
                                        r_s.buffer_status_map.get(view.index_state, view.index_state.new_index(g_i::IndexPart::or_max(i))).unwrap_or_default()
                                    )
                                ).collect()
                            )
                        );
                    }
                    CustomEvent::EditedBufferError(e) => {
                        // TODO show warning dialog to user and ask if they want to continue
                        // without edited file saving or save and restart. If they do, then it
                        // should be made obvious visually that the feature is not working right
                        // now.
                        handle_platform_error!(e);
                    }
                },
                Event::NewEvents(StartCause::Init) => {
                    // At least try to measure the first frame accurately
                    perf_viz::start_record!("main loop");
                    dt = loop_helper.loop_start();
                }
                _ => {}
            }
        });
    }
}
