use super::*;
use gpui::{Context, InteractiveElement as _, Render, VisualTestContext, px};

struct Harness {
    width: gpui::Pixels,
    selected: usize,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let entity = cx.entity().downgrade();
        let labels = [
            "Unavailable",
            "Neovim",
            "A deliberately long editor name that must fit",
        ];
        let items = labels
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                let entity = entity.clone();
                DropdownItem::new(label, move |_, cx| {
                    let _ = entity.update(cx, |this, cx| {
                        this.selected = index;
                        cx.notify();
                    });
                })
                .icon(AppIcon::Code)
                .selected(index == self.selected)
                .disabled(index == 0)
            })
            .collect();
        div().size_full().child(
            Dropdown::new("selector", labels[self.selected], items)
                .w(self.width)
                .debug_selector(|| "dropdown-trigger".into()),
        )
    }
}

fn draw(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, cx| window.draw(cx).clear(cx));
}

#[gpui::test]
fn dropdown_matches_resized_trigger_and_blocks_disabled_choices(cx: &mut gpui::TestAppContext) {
    cx.update(gpui_component::init);
    let (view, cx) = cx.add_window_view(|_, _| Harness {
        width: px(220.0),
        selected: 2,
    });
    for width in [220.0, 140.0] {
        view.update(cx, |this, cx| {
            this.width = px(width);
            this.selected = 2;
            cx.notify();
        });
        draw(cx);
        let trigger = cx.debug_bounds("dropdown-trigger").unwrap();
        cx.simulate_click(trigger.center(), Default::default());
        draw(cx);
        cx.update(|window, _| {
            let menu = gpui_base::test_support::find(window, &[], &"popup-menu".into()).unwrap();
            assert!(
                (f32::from(menu.bounds().size.width) - width).abs() <= 1.0,
                "menu must match resized trigger: {:?}",
                menu.bounds()
            );
        });
        cx.simulate_keystrokes("down enter");
        draw(cx);
        cx.update(|_, cx| {
            assert_eq!(view.read(cx).selected, 2, "disabled choice must not run");
        });
        cx.simulate_click(trigger.center(), Default::default());
        draw(cx);
        cx.simulate_keystrokes("down down enter");
        draw(cx);
        cx.update(|_, cx| {
            assert_eq!(
                view.read(cx).selected,
                1,
                "keyboard must select an enabled option"
            )
        });
    }
}
