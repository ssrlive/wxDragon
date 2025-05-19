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
#[derive(Clone)] // Added Clone for potential future use if Employee instances need to be copied
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

// Helper function to create a valid bitmap (scoped locally to this tab's creation logic)
fn create_bitmap_for_tab(art_id: ArtId, client: ArtClient) -> Bitmap {
    match ArtProvider::get_bitmap(art_id, client, Some(Size::new(16, 16))) {
        Some(bmp) => bmp,
        None => {
            let rgba = [255, 0, 0, 255]; // Red pixel fallback
            Bitmap::from_rgba(&rgba, 1, 1).unwrap_or_else(|| {
                // If even the 1x1 fails, panic is harsh for a demo, return a placeholder or log error.
                // For now, let's assume unwrap is okay for a demo if ArtProvider works.
                // A more robust solution would be a default placeholder bitmap.
                let fallback_rgba = [0,0,0,255]; // Black
                Bitmap::from_rgba(&fallback_rgba, 1,1).expect("Failed to create ultimate fallback bitmap")
            })
        }
    }
}

pub struct DataViewVirtualTabControls {
    pub panel: Panel,
    // Keep the model alive with the panel if it's not associated with a control that outlives this scope
    // Or if callbacks need to live as long as the panel.
    _model: Rc<CustomDataViewVirtualListModel> // Removed <()>
}

pub fn create_dataview_virtual_tab(parent: &impl WxWidget) -> DataViewVirtualTabControls {
    let panel = Panel::builder(parent).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    let info_text = StaticText::builder(&panel)
        .with_label("This tab demonstrates CustomDataViewVirtualListModel with callback-provided data (999 rows).")
        .build();
    sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::Expand, 10);

    // --- Sample Data Generation (copied from demo) ---
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

    let mut icon_bitmaps_master = Vec::new();
    for &art_id in art_ids.iter() {
        icon_bitmaps_master.push(create_bitmap_for_tab(art_id, ArtClient::MessageBox));
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
    // --- End Sample Data Generation ---

    let dvc = DataViewCtrl::builder(&panel)
        .with_style(DataViewStyle::RowLines | DataViewStyle::HorizontalRules)
        .build();

    dvc.append_text_column("ID", 0, 60, DataViewAlign::Left);
    dvc.append_text_column("Name", 1, 180, DataViewAlign::Left);
    dvc.append_text_column("Department", 2, 150, DataViewAlign::Left);
    dvc.append_toggle_column("Active", 3, 80, DataViewAlign::Center);
    dvc.append_progress_column("Performance", 4, 120);
    dvc.append_bitmap_column("Icon", 5, 80, DataViewAlign::Center);
    dvc.append_date_column("Hire Date", 6, 120, DataViewAlign::Center);
    dvc.append_spin_column("Hourly Rate", 7, 100, DataViewAlign::Right, 10, 100, 5);
    
    let status_choices = ["Full-time", "Part-time", "Contract"];
    dvc.append_choice_column("Status", 8, 120, DataViewAlign::Left, &status_choices);

    let employees_ref_get = Rc::clone(&employees);
    let icon_bitmaps_for_closure_get = icon_bitmaps_master.clone();
    let get_value = move |_userdata: &(), row: usize, col: usize| -> Variant {
        let employees_borrow = employees_ref_get.borrow();
        if row >= employees_borrow.len() {
            return "".to_string().into(); 
        }
        let employee = &employees_borrow[row];
        match col {
            0 => employee.id.clone().into(),
            1 => employee.name.clone().into(),
            2 => employee.department.clone().into(),
            3 => employee.active.into(),
            4 => employee.performance.into(),
            5 => {
                let bmp = &icon_bitmaps_for_closure_get[employee.icon_index];
                bmp.into()
            },
            6 => employee.hire_date.into(),
            7 => employee.hourly_rate.into(),
            8 => employee.status.clone().into(),
            _ => "".to_string().into(),
        }
    };

    let employees_ref_set = Rc::clone(&employees);
    let set_value = move |_userdata: &(), row: usize, col: usize, value: &Variant| -> bool {
        let mut employees_data = employees_ref_set.borrow_mut();
        if row >= employees_data.len() {
            return false;
        }
        let employee = &mut employees_data[row];
        match col {
            1 => { if let Variant::String(s) = value { employee.name = s.clone(); true } else { false } }
            2 => { if let Variant::String(s) = value { employee.department = s.clone(); true } else { false } }
            3 => { if let Variant::Bool(b) = value { employee.active = *b; true } else { false } }
            7 => { if let Variant::Int32(i) = value { employee.hourly_rate = *i; true } else { false } }
            8 => { if let Variant::String(s) = value { employee.status = s.clone(); true } else { false } }
            _ => false,
        }
    };

    let employees_ref_attr = Rc::clone(&employees);
    let get_attr = move |_userdata: &(), row: usize, col: usize| -> Option<DataViewItemAttr> {
        let employees_borrow = employees_ref_attr.borrow();
        if row >= employees_borrow.len() {
            return None;
        }
        let employee = &employees_borrow[row];
        match col {
            3 => {
                let attr = DataViewItemAttr::default();
                if employee.active {
                    Some(attr.with_bg_colour(0, 200, 0, 50))
                } else {
                    Some(attr.with_bg_colour(200, 0, 0, 50))
                }
            },
            4 => {
                let attr = DataViewItemAttr::default();
                if employee.performance < 50 {
                    Some(attr.with_text_colour(200, 0, 0, 255))
                } else if employee.performance > 80 {
                    Some(attr.with_text_colour(0, 150, 0, 255))
                } else {
                    None // Use default color for average performance
                }
            },
            _ => None,
        }
    };

    let is_enabled = move |_userdata: &(), _row: usize, col: usize| -> bool {
        match col {
            0 | 4 | 5 | 6 => false, // ID, Performance, Icon, Hire Date read-only
            _ => true, 
        }
    };

    let model = Rc::new(CustomDataViewVirtualListModel::new(
        employees.borrow().len(),
        (),
        get_value,
        Some(set_value),
        Some(get_attr),
        Some(is_enabled)
    ));
    
    dvc.associate_model(model.as_ref());
    
    sizer.add(&dvc, 1, SizerFlag::All | SizerFlag::Expand, 10);
    panel.set_sizer(sizer, true);

    DataViewVirtualTabControls {
        panel,
        _model: model, // Keep the model alive
    }
} 