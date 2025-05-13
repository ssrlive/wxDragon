use wxdragon::prelude::*;
use wxdragon::widgets::panel::PanelStyle;
use wxdragon::widgets::treectrl::{TreeCtrl, TreeCtrlStyle};

/// A custom data type to associate with tree items
#[derive(Debug, Clone)]
struct PersonData {
    name: String,
    age: u32,
    role: String,
}

/// Helper methods for displaying PersonData
impl PersonData {
    /// Creates a display string for the data
    fn to_display_string(&self) -> String {
        format!(
            "Person: {}\nAge: {}\nRole: {}",
            self.name, self.age, self.role
        )
    }
}

/// Another custom data type to demonstrate multiple types
#[derive(Debug, Clone)]
struct ProjectData {
    name: String,
    priority: u32,
    deadline: String,
}

/// Helper methods for displaying ProjectData
impl ProjectData {
    /// Creates a display string for the data
    fn to_display_string(&self) -> String {
        format!(
            "Project: {}\nPriority: {}\nDeadline: {}",
            self.name, self.priority, self.deadline
        )
    }
}

pub struct TreeCtrlTabControls {
    pub panel: Panel,
    pub tree_ctrl: TreeCtrl,
    pub info_text: StaticText,
}

impl TreeCtrlTabControls {
    pub fn bind_events(&self) {
        // Clone references for use in event handlers
        let tree_ctrl = self.tree_ctrl.clone();
        let info_text = self.info_text.clone();

        // Bind selection changed event for tree control
        self.tree_ctrl
            .bind(EventType::TREE_SEL_CHANGED, move |event| {
                if let Some(item_id) = event.get_item() {
                    // Get data from the selected item
                    if let Some(item_data) = tree_ctrl.get_item_data(&item_id) {
                        // Try to downcast to PersonData first - this should now work with our improved implementation
                        if let Some(person) = item_data.downcast_ref::<PersonData>() {
                            info_text.set_label(&person.to_display_string());
                        }
                        // Try to downcast to ProjectData
                        else if let Some(project) = item_data.downcast_ref::<ProjectData>() {
                            info_text.set_label(&project.to_display_string());
                        }
                        // Handle standard types
                        else if let Some(text) = item_data.downcast_ref::<String>() {
                            info_text.set_label(&format!("Text: {}", text));
                        } else if let Some(number) = item_data.downcast_ref::<i32>() {
                            info_text.set_label(&format!("Number: {}", number));
                        } else if let Some(_) = item_data.downcast_ref::<()>() {
                            info_text.set_label("This item has empty data (unit type)");
                        }
                        // Fall back to type info if no match
                        else if let Some(type_info) = item_data.get_type_info() {
                            info_text.set_label(&format!(
                                "Item has data of type: {} (ID: {:?})",
                                type_info.type_name, type_info.type_id
                            ));
                        } else {
                            info_text.set_label(&format!(
                                "Item has data of type: {}",
                                item_data.get_type_name()
                            ));
                        }
                    } else {
                        info_text.set_label("Item has no associated data");
                    }
                }
            });

        // Bind item activation (double-click) event
        let tree_ctrl = self.tree_ctrl.clone();
        let info_text = self.info_text.clone();

        self.tree_ctrl
            .bind(EventType::TREE_ITEM_ACTIVATED, move |event| {
                if let Some(item_id) = event.get_item() {
                    if let Some(item_data) = tree_ctrl.get_item_data(&item_id) {
                        info_text.set_label(&format!(
                            "Double-clicked on item with data of type: {}",
                            item_data.get_type_name()
                        ));
                    } else {
                        info_text.set_label("Double-clicked on item with no data");
                    }
                }
            });
    }
}

