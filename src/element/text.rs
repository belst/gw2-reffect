use super::{util::add_pos, Context, Render};
use crate::trigger::BuffTrigger;
use nexus::imgui::{ImColor32, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Text {
    pub buff: BuffTrigger,
    pub offset: [f32; 2],
    pub color: [u8; 4],
    pub text: String,
}

mod replace {
    pub const STACKS: &str = "%stacks";
}

impl Text {
    pub fn color(&self) -> ImColor32 {
        let [r, g, b, a] = self.color;
        ImColor32::from_rgba(r, g, b, a)
    }

    pub fn process_text(&self, ctx: &Context) -> Option<String> {
        self.buff
            .get_stacks(ctx)
            .map(|stacks| self.text.replace(replace::STACKS, &stacks.to_string()))
    }
}

impl Render for Text {
    fn load(&mut self) {}

    fn render(&mut self, ui: &Ui, ctx: &Context) {
        if let Some(text) = self.process_text(ctx) {
            let cursor = ui.cursor_screen_pos();
            let draw_list = ui.get_window_draw_list();
            let pos = add_pos(cursor, self.offset);
            draw_list.add_text(pos, self.color(), text);
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            buff: BuffTrigger::default(),
            offset: [0.0, 0.0],
            color: [255, 255, 255, 255],
            text: String::new(),
        }
    }
}
