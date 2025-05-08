use wxdragon::id;
use wxdragon::prelude::*; // Use prelude for imports // For ID_OK, ID_CANCEL etc.

use image::GenericImageView;
use std::cell::RefCell;
use std::fmt::Write;
use wxdragon::Bitmap;

const ID_TOOL_NEW: Id = ID_HIGHEST + 1;
const ID_TOOL_OPEN: Id = ID_HIGHEST + 2;
const ID_TOOL_SAVE: Id = ID_HIGHEST + 3;
const ID_BITMAP_BTN_RED: Id = ID_HIGHEST + 5;
const ID_BITMAP_BTN_ART: Id = ID_HIGHEST + 6;
const ID_LIST_CTRL: Id = ID_HIGHEST + 7;
const ID_SEARCH_CTRL: Id = ID_HIGHEST + 8;
const ID_BITMAP_COMBO_BOX: Id = ID_HIGHEST + 9;
const ID_HYPERLINK_CTRL: Id = ID_HIGHEST + 10;
const ID_ACTIVITY_INDICATOR: Id = ID_HIGHEST + 11;
const ID_ACTIVITY_START_BTN: Id = ID_HIGHEST + 12;
const ID_ACTIVITY_STOP_BTN: Id = ID_HIGHEST + 13;
const ID_SPINCTRLDOUBLE: Id = ID_HIGHEST + 14;
const ID_CALENDAR_CTRL: Id = ID_HIGHEST + 15;
const ID_SHOW_MESSAGE_DIALOG_BTN: Id = ID_HIGHEST + 16; // New ID for MessageDialog button

#[derive(Debug, Default)]
struct FrameData {
    click_count: u32,
    message: String,
}

struct BasicTabControls {
    panel: Panel,
    text_ctrl: TextCtrl,
    spin_button: SpinButton,
    checkbox: CheckBox,
    radio1: RadioButton,
    radio2: RadioButton,
    radio_status_label: StaticText,
    toggle_button: ToggleButton,
    toggle_status_label: StaticText,
    bitmap_button: BitmapButton,
    art_button: BitmapButton,
    radio_box: RadioBox,
    bitmap_combo_box: BitmapComboBox,
    colour_picker: ColourPickerCtrl,
    colour_label: StaticText,
    date_picker: DatePickerCtrl,
    date_picker_label: StaticText,
    search_ctrl: SearchCtrl,
    hyperlink_ctrl: HyperlinkCtrl,
    activity_indicator: ActivityIndicator,
    activity_start_btn: Button,
    activity_stop_btn: Button,
    spinctrl_double: SpinCtrlDouble,
    spinctrl_double_label: StaticText,
    calendar_ctrl: CalendarCtrl,
    calendar_label: StaticText,
    scroll_bar: ScrollBar,
    scrollbar_status_label: StaticText,
    cmd_link_button: CommandLinkButton,
}

struct ListsTabControls {
    panel: ScrolledWindow,
    list_box: ListBox,
    listbox_status_label: StaticText,
    checklistbox: CheckListBox,
    checklistbox_status_label: StaticText,
    choice_ctrl: Choice,
    choice_status_label: StaticText,
    combo_box: ComboBox,
    combo_status_label: StaticText,
    list_ctrl: ListCtrl,
    list_ctrl_status_label: StaticText,
}

struct AdvancedTabControls {
    tree_ctrl: TreeCtrl,
    tree_status_label: StaticText,
    gauge: Gauge,
    gauge_increase_btn: Button,
    gauge_reset_btn: Button,
    gauge_status_label: StaticText,
    slider: Slider,
    slider_label: StaticText,
    spin_ctrl: SpinCtrl,
    spin_ctrl_label: StaticText,
}

struct DataTabControls {
    panel: Panel,
    button: Button,
    data_display_label: StaticText,
}

struct BookControlsTab {
    tab_panel: Panel,
    treebook: Treebook,
}

