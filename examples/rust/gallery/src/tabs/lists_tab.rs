use wxdragon::id;
use wxdragon::prelude::*;

#[allow(dead_code)]
pub struct ListsTabControls {
    pub panel: ScrolledWindow,
    pub list_box: ListBox,
    pub listbox_status_label: StaticText,
    pub checklistbox: CheckListBox,
    pub checklistbox_status_label: StaticText,
    pub choice_ctrl: Choice,
    pub choice_status_label: StaticText,
    pub combo_box: ComboBox,
    pub combo_status_label: StaticText,
    pub list_ctrl: ListCtrl,
    pub list_ctrl_status_label: StaticText,
}

pub fn create_lists_tab(notebook: &Notebook, _frame: &Frame) -> ListsTabControls {
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
        .with_id(id::ID_HIGHEST + 7) // ID_LIST_CTRL
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

    let list_ctrl_status_label = StaticText::builder(&inner_list_panel)
        .with_label("ListCtrl Status: None")
        .build();

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

    // --- Bind events for list controls ---

    // ListCtrl events
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.bind(EventType::LIST_ITEM_SELECTED, move |event: Event| {
        let item_index = event.get_item_index();
        if item_index != -1 {
            // -1 means no item or error
            // Get text from columns using the item_index
            let id_text = list_ctrl_clone.get_item_text(item_index as i64, 0);
            let desc_text = list_ctrl_clone.get_item_text(item_index as i64, 1);
            let qty_text = list_ctrl_clone.get_item_text(item_index as i64, 2);

            list_ctrl_status_label_clone.set_label(&format!(
                "ListCtrl Item Selected: Index {}, ID: '{}', Desc: '{}', Qty: '{}'",
                item_index, id_text, desc_text, qty_text
            ));
        } else {
            list_ctrl_status_label_clone.set_label("ListCtrl Status: No item selected");
        }
    });

    // Add column click event for ListCtrl
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.bind(EventType::LIST_COL_CLICK, move |event| {
        if let Some(col_index) = event.get_column() {
            list_ctrl_status_label_clone
                .set_label(&format!("ListCtrl Column Clicked: {}", col_index));
        }
    });

    // ListBox selection event
    let list_box_clone = list_box.clone();
    let listbox_status_label_clone = listbox_status_label.clone();
    list_box.bind(EventType::COMMAND_LISTBOX_SELECTED, move |_event: Event| {
        if let Some(selected_string) = list_box_clone.get_string_selection() {
            let status_text = format!("Selected: {}", selected_string);
            listbox_status_label_clone.set_label(&status_text);
            // Note: removed status bar update as it should be in main.rs
        } else {
            listbox_status_label_clone.set_label("List Selection: None");
        }
    });

    // Choice selection event
    let choice_ctrl_clone = choice_ctrl.clone();
    let choice_status_label_clone = choice_status_label.clone();
    choice_ctrl.bind(EventType::COMMAND_CHOICE_SELECTED, move |_event: Event| {
        if let Some(selected_string) = choice_ctrl_clone.get_string_selection() {
            choice_status_label_clone.set_label(&format!("Choice Selection: {}", selected_string));
        } else {
            choice_status_label_clone.set_label("Choice Selection: None");
        }
    });

    // ComboBox events
    let combo_box_clone = combo_box.clone();
    let combo_status_label_clone = combo_status_label.clone();
    combo_box.bind(
        EventType::COMMAND_COMBOBOX_SELECTED,
        move |_event: Event| {
            if let Some(selected_string) = combo_box_clone.get_string_selection() {
                combo_status_label_clone.set_label(&format!("Combo Selected: {}", selected_string));
            } else {
                combo_status_label_clone.set_label("Combo Selected: None");
            }
        },
    );

    let combo_box_clone = combo_box.clone();
    let combo_status_label_clone = combo_status_label.clone();
    combo_box.bind(EventType::TEXT, move |_event: Event| {
        let current_text = combo_box_clone.get_value();
        combo_status_label_clone.set_label(&format!("Combo Text: {}", current_text));
    });

    let combo_box_clone = combo_box.clone();
    combo_box.bind(EventType::TEXT_ENTER, move |event: Event| {
        let current_text = combo_box_clone.get_value();
        println!("ComboBox Enter: {}", current_text);
        event.skip(false);
    });

    // CheckListBox selection/check event
    let checklistbox_clone = checklistbox.clone();
    let checklistbox_status_label_clone = checklistbox_status_label.clone();
    checklistbox.bind(
        EventType::COMMAND_CHECKLISTBOX_SELECTED,
        move |_event: Event| {
            if let Some(index) = checklistbox_clone.get_selection() {
                // TODO: Needs GetInt
                let is_checked = checklistbox_clone.is_checked(index);
                let item_text = checklistbox_clone
                    .get_string(index)
                    .unwrap_or_else(|| "?".to_string());
                checklistbox_status_label_clone.set_label(&format!(
                    "CheckList Sel: {} ('{}' {})",
                    index,
                    item_text,
                    if is_checked { "Checked" } else { "Unchecked" }
                ));
            } else {
                checklistbox_status_label_clone.set_label("CheckList Sel: None");
            }
        },
    );

    // Return the controls struct
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
        list_ctrl,
        list_ctrl_status_label,
    }
}
