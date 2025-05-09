use wxdragon::prelude::*;
use wxdragon::widgets::panel::PanelStyle;

pub struct DataTabControls {
    pub panel: Panel,
    pub button: Button,
    pub data_display_label: StaticText,
}

pub fn create_data_tab(notebook: &Notebook, _frame: &Frame) -> DataTabControls {
    let data_panel = Panel::builder(notebook)
        .with_style(PanelStyle::TabTraversal)
        .build();
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

impl DataTabControls {
    pub fn bind_events(&self, frame: &Frame, status_bar: &StatusBar) {
        // Button click event
        let frame_clone_for_button = frame.clone();
        let data_display_label_clone = self.data_display_label.clone();
        let status_bar_clone_for_data = status_bar.clone();

        self.button
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
    }
}

#[derive(Debug, Default)]
pub struct FrameData {
    pub click_count: u32,
    pub message: String,
}