fn create_basic_tab(notebook: &Notebook) -> BasicTabControls {
    let basic_panel = Panel::builder(notebook).with_style(TAB_TRAVERSAL).build();

    // --- Create Controls ---
    let static_text = StaticText::builder(&basic_panel)
        .with_label("Text Input:")
        .build();
    static_text.set_tooltip("This is a label for the text input field.");
    let text_ctrl = TextCtrl::builder(&basic_panel)
        .with_value("0")
        .with_id(115)
        .with_style(TE_PROCESS_ENTER)
        .build();
    text_ctrl.set_tooltip("Enter a numeric value here. Press Enter to update the SpinButton.");
    let spin_button_label = StaticText::builder(&basic_panel)
        .with_label("Spin Button:")
        .build();
    let spin_button = SpinButton::builder(&basic_panel)
        .with_id(116)
        .with_range(0, 100)
        .with_initial_value(0)
        .with_style(SP_VERTICAL | SP_ARROW_KEYS | SP_WRAP)
        .with_size(Size::new(-1, 28))
        .build();
    let spinctrl_double_label_widget = StaticText::builder(&basic_panel)
        .with_label("Spin Double:")
        .build();
    let spinctrl_double = SpinCtrlDouble::builder(&basic_panel)
        .with_id(ID_SPINCTRLDOUBLE)
        .with_value_str("1.23")
        .with_range(0.0, 100.0)
        .with_initial_value(50.5)
        .with_increment(0.1)
        .build();
    spinctrl_double.set_digits(2);
    let spinctrl_double_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{:.2}", spinctrl_double.get_value()))
        .build();

    let checkbox_label_widget = StaticText::builder(&basic_panel)
        .with_label("Checkbox:")
        .build();
    let checkbox = CheckBox::builder(&basic_panel)
        .with_label("Enable Feature")
        .build();
    let radio_box_label = StaticText::builder(&basic_panel)
        .with_label("Radio Box:")
        .build();
    let radio_box_choices = ["Choice X", "Choice Y", "Choice Z"];
    let radio_box = RadioBox::builder(Some(&basic_panel), &radio_box_choices)
        .with_label("")
        .with_id(120)
        .with_major_dimension(1)
        .with_style(RA_SPECIFY_COLS)
        .build();
    radio_box.set_selection(0);
    let radio_label = StaticText::builder(&basic_panel)
        .with_label("Radio Buttons:")
        .build();
    let radio1 = RadioButton::builder(&basic_panel)
        .with_label("Option 1")
        .first_in_group()
        .build();
    let radio2 = RadioButton::builder(&basic_panel)
        .with_label("Option 2")
        .build();
    radio1.set_value(true);
    let radio_status_label = StaticText::builder(&basic_panel)
        .with_label("Selected: Option 1")
        .build();

    // Group 3: Buttons
    let toggle_button_label = StaticText::builder(&basic_panel)
        .with_label("Toggle Button:")
        .build();
    let toggle_button = ToggleButton::builder(&basic_panel)
        .with_id(110)
        .with_label("Toggle Status")
        .build();
    toggle_button.set_value(true);
    let toggle_status_label = StaticText::builder(&basic_panel).with_label("ON").build();

    // CommandLinkButton
    let cmd_link_button_label = StaticText::builder(&basic_panel)
        .with_label("Cmd Link Btn:")
        .build();
    let cmd_link_button = CommandLinkButton::builder(&basic_panel)
        .with_id(ID_HIGHEST + 16) // Assign a new ID
        .with_main_label("Open System Settings")
        .with_note("Configure your display, network, and other system preferences.")
        .build();
    cmd_link_button.set_tooltip("Click to open system settings (simulated).");

    // Bitmap Button (Red Square)
    const BMP_WIDTH: u32 = 16;
    const BMP_HEIGHT: u32 = 16;
    let mut bmp_data = vec![0u8; (BMP_WIDTH * BMP_HEIGHT * 4) as usize];
    for y in 0..BMP_HEIGHT {
        for x in 0..BMP_WIDTH {
            let idx = ((y * BMP_WIDTH + x) * 4) as usize;
            if x < 2 || x >= BMP_WIDTH - 2 || y < 2 || y >= BMP_HEIGHT - 2 {
                bmp_data[idx + 3] = 0;
            } else {
                bmp_data[idx + 0] = 255;
                bmp_data[idx + 3] = 255;
            }
        }
    }
    let red_bitmap =
        Bitmap::from_rgba(&bmp_data, BMP_WIDTH, BMP_HEIGHT).expect("Failed to create test bitmap");
    let bitmap_button_label = StaticText::builder(&basic_panel)
        .with_label("Bitmap Button:")
        .build();
    let bitmap_button = BitmapButton::builder(&basic_panel)
        .with_id(ID_BITMAP_BTN_RED)
        .with_bitmap(&red_bitmap)
        .build();

    // Art Button
    let open_icon_bitmap = ArtProvider::get_bitmap(ART_FILE_OPEN, ART_BUTTON, None)
        .or_else(|| ArtProvider::get_bitmap(ART_ERROR, ART_BUTTON, None))
        .expect("Failed to get ART_FILE_OPEN or ART_ERROR icon");
    let art_button_label = StaticText::builder(&basic_panel)
        .with_label("Art Button:")
        .build();
    let art_button = BitmapButton::builder(&basic_panel)
        .with_id(ID_BITMAP_BTN_ART)
        .with_bitmap(&open_icon_bitmap)
        .build();

    // Activity Indicator Buttons
    let activity_label = StaticText::builder(&basic_panel)
        .with_label("Activity:")
        .build();
    let activity_indicator = ActivityIndicator::builder(&basic_panel)
        .with_id(ID_ACTIVITY_INDICATOR)
        .build();
    let activity_start_btn = Button::builder(&basic_panel)
        .with_id(ID_ACTIVITY_START_BTN)
        .with_label("Start")
        .build();
    let activity_stop_btn = Button::builder(&basic_panel)
        .with_id(ID_ACTIVITY_STOP_BTN)
        .with_label("Stop")
        .build();

    // Group 4: Pickers
    let colour_picker_label = StaticText::builder(&basic_panel)
        .with_label("Colour Picker:")
        .build();
    let colour_picker = ColourPickerCtrl::builder(&basic_panel)
        .with_id(122)
        .with_initial_colour(colours::RED)
        .build();
    let colour_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{:?}", colours::RED))
        .build();
    let date_picker_label_widget = StaticText::builder(&basic_panel)
        .with_label("Date Picker:")
        .build();
    let date_picker = DatePickerCtrl::builder(&basic_panel).with_id(123).build();
    let initial_selected_date = date_picker.get_value();
    let date_picker_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:04}-{:02}-{:02}",
            initial_selected_date.year,
            initial_selected_date.month + 1,
            initial_selected_date.day
        ))
        .build();
    let calendar_label_widget = StaticText::builder(&basic_panel)
        .with_label("Calendar:")
        .build();
    let calendar_ctrl = CalendarCtrl::builder(&basic_panel)
        .with_id(ID_CALENDAR_CTRL)
        .build();
    let initial_calendar_date = calendar_ctrl.get_date();
    let calendar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:04}-{:02}-{:02}",
            initial_calendar_date.year(),
            initial_calendar_date.month(),
            initial_calendar_date.day()
        ))
        .build();

    // Group 5: Other Controls
    let search_ctrl_label = StaticText::builder(&basic_panel)
        .with_label("Search Ctrl:")
        .build();
    let search_ctrl = SearchCtrl::builder(&basic_panel)
        .with_id(ID_SEARCH_CTRL)
        .with_value("Search...")
        .with_style(TE_PROCESS_ENTER)
        .build();
    search_ctrl.show_search_button(true);
    search_ctrl.show_cancel_button(true);
    let bitmap_combo_box_label = StaticText::builder(&basic_panel)
        .with_label("Bitmap Combo:")
        .build();
    let open_bmp = ArtProvider::get_bitmap(ART_FILE_OPEN, ART_MENU, None).expect("Failed art");
    let save_bmp = ArtProvider::get_bitmap(ART_FILE_SAVE, ART_MENU, None).expect("Failed art");
    let new_bmp = ArtProvider::get_bitmap(ART_NEW, ART_MENU, None).expect("Failed art");
    let bitmap_combo_box = BitmapComboBox::builder(Some(&basic_panel))
        .with_id(ID_BITMAP_COMBO_BOX)
        // .with_pos(Point::new(10, 40)) // Position handled by sizer
        .with_size(Size::new(200, -1)) // Explicit size
        .with_value("Default Value")
        .build();
    bitmap_combo_box.append("Open", Some(&open_bmp));
    bitmap_combo_box.append("Save", Some(&save_bmp));
    bitmap_combo_box.append("New", Some(&new_bmp));
    bitmap_combo_box.append("No Icon", None);
    bitmap_combo_box.set_selection(0);
    let hyperlink_label = StaticText::builder(&basic_panel)
        .with_label("Hyperlink:")
        .build();
    let hyperlink_ctrl = HyperlinkCtrl::builder(&basic_panel)
        .with_id(ID_HYPERLINK_CTRL)
        .with_label("wxWidgets Official Website")
        .with_url("https://www.wxwidgets.org")
        .build();

    // ADDED: ScrollBar
    let scrollbar_label = StaticText::builder(&basic_panel)
        .with_label("Scroll Bar:")
        .build();
    let scroll_bar = ScrollBar::builder(&basic_panel)
        .with_style(SB_HORIZONTAL)
        .build();
    scroll_bar.set_scrollbar(0, 10, 100, 10, true); // pos, thumb, range, page, refresh
    let scrollbar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{}", scroll_bar.thumb_position()))
        .build();

    // ADDED: Button to show MessageDialog
    let _show_msg_dialog_label = StaticText::builder(&basic_panel) // Prefixed with _
        .with_label("Message Dialog:")
        .build();
    let show_msg_dialog_btn = Button::builder(&basic_panel)
        .with_id(ID_SHOW_MESSAGE_DIALOG_BTN)
        .with_label("Show Info")
        .build();
    show_msg_dialog_btn.set_tooltip("Click to show an informational message dialog.");

    // --- Layout using Main Vertical BoxSizer and child FlexGridSizers ---
    let main_sizer = BoxSizer::builder(VERTICAL).build();
    let label_flags = ALIGN_RIGHT | ALIGN_CENTER_VERTICAL;
    let control_flags = EXPAND | ALIGN_CENTER_VERTICAL;

    // --- Group 1 in its own FlexGridSizer ---
    let grid_sizer_group1 = FlexGridSizer::builder(0, 2) // Use builder(rows, cols)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_group1.add_growable_col(0, 1); // Labels column, proportion 1
    grid_sizer_group1.add_growable_col(1, 3); // Controls column, proportion 3

    grid_sizer_group1.add(&static_text, 0, label_flags, 0);
    grid_sizer_group1.add(&text_ctrl, 1, control_flags, 0);

    grid_sizer_group1.add(&spin_button_label, 0, label_flags, 0);
    grid_sizer_group1.add(&spin_button, 0, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_group1.add(&spinctrl_double_label_widget, 0, label_flags, 0);
    let spin_double_sizer = BoxSizer::builder(HORIZONTAL).build();
    spin_double_sizer.add(&spinctrl_double, 0, ALIGN_CENTER_VERTICAL, 0);
    spin_double_sizer.add_spacer(5);
    spin_double_sizer.add(&spinctrl_double_status_label, 1, EXPAND, 0);
    grid_sizer_group1.add_sizer(&spin_double_sizer, 1, control_flags, 0);

    main_sizer.add_sizer(&grid_sizer_group1, 0, EXPAND | ALL, 10); // Add group 1 sizer to main

    // --- Static Line Separator ---
    let static_line_sep1 = StaticLine::builder(&basic_panel)
        .with_style(LI_HORIZONTAL)
        .build();
    main_sizer.add(&static_line_sep1, 0, EXPAND | ALL, 5); // Add line to main_sizer

    // --- Groups 2, 3, 4, 5 in another FlexGridSizer ---
    let grid_sizer_therest = FlexGridSizer::builder(0, 2) // Use builder(rows, cols)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_therest.add_growable_col(0, 1); // Labels column, proportion 1
    grid_sizer_therest.add_growable_col(1, 3); // Controls column, proportion 3

    // Group 2: CheckBox and RadioButtons
    grid_sizer_therest.add(&checkbox_label_widget, 0, label_flags, 0);
    grid_sizer_therest.add(&checkbox, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&radio_label, 0, label_flags, 0);
    let radio_button_sizer = BoxSizer::builder(HORIZONTAL).build();
    radio_button_sizer.add(&radio1, 0, ALIGN_CENTER_VERTICAL, 0);
    radio_button_sizer.add_spacer(5);
    radio_button_sizer.add(&radio2, 0, ALIGN_CENTER_VERTICAL, 0);
    radio_button_sizer.add_spacer(10);
    radio_button_sizer.add(&radio_status_label, 1, EXPAND, 0); // Removed ALIGN_CENTER_VERTICAL
    grid_sizer_therest.add_sizer(&radio_button_sizer, 1, control_flags, 0);

    grid_sizer_therest.add(&radio_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&radio_box, 1, control_flags, 0);

    // Group 3: Buttons
    grid_sizer_therest.add(&toggle_button_label, 0, label_flags, 0);
    let toggle_sizer = BoxSizer::builder(HORIZONTAL).build();
    toggle_sizer.add(&toggle_button, 0, ALIGN_CENTER_VERTICAL, 0);
    toggle_sizer.add_spacer(5);
    toggle_sizer.add(&toggle_status_label, 1, EXPAND, 0);
    grid_sizer_therest.add_sizer(&toggle_sizer, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&bitmap_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(&bitmap_button, 0, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&art_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(&art_button, 0, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&activity_label, 0, label_flags, 0);
    let activity_sizer = BoxSizer::builder(HORIZONTAL).build();
    activity_sizer.add(&activity_indicator, 0, ALIGN_CENTER_VERTICAL, 0);
    activity_sizer.add_spacer(5);
    activity_sizer.add(&activity_start_btn, 0, ALIGN_CENTER_VERTICAL, 5);
    activity_sizer.add_spacer(5);
    activity_sizer.add(&activity_stop_btn, 0, ALIGN_CENTER_VERTICAL, 5);
    grid_sizer_therest.add_sizer(&activity_sizer, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    // ADDED: ScrollBar to Group 3 (within grid_sizer_therest)
    grid_sizer_therest.add(&scrollbar_label, 0, label_flags, 0);
    let scrollbar_h_sizer = BoxSizer::builder(HORIZONTAL).build();
    scrollbar_h_sizer.add(&scroll_bar, 1, EXPAND, 0); // ScrollBar expands
    scrollbar_h_sizer.add_spacer(5);
    scrollbar_h_sizer.add(&scrollbar_status_label, 0, ALIGN_CENTER_VERTICAL, 0);
    grid_sizer_therest.add_sizer(&scrollbar_h_sizer, 1, control_flags, 0);

    // Group 4: Pickers
    grid_sizer_therest.add(&colour_picker_label, 0, label_flags, 0);
    let colour_sizer = BoxSizer::builder(HORIZONTAL).build();
    colour_sizer.add(&colour_picker, 0, ALIGN_CENTER_VERTICAL, 0);
    colour_sizer.add_spacer(5);
    colour_sizer.add(&colour_status_label, 1, EXPAND, 0);
    grid_sizer_therest.add_sizer(&colour_sizer, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&date_picker_label_widget, 0, label_flags, 0);
    let date_sizer = BoxSizer::builder(HORIZONTAL).build();
    date_sizer.add(&date_picker, 0, ALIGN_CENTER_VERTICAL, 0);
    date_sizer.add_spacer(5);
    date_sizer.add(&date_picker_status_label, 1, EXPAND, 0);
    grid_sizer_therest.add_sizer(&date_sizer, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    grid_sizer_therest.add(&calendar_label_widget, 0, label_flags, 0);
    let calendar_sizer = BoxSizer::builder(HORIZONTAL).build();
    calendar_sizer.add(&calendar_ctrl, 1, EXPAND, 0);
    calendar_sizer.add_spacer(5);
    calendar_sizer.add(&calendar_status_label, 1, EXPAND, 0);
    grid_sizer_therest.add_sizer(&calendar_sizer, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    // Group 5: Other Controls
    grid_sizer_therest.add(&search_ctrl_label, 0, label_flags, 0);
    grid_sizer_therest.add(&search_ctrl, 1, control_flags, 0);

    grid_sizer_therest.add(&bitmap_combo_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&bitmap_combo_box, 1, control_flags, 0);

    grid_sizer_therest.add(&hyperlink_label, 0, label_flags, 0);
    grid_sizer_therest.add(&hyperlink_ctrl, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);

    // ADDED: CommandLinkButton to grid_sizer_therest
    grid_sizer_therest.add(&cmd_link_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(&cmd_link_button, 1, control_flags, 0); // Expands

    // ADDED: MessageDialog button and label to grid_sizer_therest
    grid_sizer_therest.add(&_show_msg_dialog_label, 0, label_flags, 0);
    grid_sizer_therest.add(&show_msg_dialog_btn, 1, control_flags, 0); // Expands

    main_sizer.add_sizer(&grid_sizer_therest, 1, EXPAND | ALL, 10); // Add the rest of groups sizer to main, proportion 1 to take space

    basic_panel.set_sizer_and_fit(main_sizer, true);

    let basic_controls = BasicTabControls {
        panel: basic_panel.clone(),
        text_ctrl,
        spin_button,
        checkbox,
        radio1,
        radio2,
        radio_status_label: radio_status_label.clone(),
        toggle_button: toggle_button.clone(),
        toggle_status_label: toggle_status_label.clone(),
        bitmap_button,
        art_button,
        radio_box: radio_box.clone(),
        bitmap_combo_box: bitmap_combo_box.clone(),
        colour_picker: colour_picker.clone(),
        colour_label: colour_status_label.clone(), // Use status label
        date_picker: date_picker.clone(),
        date_picker_label: date_picker_status_label.clone(), // Use status label
        search_ctrl: search_ctrl.clone(),
        hyperlink_ctrl: hyperlink_ctrl.clone(),
        activity_indicator: activity_indicator.clone(),
        activity_start_btn: activity_start_btn.clone(),
        activity_stop_btn: activity_stop_btn.clone(),
        spinctrl_double: spinctrl_double.clone(),
        spinctrl_double_label: spinctrl_double_status_label.clone(), // Use status label
        calendar_ctrl: calendar_ctrl.clone(),
        calendar_label: calendar_status_label.clone(), // Use status label
        scroll_bar,                                    // ADDED
        scrollbar_status_label,                        // ADDED
        cmd_link_button: cmd_link_button.clone(),      // ADDED
    };

    // Event Handlers (Need to update clones if labels changed)
    let toggle_status_label_clone = basic_controls.toggle_status_label.clone();
    basic_controls.toggle_button.bind(
        EventType::COMMAND_TOGGLEBUTTON_CLICKED,
        move |event: Event| {
            let is_on = event.is_checked().unwrap_or(false);
            toggle_status_label_clone.set_label(if is_on { "ON" } else { "OFF" });
            println!("ToggleButton clicked, is_on: {}, Event: {:?}", is_on, event);
        },
    );

    let radio_box_clone = basic_controls.radio_box.clone();
    basic_controls
        .radio_box
        .bind(EventType::COMMAND_RADIOBOX_SELECTED, move |event: Event| {
            let selection = event.get_selection().unwrap_or(-1);
            let selection_string = if selection != -1 {
                radio_box_clone.get_string(selection)
            } else {
                "N/A".to_string()
            };
            println!(
                "RadioBox selected: {} ({}), Event: {:?}",
                selection_string, selection, event
            );
        });

    let bitmap_combo_box_clone = basic_controls.bitmap_combo_box.clone();
    basic_controls.bitmap_combo_box.bind(
        EventType::COMMAND_COMBOBOX_SELECTED,
        move |event: Event| {
            let selection_idx = event.get_selection().unwrap_or(-1);
            let mut log_output = format!("BitmapComboBox Event: {:?}", event);
            if selection_idx != -1 {
                let item_str = bitmap_combo_box_clone.get_string(selection_idx as u32);
                log_output.push_str(&format!(
                    ", Selected Item [{}]: '{}'",
                    selection_idx, item_str
                ));
                if let Some(bitmap) = bitmap_combo_box_clone.get_item_bitmap(selection_idx as u32) {
                    log_output.push_str(&format!(
                        ", Item has bitmap (w:{}, h:{})",
                        bitmap.get_width(),
                        bitmap.get_height()
                    ));
                } else {
                    log_output.push_str(", Item has no bitmap");
                }
            } else {
                let current_text_value = bitmap_combo_box_clone.get_value();
                log_output.push_str(&format!(
                    ", No item selection, text: '{}'",
                    current_text_value
                ));
            }
            println!("{}", log_output);
        },
    );

    let colour_label_clone = basic_controls.colour_label.clone();
    basic_controls
        .colour_picker
        .bind(EventType::COLOURPICKER_CHANGED, move |event: Event| {
            if let Some(colour) = event.get_colour() {
                let label_text = format!("{:?}", colour);
                colour_label_clone.set_label(&label_text);
                println!("ColourPicker changed: {:?}, Event: {:?}", colour, event);
            } else {
                println!("ColourPicker Event Error: No colour");
            }
        });

    let date_picker_clone = basic_controls.date_picker.clone();
    let date_picker_label_clone = basic_controls.date_picker_label.clone();
    basic_controls
        .date_picker
        .bind(EventType::DATE_CHANGED, move |event: Event| {
            let selected_date = date_picker_clone.get_value();
            let date_str = if selected_date.is_valid() {
                format!(
                    "{:04}-{:02}-{:02}",
                    selected_date.year,
                    selected_date.month + 1,
                    selected_date.day
                )
            } else {
                "(none)".to_string()
            };
            date_picker_label_clone.set_label(&date_str);
            println!("DatePicker changed: {}. Event: {:?}", date_str, event);
        });

    let hyperlink_ctrl_clone = basic_controls.hyperlink_ctrl.clone();
    basic_controls
        .hyperlink_ctrl
        .bind(EventType::COMMAND_HYPERLINK, move |event: Event| {
            let url = hyperlink_ctrl_clone.get_url();
            println!(
                "HYPERLINK Clicked! ID: {}, URL: \"{}\", Visited: {}. Event: {:?}",
                event.get_id(),
                url,
                hyperlink_ctrl_clone.get_visited(),
                event
            );
        });

    let indicator_clone_start = basic_controls.activity_indicator.clone();
    basic_controls.activity_start_btn.bind(
        EventType::COMMAND_BUTTON_CLICKED,
        move |_event: Event| {
            indicator_clone_start.start();
            println!(
                "ActivityIndicator started. Running: {}",
                indicator_clone_start.is_running()
            );
        },
    );

    let indicator_clone_stop = basic_controls.activity_indicator.clone();
    basic_controls.activity_stop_btn.bind(
        EventType::COMMAND_BUTTON_CLICKED,
        move |_event: Event| {
            indicator_clone_stop.stop();
            println!(
                "ActivityIndicator stopped. Running: {}",
                indicator_clone_stop.is_running()
            );
        },
    );

    let spinctrld_label_clone = basic_controls.spinctrl_double_label.clone();
    let spinctrld_clone_for_handler = basic_controls.spinctrl_double.clone();
    basic_controls
        .spinctrl_double
        .bind(EventType::SPINCTRLDOUBLE, move |event: Event| {
            let current_value = spinctrld_clone_for_handler.get_value();
            spinctrld_label_clone.set_label(&format!("{:.2}", current_value));
            println!(
                "SPINCTRLDOUBLE Event: ID: {}, Value: {:.2}",
                event.get_id(),
                current_value
            );
        });

    let calendar_label_clone = basic_controls.calendar_label.clone();
    let calendar_ctrl_clone = basic_controls.calendar_ctrl.clone();
    basic_controls
        .calendar_ctrl
        .bind(EventType::CALENDAR_SEL_CHANGED, move |event: Event| {
            let selected_date = calendar_ctrl_clone.get_date();
            let date_str = format!(
                "{:04}-{:02}-{:02}",
                selected_date.year(),
                selected_date.month(),
                selected_date.day()
            );
            calendar_label_clone.set_label(&date_str);
            println!(
                "CALENDAR_SEL_CHANGED: ID: {}, Date: {}-{}-{}",
                event.get_id(),
                selected_date.year(),
                selected_date.month(),
                selected_date.day()
            );
        });

    // ADDED: ScrollBar Event Handler
    // Use the clones here:
    let scroll_bar_clone = basic_controls.scroll_bar.clone();
    let scrollbar_status_label_clone = basic_controls.scrollbar_status_label.clone();
    basic_controls
        .scroll_bar
        .bind(EventType::SCROLL_THUMBTRACK, move |event: Event| {
            let pos = scroll_bar_clone.thumb_position();
            scrollbar_status_label_clone.set_label(&format!("{}", pos));
            event.skip(true); // Allow default processing too
        });

    let cmd_link_button_clone = basic_controls.cmd_link_button.clone();
    basic_controls.cmd_link_button.bind(
        EventType::COMMAND_BUTTON_CLICKED,
        move |_event: Event| {
            // Simplified event handler: removed FrameData and direct frame/status_bar access.
            println!("CommandLinkButton clicked. MainLabel: '{}', Note: (not directly gettable from API yet)", cmd_link_button_clone.get_label());
        },
    );

    // Bind event for the message dialog button
    let basic_panel_clone_for_dialog = basic_panel.clone(); // Clone for use in closure
    show_msg_dialog_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event| {
        println!("Show Message Dialog button clicked via dedicated button.");
        let dialog = MessageDialog::builder(
            Some(&basic_panel_clone_for_dialog),
            "This is a wxMessageDialog shown from a dedicated button click.",
            "wxDragon Message",
        )
        .with_style(OK | ICON_INFORMATION | CENTRE)
        .build();
        let result = dialog.show_modal();
        println!("MessageDialog closed with result: {}", result);
        match result {
            id::ID_OK => println!("User pressed OK"),
            id::ID_CANCEL => println!("User pressed Cancel"),
            id::ID_YES => println!("User pressed Yes"),
            id::ID_NO => println!("User pressed No"),
            _ => println!("Dialog closed with other code: {}", result),
        }
    });

    basic_controls
}

fn create_lists_tab(notebook: &Notebook) -> ListsTabControls {
    // Create the ScrolledWindow as the main container for this tab
    let scrolled_list_window = ScrolledWindow::builder(notebook)
        .with_style(TAB_TRAVERSAL) // Styles like TAB_TRAVERSAL apply here
        .build();

    // Create an inner Panel *inside* the ScrolledWindow to hold the content
    let inner_list_panel = Panel::builder(&scrolled_list_window)
        // No specific style needed here unless desired
        .build();

    // --- Create controls, parenting them to the *inner_list_panel* ---
    let list_box_items = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Fig",
        "Grape",
        "Honeydew",
        "Kiwi",
        "Lemon",
        "Lime",
        "Mango",
        "Nectarine",
        "Orange",
        "Papaya",
        "Peach",
        "Pear",
        "Plum",
        "Raspberry",
        "Strawberry",
        "Watermelon",
        "A very long item name indeed to test horizontal scrolling",
    ]; // More items
    let list_box = ListBox::builder(&inner_list_panel)
        .with_choices(&list_box_items)
        .with_style(LB_SINGLE | LB_SORT | LB_ALWAYS_SB | LB_HSCROLL)
        .build();
    let listbox_status_label = StaticText::builder(&inner_list_panel)
        .with_label("List Selection: None")
        .build();
    let checklistbox = CheckListBox::builder(&inner_list_panel)
        .with_id(109)
        .with_choices(&[
            "Option A", "Option B", "Option C", "Option D", "Option E", "Option F", "Option G",
        ])
        .with_style(LB_SORT)
        .build();
    checklistbox.check(1, true);
    let checklistbox_status_label = StaticText::builder(&inner_list_panel)
        .with_label("CheckList Status: B (Checked)")
        .build();
    let choice_items = [
        "Red", "Green", "Blue", "Yellow", "Purple", "Orange", "Cyan", "Magenta",
    ];
    let choice_ctrl = Choice::builder(&inner_list_panel)
        .with_choices(&choice_items)
        .with_style(CB_SORT)
        .build();
    choice_ctrl.set_selection(0);
    let choice_status_label = StaticText::builder(&inner_list_panel)
        .with_label("Choice Selection: Red")
        .build();
    let combo_items = [
        "Cabbage", "Carrot", "Cucumber", "Celery", "Broccoli", "Spinach", "Kale", "Lettuce",
    ];
    let combo_box = ComboBox::builder(&inner_list_panel)
        .with_choices(&combo_items)
        .with_value("Initial Combo")
        .with_style(CB_SORT)
        .build();
    let combo_status_label = StaticText::builder(&inner_list_panel)
        .with_label("Combo Status: Initial Combo")
        .build();

    // --- ADDED: ListCtrl Example ---
    let list_ctrl = ListCtrl::builder(&inner_list_panel)
        .with_id(ID_LIST_CTRL)
        .with_style(LC_REPORT | LC_SINGLE_SEL | LC_HRULES | LC_VRULES) // Report style, single selection, rules
        .build();

    // Add columns
    list_ctrl.insert_column(0, "ID", LIST_FORMAT_RIGHT, 60);
    list_ctrl.insert_column(1, "Description", LIST_FORMAT_LEFT, 150);
    list_ctrl.insert_column(2, "Quantity", LIST_FORMAT_RIGHT, 100);
    list_ctrl.insert_column(3, "Notes", LIST_FORMAT_LEFT, -1); // Fill remaining space

    // Insert items and get their indices. This sets the text for column 0.
    let _item1_idx = list_ctrl.insert_item(0, "P001");
    let _item2_idx = list_ctrl.insert_item(1, "P002");
    let _item3_idx = list_ctrl.insert_item(2, "P003");

    // To set text for other columns (e.g., 1 and 2), a method like
    // list_ctrl.set_item_column_text(item_idx, col_idx, text) would be needed.
    // The current set_item_text(item_idx, text) only affects column 0.
    // Example of what would be needed:
    // list_ctrl.set_item_column_text(item1_idx as i64, 1, "Apples");
    // list_ctrl.set_item_column_text(item1_idx as i64, 2, "50");
    // list_ctrl.set_item_column_text(item2_idx as i64, 1, "Bananas");
    // list_ctrl.set_item_column_text(item2_idx as i64, 2, "120");
    // list_ctrl.set_item_column_text(item3_idx as i64, 1, "Oranges");
    // list_ctrl.set_item_column_text(item3_idx as i64, 2, "75");

    // Set item data (optional)
    // list_ctrl.set_item_data(item1_idx, 101);
    // list_ctrl.set_item_data(item2_idx, 102);
    // list_ctrl.set_item_data(item3_idx, 103);

    let list_ctrl_status_label = StaticText::builder(&inner_list_panel)
        .with_label("ListCtrl Status: None")
        .build();
    // --- END ListCtrl Example ---

    // --- Sizer for *inner_list_panel* ---
    let list_sizer_main = BoxSizer::builder(VERTICAL).build();
    let list_row_sizer = BoxSizer::builder(HORIZONTAL).build();
    let list_box_col = BoxSizer::builder(VERTICAL).build();
    list_box_col.add(&list_box, 1, EXPAND | ALL, 5);
    list_box_col.add(&listbox_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    list_row_sizer.add_sizer(&list_box_col, 1, EXPAND | ALL, 5);
    let check_list_col = BoxSizer::builder(VERTICAL).build();
    check_list_col.add(&checklistbox, 1, EXPAND | ALL, 5);
    check_list_col.add(
        &checklistbox_status_label,
        0,
        ALIGN_CENTER_HORIZONTAL | ALL,
        5,
    );
    list_row_sizer.add_sizer(&check_list_col, 1, EXPAND | ALL, 5);
    list_sizer_main.add_sizer(&list_row_sizer, 1, EXPAND | ALL, 5);

    let choice_row_sizer = BoxSizer::builder(HORIZONTAL).build();
    let choice_col = BoxSizer::builder(VERTICAL).build();
    choice_col.add(&choice_ctrl, 0, ALL, 5);
    choice_col.add(&choice_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    choice_row_sizer.add_sizer(&choice_col, 1, EXPAND | ALL, 5);

    let combo_col = BoxSizer::builder(VERTICAL).build();
    combo_col.add(&combo_box, 0, ALL, 5);
    combo_col.add(&combo_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    choice_row_sizer.add_sizer(&combo_col, 1, EXPAND | ALL, 5);

    // ADDED: Add the choice/combo row sizer to the main vertical sizer
    list_sizer_main.add_sizer(&choice_row_sizer, 0, EXPAND | ALL, 5);

    // Add ListCtrl and its status label
    let list_ctrl_col_sizer = BoxSizer::builder(VERTICAL).build();
    list_ctrl_col_sizer.add(&list_ctrl, 1, EXPAND | ALL, 5); // ListCtrl takes available space
    list_ctrl_col_sizer.add(&list_ctrl_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    list_sizer_main.add_sizer(&list_ctrl_col_sizer, 1, EXPAND | ALL, 5); // Add ListCtrl sizer to main, taking space

    inner_list_panel.set_sizer(list_sizer_main, true);
    // Fit the inner panel to its contents initially
    inner_list_panel.fit();

    // --- Configure the ScrolledWindow ---
    // Calculate virtual size needed based on the inner panel's best size
    let inner_size = inner_list_panel.get_best_size();
    // Set scroll rate (pixels per scroll unit)
    scrolled_list_window.set_scroll_rate(10, 10);
    // Set scrollbars based on inner panel size (make virtual size a bit larger to ensure scrolling)
    scrolled_list_window.set_scrollbars(
        10,                            // pixels per unit X
        10,                            // pixels per unit Y
        (inner_size.width + 50) / 10,  // number of units X (ensure > visible width)
        (inner_size.height + 50) / 10, // number of units Y (ensure > visible height)
        0,                             // initial X position
        0,                             // initial Y position
        true,                          // no refresh immediately
    );
    // Optional: Enable only vertical scrolling if desired
    // scrolled_list_window.enable_scrolling(false, true);

    // Return the controls, using the scrolled_list_window as the main panel
    ListsTabControls {
        panel: scrolled_list_window,
        list_box,
        listbox_status_label,
        checklistbox,
        checklistbox_status_label,
        choice_ctrl,
        choice_status_label,
        combo_box,
        combo_status_label,
        list_ctrl,              // ADDED
        list_ctrl_status_label, // ADDED
    }
}

fn create_advanced_tab(notebook: &Notebook) -> (SplitterWindow, AdvancedTabControls) {
    // Create a SplitterWindow instead of a Panel for this tab's main container
    let splitter = SplitterWindow::builder(notebook)
        .with_id(200) // Give splitter an ID
        .with_style(SP_LIVE_UPDATE | SP_BORDER | SP_3D) // Added SP_3D style
        .build();

    // Create Panel 1 (Left: Tree)
    let tree_panel = Panel::builder(&splitter).build();
    let tree_ctrl = TreeCtrl::builder(&tree_panel)
        .with_id(111)
        .with_style(TR_DEFAULT_STYLE | TR_HAS_BUTTONS | TR_LINES_AT_ROOT)
        .build();
    if let Some(root_id) = tree_ctrl.add_root("Root Node") {
        if let Some(child1_id) = tree_ctrl.append_item(&root_id, "Child 1") {
            tree_ctrl.append_item(&child1_id, "Grandchild 1.1");
            tree_ctrl.append_item(&child1_id, "Grandchild 1.2");
        }
        tree_ctrl.append_item(&root_id, "Child 2");
    }
    let tree_status_label = StaticText::builder(&tree_panel)
        .with_label("Tree Selection: None")
        .build();

    // Sizer for Tree Panel
    let tree_sizer = BoxSizer::builder(VERTICAL).build();
    tree_sizer.add(&tree_ctrl, 1, EXPAND | ALL, 5);
    tree_sizer.add(&tree_status_label, 0, EXPAND | ALL, 5); // Expand label horizontally
    tree_panel.set_sizer(tree_sizer, true);

    // Create Panel 2 (Right: Gauge, Slider, Spin)
    let controls_panel = Panel::builder(&splitter).build();
    let gauge = Gauge::builder(&controls_panel)
        .with_id(112)
        .with_range(100)
        .with_size(200, 25) // Pass width and height directly
        .with_style(GA_HORIZONTAL | GA_SMOOTH)
        .build();
    gauge.set_value(25);
    let gauge_increase_btn = Button::builder(&controls_panel)
        .with_label("Increase")
        .build();
    let gauge_reset_btn = Button::builder(&controls_panel).with_label("Reset").build();
    let gauge_status_label = StaticText::builder(&controls_panel)
        .with_label("Gauge Value: 25%")
        .build();
    let slider_label = StaticText::builder(&controls_panel)
        .with_label("Slider Value: 50")
        .build();
    let slider = Slider::builder(&controls_panel)
        .with_id(113)
        .with_value(50)
        .with_min_value(0)
        .with_max_value(200)
        .with_style(SL_HORIZONTAL | SL_LABELS)
        .with_size(Size::new(-1, -1))
        .build(); // Let slider expand
    let spin_ctrl_label = StaticText::builder(&controls_panel)
        .with_label("Spin Value: 10")
        .build();
    let spin_ctrl = SpinCtrl::builder(&controls_panel)
        .with_id(114)
        .with_range(0, 50)
        .with_initial_value(10)
        .with_style(SP_ARROW_KEYS | SP_WRAP)
        .with_size(Size::new(80, -1))
        .build();

    // Sizer for Controls Panel
    let controls_sizer = BoxSizer::builder(VERTICAL).build();

    let gauge_sizer = BoxSizer::builder(HORIZONTAL).build();
    gauge_sizer.add(&gauge, 1, EXPAND | ALL, 5);
    let gauge_buttons_sizer = BoxSizer::builder(VERTICAL).build();
    gauge_buttons_sizer.add(&gauge_increase_btn, 0, ALL, 2);
    gauge_buttons_sizer.add(&gauge_reset_btn, 0, ALL, 2);
    gauge_sizer.add_sizer(&gauge_buttons_sizer, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    gauge_sizer.add(&gauge_status_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    controls_sizer.add_sizer(&gauge_sizer, 0, EXPAND | ALL, 5);

    let slider_spin_sizer = BoxSizer::builder(HORIZONTAL).build();
    slider_spin_sizer.add(&slider, 1, EXPAND | ALL, 5);
    slider_spin_sizer.add(&slider_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    slider_spin_sizer.add_spacer(20);
    slider_spin_sizer.add(&spin_ctrl, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    slider_spin_sizer.add(&spin_ctrl_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    controls_sizer.add_sizer(&slider_spin_sizer, 0, EXPAND | ALL, 5);

    controls_panel.set_sizer(controls_sizer, true);

    // Split the window
    splitter.split_vertically(&tree_panel, &controls_panel, 150); // 150 pixels for the tree initially
    splitter.set_minimum_pane_size(50); // Set minimum size for both panes

    // Return the splitter AND the controls struct
    (
        splitter,
        AdvancedTabControls {
            tree_ctrl,
            tree_status_label,
            gauge,
            gauge_increase_btn,
            gauge_reset_btn,
            gauge_status_label,
            slider,
            slider_label,
            spin_ctrl,
            spin_ctrl_label,
        },
    )
}

fn create_data_tab(notebook: &Notebook) -> DataTabControls {
    let data_panel = Panel::builder(notebook).with_style(TAB_TRAVERSAL).build();
    let button = Button::builder(&data_panel)
        .with_label("Click Me & Update Data!")
        .build();
    let data_display_label = StaticText::builder(&data_panel)
        .with_label("Frame Data: Clicks=0, Msg=Initial Message")
        .build();

    // Sizer for Data Panel
    let data_sizer = BoxSizer::builder(VERTICAL).build();
    data_sizer.add(&button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 15);
    data_sizer.add(&data_display_label, 0, ALL | ALIGN_CENTER_HORIZONTAL, 15);
    data_panel.set_sizer(data_sizer, true);

    DataTabControls {
        panel: data_panel,
        button,
        data_display_label,
    }
}

// ADDED: Function to create the Treebook Tab
fn create_book_controls_tab(notebook: &Notebook) -> BookControlsTab {
    let tab_panel = Panel::builder(notebook).with_style(TAB_TRAVERSAL).build();

    let treebook = Treebook::builder(&tab_panel)
        .with_id(ID_HIGHEST + 20) // Example ID
        .build();

    // Page 1: Info Page
    let info_page_panel = Panel::builder(&treebook).build();
    let info_label = StaticText::builder(&info_page_panel)
        .with_label("This is the Treebook's information page.")
        .build();
    let info_button = Button::builder(&info_page_panel)
        .with_label("Info Button")
        .build();
    let info_page_sizer = BoxSizer::builder(VERTICAL).build();
    info_page_sizer.add(&info_label, 0, ALL | EXPAND, 10);
    info_page_sizer.add(&info_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    info_page_panel.set_sizer(info_page_sizer, true);
    info_page_panel.fit();
    treebook.add_page(&info_page_panel, "Info", true, -1);

    // Page 2: Settings Page
    let settings_page_panel = Panel::builder(&treebook).build();
    let settings_label = StaticText::builder(&settings_page_panel)
        .with_label("Treebook settings would go here.")
        .build();
    let settings_button = Button::builder(&settings_page_panel)
        .with_label("Settings Button")
        .build();
    let settings_page_sizer = BoxSizer::builder(VERTICAL).build();
    settings_page_sizer.add(&settings_label, 0, ALL | EXPAND, 10);
    settings_page_sizer.add(&settings_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    settings_page_panel.set_sizer(settings_page_sizer, true);
    settings_page_panel.fit();
    let _settings_page_index = treebook.add_page(&settings_page_panel, "Settings", false, -1);

    // Sub-Page for Settings Page
    let advanced_settings_panel = Panel::builder(&treebook).build();
    let advanced_label = StaticText::builder(&advanced_settings_panel)
        .with_label("Advanced Treebook settings.")
        .build();
    let advanced_button = Button::builder(&advanced_settings_panel)
        .with_label("Advanced Button")
        .build();
    let advanced_sizer = BoxSizer::builder(VERTICAL).build();
    advanced_sizer.add(&advanced_label, 0, ALL | EXPAND, 10);
    advanced_sizer.add(&advanced_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    advanced_settings_panel.set_sizer(advanced_sizer, true);
    advanced_settings_panel.fit();
    treebook.add_sub_page(&advanced_settings_panel, "Advanced", false, -1);

    // Sizer for the main tab panel, to make the Treebook expand
    let main_tab_sizer = BoxSizer::builder(VERTICAL).build();
    main_tab_sizer.add(&treebook, 1, EXPAND | ALL, 5);

    // Example of a StaticBitmap using include_bytes!
    let hbox_bitmap_example = BoxSizer::builder(HORIZONTAL).build();
    let image_bytes = include_bytes!("../asset/simple.png"); // Path relative to this main.rs file
    match image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png) {
        Ok(img) => {
            let rgba_data = img.to_rgba8();
            let (width, height) = img.dimensions();
            if let Some(bitmap_obj) = Bitmap::from_rgba(rgba_data.as_raw(), width, height) {
                if let Some(static_bitmap) = StaticBitmap::builder(&tab_panel)
                    .with_bitmap(bitmap_obj) // Use with_bitmap
                    .with_size(Size::new(width as i32, height as i32))
                    .build()
                {
                    let bmp_label = StaticText::builder(&tab_panel)
                        .with_label("StaticBitmap (from bytes):")
                        .build();
                    hbox_bitmap_example.add(&bmp_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                    hbox_bitmap_example.add(&static_bitmap, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                } else {
                    println!("Failed to create StaticBitmap from Bitmap object.");
                    let bmp_error_label = StaticText::builder(&tab_panel)
                        .with_label("StaticBitmap: Error creating from Bitmap obj")
                        .build();
                    hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                }
            } else {
                println!("Failed to create wxdragon::Bitmap from RGBA data.");
                let bmp_error_label = StaticText::builder(&tab_panel)
                    .with_label("StaticBitmap: Error creating wxBitmap from RGBA")
                    .build();
                hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
            }
        }
        Err(e) => {
            println!("Failed to decode PNG from memory: {}", e);
            let bmp_error_label = StaticText::builder(&tab_panel)
                .with_label("StaticBitmap: Error decoding PNG")
                .build();
            hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
        }
    }
    main_tab_sizer.add_sizer(&hbox_bitmap_example, 0, ALIGN_LEFT | ALL, 5);

    tab_panel.set_sizer(main_tab_sizer, true);
    tab_panel.fit();

    BookControlsTab {
        tab_panel,
        treebook,
    }
}

// --- Main Application Logic ---

fn main() {
    // Call the safe entry point, passing the initialization logic as a closure.
    // The closure now receives a `&mut WxdAppHandle`.
    // Keep wxdragon::main call for clarity / avoid name clash
    wxdragon::main(|handle: &mut WxdAppHandle| {
        // WxdAppHandle type should be found via prelude
        // Use types re-exported in lib.rs or via prelude
        let frame = Frame::builder()
            .with_title("wxDragon Notebook Example")
            .with_size(Size::new(800, 900)) // Adjusted size
            .build();

        // --- Menu Bar ---
        let file_menu = Menu::builder()
            .append_item(ID_EXIT, "E&xit\tAlt-X", "Quit this program") // Use standard ID_EXIT
            .build();
        let help_menu = Menu::builder()
            .append_item(ID_ABOUT, "&About...", "Show about dialog") // Use standard ID_ABOUT
            .build();
        let menubar = MenuBar::builder()
            .append(file_menu, "&File")
            .append(help_menu, "&Help")
            .build();
        frame.set_menu_bar(menubar);

        // --- Status Bar ---
        let status_bar = StatusBar::builder(&frame)
            .with_fields_count(3)
            .with_status_widths(vec![-1, 150, 100]) // Field 0 flexible, Field 1 150px, Field 2 100px
            .add_initial_text(0, "Ready")
            .add_initial_text(1, "Center Field")
            .add_initial_text(2, "Right Field")
            .build(); // build() creates and attaches to frame

        // --- User Data Setup ---
        let frame_user_data = RefCell::new(FrameData {
            click_count: 0,
            message: "Initial Message".to_string(),
        });
        frame.set_user_data(Box::new(frame_user_data));

        // --- Create the Notebook ---
        let notebook = Notebook::builder(&frame)
            .with_id(120) // Give notebook an ID
            // .with_style(NB_BOTTOM) // Example: Change tab position
            .build();

        // Create Tabs by calling functions
        let basic_controls = create_basic_tab(&notebook);
        let list_controls = create_lists_tab(&notebook);
        // Destructure the return tuple
        let (advanced_splitter, advanced_controls) = create_advanced_tab(&notebook);
        let data_controls = create_data_tab(&notebook);
        let book_controls = create_book_controls_tab(&notebook);

        // --- ToolBar Setup ---
        let tb_style = TB_TEXT | TB_HORIZONTAL;
        // Use frame.create_tool_bar now
        if let Some(toolbar) = frame.create_tool_bar(tb_style, wxdragon::id::ID_ANY as i32) {
            // Corrected to ID_ANY
            // Fetch Bitmaps (using ART_TOOLBAR client id)
            let new_bmp = ArtProvider::get_bitmap(ART_NEW, ART_TOOLBAR, None)
                .expect("Failed to get ART_NEW icon");
            let open_bmp = ArtProvider::get_bitmap(ART_FILE_OPEN, ART_TOOLBAR, None)
                .expect("Failed to get ART_FILE_OPEN icon");
            let save_bmp = ArtProvider::get_bitmap(ART_FILE_SAVE, ART_TOOLBAR, None)
                .expect("Failed to get ART_FILE_SAVE icon");
            let exit_bmp = ArtProvider::get_bitmap(ART_QUIT, ART_TOOLBAR, None)
                .expect("Failed to get ART_QUIT icon");

            // Add tools
            toolbar.add_tool(ID_TOOL_NEW, "New", &new_bmp, "Create a new file");
            toolbar.add_tool(ID_TOOL_OPEN, "Open", &open_bmp, "Open an existing file");
            toolbar.add_tool(ID_TOOL_SAVE, "Save", &save_bmp, "Save the current file");
            toolbar.add_separator();
            toolbar.add_tool(ID_EXIT, "Exit", &exit_bmp, "Exit the application"); // Use standard ID_EXIT

            // Explicit Realize call is necessary for visibility on macOS, even with CreateToolBar.
            toolbar.realize();

            // No need to call frame.set_tool_bar() as CreateToolBar handles it.
        } else {
            println!("ERROR: Failed to create toolbar using frame.create_tool_bar()");
        }

        // Add pages to notebook (Use the panel field from the returned structs)
        notebook.add_page(&basic_controls.panel, "Basic", true);
        notebook.add_page(&list_controls.panel, "Lists", false);
        // Add the returned splitter directly
        notebook.add_page(&advanced_splitter, "Advanced", false);
        notebook.add_page(&data_controls.panel, "Data", false);
        notebook.add_page(&book_controls.tab_panel, "Book Controls", false);

        // Set Frame Sizer
        let main_sizer = BoxSizer::builder(VERTICAL).build();
        // Add the NOTEBOOK to the frame sizer, with minimal border
        main_sizer.add(&notebook, 1, EXPAND | ALL, 1);
        frame.set_sizer(main_sizer, true);

        // --- Bind event handlers ---
        // Cloning needs adjustment if controls/labels are now on different panels

        // Basic Panel Clones (Update based on create_basic_tab changes)
        let radio_status_label_clone = basic_controls.radio_status_label.clone();
        let radio1_clone = basic_controls.radio1.clone();
        let text_ctrl_clone_for_spin = basic_controls.text_ctrl.clone(); // For spin button

        // List Panel Clones
        let listbox_status_label_clone = list_controls.listbox_status_label.clone();
        let list_box_clone = list_controls.list_box.clone();
        let choice_status_label_clone = list_controls.choice_status_label.clone();
        let choice_ctrl_clone = list_controls.choice_ctrl.clone();
        let combo_status_label_clone = list_controls.combo_status_label.clone();
        let combo_box_clone = list_controls.combo_box.clone();
        let checklistbox_status_label_clone = list_controls.checklistbox_status_label.clone();
        let checklistbox_clone = list_controls.checklistbox.clone();
        let list_ctrl_status_label_clone = list_controls.list_ctrl_status_label.clone();
        let list_ctrl_clone = list_controls.list_ctrl.clone(); // Clone for use in closure

        // Advanced Panel Clones
        let tree_status_label_clone = advanced_controls.tree_status_label.clone();
        let gauge_clone_for_inc = advanced_controls.gauge.clone();
        let gauge_status_label_clone_for_inc = advanced_controls.gauge_status_label.clone();
        let gauge_clone_for_reset = advanced_controls.gauge.clone();
        let gauge_status_label_clone_for_reset = advanced_controls.gauge_status_label.clone();
        let slider_label_clone = advanced_controls.slider_label.clone();

        // Data Panel Clones
        let frame_clone_for_button = frame.clone(); // Need frame for data access
        let data_display_label_clone = data_controls.data_display_label.clone(); // Label on data panel
        let status_bar_clone_for_data = status_bar.clone(); // Clone status bar for data button
        let status_bar_clone_for_list = status_bar.clone(); // Clone status bar for listbox

        // Menu Event Handlers (Bind to frame - remains the same)
        let frame_clone_for_exit = frame.clone();
        frame.bind(EventType::MENU, move |event: Event| {
            match event.get_id() {
                id if id == ID_EXIT => {
                    println!("Menu/Toolbar: Exit clicked!");
                    frame_clone_for_exit.close();
                }
                id if id == ID_ABOUT => {
                    println!("Menu: About clicked!");
                }
                // ADDED: Handle ToolBar events using constants
                id if id == ID_TOOL_NEW => {
                    println!("Toolbar: New clicked!");
                }
                id if id == ID_TOOL_OPEN => {
                    println!("Toolbar: Open clicked!");
                }
                id if id == ID_TOOL_SAVE => {
                    println!("Toolbar: Save clicked!");
                }
                // REMOVED: Explicit ID_TOOL_EXIT case, handled by ID_EXIT above
                _ => {
                    println!("Unhandled Menu/Tool ID: {}", event.get_id());
                    event.skip(true);
                }
            }
        });

        // Button click event (Data Panel)
        data_controls
            .button
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |event: Event| {
                println!("Data Panel Button clicked! ID: {}", event.get_id());
                let mut updated_label_text = String::new();
                let mut current_click_count = 0; // Variable to store count
                let modified =
                    frame_clone_for_button.with_borrowed_data_mut::<FrameData, _, _>(|data| {
                        data.click_count += 1;
                        data.message = format!("Button clicked {} times!", data.click_count);
                        updated_label_text = format!(
                            "Frame Data: Clicks={}, Msg={}",
                            data.click_count, data.message
                        );
                        current_click_count = data.click_count; // Store count here
                    });
                if modified {
                    data_display_label_clone.set_label(&updated_label_text);
                    status_bar_clone_for_data
                        .set_status_text(&format!("Clicks: {}", current_click_count), 0);
                    // Use stored count
                }
                event.skip(false);
            });

        // BitmapButton click event (Basic Panel)
        basic_controls
            .bitmap_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, |event: Event| {
                println!("Bitmap Button clicked: {}", event.get_id());
            });

        // ArtProvider BitmapButton click event (Basic Panel)
        basic_controls
            .art_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, |event: Event| {
                // Now print ID even if it's wxID_ANY (-1)
                println!(
                    "ArtProvider Button (ID: {}) clicked! Icon from ArtProvider works.",
                    event.get_id()
                );
            });

        // Checkbox toggle event (Basic Panel)
        basic_controls
            .checkbox
            .bind(EventType::CHECKBOX, |event: Event| {
                println!("Checkbox: {}", event.is_checked().unwrap_or(false));
            });

        // TextCtrl text change event (Basic Panel)
        basic_controls
            .text_ctrl
            .bind(EventType::TEXT, |_event: Event| {
                // println!("Text: {}", _event.get_string().unwrap_or_default());
            });

        // TextCtrl Enter key event (Basic Panel)
        basic_controls.text_ctrl.bind(EventType::TEXT_ENTER, {
            let spin_button_clone = basic_controls.spin_button.clone();
            let text_ctrl_clone = basic_controls.text_ctrl.clone();
            move |event: Event| {
                if let Some(text) = event.get_string() {
                    if let Ok(num) = text.parse::<i32>() {
                        let min = spin_button_clone.min();
                        let max = spin_button_clone.max();
                        spin_button_clone.set_value(num.clamp(min, max));
                    } else {
                        text_ctrl_clone.set_value(&spin_button_clone.value().to_string());
                    }
                }
                event.skip(false);
            }
        });

        // Radio button selected event (Basic Panel - bind to both)
        let radio_status_update = move |_event: Event| {
            let selected_option = if radio1_clone.get_value() {
                "Option 1"
            } else {
                "Option 2"
            };
            radio_status_label_clone.set_label(&format!("Selected: {}", selected_option));
        };
        basic_controls.radio1.bind(
            EventType::COMMAND_RADIOBUTTON_SELECTED,
            radio_status_update.clone(),
        );
        basic_controls
            .radio2
            .bind(EventType::COMMAND_RADIOBUTTON_SELECTED, radio_status_update);

        // ToggleButton click event (Basic Panel)
        // This is an older, duplicated event handler section. The one above in create_basic_tab is active.
        // I will remove this duplicated section to avoid confusion and potential bugs.
        /*
        let toggle_button_clone = basic_controls.toggle_button.clone();
        let toggle_status_label_clone = basic_controls.toggle_status_label.clone(); // This was the source of the dead_code warning if it was this one
        basic_controls.toggle_button.bind(EventType::COMMAND_TOGGLEBUTTON_CLICKED, move |event: Event| {
            let is_on = event.is_checked().unwrap_or(false);
            println!("ToggleButton clicked, is_on: {}, Event: {:?}", is_on, event);
        });
        */

        // SpinButton Event Bindings (Basic Panel)
        basic_controls.spin_button.bind(EventType::SPIN, {
            let text_ctrl_clone = text_ctrl_clone_for_spin.clone();
            move |event: Event| {
                if let Some(value) = event.get_int() {
                    text_ctrl_clone.set_value(&value.to_string());
                }
            }
        });

        // ListBox selection event (List Panel)
        list_box_clone.bind(EventType::COMMAND_LISTBOX_SELECTED, {
            let list_box = list_box_clone.clone();
            let status_label = listbox_status_label_clone.clone();
            move |_event: Event| {
                if let Some(selected_string) = list_box.get_string_selection() {
                    let status_text = format!("Selected: {}", selected_string);
                    status_label.set_label(&status_text);
                    status_bar_clone_for_list.set_status_text(&status_text, 0); // Update status bar too
                } else {
                    status_label.set_label("List Selection: None");
                    status_bar_clone_for_list.set_status_text("Ready", 0); // Reset status bar
                }
            }
        });

        // Choice selection event (List Panel)
        choice_ctrl_clone.bind(EventType::COMMAND_CHOICE_SELECTED, {
            let status_label = choice_status_label_clone.clone();
            let choice_ctrl = choice_ctrl_clone.clone();
            move |_event: Event| {
                if let Some(selected_string) = choice_ctrl.get_string_selection() {
                    status_label.set_label(&format!("Choice Selection: {}", selected_string));
                } else {
                    status_label.set_label("Choice Selection: None");
                }
            }
        });

        // ComboBox events (List Panel)
        combo_box_clone.bind(EventType::COMMAND_COMBOBOX_SELECTED, {
            let status_label = combo_status_label_clone.clone();
            let combo = combo_box_clone.clone();
            move |_event: Event| {
                if let Some(selected_string) = combo.get_string_selection() {
                    status_label.set_label(&format!("Combo Selected: {}", selected_string));
                } else {
                    status_label.set_label("Combo Selected: None");
                }
            }
        });
        combo_box_clone.bind(EventType::TEXT, {
            let status_label = combo_status_label_clone.clone();
            let combo = combo_box_clone.clone();
            move |_event: Event| {
                let current_text = combo.get_value();
                status_label.set_label(&format!("Combo Text: {}", current_text));
            }
        });
        combo_box_clone.bind(EventType::TEXT_ENTER, {
            let combo = combo_box_clone.clone();
            move |event: Event| {
                let current_text = combo.get_value();
                println!("ComboBox Enter: {}", current_text);
                event.skip(false);
            }
        });

        // CheckListBox selection/check event (List Panel)
        checklistbox_clone.bind(EventType::COMMAND_CHECKLISTBOX_SELECTED, {
            let checklistbox = checklistbox_clone.clone();
            let status_label = checklistbox_status_label_clone.clone();
            move |_event: Event| {
                if let Some(index) = checklistbox.get_selection() {
                    // TODO: Needs GetInt
                    let is_checked = checklistbox.is_checked(index);
                    let item_text = checklistbox
                        .get_string(index)
                        .unwrap_or_else(|| "?".to_string());
                    status_label.set_label(&format!(
                        "CheckList Sel: {} ('{}' {})",
                        index,
                        item_text,
                        if is_checked { "Checked" } else { "Unchecked" }
                    ));
                } else {
                    status_label.set_label("CheckList Sel: None");
                }
            }
        });

        // TreeCtrl Selection Changed event (Advanced Panel)
        advanced_controls
            .tree_ctrl
            .bind(EventType::TREE_SEL_CHANGED, move |event: Event| {
                if let Some(selected_item) = event.get_item() {
                    let mut status = String::new();
                    write!(status, "Tree Selection: Item {:?}", selected_item).unwrap();
                    tree_status_label_clone.set_label(&status);
                } else {
                    tree_status_label_clone.set_label("Tree Selection: None");
                }
            });

        // Gauge button events (Advanced Panel)
        advanced_controls
            .gauge_increase_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, {
                let gauge = gauge_clone_for_inc;
                let gauge_status_label = gauge_status_label_clone_for_inc;
                move |_: Event| {
                    let current_value = gauge.get_value();
                    let new_value = std::cmp::min(current_value + 10, 100);
                    gauge.set_value(new_value);
                    gauge_status_label.set_label(&format!("Gauge Value: {}%", new_value));
                }
            });
        advanced_controls
            .gauge_reset_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, {
                let gauge = gauge_clone_for_reset;
                let gauge_status_label = gauge_status_label_clone_for_reset;
                move |_: Event| {
                    gauge.set_value(0);
                    gauge_status_label.set_label("Gauge Value: 0%");
                }
            });

        // Slider Event Binding (Advanced Panel)
        advanced_controls.slider.bind(EventType::SLIDER, {
            let status_label = slider_label_clone.clone();
            move |event| {
                if let Some(value) = event.get_int() {
                    status_label.set_label(&format!("Slider Value: {}", value));
                }
            }
        });

        // SpinCtrl Event Binding (Advanced Panel)
        advanced_controls
            .spin_ctrl
            .bind(EventType::SPINCTRL, move |event| {
                if let Some(value) = event.get_int() {
                    // This label might have been removed or changed in create_basic_tab
                    // spin_ctrl_label_clone.set_label(&format!("Spin Value: {}", value));
                    println!(
                        "SPINCTRL Event (Advanced Tab): ID: {}, Value: {}",
                        event.get_id(),
                        value
                    );
                }
            });

        // Notebook Page Changed Event (Bind to Notebook itself)
        notebook.bind(EventType::NOTEBOOK_PAGE_CHANGED, |event| {
            if let Some(selection) = event.get_selection() {
                // Use the new get_selection
                println!("Notebook page changed to: {}", selection);
            } else {
                println!("Notebook page change event, but no selection?");
            }
            // You might want to Skip() or not depending on default behavior
            // event.skip(true);
        });

        // --- ADDED: Bind ListCtrl Events ---
        list_controls
            .list_ctrl
            .bind(EventType::LIST_ITEM_SELECTED, {
                let status_label = list_ctrl_status_label_clone.clone();
                let list_ctrl_clone = list_ctrl_clone.clone(); // Clone for use in closure
                move |event| {
                    let item_index = event.get_item_index();
                    if item_index != -1 {
                        // -1 means no item or error
                        // Get text from columns using the item_index
                        let id_text = list_ctrl_clone.get_item_text(item_index as i64, 0);

                        let desc_text = list_ctrl_clone.get_item_text(item_index as i64, 1);

                        let qty_text = list_ctrl_clone.get_item_text(item_index as i64, 2);

                        status_label.set_label(&format!(
                            "ListCtrl Item Selected: Index {}, ID: '{}', Desc: '{}', Qty: '{}'",
                            item_index, id_text, desc_text, qty_text
                        ));
                    } else {
                        status_label.set_label("ListCtrl Status: No item selected");
                    }
                }
            });

        list_controls.list_ctrl.bind(EventType::LIST_COL_CLICK, {
            let status_label = list_ctrl_status_label_clone.clone();
            move |event| {
                if let Some(col_index) = event.get_column() {
                    status_label.set_label(&format!("ListCtrl Column Clicked: {}", col_index));
                }
            }
        });
        // --- END ListCtrl Events ---

        // Event binding for RadioBox
        let radio_box_clone = basic_controls.radio_box.clone();
        basic_controls
            .radio_box
            .bind(EventType::COMMAND_RADIOBOX_SELECTED, move |event: Event| {
                let selection = event.get_selection().unwrap_or(-1);
                let selection_string = if selection != -1 {
                    radio_box_clone.get_string(selection)
                } else {
                    "N/A".to_string()
                };
                println!(
                    "RadioBox selected: {} ({}), Event: {:?}",
                    selection_string, selection, event
                );
            });

        // --- BitmapComboBox Event ---
        let bitmap_combo_box_clone = basic_controls.bitmap_combo_box.clone();
        basic_controls.bitmap_combo_box.bind(
            EventType::COMMAND_COMBOBOX_SELECTED,
            move |event: Event| {
                let selection_idx = event.get_selection().unwrap_or(-1);
                let mut log_output = format!("BitmapComboBox Event: {:?}", event);

                if selection_idx != -1 {
                    let item_str = bitmap_combo_box_clone.get_string(selection_idx as u32);
                    log_output.push_str(&format!(
                        ", Selected Item [{}]: '{}'",
                        selection_idx, item_str
                    ));

                    if let Some(bitmap) =
                        bitmap_combo_box_clone.get_item_bitmap(selection_idx as u32)
                    {
                        log_output.push_str(&format!(
                            ", Item has bitmap (w:{}, h:{})",
                            bitmap.get_width(),
                            bitmap.get_height()
                        ));
                    } else {
                        log_output.push_str(", Item has no bitmap or bitmap is invalid");
                    }
                } else {
                    let current_text_value = bitmap_combo_box_clone.get_value();
                    log_output.push_str(&format!(
                        ", No item selection, current text: '{}'",
                        current_text_value
                    ));
                }

                println!("{}", log_output);
            },
        );

        // ADDED: Bind Treebook Page Changed Event
        book_controls
            .treebook
            .bind(EventType::TREEBOOK_PAGE_CHANGED, |event: Event| {
                println!(
                    "TREEBOOK_PAGE_CHANGED Event: OldSel={}, NewSel={}, Event={:?}",
                    event.get_old_selection().unwrap_or(-2),
                    event.get_selection().unwrap_or(-2),
                    event
                );
            });

        // --- SearchCtrl Event Handlers (in BasicTabControls) ---
        let search_ctrl_clone_search = basic_controls.search_ctrl.clone();
        basic_controls.search_ctrl.bind(
            EventType::COMMAND_SEARCHCTRL_SEARCH_BTN,
            move |event: Event| {
                println!(
                    "SEARCH_CTRL Event: Search Button Clicked! ID: {}, Value: \"{}\"",
                    event.get_id(),
                    search_ctrl_clone_search.get_value()
                );
            },
        );

        let search_ctrl_clone_cancel = basic_controls.search_ctrl.clone();
        basic_controls.search_ctrl.bind(
            EventType::COMMAND_SEARCHCTRL_CANCEL_BTN,
            move |event: Event| {
                println!(
                    "SEARCH_CTRL Event: Cancel Button Clicked! ID: {}, Value was: \"{}\"",
                    event.get_id(),
                    search_ctrl_clone_cancel.get_value()
                );
                search_ctrl_clone_cancel.set_value("");
            },
        );

        let search_ctrl_clone_enter = basic_controls.search_ctrl.clone();
        basic_controls
            .search_ctrl
            .bind(EventType::TEXT_ENTER, move |event: Event| {
                println!(
                    "SEARCH_CTRL Event: Text Entered! ID: {}, Value: \"{}\"",
                    event.get_id(),
                    search_ctrl_clone_enter.get_value()
                );
            });

        let spin_ctrl_label_clone = advanced_controls.spin_ctrl_label.clone();
        advanced_controls
            .spin_ctrl
            .bind(EventType::SPINCTRL, move |event| {
                if let Some(value) = event.get_int() {
                    spin_ctrl_label_clone.set_label(&format!("Spin Value: {}", value));
                }
            });

        // --- Final Steps ---
        // Show and center the frame
        frame.show(true);
        frame.centre();
        handle.preserve(frame.clone());

        // --- Bind Event Handlers (After frame setup) ---
        // Menu Event Handlers (Bind to frame - remains the same)
        // ... (rest of the event handlers remain unchanged)

        true
    });
}
