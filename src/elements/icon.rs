use super::{IconSource, RenderState, TextAlign, TextDecoration};
use crate::{
    colors::{self, with_alpha},
    component_wise::ComponentWise,
    context::Context,
    render_util::spinner,
    traits::RenderOptions,
    trigger::{BuffTrigger, Trigger},
};
use nexus::imgui::{ColorEdit, ColorPreview, Style, StyleVar, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Icon {
    pub buff: BuffTrigger,

    #[serde(rename = "icon")]
    pub source: IconSource,

    pub stacks: bool,

    pub color: [f32; 4],
}

impl Icon {
    pub fn load(&mut self) {
        self.source.load();
    }

    fn texture_color(&self, ui: &Ui) -> [f32; 4] {
        let Style { alpha, .. } = ui.clone_style();
        let [r, g, b, a] = self.color;
        [r, g, b, a * alpha]
    }

    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        self.buff.is_active_or_edit(ctx, state)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState, size: [f32; 2]) {
        if let Some(texture) = self.source.get_texture() {
            // render icon
            let half_size = size.mul_scalar(0.5);
            let start = state.pos.sub(half_size);
            let end = state.pos.add(half_size);
            let color = self.texture_color(ui);
            ui.get_window_draw_list()
                .add_image(texture, start, end)
                .col(color)
                .build();

            // render stack count
            if self.stacks {
                if let Some(stacks) = self.buff.active_stacks_or_edit(ctx, state) {
                    // override alpha
                    let _style = ui.push_style_var(StyleVar::Alpha(0.8));

                    let text = stacks.to_string();

                    let [_, height] = size;
                    ui.set_window_font_scale(1.0);
                    let font_scale = 0.5 * height / ui.current_font_size();
                    ui.set_window_font_scale(font_scale);
                    let [x_offset, _] = TextAlign::Right.calc_pos(ui, &text);
                    let pad = [1.0, 1.0];
                    let line_height = ui.text_line_height();
                    let text_pos = end.add([x_offset, -line_height]).sub(pad);

                    let color = colors::WHITE;
                    let shadow_color = colors::BLACK;
                    ui.set_cursor_screen_pos(text_pos);
                    TextDecoration::Shadow.render(ui, &text, shadow_color);
                    ui.text_colored(color, &text);

                    ui.set_window_font_scale(1.0);
                }
            }
        } else {
            ui.set_cursor_screen_pos(state.pos);
            let [x, _] = size;
            spinner(ui, 0.4 * x, colors::WHITE, with_alpha(colors::WHITE, 0.3))
        }
    }
}

impl RenderOptions for Icon {
    fn render_options(&mut self, ui: &Ui) {
        self.buff.render_options(ui);

        self.source.render_select(ui);

        ColorEdit::new("Color", &mut self.color)
            .preview(ColorPreview::Alpha)
            .build(ui);

        ui.checkbox("Stacks", &mut self.stacks);
        // TODO: customizable stacks text offset
    }
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            source: IconSource::Unknown,
            stacks: false,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
