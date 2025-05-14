use wxdragon::id;
use wxdragon::prelude::*;
use wxdragon::widgets::checklistbox::CheckListBoxStyle;
use wxdragon::widgets::choice::ChoiceStyle;
use wxdragon::widgets::combobox::ComboBoxStyle;
use wxdragon::widgets::editablelistbox::{EditableListBox, EditableListBoxStyle};
use wxdragon::widgets::listbox::ListBoxStyle;
use wxdragon::widgets::list_ctrl::{ListCtrl, ListCtrlStyle, ListColumnFormat, ListItemState};
use wxdragon::widgets::panel::PanelStyle;

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
    pub editable_listbox: EditableListBox,
    pub editable_listbox_status_label: StaticText,
}

pub fn create_lists_tab(notebook: &Notebook, _frame: &Frame) -> ListsTabControls {
    // Create the ScrolledWindow as the main container for this tab
    let scrolled_list_window = ScrolledWindow::builder(notebook)
        // .with_style(PanelStyle::TabTraversal) // Removed - ScrolledWindow doesn't take PanelStyle
        .build();

    // Main panel inside the scrolled window, this is where TAB_TRAVERSAL should apply if needed for its children
    let panel = Panel::builder(&scrolled_list_window)
        .with_style(PanelStyle::TabTraversal) // Panel takes PanelStyle
        .build();
    let _sizer = BoxSizer::builder(VERTICAL).build();

    // --- Create controls, parenting them to the *inner_list_panel* ---
    let list_box_items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
    ];
    let list_box = ListBox::builder(&panel)
        .with_choices(list_box_items)
        // .with_style(ListBoxStyle::Default | ListBoxStyle::Sort | ListBoxStyle::AlwaysScrollbar | ListBoxStyle::HorizontalScrollbar) // Old complex
        .with_style(ListBoxStyle::Sort) // Simplified to test
        .build();
    let listbox_status_label = StaticText::builder(&panel)
        .with_label("List Selection: None")
        .build();
    let checklistbox = CheckListBox::builder(&panel)
        .with_id(109)
        .with_choices(vec![
            "Option A".to_string(),
            "Option B".to_string(),
            "Option C".to_string(),
            "Option D".to_string(),
            "Option E".to_string(),
            "Option F".to_string(),
            "Option G".to_string(),
        ])
        // .with_style(LB_SORT) // Old
        .with_style(CheckListBoxStyle::Sort) // Correct type
        .build();
    checklistbox.check(1, true);
    let checklistbox_status_label = StaticText::builder(&panel)
        .with_label("CheckList Status: B (Checked)")
        .build();
    let choice_items = [
        "Red", "Green", "Blue", "Yellow", "Purple", "Orange", "Cyan", "Magenta",
    ];
    let choice_ctrl = Choice::builder(&panel)
        .with_choices(choice_items.iter().map(|s| s.to_string()).collect())
        .with_style(ChoiceStyle::Sort) // Now we can use the enum
        .build();
    choice_ctrl.set_selection(0);
    let choice_status_label = StaticText::builder(&panel)
        .with_label("Choice Selection: Red")
        .build();
    let combo_items = [
        "Cabbage", "Carrot", "Cucumber", "Celery", "Broccoli", "Spinach", "Kale", "Lettuce",
    ];
    let combo_box = ComboBox::builder(&panel)
        .with_string_choices(&combo_items)
        // .with_style(CB_SORT) // Old
        .with_style(ComboBoxStyle::Sort | ComboBoxStyle::ProcessEnter) // Ensure ProcessEnter for TEXT_ENTER event
        .build();
    let combo_status_label = StaticText::builder(&panel)
        .with_label("Combo Status: Initial Combo")
        .build();

    // --- ADDED: ListCtrl Example ---
    let list_ctrl = ListCtrl::builder(&panel)
        .with_id(id::ID_HIGHEST + 7) // ID_LIST_CTRL
        .with_style(
            ListCtrlStyle::Report
                | ListCtrlStyle::SingleSel
                | ListCtrlStyle::HRules
                | ListCtrlStyle::VRules,
        ) // Report style, single selection, rules
        .build();

    // Add columns
    list_ctrl.insert_column(0, "ID", ListColumnFormat::Right, 60);
    list_ctrl.insert_column(1, "Description", ListColumnFormat::Left, 150);
    list_ctrl.insert_column(2, "Quantity", ListColumnFormat::Right, 100);
    list_ctrl.insert_column(3, "Notes", ListColumnFormat::Left, -1); // Fill remaining space

    // Insert items (only sets the text for column 0)
    let _item1_idx = list_ctrl.insert_item(0, "P001");
    let _item2_idx = list_ctrl.insert_item(1, "P002");
    let _item3_idx = list_ctrl.insert_item(2, "P003");
    let _item4_idx = list_ctrl.insert_item(3, "P004");
    let _item5_idx = list_ctrl.insert_item(4, "P005");
    
    // We need a better column data setting API, but for now, we can use 
    // the current API to manually set text for each column
    
    // Set up some colors - use valid RGB values
    let blue_color = Colour::new(230, 240, 255, 255);  // Light blue
    let yellow_color = Colour::new(255, 255, 230, 255); // Light yellow
    let red_text = Colour::new(200, 0, 0, 255);        // Dark red
    let green_text = Colour::new(0, 150, 0, 255);      // Dark green
    
    // Format specific rows
    list_ctrl.set_item_background_colour(0, &blue_color);
    list_ctrl.set_item_background_colour(2, &yellow_color);
    list_ctrl.set_item_background_colour(4, &blue_color);
    
    // Format specific cell text
    list_ctrl.set_item_text_colour(1, &red_text);
    list_ctrl.set_item_text_colour(3, &green_text);
    
    // Set item data - allows storing arbitrary integer data with each row
    list_ctrl.set_item_data(0, 1001);
    list_ctrl.set_item_data(1, 2002);
    list_ctrl.set_item_data(2, 3003);
    list_ctrl.set_item_data(3, 4004);
    list_ctrl.set_item_data(4, 5005);
    
    // Set up selection
    list_ctrl.set_item_state(0, ListItemState::Selected, ListItemState::Selected);
    
    // Set up status display
    let list_ctrl_status_label = StaticText::builder(&panel)
        .with_label("ListCtrl Status: None")
        .build();
        
    // Add buttons to interact with the list control
    let list_ctrl_button_sizer = BoxSizer::builder(HORIZONTAL).build();
    
    let add_button = Button::builder(&panel)
        .with_label("Add Item")
        .build();
        
    let remove_button = Button::builder(&panel)
        .with_label("Remove Selected")
        .build();
        
    let select_button = Button::builder(&panel)
        .with_label("Select First")
        .build();
        
    let edit_button = Button::builder(&panel)
        .with_label("Edit Selected")
        .build();
        
    list_ctrl_button_sizer.add(&add_button, 0, ALL, 5);
    list_ctrl_button_sizer.add(&remove_button, 0, ALL, 5);
    list_ctrl_button_sizer.add(&select_button, 0, ALL, 5);
    list_ctrl_button_sizer.add(&edit_button, 0, ALL, 5);

    // --- ADDED: EditableListBox Example ---
    let editable_listbox = EditableListBox::builder(&panel)
        .with_label("Editable List")
        .with_style(EditableListBoxStyle::AllowNew|EditableListBoxStyle::AllowEdit|EditableListBoxStyle::AllowDelete)
        .build();

    // Add some initial items to the editable listbox
    editable_listbox.set_strings(&[
        "Task 1", 
        "Task 2", 
        "Task 3", 
        "Click + to add more tasks",
        "Select a task and click - to remove it",
        "Use the up/down buttons to reorder tasks"
    ]);

    let editable_listbox_status_label = StaticText::builder(&panel)
        .with_label("EditableListBox: Use buttons to modify list")
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
    list_ctrl_col_sizer.add_sizer(&list_ctrl_button_sizer, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    list_ctrl_col_sizer.add(&list_ctrl_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    list_sizer_main.add_sizer(&list_ctrl_col_sizer, 1, EXPAND | ALL, 5); // Add ListCtrl sizer to main, taking space

    // Add EditableListBox and its status label
    let editable_listbox_sizer = BoxSizer::builder(VERTICAL).build();
    editable_listbox_sizer.add(&editable_listbox, 1, EXPAND | ALL, 5);
    editable_listbox_sizer.add(&editable_listbox_status_label, 0, ALIGN_CENTER_HORIZONTAL | ALL, 5);
    list_sizer_main.add_sizer(&editable_listbox_sizer, 1, EXPAND | ALL, 5);

    panel.set_sizer(list_sizer_main, true);
    // Fit the inner panel to its contents initially
    panel.fit();

    // --- Configure the ScrolledWindow ---
    // Calculate virtual size needed based on the inner panel's best size
    let inner_size = panel.get_best_size();
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
            // Get the item ID (the text in column 0)
            let id_text = list_ctrl_clone.get_item_text(item_index as i64, 0);
            
            // Get the item data (the integer we associated with this row)
            let item_data = list_ctrl_clone.get_item_data(item_index as i64);
            
            // Format a status message
            list_ctrl_status_label_clone.set_label(&format!(
                "Selected: {} (index: {}, data: {})",
                id_text, item_index, item_data
            ));
        } else {
            list_ctrl_status_label_clone.set_label("No item selected");
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

    // EditableListBox events - basic selection event
    let status_label_clone1 = editable_listbox_status_label.clone();
    editable_listbox.bind(EventType::COMMAND_LISTBOX_SELECTED, 
        move |_event: Event| {
            // Simple update without accessing complex methods
            status_label_clone1.set_label("Item selected in EditableListBox");
        }
    );
    
    // Bind event for when editing begins
    let status_label_clone2 = editable_listbox_status_label.clone();
    editable_listbox.bind(EventType::LIST_BEGIN_LABEL_EDIT, 
        move |_event: Event| {
            status_label_clone2.set_label("Editing item...");
        }
    );
    
    // Bind event for when editing ends
    let status_label_clone3 = editable_listbox_status_label.clone();
    editable_listbox.bind(EventType::LIST_END_LABEL_EDIT, 
        move |_event: Event| {
            status_label_clone3.set_label("Item edited");
        }
    );

    // Edit button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    edit_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_| {
        let selected_item = list_ctrl_clone.get_first_selected_item();
        if selected_item >= 0 {
            // Start editing the label of the selected item
            let text_ctrl = list_ctrl_clone.edit_label(selected_item as i64);
            text_ctrl.set_value("New Label");
            // You can access the TextCtrl here if needed
            list_ctrl_status_label_clone.set_label(&format!("Editing item {}", selected_item));
        } else {
            list_ctrl_status_label_clone.set_label("Please select an item to edit");
        }
    });
    
    // Add button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    add_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_| {
        let count = list_ctrl_clone.get_item_count();
        let item_idx = list_ctrl_clone.insert_item(count as i64, &format!("P{:03}", count + 1));
        if item_idx >= 0 {
            // Select the new item
            list_ctrl_clone.set_item_state(
                item_idx as i64, 
                ListItemState::Selected, 
                ListItemState::Selected
            );
            list_ctrl_status_label_clone.set_label(&format!("Added new item {}", item_idx));
        }
    });
    
    // Remove button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    remove_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_| {
        let selected_item = list_ctrl_clone.get_first_selected_item();
        if selected_item >= 0 {
            // Delete the selected item
            if list_ctrl_clone.delete_item(selected_item as i64) {
                list_ctrl_status_label_clone.set_label(&format!("Removed item {}", selected_item));
            } else {
                list_ctrl_status_label_clone.set_label(&format!("Failed to remove item {}", selected_item));
            }
        } else {
            list_ctrl_status_label_clone.set_label("Please select an item to remove");
        }
    });
    
    // Select first button
    let list_ctrl_clone = list_ctrl.clone();
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    select_button.bind(EventType::COMMAND_BUTTON_CLICKED, move |_| {
        // Ensure first item exists
        if list_ctrl_clone.get_item_count() > 0 {
            // Select the first item
            list_ctrl_clone.set_item_state(0, ListItemState::Selected, ListItemState::Selected);
            // Ensure it's visible
            list_ctrl_clone.ensure_visible(0);
            list_ctrl_status_label_clone.set_label("Selected first item");
        } else {
            list_ctrl_status_label_clone.set_label("List is empty");
        }
    });
    
    // Bind events for item editing
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.bind(EventType::LIST_BEGIN_LABEL_EDIT, move |_| {
        list_ctrl_status_label_clone.set_label("Started editing label...");
    });
    
    let list_ctrl_status_label_clone = list_ctrl_status_label.clone();
    list_ctrl.bind(EventType::LIST_END_LABEL_EDIT, move |event: Event| {
        // Check if edit was cancelled - note: we need to handle this correctly
        let cancelled = event.is_edit_cancelled().unwrap_or(true);
        if cancelled {
            list_ctrl_status_label_clone.set_label("Label edit cancelled");
        } else {
            let _item_index = event.get_item_index();
            let label = event.get_label().unwrap_or_else(|| String::from("<no label>"));
            list_ctrl_status_label_clone.set_label(&format!("Label changed to: {}", label));
        }
    });

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
        editable_listbox,
        editable_listbox_status_label,
    }
}
