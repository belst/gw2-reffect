use nexus::imgui::{
    sys, InputTextFlags, Selectable, StyleColor, StyleStackToken, StyleVar, TreeNode, Ui,
};
use std::{borrow::Cow, ffi::CString};
use strum::IntoEnumIterator;

pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]))
}

pub fn input_u32(ui: &Ui, label: impl AsRef<str>, value: &mut u32) {
    let mut int = *value as _;
    if ui.input_int(label, &mut int).step(0).step_fast(0).build() {
        *value = int as _;
    }
}

pub fn input_float_with_format(
    label: impl Into<String>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl Into<String>,
    flags: InputTextFlags,
) -> bool {
    if let (Ok(label), Ok(format)) = (CString::new(label.into()), CString::new(format.into())) {
        unsafe {
            sys::igInputFloat(
                label.as_ptr(),
                value as *mut f32,
                step,
                step_fast,
                format.as_ptr(),
                flags.bits() as i32,
            )
        }
    } else {
        false
    }
}

pub fn combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    all: impl IntoIterator<Item = T>,
    current: &mut T,
    item_label: impl Fn(&T) -> Cow<str>,
    item_color: impl Fn(&T) -> Option<[f32; 4]>,
) -> bool
where
    T: PartialEq,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo(label, item_label(current).as_ref()) {
        for entry in all {
            let selected = entry == *current;

            // apply color to selectable
            let style =
                item_color(&entry).map(|color| ui.push_style_color(StyleColor::Text, color));
            if Selectable::new(item_label(&entry).as_ref())
                .selected(selected)
                .build(ui)
            {
                changed = true;
                *current = entry;
            }
            drop(style);

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

pub fn enum_combo<T>(ui: &Ui, label: impl AsRef<str>, current: &mut T) -> bool
where
    T: PartialEq + AsRef<str> + IntoEnumIterator,
{
    combo(
        ui,
        label,
        T::iter(),
        current,
        |item| item.as_ref().into(),
        |_| None,
    )
}

pub fn tree_select(
    ui: &Ui,
    tree_id: impl AsRef<str>,
    select_label: impl AsRef<str>,
    selected: bool,
    leaf: bool,
    children: impl FnOnce(),
) -> bool {
    let _style = ui.push_style_var(StyleVar::IndentSpacing(10.0));
    let token = {
        let transparent = [0.0, 0.0, 0.0, 0.0];
        let _color = ui.push_style_color(StyleColor::Header, transparent);
        let _color = ui.push_style_color(StyleColor::HeaderHovered, transparent);
        let _color = ui.push_style_color(StyleColor::HeaderActive, transparent);

        TreeNode::new(tree_id)
            .label::<&str, _>("") // FIXME: unnecessary type param in imgui-rs
            .allow_item_overlap(true)
            .open_on_arrow(true)
            .default_open(false)
            .leaf(leaf)
            .push(ui)
    };

    ui.same_line();
    let clicked = Selectable::new(select_label)
        .close_popups(false)
        .selected(selected)
        .build(ui);

    if token.is_some() {
        children();
    }

    clicked
}
