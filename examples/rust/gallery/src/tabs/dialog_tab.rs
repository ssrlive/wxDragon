use wxdragon::prelude::*;
use wxdragon::id;

use wxdragon::dialogs::file_dialog::{self as fd_const, FileDialog};
use wxdragon::dialogs::text_entry_dialog::TextEntryDialog;
use wxdragon::dialogs::colour_dialog::ColourDialog;
use wxdragon::dialogs::font_dialog::FontDialog;

#[allow(dead_code)]
pub struct DialogTabControls {
    pub panel: Panel,
    pub show_msg_dialog_btn: Button,
    pub open_file_btn: Button,
    pub save_file_btn: Button,
    pub file_dialog_status_label: StaticText,
    pub get_text_btn: Button,
    pub get_password_btn: Button,
    pub text_entry_status_label: StaticText,
    pub choose_colour_btn: Button,
    pub colour_dialog_status_label: StaticText,
    pub font_button: Button,
    pub font_sample_text: StaticText,
}

pub fn create_dialog_tab(notebook: &Notebook, _frame: &Frame) -> DialogTabControls {
    let dialog_panel = Panel::builder(notebook).with_style(TAB_TRAVERSAL).build();

    // Message Dialog section
    let show_msg_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Message Dialog:")
        .build();
    let show_msg_dialog_btn = Button::builder(&dialog_panel)
        .with_label("Show Info")
        .build();
    show_msg_dialog_btn.set_tooltip("Click to show an informational message dialog.");

    // File Dialog section
    let file_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("File Dialog:")
        .build();
    let open_file_btn = Button::builder(&dialog_panel)
        .with_label("Open File...")
        .build();
    open_file_btn.set_tooltip("Click to show an open file dialog.");
    let save_file_btn = Button::builder(&dialog_panel)
        .with_label("Save File...")
        .build();
    save_file_btn.set_tooltip("Click to show a save file dialog.");
    let file_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("File Dialog Status: -")
        .build();

    // Text Entry Dialog section
    let text_entry_label = StaticText::builder(&dialog_panel)
        .with_label("Text Entry:")
        .build();
    let get_text_btn = Button::builder(&dialog_panel)
        .with_label("Get Input...")
        .build();
    get_text_btn.set_tooltip("Click to show a text entry dialog.");
    let get_password_btn = Button::builder(&dialog_panel)
        .with_label("Get Password...")
        .build();
    get_password_btn.set_tooltip("Click to show a password entry dialog.");
    let text_entry_status_label = StaticText::builder(&dialog_panel)
        .with_label("Text Entry Status: -")
        .build();
    
    // Colour Dialog section
    let colour_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Colour Dialog:")
        .build();
    let choose_colour_btn = Button::builder(&dialog_panel)
        .with_label("Choose Colour...")
        .build();
    choose_colour_btn.set_tooltip("Click to show a colour picker dialog.");
    let colour_dialog_status_label = StaticText::builder(&dialog_panel)
        .with_label("Colour Dialog Status: -")
        .build();

    // Add FontDialog button after the ColourDialog
    // Create button to choose a font
    let font_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Font Dialog:")
        .build();
    let font_button = Button::builder(&dialog_panel)
        .with_label("Choose Font...")
        .build();
    font_button.set_tooltip("Click to show a font picker dialog.");
    let font_sample_text = StaticText::builder(&dialog_panel)
        .with_label("Font Sample")
        .build();

    // Layout using Main Vertical BoxSizer and child FlexGridSizer
    let main_sizer = BoxSizer::builder(VERTICAL).build();
    let label_flags = ALIGN_RIGHT | ALIGN_CENTER_VERTICAL;
    let control_flags = EXPAND;

    let grid_sizer = FlexGridSizer::builder(0, 2)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer.add_growable_col(0, 1);
    grid_sizer.add_growable_col(1, 3);

    // Add Message Dialog controls
    grid_sizer.add(&show_msg_dialog_label, 0, label_flags, 0);
    grid_sizer.add(&show_msg_dialog_btn, 1, control_flags, 0);

    // Add File Dialog controls
    grid_sizer.add(&file_dialog_label, 0, label_flags, 0);
    let file_btns_sizer = BoxSizer::builder(HORIZONTAL).build();
    file_btns_sizer.add(&open_file_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    file_btns_sizer.add(&save_file_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    file_btns_sizer.add_spacer(10);
    file_btns_sizer.add(&file_dialog_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&file_btns_sizer, 1, EXPAND, 0);

    // Add Text Entry Dialog controls
    grid_sizer.add(&text_entry_label, 0, label_flags, 0);
    let text_entry_btns_sizer = BoxSizer::builder(HORIZONTAL).build();
    text_entry_btns_sizer.add(&get_text_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    text_entry_btns_sizer.add(&get_password_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    text_entry_btns_sizer.add_spacer(10);
    text_entry_btns_sizer.add(&text_entry_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&text_entry_btns_sizer, 1, EXPAND, 0);
    
    // Add Colour Dialog controls
    grid_sizer.add(&colour_dialog_label, 0, label_flags, 0);
    let colour_dialog_sizer = BoxSizer::builder(HORIZONTAL).build();
    colour_dialog_sizer.add(&choose_colour_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    colour_dialog_sizer.add_spacer(10);
    colour_dialog_sizer.add(&colour_dialog_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&colour_dialog_sizer, 1, EXPAND, 0);

    // Add FontDialog controls
    grid_sizer.add(&font_dialog_label, 0, label_flags, 0);
    let font_dialog_sizer = BoxSizer::builder(HORIZONTAL).build();
    font_dialog_sizer.add(&font_button, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    font_dialog_sizer.add_spacer(10);
    font_dialog_sizer.add(&font_sample_text, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&font_dialog_sizer, 1, EXPAND, 0);

    main_sizer.add_sizer(&grid_sizer, 1, EXPAND | ALL, 10);
    dialog_panel.set_sizer_and_fit(main_sizer, true);

    // Event bindings moved to bind_events method

    DialogTabControls {
        panel: dialog_panel,
        show_msg_dialog_btn,
        open_file_btn,
        save_file_btn,
        file_dialog_status_label,
        get_text_btn,
        get_password_btn,
        text_entry_status_label,
        choose_colour_btn,
        colour_dialog_status_label,
        font_button,
        font_sample_text,
    }
}

impl DialogTabControls {
    pub fn bind_events(&self, frame: &Frame) {
        // Event handlers for Message Dialog
        let dialog_panel_clone = self.panel.clone();
        self.show_msg_dialog_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Show Message Dialog button clicked.");
            let dialog = MessageDialog::builder(
                Some(&dialog_panel_clone),
                "This is an informational message from wxDragon!",
                "Info",
            )
            .with_style(OK | ICON_INFORMATION)
            .build();
            dialog.show_modal();
            println!("Message Dialog Closed.");
        });

        // Event handlers for File Dialog
        let fd_status_label_clone = self.file_dialog_status_label.clone();
        let frame_for_fd_clone = frame.clone();
        
        let status_label_open_clone = fd_status_label_clone.clone();
        let frame_parent_open_ctx = frame_for_fd_clone.clone();
        self.open_file_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Open File button clicked.");
            let dialog = FileDialog::builder(Some(&frame_parent_open_ctx))
                .with_message("Choose a file")
                .with_style(fd_const::FD_OPEN | fd_const::FD_FILE_MUST_EXIST | fd_const::FD_MULTIPLE)
                .with_wildcard("Rust files (*.rs)|*.rs|Text files (*.txt)|*.txt|All files (*.*)|*.*")
                .build();
            if dialog.show_modal() == id::ID_OK {
                let paths = dialog.get_paths();
                let status = format!("Opened: {:?}", paths);
                status_label_open_clone.set_label(&status);
                println!("{}", status);
            } else {
                status_label_open_clone.set_label("Open File Cancelled.");
                println!("Open File Cancelled.");
            }
            println!("Open File Dialog Closed.");
        });

        let status_label_save_clone = fd_status_label_clone.clone();
        let frame_parent_save_ctx = frame_for_fd_clone.clone();
        self.save_file_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Save File button clicked.");
            let dialog = FileDialog::builder(Some(&frame_parent_save_ctx))
                .with_message("Save file as")
                .with_style(fd_const::FD_SAVE | fd_const::FD_OVERWRITE_PROMPT)
                .with_default_file("my_document.txt")
                .with_wildcard("Text files (*.txt)|*.txt|All files (*.*)|*.*")
                .build();
            if dialog.show_modal() == id::ID_OK {
                if let Some(path) = dialog.get_path() {
                    let status = format!("Saved: {}", path);
                    status_label_save_clone.set_label(&status);
                    println!("{}", status);
                } else {
                    status_label_save_clone.set_label("Save File Error: No path returned.");
                    println!("Save File Error: No path returned.");
                }
            } else {
                status_label_save_clone.set_label("Save File Cancelled.");
                println!("Save File Cancelled.");
            }
            println!("Save File Dialog Closed.");
        });

        // Event handlers for Text Entry Dialog
        let te_status_label_clone = self.text_entry_status_label.clone();
        let frame_for_te_clone = frame.clone();

        let te_status_text_clone = te_status_label_clone.clone();
        let frame_parent_text_ctx = frame_for_te_clone.clone();
        self.get_text_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Get Text button clicked.");
            let dialog = TextEntryDialog::builder(
                Some(&frame_parent_text_ctx),
                "Enter some text:",
                "Text Input",
            )
            .with_default_value("Default text")
            .build();
            if dialog.show_modal() == id::ID_OK {
                let entered_value = dialog.get_value().unwrap_or_else(|| "<empty>".to_string());
                te_status_text_clone.set_label(&format!("Text Entered: {}", entered_value));
            } else {
                te_status_text_clone.set_label("Text Entry Cancelled.");
            }
            println!("Text Entry Dialog Closed.");
        });

        let te_status_pass_clone = te_status_label_clone.clone();
        let frame_parent_pass_ctx = frame_for_te_clone.clone();
        self.get_password_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Get Password button clicked.");
            let dialog = TextEntryDialog::builder(
                Some(&frame_parent_pass_ctx),
                "Enter your password:",
                "Password Input",
            )
            .password()
            .build();
            if dialog.show_modal() == id::ID_OK {
                te_status_pass_clone.set_label("Password Entered (value not shown for security)");
            } else {
                te_status_pass_clone.set_label("Password Entry Cancelled.");
            }
            println!("Password Entry Dialog Closed.");
        });
        
        // Event handler for Colour Dialog
        let colour_status_label_clone = self.colour_dialog_status_label.clone();
        let frame_parent_colour_ctx = frame.clone();
        self.choose_colour_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Choose Colour button clicked.");
            
            // Create the dialog without custom initial data
            let dialog = ColourDialog::builder(Some(&frame_parent_colour_ctx))
                .with_title("Choose a colour")
                .build();
                
            if dialog.show_modal() == id::ID_OK {
                // Get the chosen colour
                if let Some(colour) = dialog.get_colour() {
                    let status = format!("Selected colour: RGB({}, {}, {})", 
                        colour.r, colour.g, colour.b);
                    colour_status_label_clone.set_label(&status);
                    println!("{}", status);
                } else {
                    colour_status_label_clone.set_label("Colour Dialog: No colour returned.");
                    println!("Colour Dialog: No colour returned.");
                }
            } else {
                colour_status_label_clone.set_label("Colour Selection Cancelled.");
                println!("Colour Selection Cancelled.");
            }
            println!("Colour Dialog Closed.");
        });
        
        // Add a handler for the font button
        let frame_parent_font_ctx = frame.clone();
        let font_sample_text_clone = self.font_sample_text.clone();
        self.font_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
            println!("Choose Font button clicked.");
            
            // Create the dialog without a specific FontData
            let dialog = FontDialog::builder(Some(&frame_parent_font_ctx))
                .with_title("Choose a font")
                .build();
            
            // Show the dialog
            if dialog.show_modal() == id::ID_OK {
                // Get the selected font
                if let Some(font) = dialog.get_font() {
                    let font_info = format!("Font: {} ({} pt, {})",
                        font.get_face_name(),
                        font.get_point_size(),
                        if font.is_underlined() { "underlined" } else { "not underlined" }
                    );
                    font_sample_text_clone.set_label(&font_info);
                    println!("Selected {}", font_info);
                    // Ideally, we would also set the font on the sample text,
                    // but that would require more bindings not yet implemented
                } else {
                    font_sample_text_clone.set_label("No font selected");
                    println!("No font was returned by the dialog");
                }
            } else {
                println!("Font Dialog Cancelled.");
            }
        });
    }
} 