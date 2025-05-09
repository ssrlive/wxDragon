use wxdragon::prelude::*;
use wxdragon::widgets::panel::PanelStyle;
use wxdragon::widgets::radiobox::RadioBoxStyle;
use wxdragon::widgets::scrollbar::ScrollBarStyle;
use wxdragon::widgets::search_ctrl::SearchCtrlStyle;
use wxdragon::widgets::static_line::StaticLineStyle;
use wxdragon::widgets::textctrl::TextCtrlStyle;

use wxdragon::art_provider::ArtProvider;
use wxdragon::bitmap::Bitmap;

#[allow(dead_code)]
pub struct BasicTabControls {
    pub panel: Panel,
    pub text_ctrl: TextCtrl,
    pub spin_button: SpinButton,
    pub checkbox: CheckBox,
    pub radio1: RadioButton,
    pub radio2: RadioButton,
    pub radio_status_label: StaticText,
    pub toggle_button: ToggleButton,
    pub toggle_status_label: StaticText,
    pub bitmap_button: BitmapButton,
    pub art_button: BitmapButton,
    pub radio_box: RadioBox,
    pub bitmap_combo_box: BitmapComboBox,
    pub colour_picker: ColourPickerCtrl,
    pub colour_label: StaticText,
    pub date_picker: DatePickerCtrl,
    pub date_picker_label: StaticText,
    pub search_ctrl: SearchCtrl,
    pub hyperlink_ctrl: HyperlinkCtrl,
    pub activity_indicator: ActivityIndicator,
    pub activity_start_btn: Button,
    pub activity_stop_btn: Button,
    pub spinctrl_double: SpinCtrlDouble,
    pub spinctrl_double_label: StaticText,
    pub calendar_ctrl: CalendarCtrl,
    pub calendar_label: StaticText,
    pub scroll_bar: ScrollBar,
    pub scrollbar_status_label: StaticText,
    pub cmd_link_button: CommandLinkButton,
}

