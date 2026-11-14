use gpui::{FontWeight, Styled};

use crate::app::ui::theme::theme;

/// Shared appearance for a selected choice, tab, or row. Keep semantics on
/// the control itself (`selected`, `toggled`, or `aria_selected`).
pub(crate) trait SelectionStyle: Styled + Sized {
    fn selection_style(self, selected: bool) -> Self {
        if selected {
            self.bg(theme().colors.selected_fill())
                .text_color(theme().colors.selected_text())
                .font_weight(FontWeight::SEMIBOLD)
        } else {
            self
        }
    }
}

impl<T: Styled> SelectionStyle for T {}
