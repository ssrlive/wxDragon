use wxdragon::prelude::*;
use wxdragon::widgets::{
    DataViewCtrl, DataViewListModel, DataViewStyle,
};
use wxdragon_sys as ffi;

// Main application
fn main() {
    wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("DataViewCtrl Demo")
            .with_size(Size::new(800, 500))
            .build();

        // Create a panel with controls
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add some explanatory text
        let info_text = StaticText::builder(&panel)
            .with_label("This demo shows a DataViewCtrl")
            .build();
        sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::Expand, 10);

        // Create the DataViewCtrl
        let dvc = DataViewCtrl::builder(&panel)
            .with_style(DataViewStyle::RowLines | DataViewStyle::HorizontalRules)
            .build();

        // Add columns
        dvc.append_text_column("ID", 0, 60, ffi::WXD_ALIGN_LEFT);
        dvc.append_text_column("Name", 1, 180, ffi::WXD_ALIGN_LEFT);
        dvc.append_text_column("Department", 2, 150, ffi::WXD_ALIGN_LEFT);
        dvc.append_toggle_column("Active", 3, 80, ffi::WXD_ALIGN_CENTER);
        dvc.append_progress_column("Performance", 4, 120);

        // Create and set up the model
        let mut model = DataViewListModel::new();
        
        // Define columns in the model
        model.append_column("ID");
        model.append_column("Name");
        model.append_column("Department");
        model.append_column("Active");
        model.append_column("Performance");
        
        // Associate model with control
        dvc.associate_model(&model);
        
        // Add example data
        add_employee_data(&model);

        // Add DataViewCtrl to sizer
        sizer.add(&dvc, 1, SizerFlag::All | SizerFlag::Expand, 10);
        
        // Set panel sizer
        panel.set_sizer(sizer, true);

        // Show frame
        frame.show(true);
        frame.centre();
    });
}

// Add sample data
fn add_employee_data(model: &DataViewListModel) {
    // Employee 1
    model.append_row();
    model.set_value_as(0, 0, "001");
    model.set_value_as(0, 1, "John Smith");
    model.set_value_as(0, 2, "Engineering");
    model.set_value_as(0, 3, true);
    model.set_value_as(0, 4, 92i64);
    
    // Employee 2
    model.append_row();
    model.set_value_as(1, 0, "002");
    model.set_value_as(1, 1, "Mary Johnson");
    model.set_value_as(1, 2, "Marketing");
    model.set_value_as(1, 3, true);
    model.set_value_as(1, 4, 78i64);
    
    // Employee 3
    model.append_row();
    model.set_value_as(2, 0, "003");
    model.set_value_as(2, 1, "Robert Brown");
    model.set_value_as(2, 2, "Sales");
    model.set_value_as(2, 3, false);
    model.set_value_as(2, 4, 45i64);
    
    // Employee 4
    model.append_row();
    model.set_value_as(3, 0, "004");
    model.set_value_as(3, 1, "Patricia Davis");
    model.set_value_as(3, 2, "Human Resources");
    model.set_value_as(3, 3, true);
    model.set_value_as(3, 4, 85i64);
    
    // Employee 5
    model.append_row();
    model.set_value_as(4, 0, "005");
    model.set_value_as(4, 1, "Michael Wilson");
    model.set_value_as(4, 2, "Engineering");
    model.set_value_as(4, 3, true);
    model.set_value_as(4, 4, 95i64);

    // Employee 6
    model.append_row();
    model.set_value_as(5, 0, "006");
    model.set_value_as(5, 1, "Elizabeth Taylor");
    model.set_value_as(5, 2, "Product");
    model.set_value_as(5, 3, true);
    model.set_value_as(5, 4, 82i64);
} 