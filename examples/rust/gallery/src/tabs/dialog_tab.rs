use wxdragon::id;
use wxdragon::prelude::*;

use std::thread;
use std::time::Duration;
use wxdragon::dialogs::colour_dialog::ColourDialog;
use wxdragon::dialogs::file_dialog::{self as fd_const, FileDialog};
use wxdragon::dialogs::font_dialog::FontDialog;
use wxdragon::dialogs::progress_dialog::ProgressDialog;
use wxdragon::dialogs::text_entry_dialog::TextEntryDialog;
use wxdragon::widgets::panel::PanelStyle;

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
    pub progress_button: Button,
    // Added for FilePickerCtrl
    pub file_picker_ctrl_label: StaticText,
    pub file_picker_ctrl: FilePickerCtrl,
    pub file_picker_status_label: StaticText,
    // Added for DirPickerCtrl
    pub dir_picker_ctrl_label: StaticText,
    pub dir_picker_ctrl: DirPickerCtrl,
    pub dir_picker_status_label: StaticText,
    // Added for FontPickerCtrl
    pub font_picker_ctrl_label: StaticText,
    pub font_picker_ctrl: FontPickerCtrl,
    pub font_picker_status_label: StaticText,
    // Added for NotificationMessage
    pub show_notification_btn: Button,
    pub notification_status_label: StaticText,
}

pub fn create_dialog_tab(notebook: &Notebook, _frame: &Frame) -> DialogTabControls {
    let dialog_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

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

    // Progress Dialog section
    let progress_dialog_label = StaticText::builder(&dialog_panel)
        .with_label("Progress Dialog:")
        .build();
    let progress_button = Button::builder(&dialog_panel)
        .with_label("Show Progress...")
        .build();
    progress_button.set_tooltip("Click to show a progress dialog demonstration.");

    // FilePickerCtrl section
    let file_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("FilePickerCtrl:")
        .build();
    let file_picker_ctrl = FilePickerCtrl::builder(&dialog_panel)
        .with_message("Select a file with FilePickerCtrl")
        .with_wildcard("Rust files (*.rs)|*.rs|Text files (*.txt)|*.txt|All files (*.*)|*.*")
        .with_style(FilePickerCtrlStyle::Open | FilePickerCtrlStyle::FileMustExist)
        .build();
    file_picker_ctrl.set_tooltip("Select a file to see its path below.");
    let file_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("FilePickerCtrl Path: -")
        .build();

    // Layout using Main Vertical BoxSizer and child FlexGridSizer
    let main_sizer = BoxSizer::builder(VERTICAL).build();
    let label_flags = ALIGN_RIGHT | ALIGN_CENTER_VERTICAL;
    let control_flags = EXPAND; // General flag for controls spanning the column

    let grid_sizer = FlexGridSizer::builder(0, 2) // 0 rows means flexible, 2 columns
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer.add_growable_col(0, 1); // Label column (flex factor 1)
    grid_sizer.add_growable_col(1, 3); // Control column (flex factor 3)

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

    // Add ProgressDialog controls
    grid_sizer.add(&progress_dialog_label, 0, label_flags, 0);
    let progress_dialog_sizer = BoxSizer::builder(HORIZONTAL).build();
    progress_dialog_sizer.add(&progress_button, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    grid_sizer.add_sizer(&progress_dialog_sizer, 1, EXPAND, 0);

    // Add FilePickerCtrl controls
    grid_sizer.add(&file_picker_ctrl_label, 0, label_flags, 0);
    let fpc_sizer = BoxSizer::builder(HORIZONTAL).build();
    fpc_sizer.add(&file_picker_ctrl, 1, EXPAND | ALL, 2);
    fpc_sizer.add_spacer(5); // Small spacer
    fpc_sizer.add(&file_picker_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&fpc_sizer, 1, EXPAND, 0);

    // --- DirPickerCtrl ---
    let dir_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("Dir Picker Ctrl:")
        .build();
    let dir_picker_ctrl = DirPickerCtrl::builder(&dialog_panel)
        .with_style(DirPickerCtrlStyle::Default | DirPickerCtrlStyle::DirMustExist)
        .with_message("Choose a directory")
        .build();
    let dir_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("DirPicker: No directory selected yet.")
        .build();

    grid_sizer.add(&dir_picker_ctrl_label, 0, label_flags, 0);
    let dpc_sizer = BoxSizer::builder(HORIZONTAL).build();
    dpc_sizer.add(&dir_picker_ctrl, 1, EXPAND | ALL, 2);
    dpc_sizer.add_spacer(5);
    dpc_sizer.add(&dir_picker_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&dpc_sizer, 1, EXPAND, 0);

    // --- FontPickerCtrl ---
    let font_picker_ctrl_label = StaticText::builder(&dialog_panel)
        .with_label("Font Picker Ctrl:")
        .build();
    let initial_font_for_picker = Font::new(); // Use default font for now
    let font_picker_ctrl = FontPickerCtrl::builder(&dialog_panel)
        .with_initial_font(initial_font_for_picker)
        .with_style(FontPickerCtrlStyle::Default | FontPickerCtrlStyle::UseFontForLabel)
        .build();
    let font_picker_status_label = StaticText::builder(&dialog_panel)
        .with_label("FontPicker: Initial font set.")
        .build();

    grid_sizer.add(&font_picker_ctrl_label, 0, label_flags, 0);
    let font_pc_sizer = BoxSizer::builder(HORIZONTAL).build();
    font_pc_sizer.add(&font_picker_ctrl, 1, EXPAND | ALL, 2);
    font_pc_sizer.add_spacer(5);
    font_pc_sizer.add(&font_picker_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&font_pc_sizer, 1, EXPAND, 0);

    // --- NotificationMessage ---
    let notification_label = StaticText::builder(&dialog_panel)
        .with_label("Notification:")
        .build();
    let show_notification_btn = Button::builder(&dialog_panel)
        .with_label("Show Notification")
        .build();
    show_notification_btn.set_tooltip("Click to show a notification message with actions.");
    let notification_status_label = StaticText::builder(&dialog_panel)
        .with_label("Notification Status: -")
        .build();

    grid_sizer.add(&notification_label, 0, label_flags, 0);
    let notification_sizer = BoxSizer::builder(HORIZONTAL).build();
    notification_sizer.add(&show_notification_btn, 0, ALIGN_CENTER_VERTICAL | ALL, 2);
    notification_sizer.add_spacer(10);
    notification_sizer.add(&notification_status_label, 1, EXPAND | ALL, 2);
    grid_sizer.add_sizer(&notification_sizer, 1, EXPAND, 0);

    main_sizer.add_sizer(&grid_sizer, 1, EXPAND | ALL, 10);
    dialog_panel.set_sizer_and_fit(main_sizer, true);

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
        progress_button,
        // Added for FilePickerCtrl
        file_picker_ctrl_label,
        file_picker_ctrl,
        file_picker_status_label,
        // Added for DirPickerCtrl
        dir_picker_ctrl_label,
        dir_picker_ctrl,
        dir_picker_status_label,
        // Added for FontPickerCtrl
        font_picker_ctrl_label,
        font_picker_ctrl,
        font_picker_status_label,
        // NotificationMessage
        show_notification_btn,
        notification_status_label,
    }
}

