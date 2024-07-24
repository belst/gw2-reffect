mod amount_type;
mod threshold_type;

pub use self::{amount_type::*, threshold_type::*};

use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, helper},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgressThreshold {
    /// Threshold type.
    pub threshold_type: ThresholdType,

    /// Amount type.
    pub amount_type: AmountType,
}

impl ProgressThreshold {
    pub fn is_met(&self, active: &ProgressActive, ctx: &Context) -> bool {
        let amount = self.amount_type.amount(active, ctx);
        match self.threshold_type {
            ThresholdType::Always => true,
            ThresholdType::Present => active.intensity() > 0, // we use intensity for present checks
            ThresholdType::Missing => active.intensity() == 0,
            ThresholdType::Min(required) => amount >= required,
            ThresholdType::Max(required) => amount <= required,
            ThresholdType::Exact(required) => amount == required,
            ThresholdType::Between(min, max) => (min..=max).contains(&amount),
        }
    }
}

impl From<ThresholdType> for ProgressThreshold {
    fn from(threshold_type: ThresholdType) -> Self {
        Self {
            threshold_type,
            amount_type: AmountType::Intensity,
        }
    }
}

impl RenderOptions for ProgressThreshold {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        enum_combo(
            ui,
            "Threshold",
            &mut self.threshold_type,
            ComboBoxFlags::empty(),
        );
        helper(ui, || ui.text("When to display"));

        match &mut self.threshold_type {
            ThresholdType::Always | ThresholdType::Present | ThresholdType::Missing => {}
            ThresholdType::Min(required)
            | ThresholdType::Max(required)
            | ThresholdType::Exact(required) => {
                self.amount_type.render_options(ui, state);
                self.amount_type.render_input(ui, "Amount", required);
            }
            ThresholdType::Between(min, max) => {
                self.amount_type.render_options(ui, state);
                self.amount_type.render_input(ui, "Min", min);
                self.amount_type.render_input(ui, "Max", max);
            }
        }
    }
}