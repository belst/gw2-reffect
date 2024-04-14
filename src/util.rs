use nexus::imgui::{sys, InputTextFlags, Selectable, StyleVar, TreeNode, Ui};
use std::{ffi::CString, mem};
use strum::VariantArray;

pub fn input_u32(ui: &Ui, label: impl AsRef<str>, value: &mut u32) {
    let mut int = *value as _;
    if ui.input_int(label, &mut int).step(0).step_fast(0).build() {
        if let Ok(new) = u32::try_from(int) {
            *value = new;
        }
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

pub trait EnumStaticVariants: Sized {
    fn static_variants() -> &'static [Self];
}

impl<T> EnumStaticVariants for T
where
    T: VariantArray,
{
    fn static_variants() -> &'static [Self] {
        Self::VARIANTS
    }
}

/// Helper to implement [`EnumStaticVariants`] for enums already implementing [`IntoEnumIterator`].
macro_rules! impl_static_variants {
    ($ty:ty) => {
        impl $crate::util::EnumStaticVariants for $ty {
            fn static_variants() -> &'static [Self] {
                use ::std::sync::OnceLock;
                use ::strum::IntoEnumIterator;

                static VARIANTS: OnceLock<Vec<$ty>> = OnceLock::new();
                VARIANTS.get_or_init(|| <Self as IntoEnumIterator>::iter().collect())
            }
        }
    };
}

pub(crate) use impl_static_variants;

pub fn enum_combo<T>(
    ui: &Ui,
    label: impl AsRef<str>,
    current: &mut T,
    flags: nexus::imgui::ComboBoxFlags,
) -> bool
where
    T: Clone + AsRef<str> + EnumStaticVariants + 'static,
{
    let mut changed = false;
    if let Some(_token) = ui.begin_combo_with_flags(label, current.as_ref(), flags) {
        for entry in T::static_variants() {
            // distinguish only discriminants
            let selected = mem::discriminant(entry) == mem::discriminant(current);
            if Selectable::new(entry.as_ref()).selected(selected).build(ui) {
                changed = true;
                *current = entry.clone();
            }

            // handle focus
            if selected {
                ui.set_item_default_focus();
            }
        }
    }
    changed
}

pub fn tree_select(
    ui: &Ui,
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    selected: bool,
    leaf: bool,
    children: impl FnOnce(),
) -> bool {
    let _style = ui.push_style_var(StyleVar::IndentSpacing(10.0));
    let token = TreeNode::new(id)
        .label::<&str, _>(label.as_ref()) // FIXME: unnecessary type param in imgui-rs
        .open_on_arrow(true)
        .selected(selected)
        .leaf(leaf)
        .tree_push_on_open(!leaf)
        .push(ui);
    let clicked = ui.is_item_clicked() && !ui.is_item_toggled_open();
    if token.is_some() {
        children();
    }
    clicked
}

pub fn item_context_menu(str_id: impl Into<String>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.into()) {
        if unsafe {
            sys::igBeginPopupContextItem(
                str_id.as_ptr(),
                sys::ImGuiPopupFlags_MouseButtonRight as _,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}
