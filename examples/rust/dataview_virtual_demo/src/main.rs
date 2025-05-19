use wxdragon::prelude::*;
use wxdragon::widgets::dataview::{
    DataViewCtrl, CustomDataViewVirtualListModel, 
    DataViewStyle, DataViewAlign, DataViewItemAttr, Variant
};
use wxdragon::art_provider::{ArtProvider, ArtId, ArtClient};
use wxdragon::bitmap::Bitmap;
use wxdragon::geometry::Size;
use wxdragon::datetime::DateTime;
use std::cell::RefCell;
use std::rc::Rc;

// Define a struct to hold our employee data
struct Employee {
    id: String,
    name: String,
    department: String,
    active: bool,
    performance: i32,
    icon_index: usize,
    hire_date: DateTime,
    hourly_rate: i32,
    status: String,
}

// Main application
fn main() {
    wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("DataViewVirtualListModel Demo")
            .with_size(Size::new(800, 600))
            .build();

        // Create a panel with controls
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add some explanatory text
        let info_text = StaticText::builder(&panel)
            .with_label("This demo shows CustomDataViewVirtualListModel with callback-provided data")
            .build();
        sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::Expand, 10);

        // Create sample data
        let mut initial_employees = vec![
            Employee {
                id: "001".to_string(),
                name: "John Smith".to_string(),
                department: "Engineering".to_string(),
                active: true,
                performance: 92,
                icon_index: 0,
                hire_date: DateTime::new(2020, 3, 15, 10, 30, 0),
                hourly_rate: 65,
                status: "Full-time".to_string(),
            },
            Employee {
                id: "002".to_string(),
                name: "Jane Doe".to_string(),
                department: "Marketing".to_string(),
                active: true,
                performance: 78,
                icon_index: 1,
                hire_date: DateTime::new(2021, 6, 10, 9, 0, 0),
                hourly_rate: 55,
                status: "Part-time".to_string(),
            },
            Employee {
                id: "003".to_string(),
                name: "Bob Johnson".to_string(),
                department: "Finance".to_string(),
                active: false,
                performance: 45,
                icon_index: 2,
                hire_date: DateTime::new(2022, 9, 5, 8, 15, 0),
                hourly_rate: 72,
                status: "Contract".to_string(),
            },
        ];

        let art_ids = [
            ArtId::HelpSettings, ArtId::Information, ArtId::Question, ArtId::Warning, 
            ArtId::Error, ArtId::AddBookmark, ArtId::DeleteBookmark, ArtId::HelpBook
        ];
        let departments = ["Engineering", "Marketing", "Finance", "HR", "Sales", "Operations", "R&D"];
        let statuses = ["Full-time", "Part-time", "Contract", "Intern"];

        // Pre-create icon bitmaps to reduce FFI calls in the loop
        let mut icon_bitmaps_master = Vec::new();
        for &art_id in art_ids.iter() {
            icon_bitmaps_master.push(create_bitmap(art_id, ArtClient::MessageBox));
        }

        for i in 4..=999 {
            let art_id_index = (i - 4) % art_ids.len();
            let dept_index = (i - 4) % departments.len();
            let status_index = (i - 4) % statuses.len();

            initial_employees.push(Employee {
                id: format!("{:03}", i),
                name: format!("Employee {}", i),
                department: departments[dept_index].to_string(),
                active: (i % 2 == 0),
                performance: (i % 100) as i32,
                icon_index: art_id_index,
                hire_date: DateTime::new(2022, (i % 12 + 1) as u16, (i % 28 + 1) as i16, 9, 0, 0),
                hourly_rate: (50 + (i % 50)) as i32,
                status: statuses[status_index].to_string(),
            });
        }
        let employees = Rc::new(RefCell::new(initial_employees));

        // Helper function to create a valid bitmap
        fn create_bitmap(art_id: ArtId, client: ArtClient) -> Bitmap {
            // Get a standard art bitmap with the specified size
            match ArtProvider::get_bitmap(art_id, client, Some(Size::new(16, 16))) {
                Some(bmp) => bmp,
                None => {
                    // Create a simple colored bitmap as fallback
                    let rgba = [255, 0, 0, 255];  // Red pixel
                    Bitmap::from_rgba(&rgba, 1, 1).unwrap()
                }
            }
        }

        // Create the DataViewCtrl
        let dvc = DataViewCtrl::builder(&panel)
            .with_style(DataViewStyle::RowLines | DataViewStyle::HorizontalRules)
            .build();

        // Add columns with different renderers
        dvc.append_text_column("ID", 0, 60, DataViewAlign::Left);
        dvc.append_text_column("Name", 1, 180, DataViewAlign::Left);
        dvc.append_text_column("Department", 2, 150, DataViewAlign::Left);
        dvc.append_toggle_column("Active", 3, 80, DataViewAlign::Center);
        dvc.append_progress_column("Performance", 4, 120);
        dvc.append_bitmap_column("Icon", 5, 80, DataViewAlign::Center);
        dvc.append_date_column("Hire Date", 6, 120, DataViewAlign::Center);
        dvc.append_spin_column("Hourly Rate", 7, 100, DataViewAlign::Right, 10, 100, 5);
        
        // Choices renderer
        let status_choices = ["Full-time", "Part-time", "Contract"];
        dvc.append_choice_column("Status", 8, 120, DataViewAlign::Left, &status_choices);

        // Create a CustomDataViewVirtualListModel
        let employees_ref = Rc::clone(&employees);
        let icon_bitmaps_for_closure = icon_bitmaps_master.clone(); // Clone for the closure
        let get_value = move |_userdata: &(), row: usize, col: usize| -> Variant {
            let employees_borrow = employees_ref.borrow();
            if row >= employees_borrow.len() {
                return "".to_string().into(); // Return empty string variant for out-of-bounds
            }
            let employee = &employees_borrow[row];
            match col {
                0 => employee.id.clone().into(),
                1 => employee.name.clone().into(),
                2 => employee.department.clone().into(),
                3 => employee.active.into(),
                4 => employee.performance.into(),
                5 => {
                    let bmp = &icon_bitmaps_for_closure[employee.icon_index];
                    bmp.into()
                },
                6 => employee.hire_date.into(),
                7 => employee.hourly_rate.into(),
                8 => employee.status.clone().into(),
                _ => "".to_string().into(),
            }
        };

        let employees_set_ref = Rc::clone(&employees);
        let set_value = move |_userdata: &(), row: usize, col: usize, value: &Variant| -> bool {
            let mut employees_data = employees_set_ref.borrow_mut();
            if row >= employees_data.len() {
                return false;
            }
            let employee = &mut employees_data[row];
            let success = match col {
                1 => { // Name
                    if let Variant::String(s) = value { employee.name = s.clone(); true } else { false }
                }
                2 => { // Department
                    if let Variant::String(s) = value { employee.department = s.clone(); true } else { false }
                }
                3 => { // Active (Toggle)
                    if let Variant::Bool(b) = value { employee.active = *b; true } else { false }
                }
                // Column 4 (Performance - wxDataViewProgressRenderer) is typically not directly editable by user input.
                // It usually reflects a value set programmatically.
                // If it were, e.g., a wxDataViewSpinRenderer, this would be how to handle it:
                // 4 => { 
                //     if let Variant::Int32(i) = value { employee.performance = *i; true } else { false }
                // }
                7 => { // Hourly Rate (SpinCtrl)
                    if let Variant::Int32(i) = value { employee.hourly_rate = *i; true } else { false }
                }
                8 => { // Status (Choice)
                    if let Variant::String(s) = value { employee.status = s.clone(); true } else { false }
                }
                _ => false, // Other columns (ID, Icon, HireDate, unhandled Performance) are not made editable here
            };
            if success {
                // Optional: If you need to inform the DataViewCtrl that a specific cell has changed
                // (e.g., if the change wasn't directly from user edit causing an event),
                // you might call `dvc.model().unwrap().row_value_changed(row, col);`
                // However, for direct edits via UI, the control usually handles repaint.
            }
            success
        };

        let employees_attr_ref = Rc::clone(&employees);
        let get_attr = move |_userdata: &(), row: usize, col: usize| -> Option<DataViewItemAttr> {
            let employees = employees_attr_ref.borrow();
            if row >= employees.len() {
                return None;
            }
            
            let employee = &employees[row];
            
            // Customize appearance based on column and data
            match col {
                3 => {
                    // Highlight active status
                    if employee.active {
                        let attr = DataViewItemAttr::default()
                            .with_bg_colour(0, 200, 0, 50); // Light green background
                        Some(attr)
                    } else {
                        let attr = DataViewItemAttr::default()
                            .with_bg_colour(200, 0, 0, 50); // Light red background
                        Some(attr)
                    }
                },
                4 => {
                    // Color code performance
                    let attr = DataViewItemAttr::default();
                    if employee.performance < 50 {
                        let attr = attr.with_text_colour(200, 0, 0, 255); // Red text for low performance
                        Some(attr)
                    } else if employee.performance > 80 {
                        let attr = attr.with_text_colour(0, 150, 0, 255); // Green text for high performance
                        Some(attr)
                    } else {
                        Some(attr) // Default color for average performance
                    }
                },
                _ => None,
            }
        };

        let is_enabled = move |_userdata: &(), _row: usize, col: usize| -> bool {
            match col {
                0 => false, // ID column read-only
                4 => false, // Performance (Progress Bar) read-only by nature
                5 => false, // Icon column read-only
                6 => false, // Hire Date read-only for this demo
                _ => true,  // Name, Department, Active, Hourly Rate, Status are editable
            }
        };

        // Create the custom virtual list model with our callbacks
        let model = CustomDataViewVirtualListModel::new(
            employees.borrow().len(),
            (),
            get_value,
            Some(set_value),
            Some(get_attr),
            Some(is_enabled)
        );
        
        // Associate model with control
        dvc.associate_model(&model);
        
        // Add DataViewCtrl to sizer
        sizer.add(&dvc, 1, SizerFlag::All | SizerFlag::Expand, 10);
        
        // Set panel sizer
        panel.set_sizer(sizer, true);

        // Show frame
        frame.show(true);
        frame.centre();
        
        // Keep the model alive for the lifetime of our application
        let _model = model;
    });
}