pub fn create_basic_tab(notebook: &Notebook, _frame: &Frame) -> BasicTabControls {
    let basic_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();

    let static_text_label = StaticText::builder(&basic_panel)
        .with_label("Text Input:")
        .build();
    static_text_label.set_tooltip("This is a label for the text input field.");
    let text_ctrl = TextCtrl::builder(&basic_panel)
        .with_value("Edit me!")
        .with_style(TextCtrlStyle::ProcessEnter)
        .build();
    text_ctrl.set_tooltip("Enter text here.");
    let spin_button_label = StaticText::builder(&basic_panel)
        .with_label("Spin Button:")
        .build();
    let spin_button = SpinButton::builder(&basic_panel)
        .with_range(0, 100)
        .with_initial_value(0)
        .build();
    spin_button.set_tooltip("Click arrows or use keys to change the value (wraps around).");
    let spinctrl_double_label_widget = StaticText::builder(&basic_panel)
        .with_label("Spin Double:")
        .build();
    let spinctrl_double = SpinCtrlDouble::builder(&basic_panel)
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
    checkbox.set_tooltip("Toggle this checkbox.");
    let radio_box_label = StaticText::builder(&basic_panel)
        .with_label("Radio Box:")
        .build();
    let radio_box_choices = ["Option 1", "Option 2", "Option 3"];
    let radio_box = RadioBox::builder(Some(&basic_panel), &radio_box_choices)
        .with_label("RadioBox Title")
        .with_major_dimension(1) // 1 column
        .with_style(RadioBoxStyle::SpecifyCols)
        .build();
    radio_box.set_selection(0);
    radio_box.set_tooltip("Select one option from the radio box.");
    let radio_label = StaticText::builder(&basic_panel)
        .with_label("Radio Buttons:")
        .build();
    let radio1 = RadioButton::builder(&basic_panel)
        .with_label("Option 1")
        .first_in_group()
        .build();
    radio1.set_tooltip("Select Option 1.");
    let radio2 = RadioButton::builder(&basic_panel)
        .with_label("Option 2")
        .build();
    radio2.set_tooltip("Select Option 2.");
    radio1.set_value(true);
    let radio_status_label = StaticText::builder(&basic_panel)
        .with_label("Selected: Option 1")
        .build();

    let toggle_button_label = StaticText::builder(&basic_panel)
        .with_label("Toggle Button:")
        .build();
    let toggle_button = ToggleButton::builder(&basic_panel)
        .with_label("Toggle Status")
        .build();
    toggle_button.set_value(true);
    toggle_button.set_tooltip("Click to toggle ON/OFF.");
    let toggle_status_label = StaticText::builder(&basic_panel).with_label("ON").build();

    let cmd_link_button_label = StaticText::builder(&basic_panel)
        .with_label("Cmd Link Btn:")
        .build();
    let cmd_link_button = CommandLinkButton::builder(&basic_panel)
        .with_main_label("Open System Settings")
        .with_note("Configure your display, network, and other system preferences.")
        .build();
    cmd_link_button.set_tooltip("Click to open system settings (simulated).");

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
        .with_bitmap(&red_bitmap)
        .build();
    bitmap_button.set_tooltip("A button with a custom red square bitmap.");

    let open_icon_bitmap = ArtProvider::get_bitmap(ArtId::FileOpen, ArtClient::Button, None)
        .or_else(|| ArtProvider::get_bitmap(ArtId::Error, ArtClient::Button, None))
        .expect("Failed to get ART_FILE_OPEN or ART_ERROR icon");
    let art_button_label = StaticText::builder(&basic_panel)
        .with_label("Art Button:")
        .build();
    let art_button = BitmapButton::builder(&basic_panel)
        .with_bitmap(&open_icon_bitmap)
        .build();
    art_button.set_tooltip("A button using an icon from the ArtProvider.");

    let activity_label = StaticText::builder(&basic_panel)
        .with_label("Activity:")
        .build();
    let activity_indicator = ActivityIndicator::builder(&basic_panel).build();
    let activity_start_btn = Button::builder(&basic_panel).with_label("Start").build();
    activity_start_btn.set_tooltip("Start the activity indicator.");
    let activity_stop_btn = Button::builder(&basic_panel).with_label("Stop").build();
    activity_stop_btn.set_tooltip("Stop the activity indicator.");

    let colour_picker_label = StaticText::builder(&basic_panel)
        .with_label("Colour Picker:")
        .build();
    let colour_picker = ColourPickerCtrl::builder(&basic_panel)
        .with_initial_colour(colours::RED)
        .build();
    colour_picker.set_tooltip("Click to choose a colour.");
    let colour_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{:?}", colours::RED))
        .build();
    let date_picker_label_widget = StaticText::builder(&basic_panel)
        .with_label("Date Picker:")
        .build();
    let date_picker = DatePickerCtrl::builder(&basic_panel).build();
    date_picker.set_tooltip("Click to choose a date.");
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
    let calendar_ctrl = CalendarCtrl::builder(&basic_panel).build();
    calendar_ctrl.set_tooltip("Select a date from the calendar.");
    let initial_calendar_date = calendar_ctrl.get_date();
    let calendar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!(
            "{:04}-{:02}-{:02}",
            initial_calendar_date.year(),
            initial_calendar_date.month(),
            initial_calendar_date.day()
        ))
        .build();

    let search_ctrl_label = StaticText::builder(&basic_panel)
        .with_label("Search Ctrl:")
        .build();
    let search_ctrl = SearchCtrl::builder(&basic_panel)
        .with_value("Search...")
        .with_style(SearchCtrlStyle::ProcessEnter)
        .build();
    search_ctrl.show_search_button(true);
    search_ctrl.show_cancel_button(true);
    search_ctrl.set_tooltip("Enter search text here.");
    let bitmap_combo_box_label = StaticText::builder(&basic_panel)
        .with_label("Bitmap Combo:")
        .build();
    let open_bmp =
        ArtProvider::get_bitmap(ArtId::FileOpen, ArtClient::Menu, None).expect("Failed art");
    let save_bmp =
        ArtProvider::get_bitmap(ArtId::FileSave, ArtClient::Menu, None).expect("Failed art");
    let new_bmp = ArtProvider::get_bitmap(ArtId::New, ArtClient::Menu, None).expect("Failed art");
    let bitmap_combo_box = BitmapComboBox::builder(Some(&basic_panel))
        .with_size(Size::new(200, -1))
        .with_value("Default Value")
        .build();
    bitmap_combo_box.append("Open", Some(&open_bmp));
    bitmap_combo_box.append("Save", Some(&save_bmp));
    bitmap_combo_box.append("New", Some(&new_bmp));
    bitmap_combo_box.append("No Icon", None);
    bitmap_combo_box.set_selection(0);
    bitmap_combo_box.set_tooltip("Select an item from the list with icons.");

    let hyperlink_label = StaticText::builder(&basic_panel)
        .with_label("Hyperlink:")
        .build();
    let hyperlink_ctrl = HyperlinkCtrl::builder(&basic_panel)
        .with_label("wxWidgets Official Website")
        .with_url("https://www.wxwidgets.org")
        .build();
    hyperlink_ctrl.set_tooltip("Click to visit the wxWidgets website.");

    let scrollbar_label = StaticText::builder(&basic_panel)
        .with_label("Scroll Bar:")
        .build();
    let scroll_bar = ScrollBar::builder(&basic_panel)
        .with_style(ScrollBarStyle::Default) // Use Default for Horizontal
        .build();
    scroll_bar.set_scrollbar(0, 10, 100, 10, true); // position, thumb_size, range, page_size, refresh
    let scrollbar_status_label = StaticText::builder(&basic_panel)
        .with_label(&format!("{}", scroll_bar.thumb_position()))
        .build();

    // --- Layout using Main Vertical BoxSizer and child FlexGridSizers ---
    let main_sizer = BoxSizer::builder(VERTICAL).build();
    let label_flags = ALIGN_RIGHT | ALIGN_CENTER_VERTICAL;
    let control_flags = EXPAND;

    let grid_sizer_group1 = FlexGridSizer::builder(0, 2)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_group1.add_growable_col(0, 1);
    grid_sizer_group1.add_growable_col(1, 3);
    grid_sizer_group1.add(&static_text_label, 0, label_flags, 0);
    grid_sizer_group1.add(&text_ctrl, 1, control_flags, 0);
    grid_sizer_group1.add(&spin_button_label, 0, label_flags, 0);
    grid_sizer_group1.add(&spin_button, 0, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);
    grid_sizer_group1.add(&spinctrl_double_label_widget, 0, label_flags, 0);
    let spin_double_sizer = BoxSizer::builder(HORIZONTAL).build();
    spin_double_sizer.add(&spinctrl_double, 0, ALIGN_CENTER_VERTICAL, 0);
    spin_double_sizer.add_spacer(5);
    spin_double_sizer.add(&spinctrl_double_status_label, 1, EXPAND, 0);
    grid_sizer_group1.add_sizer(&spin_double_sizer, 1, control_flags, 0);
    main_sizer.add_sizer(&grid_sizer_group1, 0, EXPAND | ALL, 10);

    let static_line_sep1 = StaticLine::builder(&basic_panel)
        .with_style(StaticLineStyle::Default) // Use Default for Horizontal
        .build();
    main_sizer.add(&static_line_sep1, 0, EXPAND | ALL, 5);

    let grid_sizer_therest = FlexGridSizer::builder(0, 2)
        .with_vgap(5)
        .with_hgap(5)
        .build();
    grid_sizer_therest.add_growable_col(0, 1);
    grid_sizer_therest.add_growable_col(1, 3);

    grid_sizer_therest.add(&checkbox_label_widget, 0, label_flags, 0);
    grid_sizer_therest.add(&checkbox, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);
    grid_sizer_therest.add(&radio_label, 0, label_flags, 0);
    let radio_button_sizer = BoxSizer::builder(HORIZONTAL).build();
    radio_button_sizer.add(&radio1, 0, ALIGN_CENTER_VERTICAL, 0);
    radio_button_sizer.add_spacer(5);
    radio_button_sizer.add(&radio2, 0, ALIGN_CENTER_VERTICAL, 0);
    radio_button_sizer.add_spacer(10);
    radio_button_sizer.add(&radio_status_label, 1, EXPAND, 0);
    grid_sizer_therest.add_sizer(&radio_button_sizer, 1, control_flags, 0);
    grid_sizer_therest.add(&radio_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&radio_box, 1, control_flags, 0);

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
    grid_sizer_therest.add(&scrollbar_label, 0, label_flags, 0);
    let scrollbar_h_sizer = BoxSizer::builder(HORIZONTAL).build();
    scrollbar_h_sizer.add(&scroll_bar, 1, EXPAND, 0);
    scrollbar_h_sizer.add_spacer(5);
    scrollbar_h_sizer.add(&scrollbar_status_label, 0, ALIGN_CENTER_VERTICAL, 0);
    grid_sizer_therest.add_sizer(&scrollbar_h_sizer, 1, control_flags, 0);

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

    grid_sizer_therest.add(&search_ctrl_label, 0, label_flags, 0);
    grid_sizer_therest.add(&search_ctrl, 1, control_flags, 0);
    grid_sizer_therest.add(&bitmap_combo_box_label, 0, label_flags, 0);
    grid_sizer_therest.add(&bitmap_combo_box, 1, control_flags, 0);
    grid_sizer_therest.add(&hyperlink_label, 0, label_flags, 0);
    grid_sizer_therest.add(&hyperlink_ctrl, 1, ALIGN_LEFT | ALIGN_CENTER_VERTICAL, 0);
    grid_sizer_therest.add(&cmd_link_button_label, 0, label_flags, 0);
    grid_sizer_therest.add(&cmd_link_button, 1, control_flags, 0);

    main_sizer.add_sizer(&grid_sizer_therest, 1, EXPAND | ALL, 10);
    basic_panel.set_sizer_and_fit(main_sizer, true);

    // Event Handlers (Original and Moved)
    let toggle_status_label_clone = toggle_status_label.clone();
    toggle_button.bind(
        EventType::COMMAND_TOGGLEBUTTON_CLICKED,
        move |event: Event| {
            let is_on = event.is_checked().unwrap_or(false);
            toggle_status_label_clone.set_label(if is_on { "ON" } else { "OFF" });
            println!("ToggleButton clicked, is_on: {}", is_on);
        },
    );

    let radio_box_clone = radio_box.clone();
    radio_box.bind(EventType::COMMAND_RADIOBOX_SELECTED, move |event: Event| {
        let selection = event.get_selection().unwrap_or(-1);
        let selection_string = if selection != -1 {
            radio_box_clone.get_string(selection)
        } else {
            "N/A".to_string()
        };
        println!("RadioBox selected: {} ({})", selection_string, selection);
    });

    let bitmap_combo_box_clone = bitmap_combo_box.clone();
    bitmap_combo_box.bind(EventType::COMMAND_COMBOBOX_SELECTED, move |event: Event| {
        let selection_idx = event.get_selection().unwrap_or(-1);
        let mut log_output = String::new();
        if selection_idx != -1 {
            let item_str = bitmap_combo_box_clone.get_string(selection_idx as u32);
            log_output.push_str(&format!(
                "BitmapComboBox Selected Item [{}]: '{}'",
                selection_idx, item_str
            ));
            if let Some(bitmap_obj) = bitmap_combo_box_clone.get_item_bitmap(selection_idx as u32) {
                log_output.push_str(&format!(
                    ", has bitmap (w:{}, h:{})",
                    bitmap_obj.get_width(),
                    bitmap_obj.get_height()
                ));
            } else {
                log_output.push_str(", Item has no bitmap");
            }
        } else {
            let current_text_value = bitmap_combo_box_clone.get_value();
            log_output.push_str(&format!(
                "BitmapComboBox Text Entered: '{}'",
                current_text_value
            ));
        }
        println!("{}", log_output);
    });

    let colour_label_clone = colour_status_label.clone();
    colour_picker.bind(EventType::COLOURPICKER_CHANGED, move |event: Event| {
        if let Some(colour) = event.get_colour() {
            let label_text = format!("{:?}", colour);
            colour_label_clone.set_label(&label_text);
            println!("ColourPicker changed: {:?}", colour);
        } else {
            println!("ColourPicker Event Error: No colour");
        }
    });

    let date_picker_clone = date_picker.clone();
    let date_picker_label_clone = date_picker_status_label.clone();
    date_picker.bind(EventType::DATE_CHANGED, move |_event: Event| {
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
        println!("DatePicker changed: {}", date_str);
    });

    let hyperlink_ctrl_clone = hyperlink_ctrl.clone();
    hyperlink_ctrl.bind(EventType::COMMAND_HYPERLINK, move |_event: Event| {
        let url = hyperlink_ctrl_clone.get_url();
        println!(
            "HYPERLINK Clicked! URL: \"{}\", Visited: {}",
            url,
            hyperlink_ctrl_clone.get_visited(),
        );
    });

    let indicator_clone_start = activity_indicator.clone();
    activity_start_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event: Event| {
        indicator_clone_start.start();
        println!("ActivityIndicator started.");
    });

    let indicator_clone_stop = activity_indicator.clone();
    activity_stop_btn.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event: Event| {
        indicator_clone_stop.stop();
        println!("ActivityIndicator stopped.");
    });

    let spinctrld_label_clone = spinctrl_double_status_label.clone();
    let spinctrld_clone_for_handler = spinctrl_double.clone();
    spinctrl_double.bind(EventType::SPINCTRLDOUBLE, move |_event: Event| {
        let current_value = spinctrld_clone_for_handler.get_value();
        spinctrld_label_clone.set_label(&format!("{:.2}", current_value));
        println!("SPINCTRLDOUBLE Value: {:.2}", current_value);
    });

    let calendar_label_clone = calendar_status_label.clone();
    let calendar_ctrl_clone = calendar_ctrl.clone();
    calendar_ctrl.bind(EventType::CALENDAR_SEL_CHANGED, move |_event: Event| {
        let selected_date = calendar_ctrl_clone.get_date();
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            selected_date.year(),
            selected_date.month(),
            selected_date.day()
        );
        calendar_label_clone.set_label(&date_str);
        println!("CALENDAR_SEL_CHANGED: Date: {}", date_str);
    });

    let scroll_bar_clone = scroll_bar.clone();
    let scrollbar_status_label_clone = scrollbar_status_label.clone();
    scroll_bar.bind(EventType::SCROLL_THUMBTRACK, move |event: Event| {
        let pos = scroll_bar_clone.thumb_position();
        scrollbar_status_label_clone.set_label(&format!("{}", pos));
        println!("Scroll Pos: {}", pos);
        event.skip(true);
    });

    let cmd_link_button_clone = cmd_link_button.clone();
    cmd_link_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_event: Event| {
        println!(
            "CommandLinkButton clicked. MainLabel: '{}'",
            cmd_link_button_clone.get_label()
        );
    });

    bitmap_button.bind(EventType::COMMAND_BUTTON_CLICKED, |_event: Event| {
        println!("Bitmap Button (Red) clicked.");
    });

    art_button.bind(EventType::COMMAND_BUTTON_CLICKED, |_event: Event| {
        println!("ArtProvider Button clicked!");
    });

    checkbox.bind(EventType::CHECKBOX, |event: Event| {
        println!("Checkbox: {}", event.is_checked().unwrap_or(false));
    });

    text_ctrl.bind(EventType::TEXT, |_event: Event| {
        // println!("Text: {}", _event.get_string().unwrap_or_default());
    });

    let tc_clone_for_enter = text_ctrl.clone();
    let sb_clone_for_enter = spin_button.clone();
    text_ctrl.bind(EventType::TEXT_ENTER, move |event: Event| {
        if let Some(text_from_event) = event.get_string() {
            println!("Text Enter: {}", text_from_event);
            if let Ok(num) = text_from_event.parse::<i32>() {
                let min_val = sb_clone_for_enter.min();
                let max_val = sb_clone_for_enter.max();
                sb_clone_for_enter.set_value(num.clamp(min_val, max_val));
            } else {
                tc_clone_for_enter.set_value(&sb_clone_for_enter.value().to_string());
            }
        }
        event.skip(false);
    });

    let r1_clone_for_handler = radio1.clone();
    let rsl_clone_for_handler = radio_status_label.clone();
    let radio_status_update_handler = move |_event: Event| {
        let selected_option = if r1_clone_for_handler.get_value() {
            "Option 1"
        } else {
            "Option 2"
        };
        rsl_clone_for_handler.set_label(&format!("Selected: {}", selected_option));
        println!("Radio Selection: {}", selected_option);
    };
    radio1.bind(
        EventType::COMMAND_RADIOBUTTON_SELECTED,
        radio_status_update_handler.clone(),
    );
    radio2.bind(
        EventType::COMMAND_RADIOBUTTON_SELECTED,
        radio_status_update_handler,
    );

    let tc_clone_for_spin_event = text_ctrl.clone();
    spin_button.bind(EventType::SPIN, move |event: Event| {
        if let Some(value) = event.get_int() {
            println!("SpinButton Event: Value {}", value);
            tc_clone_for_spin_event.set_value(&value.to_string());
        }
    });

    let sc_search_clone = search_ctrl.clone();
    search_ctrl.bind(
        EventType::COMMAND_SEARCHCTRL_SEARCH_BTN,
        move |event: Event| {
            println!(
                "SEARCH_CTRL Event: Search Button Clicked! ID: {}, Value: \"{}\"",
                event.get_id(),
                sc_search_clone.get_value()
            );
        },
    );

    let sc_cancel_clone = search_ctrl.clone();
    search_ctrl.bind(
        EventType::COMMAND_SEARCHCTRL_CANCEL_BTN,
        move |event: Event| {
            let value_before_clear = sc_cancel_clone.get_value();
            sc_cancel_clone.set_value("");
            println!(
                "SEARCH_CTRL Event: Cancel Button Clicked! ID: {}, Value was: \"{}\"",
                event.get_id(),
                value_before_clear
            );
        },
    );

    // TEXT_ENTER for SearchCtrl
    search_ctrl.bind(EventType::TEXT_ENTER, move |event: Event| {
        println!(
            "SEARCH_CTRL Event: Text Entered! Value: \"{}\"",
            event.get_string().unwrap_or_default()
        );
    });

    BasicTabControls {
        panel: basic_panel,
        text_ctrl,
        spin_button,
        checkbox,
        radio1,
        radio2,
        radio_status_label,
        toggle_button,
        toggle_status_label,
        bitmap_button,
        art_button,
        radio_box,
        bitmap_combo_box,
        colour_picker,
        colour_label: colour_status_label,
        date_picker,
        date_picker_label: date_picker_status_label,
        search_ctrl,
        hyperlink_ctrl,
        activity_indicator,
        activity_start_btn,
        activity_stop_btn,
        spinctrl_double,
        spinctrl_double_label: spinctrl_double_status_label,
        calendar_ctrl,
        calendar_label: calendar_status_label,
        scroll_bar,
        scrollbar_status_label,
        cmd_link_button,
    }
}

