use wxdragon::prelude::*;
use wxdragon::widgets::{
    DataViewCtrl, DataViewListModel, DataViewStyle, DataViewAlign
};
use wxdragon::art_provider::{ArtProvider, ArtId, ArtClient};
use wxdragon::bitmap::Bitmap;
use wxdragon::geometry::Size;
use wxdragon::datetime::DateTime;

// Main application
fn main() {
    wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("DataViewCtrl Demo")
            .with_size(Size::new(1100, 600))
            .build();

        // Create a panel with controls
        let panel = Panel::builder(&frame).build();
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add some explanatory text
        let info_text = StaticText::builder(&panel)
            .with_label("This demo shows DataViewCtrl with all available renderers")
            .build();
        sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::Expand, 10);

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

        // Create and set up the model
        let mut model = DataViewListModel::new();
        
        // Define columns in the model
        model.append_column("ID");               // 0: Text
        model.append_column("Name");             // 1: Text
        model.append_column("Department");       // 2: Text
        model.append_column("Active");           // 3: Toggle
        model.append_column("Performance");      // 4: Progress
        model.append_column("Icon");             // 5: Bitmap
        model.append_column("Hire Date");        // 6: Date
        model.append_column("Hourly Rate");      // 7: Spin
        model.append_column("Status");           // 8: Choice
        
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
    // Get some bitmaps from ArtProvider for the demo
    let user_icon = ArtProvider::get_bitmap(ArtId::HelpSettings, ArtClient::MessageBox, Some(Size::new(16, 16))).unwrap_or_else(|| {
        // Fallback if icon not found
        Bitmap::from_rgba(&[255, 255, 255, 255], 1, 1).unwrap()
    });
    
    // Employee 1
    model.append_row();
    model.set_value(0, 0, "001");
    model.set_value(0, 1, "John Smith");
    model.set_value(0, 2, "Engineering");
    model.set_value(0, 3, true);
    model.set_value(0, 4, 92i64);
    model.set_value(0, 5, user_icon.clone());
    let dt1 = DateTime::new(2020, 3, 15, 10, 30, 0);
    model.set_value(0, 6, dt1);
    model.set_value(0, 7, 65i64);
    model.set_value(0, 8, "Full-time");
    
    // Employee 2
    model.append_row();
    model.set_value(1, 0, "002");
    model.set_value(1, 1, "Mary Johnson");
    model.set_value(1, 2, "Marketing");
    model.set_value(1, 3, true);
    model.set_value(1, 4, 78i64);
    model.set_value(1, 5, user_icon.clone());
    let dt2 = DateTime::new(2021, 6, 1, 9, 15, 0);
    model.set_value(1, 6, dt2);
    model.set_value(1, 7, 72i64);
    model.set_value(1, 8, "Part-time");
    
    // Employee 3
    model.append_row();
    model.set_value(2, 0, "003");
    model.set_value(2, 1, "Robert Wilson");
    model.set_value(2, 2, "HR");
    model.set_value(2, 3, false);
    model.set_value(2, 4, 45i64);
    model.set_value(2, 5, user_icon.clone());
    let dt3 = DateTime::new(2019, 4, 10, 14, 0, 0);
    model.set_value(2, 6, dt3);
    model.set_value(2, 7, 58i64);
    model.set_value(2, 8, "Contract");
    
    // Employee 4
    model.append_row();
    model.set_value(3, 0, "004");
    model.set_value(3, 1, "Susan Brown");
    model.set_value(3, 2, "Finance");
    model.set_value(3, 3, true);
    model.set_value(3, 4, 88i64);
    model.set_value(3, 5, user_icon.clone());
    let dt4 = DateTime::new(2022, 9, 5, 11, 45, 0);
    model.set_value(3, 6, dt4);
    model.set_value(3, 7, 80i64);
    model.set_value(3, 8, "Full-time");
    
    // Employee 5
    model.append_row();
    model.set_value(4, 0, "005");
    model.set_value(4, 1, "David Lee");
    model.set_value(4, 2, "IT Support");
    model.set_value(4, 3, true);
    model.set_value(4, 4, 62i64);
    model.set_value(4, 5, user_icon.clone());
    let dt5 = DateTime::new(2020, 11, 15, 8, 30, 0);
    model.set_value(4, 6, dt5);
    model.set_value(4, 7, 60i64);
    model.set_value(4, 8, "Contract");
    
    // Employee 6
    model.append_row();
    model.set_value(5, 0, "006");
    model.set_value(5, 1, "Elizabeth Taylor");
    model.set_value(5, 2, "Product");
    model.set_value(5, 3, true);
    model.set_value(5, 4, 82i64);
    model.set_value(5, 5, user_icon.clone());
    let dt6 = DateTime::now();
    model.set_value(5, 6, dt6);
    model.set_value(5, 7, 50i64);
    model.set_value(5, 8, "Contract");
} 