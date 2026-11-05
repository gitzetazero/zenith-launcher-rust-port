//! The Zenith Launcher window: dark theme, sidebar with the stage-2 pages as placeholders.

use gpui::{prelude::*, *};
use zl_ui::theme::DARK;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Page {
    Home,
    Instances,
    Account,
    Settings,
}

impl Page {
    const ALL: [Page; 4] = [Page::Home, Page::Instances, Page::Account, Page::Settings];

    fn title(self) -> &'static str {
        match self {
            Page::Home => "Inicio",
            Page::Instances => "Instancias",
            Page::Account => "Cuenta",
            Page::Settings => "Ajustes",
        }
    }
}

struct Launcher {
    page: Page,
}

impl Render for Launcher {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = Page::ALL.iter().fold(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .w(px(180.))
                .p_3()
                .bg(rgb(DARK.surface)),
            |bar, &page| {
                let active = page == self.page;
                bar.child(
                    div()
                        .id(page.title())
                        .px_3()
                        .py_2()
                        .rounded_md()
                        .cursor_pointer()
                        .text_color(rgb(if active { DARK.text } else { DARK.text_muted }))
                        .bg(rgb(if active {
                            DARK.surface_hover
                        } else {
                            DARK.surface
                        }))
                        .hover(|s| s.bg(rgb(DARK.surface_hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.page = page;
                            cx.notify();
                        }))
                        .child(page.title()),
                )
            },
        );
        div()
            .flex()
            .size_full()
            .bg(rgb(DARK.background))
            .text_color(rgb(DARK.text))
            .child(sidebar)
            .child(
                div()
                    .flex_1()
                    .p_6()
                    .child(div().text_xl().child(self.page.title()))
                    .child(
                        div()
                            .mt_2()
                            .text_color(rgb(DARK.text_muted))
                            .child("Pendiente: esta pagina todavia no existe (etapa 2)."),
                    ),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(800.), px(400.))),
                titlebar: Some(TitlebarOptions {
                    title: Some(zl_core::APP_NAME.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Launcher { page: Page::Home }),
        )
        .expect("window");
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn every_page_has_a_unique_title() {
        let mut titles: Vec<_> = Page::ALL.iter().map(|p| p.title()).collect();
        titles.sort();
        titles.dedup();
        assert_eq!(titles.len(), Page::ALL.len());
    }
}
