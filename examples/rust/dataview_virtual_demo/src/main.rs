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
    icon: Bitmap,
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
        let employees = Rc::new(RefCell::new(vec![
            Employee {
                id: "001".to_string(),
                name: "John Smith".to_string(),
                department: "Engineering".to_string(),
                active: true,
                performance: 92,
                icon: create_bitmap(ArtId::HelpSettings, ArtClient::MessageBox),
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
                icon: create_bitmap(ArtId::Information, ArtClient::MessageBox),
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
                icon: create_bitmap(ArtId::Question, ArtClient::MessageBox),
                hire_date: DateTime::new(2022, 9, 5, 8, 15, 0),
                hourly_rate: 72,
                status: "Contract".to_string(),
            },
        ]));

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
        let get_value = move |_userdata: &(), row: usize, col: usize| -> Variant {
            let employees = employees_ref.borrow();
            if row >= employees.len() {
                return Variant::String(String::new());
            }
            
            let employee = &employees[row];
            match col {
                0 => Variant::String(employee.id.clone()),
                1 => Variant::String(employee.name.clone()),
                2 => Variant::String(employee.department.clone()),
                3 => Variant::Bool(employee.active),
                4 => Variant::Int32(employee.performance),
                5 => {
                    // Always create a fresh clone of the bitmap to avoid lifetime issues
                    let cloned_bitmap = employee.icon.clone();
                    Variant::Bitmap(cloned_bitmap)
                },
                6 => Variant::DateTime(employee.hire_date),
                7 => Variant::Int32(employee.hourly_rate),
                8 => Variant::String(employee.status.clone()),
                _ => Variant::String(String::new()),
            }
        };
        
        // Define a get_attr callback to customize cell appearance
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
        
        // Create the custom virtual list model with our callbacks
        let model = CustomDataViewVirtualListModel::new(
            employees.borrow().len(),   // Initial size
            (),                         // User data (not used in this example)
            get_value,                  // Value provider callback
            None::<fn(&(), usize, usize, &Variant) -> bool>,  // No set_value callback (read-only)
            Some(get_attr),             // Custom attributes callback
            None::<fn(&(), usize, usize) -> bool>  // Default enabled status
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
