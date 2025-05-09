use wxdragon::prelude::*;
use wxdragon::widgets::gauge::GaugeStyle;
use wxdragon::widgets::slider::SliderStyle;
use wxdragon::widgets::spinctrl::SpinCtrlStyle;
use wxdragon::widgets::splitterwindow::SplitterWindowStyle;
// use wxdragon::widgets::splitterwindow; // For SP_XXX constants if available
// use wxdragon::widgets::slider; // For SL_XXX constants if available
// use wxdragon::widgets::spinctrl; // For SP_XXX constants if available

pub struct AdvancedTabControls {
    pub tree_ctrl: TreeCtrl,
    pub tree_status_label: StaticText,
    pub gauge: Gauge,
    pub gauge_increase_btn: Button,
    pub gauge_reset_btn: Button,
    pub gauge_status_label: StaticText,
    pub slider: Slider,
    pub slider_label: StaticText,
    pub spin_ctrl: SpinCtrl,
    pub spin_ctrl_label: StaticText,
}

pub fn create_advanced_tab(notebook: &Notebook) -> (SplitterWindow, AdvancedTabControls) {
    // Create a SplitterWindow instead of a Panel for this tab's main container
    let splitter = SplitterWindow::builder(notebook)
        // .with_style(SP_LIVE_UPDATE | SP_BORDER | SP_3D) // Old - Commenting out for now
        .with_id(200) // Give splitter an ID
        .with_style(SplitterWindowStyle::LiveUpdate | SplitterWindowStyle::Default)
        .with_size(Size::new(400, 200))
        .build();

    // Create Panel 1 (Left: Tree)
    let tree_panel = Panel::builder(&splitter).build();
    let tree_ctrl = TreeCtrl::builder(&tree_panel)
        .with_id(111)
        .with_style(TR_DEFAULT_STYLE | TR_HAS_BUTTONS | TR_LINES_AT_ROOT)
        .build();
    if let Some(root_id) = tree_ctrl.add_root("Root Node") {
        if let Some(child1_id) = tree_ctrl.append_item(&root_id, "Child 1") {
            tree_ctrl.append_item(&child1_id, "Grandchild 1.1");
            tree_ctrl.append_item(&child1_id, "Grandchild 1.2");
        }
        tree_ctrl.append_item(&root_id, "Child 2");
    }
    let tree_status_label = StaticText::builder(&tree_panel)
        .with_label("Tree Selection: None")
        .build();

    // Sizer for Tree Panel
    let tree_sizer = BoxSizer::builder(VERTICAL).build();
    tree_sizer.add(&tree_ctrl, 1, EXPAND | ALL, 5);
    tree_sizer.add(&tree_status_label, 0, EXPAND | ALL, 5); // Expand label horizontally
    tree_panel.set_sizer(tree_sizer, true);

    // Create Panel 2 (Right: Gauge, Slider, Spin)
    let controls_panel = Panel::builder(&splitter).build();
    let gauge = Gauge::builder(&controls_panel)
        .with_id(112)
        .with_range(100)
        // .with_style(GA_HORIZONTAL | GA_SMOOTH) // Old - Commenting out for now
        .with_style(GaugeStyle::Default | GaugeStyle::Smooth)
        .with_size(200, 25)
        .build();
    gauge.set_value(25);
    let gauge_increase_btn = Button::builder(&controls_panel)
        .with_label("Increase")
        .build();
    let gauge_reset_btn = Button::builder(&controls_panel).with_label("Reset").build();
    let gauge_status_label = StaticText::builder(&controls_panel)
        .with_label("Gauge Value: 25%")
        .build();
    let slider_label = StaticText::builder(&controls_panel)
        .with_label("Slider Value: 50")
        .build();
    let slider = Slider::builder(&controls_panel)
        .with_id(113)
        .with_value(50)
        .with_min_value(0)
        .with_max_value(200)
        // .with_style(SL_HORIZONTAL | SL_LABELS) // Old - Commenting out for now
        .with_style(SliderStyle::Default | SliderStyle::Labels)
        .with_size(Size::new(-1, -1))
        .build(); // Let slider expand
    let spin_ctrl_label = StaticText::builder(&controls_panel)
        .with_label("Spin Value: 10")
        .build();
    let spin_ctrl = SpinCtrl::builder(&controls_panel)
        .with_id(114)
        .with_range(0, 50)
        .with_initial_value(10)
        // .with_style(SP_ARROW_KEYS | SP_WRAP) // Old - Commenting out for now
        .with_style(SpinCtrlStyle::Horizontal | SpinCtrlStyle::ArrowKeys | SpinCtrlStyle::Wrap)
        .with_size(Size::new(80, -1))
        .build();

    // Sizer for Controls Panel
    let controls_sizer = BoxSizer::builder(VERTICAL).build();

    let gauge_sizer = BoxSizer::builder(HORIZONTAL).build();
    gauge_sizer.add(&gauge, 1, EXPAND | ALL, 5);
    let gauge_buttons_sizer = BoxSizer::builder(VERTICAL).build();
    gauge_buttons_sizer.add(&gauge_increase_btn, 0, ALL, 2);
    gauge_buttons_sizer.add(&gauge_reset_btn, 0, ALL, 2);
    gauge_sizer.add_sizer(&gauge_buttons_sizer, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    gauge_sizer.add(&gauge_status_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    controls_sizer.add_sizer(&gauge_sizer, 0, EXPAND | ALL, 5);

    let slider_spin_sizer = BoxSizer::builder(HORIZONTAL).build();
    slider_spin_sizer.add(&slider, 1, EXPAND | ALL, 5);
    slider_spin_sizer.add(&slider_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    slider_spin_sizer.add_spacer(20);
    slider_spin_sizer.add(&spin_ctrl, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    slider_spin_sizer.add(&spin_ctrl_label, 0, ALL | ALIGN_CENTER_VERTICAL, 5);
    controls_sizer.add_sizer(&slider_spin_sizer, 0, EXPAND | ALL, 5);

    controls_panel.set_sizer(controls_sizer, true);

    // Split the window
    splitter.split_vertically(&tree_panel, &controls_panel, 150); // 150 pixels for the tree initially
    splitter.set_minimum_pane_size(50); // Set minimum size for both panes

    // Return the splitter AND the controls struct
    (
        splitter,
        AdvancedTabControls {
            tree_ctrl,
            tree_status_label,
            gauge,
            gauge_increase_btn,
            gauge_reset_btn,
            gauge_status_label,
            slider,
            slider_label,
            spin_ctrl,
            spin_ctrl_label,
        },
    )
}

impl AdvancedTabControls {
    pub fn bind_events(&self) {
        // TreeCtrl Selection Changed event
        let tree_status_label_clone = self.tree_status_label.clone();
        self.tree_ctrl
            .bind(EventType::TREE_SEL_CHANGED, move |event: Event| {
                if let Some(selected_item) = event.get_item() {
                    let mut status = String::new();
                    std::fmt::Write::write_fmt(
                        &mut status,
                        format_args!("Tree Selection: Item {:?}", selected_item),
                    )
                    .unwrap();
                    tree_status_label_clone.set_label(&status);
                } else {
                    tree_status_label_clone.set_label("Tree Selection: None");
                }
            });

        // Gauge button events
        let gauge_clone_for_inc = self.gauge.clone();
        let gauge_status_label_clone_for_inc = self.gauge_status_label.clone();
        self.gauge_increase_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_: Event| {
                let current_value = gauge_clone_for_inc.get_value();
                let new_value = std::cmp::min(current_value + 10, 100);
                gauge_clone_for_inc.set_value(new_value);
                gauge_status_label_clone_for_inc.set_label(&format!("Gauge Value: {}%", new_value));
            });

        let gauge_clone_for_reset = self.gauge.clone();
        let gauge_status_label_clone_for_reset = self.gauge_status_label.clone();
        self.gauge_reset_btn
            .bind(EventType::COMMAND_BUTTON_CLICKED, move |_: Event| {
                gauge_clone_for_reset.set_value(0);
                gauge_status_label_clone_for_reset.set_label("Gauge Value: 0%");
            });

        // Slider Event Binding
        let slider_label_clone = self.slider_label.clone();
        self.slider.bind(EventType::SLIDER, move |event| {
            if let Some(value) = event.get_int() {
                slider_label_clone.set_label(&format!("Slider Value: {}", value));
            }
        });

        // SpinCtrl Event Binding
        let spin_ctrl_label_clone = self.spin_ctrl_label.clone();
        self.spin_ctrl.bind(EventType::SPINCTRL, move |event| {
            if let Some(value) = event.get_int() {
                spin_ctrl_label_clone.set_label(&format!("Spin Value: {}", value));
                println!(
                    "SPINCTRL Event (Advanced Tab): ID: {}, Value: {}",
                    event.get_id(),
                    value
                );
            }
        });
    }
}