pub fn create_treectrl_tab(parent: &Notebook) -> TreeCtrlTabControls {
    // Create the main panel
    let panel = Panel::builder(parent)
        .with_style(PanelStyle::TabTraversal)
        .build();

    // Create the tree control with some styles
    let tree_ctrl = TreeCtrl::builder(&panel)
        .with_style(TreeCtrlStyle::HasButtons | TreeCtrlStyle::LinesAtRoot)
        .build();

    // Create info text control to display data
    let info_text = StaticText::builder(&panel)
        .with_label("Select a tree item to see its data")
        .build();

    // Populate the tree with example data

    // 1. Create root item with PersonData
    let ceo_data = PersonData {
        name: "John Smith".to_string(),
        age: 52,
        role: "CEO".to_string(),
    };
    let root_id = tree_ctrl
        .add_root_with_data("Company Hierarchy", ceo_data)
        .unwrap();

    // 2. Add departments with different data types

    // Engineering department with ProjectData
    let eng_project = ProjectData {
        name: "Engineering Department".to_string(),
        priority: 1,
        deadline: "2024-12-31".to_string(),
    };
    let eng_id = tree_ctrl
        .append_item_with_data(&root_id, "Engineering", eng_project)
        .unwrap();

    // Add engineers with PersonData
    let eng_lead = PersonData {
        name: "Alice Johnson".to_string(),
        age: 38,
        role: "Lead Engineer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Alice Johnson", eng_lead)
        .unwrap();

    let dev1 = PersonData {
        name: "Bob Williams".to_string(),
        age: 29,
        role: "Software Developer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Bob Williams", dev1)
        .unwrap();

    let dev2 = PersonData {
        name: "Carol Davis".to_string(),
        age: 32,
        role: "QA Engineer".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&eng_id, "Carol Davis", dev2)
        .unwrap();

    // Marketing department with String data
    let marketing_id = tree_ctrl
        .append_item_with_data(
            &root_id,
            "Marketing",
            "Marketing department handles all promotional activities.".to_string(),
        )
        .unwrap();

    // Add marketing staff with mixed data types
    tree_ctrl
        .append_item_with_data(
            &marketing_id,
            "David Wilson",
            PersonData {
                name: "David Wilson".to_string(),
                age: 41,
                role: "Marketing Director".to_string(),
            },
        )
        .unwrap();

    tree_ctrl
        .append_item_with_data(
            &marketing_id,
            "Current Campaign",
            ProjectData {
                name: "Summer Sale".to_string(),
                priority: 2,
                deadline: "2024-08-31".to_string(),
            },
        )
        .unwrap();

    // Finance department with i32 data (budget in thousands)
    let finance_id = tree_ctrl
        .append_item_with_data(&root_id, "Finance", 250)
        .unwrap();

    // Add finance staff
    let finance_lead = PersonData {
        name: "Eve Brown".to_string(),
        age: 45,
        role: "CFO".to_string(),
    };
    tree_ctrl
        .append_item_with_data(&finance_id, "Eve Brown", finance_lead)
        .unwrap();

    // Expand the root item to show the structure
    tree_ctrl.select_item(&root_id);

    // Create sizers for layout
    let main_sizer = BoxSizer::builder(HORIZONTAL).build();

    // Left side: Tree control
    let tree_sizer = BoxSizer::builder(VERTICAL).build();
    tree_sizer.add(&tree_ctrl, 1, EXPAND | ALL, 10);

    // Right side: Info panel
    let info_sizer = BoxSizer::builder(VERTICAL).build();
    let info_title = StaticText::builder(&panel)
        .with_label("Item Information:")
        .build();

    info_sizer.add(&info_title, 0, ALL, 5);
    info_sizer.add(&info_text, 1, EXPAND | ALL, 10);

    // Add both sides to main sizer
    main_sizer.add_sizer(&tree_sizer, 3, EXPAND | ALL, 10);
    main_sizer.add_sizer(&info_sizer, 2, EXPAND | ALL, 10);

    // Set the panel's sizer
    panel.set_sizer(main_sizer, true);

    // Return the controls
    TreeCtrlTabControls {
        panel,
        tree_ctrl,
        info_text,
    }
}