// Add this method to bind all events for basic tab controls
impl BasicTabControls {
    pub fn bind_events(&self) {
        // BitmapButton click event
        self.bitmap_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, |event: Event| {
                println!("Bitmap Button clicked: {}", event.get_id());
            });

        // ArtProvider BitmapButton click event
        self.art_button
            .bind(EventType::COMMAND_BUTTON_CLICKED, |event: Event| {
                println!(
                    "ArtProvider Button (ID: {}) clicked! Icon from ArtProvider works.",
                    event.get_id()
                );
            });

        // Checkbox toggle event
        self.checkbox.bind(EventType::CHECKBOX, |event: Event| {
            println!("Checkbox: {}", event.is_checked().unwrap_or(false));
        });

        // TextCtrl text change event
        self.text_ctrl.bind(EventType::TEXT, |_event: Event| {
            // println!("Text: {}", _event.get_string().unwrap_or_default());
        });

        // TextCtrl Enter key event
        let spin_button_clone = self.spin_button.clone();
        let text_ctrl_clone = self.text_ctrl.clone();
        self.text_ctrl
            .bind(EventType::TEXT_ENTER, move |event: Event| {
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
            });

        // Radio button selected event (bind to both)
        let radio_status_label_clone = self.radio_status_label.clone();
        let radio1_clone = self.radio1.clone();
        let radio_status_update = move |_event: Event| {
            let selected_option = if radio1_clone.get_value() {
                "Option 1"
            } else {
                "Option 2"
            };
            radio_status_label_clone.set_label(&format!("Selected: {}", selected_option));
        };
        self.radio1.bind(
            EventType::COMMAND_RADIOBUTTON_SELECTED,
            radio_status_update.clone(),
        );
        self.radio2
            .bind(EventType::COMMAND_RADIOBUTTON_SELECTED, radio_status_update);

