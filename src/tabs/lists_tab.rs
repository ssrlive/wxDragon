use wxdragon::id;
use wxdragon::prelude::*;
use wxdragon::widgets::checklistbox::CheckListBoxStyle;
use wxdragon::widgets::choice::ChoiceStyle;
use wxdragon::widgets::combobox::ComboBoxStyle;
use wxdragon::widgets::editablelistbox::{EditableListBox, EditableListBoxStyle};
use wxdragon::widgets::listbox::ListBoxStyle;
use wxdragon::widgets::list_ctrl::{ListCtrl, ListCtrlStyle, ListColumnFormat, ListItemState};
use wxdragon::widgets::panel::PanelStyle;
use wxdragon::HasItemData;
use wxdragon::widgets::{
    Button, CheckListBox, Choice, ComboBox, ListBox, 
    ListCtrlBuilder, ScrolledWindow, StaticText
};

// Define a custom data type for our list items 