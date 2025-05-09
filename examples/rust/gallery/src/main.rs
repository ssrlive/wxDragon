use wxdragon::prelude::*;

use std::cell::RefCell;

mod tabs;
use tabs::advanced_tab::create_advanced_tab;
use tabs::basic_tab::create_basic_tab;
use tabs::book_controls_tab::create_book_controls_tab;
use tabs::data_tab::{create_data_tab, FrameData};
use tabs::dialog_tab::create_dialog_tab;
use tabs::lists_tab::create_lists_tab;
use tabs::media_tab::create_media_tab;

// Tool IDs - used in main.rs
const ID_TOOL_NEW: Id = ID_HIGHEST + 1;
const ID_TOOL_OPEN: Id = ID_HIGHEST + 2;
const ID_TOOL_SAVE: Id = ID_HIGHEST + 3;

// --- Main Application Logic ---

fn main() {
    // Initialize the wxWidgets application
    wxdragon::main(|handle: &mut WxdAppHandle| {
        // Create the main application frame
        let frame = Frame::builder()
            .with_title("wxDragon Notebook Example")
            .with_size(Size::new(800, 900))
            .build();

        // --- Menu Bar Setup ---
        let file_menu = Menu::builder()
            .append_item(ID_EXIT, "E&xit\tAlt-X", "Quit this program")
            .build();
        let help_menu = Menu::builder()
            .append_item(ID_ABOUT, "&About...", "Show about dialog")
            .build();
        let menubar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(help_menu, "&Help")
            .build();
        frame.set_menu_bar(menubar);

        // --- Status Bar Setup ---
        let status_bar = StatusBar::builder(&frame)
            .with_fields_count(3)
            .with_status_widths(vec![-1, 150, 100])
            .add_initial_text(0, "Ready")
            .add_initial_text(1, "Center Field")
            .add_initial_text(2, "Right Field")
            .build();

        // --- User Data Setup ---
        let frame_user_data = RefCell::new(FrameData {
            click_count: 0,
            message: "Initial Message".to_string(),
        });
        frame.set_user_data(Box::new(frame_user_data));

        // --- Create the Notebook ---
        let notebook = Notebook::builder(&frame).with_id(120).build();

        // --- Create Tabs ---
        let (advanced_splitter, advanced_controls) = create_advanced_tab(&notebook);
        let basic_controls = create_basic_tab(&notebook, &frame);
        let list_controls = create_lists_tab(&notebook, &frame);
        let data_controls = create_data_tab(&notebook, &frame);
        let book_controls = create_book_controls_tab(&notebook);
        let dialog_controls = create_dialog_tab(&notebook, &frame);
        let media_controls = create_media_tab(&notebook);

        // --- ToolBar Setup ---
        // let tb_style = TB_TEXT | TB_HORIZONTAL; // Old - Commenting out for now as Toolbar not refactored
        // if let Some(toolbar) = frame.create_tool_bar(Some(tb_style), ID_ANY) {
        //     let open_icon = ArtProvider::get_bitmap(ART_FILE_OPEN, ART_TOOLBAR, None)
        //         .expect("Failed to get ART_FILE_OPEN for toolbar");
        //     toolbar.add_tool(
        //         1, // Tool ID
        //         "Open",
        //         &open_icon,
        //         "Open a file",
        //         ItemKind::Normal, // wxITEM_NORMAL
        //     );
        //     toolbar.realize();
        // }

        // --- Add Pages to Notebook ---
        notebook.add_page(&basic_controls.panel, "Basic Controls", true);
        notebook.add_page(&list_controls.panel, "Lists", false);
        notebook.add_page(&advanced_splitter, "Advanced", false);
        notebook.add_page(&data_controls.panel, "Data", false);
        notebook.add_page(&book_controls.tab_panel, "Book Controls", false);
        notebook.add_page(&dialog_controls.panel, "Dialogs", false);
        notebook.add_page(&media_controls.panel, "Media", false);

        // --- Set Frame Sizer ---
        let main_sizer = BoxSizer::builder(VERTICAL).build();
        main_sizer.add(&notebook, 1, EXPAND | ALL, 1);
        frame.set_sizer(main_sizer, true);

        // --- Bind Event Handlers ---

        // Bind tab-specific events
        basic_controls.bind_events();
        advanced_controls.bind_events();
        data_controls.bind_events(&frame, &status_bar);
        book_controls.bind_events();
        dialog_controls.bind_events(&frame);
        media_controls.bind_events();

        // Menu & Toolbar Event Handlers
        let frame_clone_for_menu = frame.clone();
        frame.bind(EventType::MENU, move |event: Event| match event.get_id() {
            id if id == ID_EXIT => {
                println!("Menu/Toolbar: Exit clicked!");
                frame_clone_for_menu.close();
            }
            id if id == ID_ABOUT => {
                println!("Menu: About clicked!");
            }
            id if id == ID_TOOL_NEW => {
                println!("Toolbar: New clicked!");
            }
            id if id == ID_TOOL_OPEN => {
                println!("Toolbar: Open clicked!");
            }
            id if id == ID_TOOL_SAVE => {
                println!("Toolbar: Save clicked!");
            }
            _ => {
                println!("Unhandled Menu/Tool ID: {}", event.get_id());
                event.skip(true);
            }
        });

        // Notebook Page Changed Event
        notebook.bind(EventType::NOTEBOOK_PAGE_CHANGED, |event| {
            if let Some(selection) = event.get_selection() {
                println!("Notebook page changed to: {}", selection);
            }
        });

        // --- Final Setup ---
        frame.show(true);
        frame.centre();
        handle.preserve(frame.clone());

        true
    });
}