impl DialogTabControls {
    pub fn bind_events(&self, frame: &Frame) {
        // Event handlers for Message Dialog
        let dialog_panel_clone = self.panel.clone();
        self.show_msg_dialog_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
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
        self.open_file_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Open File button clicked.");
                let dialog = FileDialog::builder(Some(&frame_parent_open_ctx))
                    .with_message("Choose a file")
                    .with_style(
                        fd_const::FD_OPEN | fd_const::FD_FILE_MUST_EXIST | fd_const::FD_MULTIPLE,
                    )
                    .with_wildcard(
                        "Rust files (*.rs)|*.rs|Text files (*.txt)|*.txt|All files (*.*)|*.*",
                    )
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
        self.save_file_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
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
        self.get_text_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Get Text button clicked.");
                let dialog = TextEntryDialog::builder(
                    Some(&frame_parent_text_ctx),
                    "Enter some text:",
                    "Text Input",
                )
                .with_default_value("Default text") // Assuming with_default_value is correct for original
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
        self.get_password_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Get Password button clicked.");
                let dialog = TextEntryDialog::builder(
                    Some(&frame_parent_pass_ctx),
                    "Enter your password:",
                    "Password Input",
                )
                .password() // Assuming .password() is correct for original
                .build();
                if dialog.show_modal() == id::ID_OK {
                    te_status_pass_clone
                        .set_label("Password Entered (value not shown for security)");
                } else {
                    te_status_pass_clone.set_label("Password Entry Cancelled.");
                }
                println!("Password Entry Dialog Closed.");
            });

        // Event handler for Colour Dialog
        let colour_status_label_clone = self.colour_dialog_status_label.clone();
        let frame_parent_colour_ctx = frame.clone();
        self.choose_colour_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Choose Colour button clicked.");
                let dialog = ColourDialog::builder(Some(&frame_parent_colour_ctx))
                    .with_title("Choose a colour") // Assuming this is correct for original
                    .build();
                if dialog.show_modal() == id::ID_OK {
                    if let Some(colour) = dialog.get_colour() {
                        let status = format!(
                            "Selected colour: RGB({}, {}, {})",
                            colour.r, colour.g, colour.b
                        );
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
        self.font_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Choose Font button clicked.");
                let dialog = FontDialog::builder(Some(&frame_parent_font_ctx))
                    .with_title("Choose a font") // Assuming this is correct for original
                    .build();
                if dialog.show_modal() == id::ID_OK {
                    if let Some(font) = dialog.get_font() {
                        let font_info = format!(
                            "Font: {} ({} pt, {})",
                            font.get_face_name(),
                            font.get_point_size(),
                            if font.is_underlined() {
                                "underlined"
                            } else {
                                "not underlined"
                            }
                        );
                        font_sample_text_clone.set_label(&font_info);
                        println!("Selected {}", font_info);
                    } else {
                        font_sample_text_clone.set_label("No font selected");
                        println!("No font was returned by the dialog");
                    }
                } else {
                    println!("Font Dialog Cancelled.");
                }
            });

        // Add a handler for the progress dialog button
        let frame_parent_progress_ctx = frame.clone();
        self.progress_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                println!("Show Progress Dialog button clicked.");
                let dialog = ProgressDialog::builder(
                    // Assuming original signature
                    Some(&frame_parent_progress_ctx),
                    "Progress Demonstration",
                    "Processing items...",
                    100, // Max value
                )
                .can_abort()
                .can_skip()
                .show_elapsed_time()
                .show_remaining_time()
                .build();
                let mut continue_progress = true;
                for i in 0..=100 {
                    if !continue_progress {
                        break;
                    }
                    thread::sleep(Duration::from_millis(50));
                    let message = if i % 10 == 0 && i > 0 {
                        Some(format!("Processed {} items...", i))
                    } else {
                        None
                    };
                    let (should_continue, was_skipped) =
                        dialog.update_with_skip(i, message.as_deref());
                    continue_progress = should_continue;
                    if was_skipped {
                        println!("Progress operation was skipped at step {}.", i);
                    }
                    if dialog.was_cancelled() {
                        println!("Progress operation was cancelled by user.");
                        break;
                    }
                }
                println!("Progress Dialog Closed.");
            });

        // Event handler for FilePickerCtrl
        let fpc_status_label_clone = self.file_picker_status_label.clone();
        let file_picker_ctrl_clone = self.file_picker_ctrl.clone();
        self.file_picker_ctrl
            .bind(EventType::FILE_PICKER_CHANGED, move |_event: Event| {
                let path_string: String = file_picker_ctrl_clone.get_path();

                if !path_string.is_empty() {
                    let status = format!("Selected Path: {}", path_string);
                    fpc_status_label_clone.set_label(&status);
                    println!("FilePickerCtrl Path Changed: {}", path_string);

                    // Set a new font (currently default) to the status label
                    let new_font = Font::new();
                    fpc_status_label_clone.set_font(&new_font);
                    // Note: new_font is dropped here, but the label should have its font set.
                } else {
                    fpc_status_label_clone
                        .set_label("FilePickerCtrl: No path selected or path is empty.");
                    println!("FilePickerCtrl Path Changed: No path selected or path is empty.");
                    // Optionally reset to a default font here too if desired
                    let default_font = Font::new();
                    fpc_status_label_clone.set_font(&default_font);
                }
            });

        // Event handler for DirPickerCtrl
        let dpc_status_label_clone = self.dir_picker_status_label.clone();
        let dir_picker_ctrl_clone = self.dir_picker_ctrl.clone(); // Clone for the closure
        self.dir_picker_ctrl
            .bind(EventType::DIR_PICKER_CHANGED, move |_event: Event| {
                let path_string: String = dir_picker_ctrl_clone.get_path();
                if !path_string.is_empty() {
                    let status = format!("Selected Dir: {}", path_string);
                    dpc_status_label_clone.set_label(&status);
                    println!("DirPickerCtrl Path Changed: {}", path_string);
                } else {
                    dpc_status_label_clone
                        .set_label("DirPickerCtrl: No directory selected or path is empty.");
                    println!("DirPickerCtrl Path Changed: No directory selected or path is empty.");
                }
            });

        // Event handler for FontPickerCtrl
        let font_pc_status_label_clone = self.font_picker_status_label.clone();
        let font_picker_ctrl_clone = self.font_picker_ctrl.clone(); // Clone for the closure
        self.font_picker_ctrl
            .bind(EventType::FONT_PICKER_CHANGED, move |_event: Event| {
                if let Some(selected_font) = font_picker_ctrl_clone.get_selected_font() {
                    let status = format!(
                        "Selected Font: {} pt {}",
                        selected_font.get_point_size(),
                        selected_font.get_face_name()
                    );
                    font_pc_status_label_clone.set_label(&status);
                    // Apply the selected font to the status label
                    font_pc_status_label_clone.set_font(&selected_font);
                    println!(
                        "FontPickerCtrl Font Changed: {} pt {}",
                        selected_font.get_point_size(),
                        selected_font.get_face_name()
                    );
                } else {
                    font_pc_status_label_clone
                        .set_label("FontPickerCtrl: No font selected or font is invalid.");
                    println!("FontPickerCtrl Font Changed: No font selected or font is invalid.");
                }
            });

        // --- NotificationMessage Events ---
        let notification_status_label_clone = self.notification_status_label.clone();
        let panel_for_notif_handler = self.panel.clone(); // Clone for notification related event handlers
        let frame_for_notif_parent = frame.clone(); // Clone the frame for the notification parent

        self.show_notification_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
                let notif_builder = NotificationMessage::builder()
                    .with_title("Hello from wxDragon!")
                    .with_message("This is a notification with actions.")
                    .with_flags(ICON_INFORMATION); // Use one of the imported constants

                match notif_builder.build() {
                    Ok(notif_msg) => {
                        // Make notif_msg mutable to call set_parent
                        // Set the main frame as the parent
                        if let Err(e) = notif_msg.set_parent(Some(&frame_for_notif_parent)) {
                            println!("Error: Failed to set notification parent: {:?}", e);
                        }

                        if let Err(e) = notif_msg.add_action(101, "Action 1") {
                            println!("Error: Failed to add action 1: {:?}", e);
                        }
                        if let Err(e) = notif_msg.add_action(102, "Action 2") {
                            println!("Error: Failed to add action 2: {:?}", e);
                        }

                        if notif_msg.show(TIMEOUT_NEVER) {
                            // Changed to TIMEOUT_NEVER
                            notification_status_label_clone
                                .set_label("Notification shown (TIMEOUT_NEVER).");
                            println!("Info: Notification shown (TIMEOUT_NEVER).");
                        } else {
                            notification_status_label_clone
                                .set_label("Failed to show notification.");
                            println!("Error: Failed to show notification.");
                        }
                    }
                    Err(e) => {
                        notification_status_label_clone.set_label("Failed to build notification.");
                        println!("Error: Failed to build notification: {:?}", e);
                    }
                }
            });

        // Bind notification events to the panel
        let notif_status_click_clone = self.notification_status_label.clone();
        panel_for_notif_handler.bind(EventType::NOTIFICATION_MESSAGE_CLICK, move |_event| {
            notif_status_click_clone.set_label("Notification: Clicked!");
            println!("Info: Notification Clicked");
        });

        let notif_status_dismiss_clone = self.notification_status_label.clone();
        panel_for_notif_handler.bind(EventType::NOTIFICATION_MESSAGE_DISMISSED, move |_event| {
            notif_status_dismiss_clone.set_label("Notification: Dismissed!");
            println!("Info: Notification Dismissed");
        });

        let notif_status_action_clone = self.notification_status_label.clone();
        panel_for_notif_handler.bind(EventType::NOTIFICATION_MESSAGE_ACTION, move |event| {
            let action_id = event.get_id(); // wxCommandEvent::GetId()
            notif_status_action_clone
                .set_label(&format!("Notification: Action {} clicked!", action_id));
            println!("Info: Notification Action {} clicked", action_id);
        });
    }
}
