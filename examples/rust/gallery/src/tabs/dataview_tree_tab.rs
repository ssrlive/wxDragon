use wxdragon::prelude::*;

// Helper function to create a valid bitmap for this tab
fn create_icon(art_id: ArtId) -> Bitmap {
    ArtProvider::get_bitmap(art_id, ArtClient::MessageBox, Some(Size::new(16, 16))).unwrap_or_else(
        || {
            // Fallback if ArtProvider fails for some reason
            let fallback_rgba = [0, 0, 0, 255]; // Black square
            Bitmap::from_rgba(&fallback_rgba, 1, 1)
                .expect("Failed to create ultimate fallback bitmap")
        },
    )
}

pub struct DataViewTreeTabControls {
    pub panel: Panel,
}

pub fn create_dataview_tree_tab(parent: &impl WxWidget) -> DataViewTreeTabControls {
    let panel = Panel::builder(parent).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    let info_text = StaticText::builder(&panel)
        .with_label("This tab demonstrates DataViewTreeCtrl with an ImageList.")
        .build();
    sizer.add(&info_text, 0, SizerFlag::All | SizerFlag::Expand, 10);

    let image_list = ImageList::new(16, 16, true, 5);

    let icon_folder = create_icon(ArtId::Folder);
    let icon_file = create_icon(ArtId::NormalFile);
    let icon_settings = create_icon(ArtId::HelpSettings);

    let idx_folder = image_list.add_bitmap(&icon_folder);
    let idx_file = image_list.add_bitmap(&icon_file);
    let idx_settings = image_list.add_bitmap(&icon_settings);

    let dvc_tree = DataViewTreeCtrl::builder(&panel).build();

    // Create all column objects first
    let expander_renderer = DataViewIconTextRenderer::new(
        VariantType::IconText,
        DataViewCellMode::Inert,
        DataViewAlign::Left,
    );
    let expander_col = DataViewColumn::new(
        "Hierarchy",                    // Header text for the tree column
        &expander_renderer,             // Renderer
        0,                              // Model column 0 (for the main item text/icon)
        200,                            // Initial width
        DataViewAlign::Left,            // Alignment
        DataViewColumnFlags::Resizable, // Flags (make it resizable)
    );

    let aux_renderer = DataViewIconTextRenderer::new(
        VariantType::IconText,
        DataViewCellMode::Inert,
        DataViewAlign::Left,
    );
    let aux_col = DataViewColumn::new(
        "Auxiliary Info (IconText)",    // Initial title, will be overridden
        &aux_renderer,                  // renderer
        1,                              // model_column
        150,                            // width
        DataViewAlign::Left,            // align
        DataViewColumnFlags::Resizable, // flags
    );

    // Add columns in order
    dvc_tree.prepend_column(&expander_col);
    dvc_tree.append_column(&aux_col);

    // Set expander column *after* all columns are added
    dvc_tree.set_expander_column(&expander_col);

    // Explicitly set properties on the second column (index 1) due to wxWidgets quirk
    if let Some(col_to_fix) = dvc_tree.get_column(1) {
        col_to_fix.set_title("Auxiliary Info (Explicit)");
        col_to_fix.set_resizeable(true);
    } else {
        println!("Could not get column at index 1 to fix properties.");
    }

    dvc_tree.set_image_list(image_list);
    let root_item = DataViewItem::new_invalid();

    let cat_a = dvc_tree.append_container(&root_item, "Category A", idx_folder, idx_folder);
    let _item_a1 = dvc_tree.append_item(&cat_a, "Item A.1 (File)", idx_file);
    dvc_tree.append_item(&cat_a, "Item A.2 (Settings)", idx_settings);

    let cat_b = dvc_tree.append_container(&root_item, "Category B", idx_folder, idx_folder);
    let sub_cat_b1 = dvc_tree.append_container(&cat_b, "Sub-Category B.1", idx_folder, idx_folder);
    dvc_tree.append_item(&sub_cat_b1, "Item B.1.1 (File)", idx_file);
    dvc_tree.append_item(&cat_b, "Item B.2 (File)", idx_file);

    let _cat_c =
        dvc_tree.append_container(&root_item, "Category C (Empty)", idx_folder, idx_folder);

    dvc_tree.expand(&cat_a);
    dvc_tree.expand(&cat_b);
    dvc_tree.expand(&sub_cat_b1);

    sizer.add(&dvc_tree, 1, SizerFlag::All | SizerFlag::Expand, 10);
    panel.set_sizer(sizer, true);

    DataViewTreeTabControls { panel }
}