        // SpinButton Event Bindings
        let text_ctrl_clone = self.text_ctrl.clone();
        self.spin_button.bind(EventType::SPIN, move |event: Event| {
            if let Some(value) = event.get_int() {
                text_ctrl_clone.set_value(&value.to_string());
            }
        });

        // Event binding for RadioBox
        let radio_box_clone = self.radio_box.clone();
        self.radio_box
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

        // BitmapComboBox Event
        let bitmap_combo_box_clone = self.bitmap_combo_box.clone();
        self.bitmap_combo_box
            .bind(EventType::COMMAND_COMBOBOX_SELECTED, move |event: Event| {
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
            });

        // SearchCtrl Event Handlers
        let search_ctrl_clone_search = self.search_ctrl.clone();
        self.search_ctrl.bind(
            EventType::COMMAND_SEARCHCTRL_SEARCH_BTN,
            move |event: Event| {
                println!(
                    "SEARCH_CTRL Event: Search Button Clicked! ID: {}, Value: \"{}\"",
                    event.get_id(),
                    search_ctrl_clone_search.get_value()
                );
            },
        );

        let search_ctrl_clone_cancel = self.search_ctrl.clone();
        self.search_ctrl.bind(
            EventType::COMMAND_SEARCHCTRL_CANCEL_BTN,
            move |event: Event| {
                let value_before_clear = search_ctrl_clone_cancel.get_value();
                search_ctrl_clone_cancel.set_value("");
                println!(
                    "SEARCH_CTRL Event: Cancel Button Clicked! ID: {}, Value was: \"{}\"",
                    event.get_id(),
                    value_before_clear
                );
            },
        );

        // TEXT_ENTER for SearchCtrl
        self.search_ctrl
            .bind(EventType::TEXT_ENTER, move |event: Event| {
                println!(
                    "SEARCH_CTRL Event: Text Entered! Value: \"{}\"",
                    event.get_string().unwrap_or_default()
                );
            });
    }
}
