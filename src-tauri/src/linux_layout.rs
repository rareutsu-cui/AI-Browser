//! Linux の WebView を縦並びではなく矩形で配置する。
use gtk::prelude::*;
use std::cell::RefCell;
use tauri::{Rect, Runtime, Webview};

thread_local! {
    static LAST_BOUNDS: std::cell::RefCell<
        std::collections::HashMap<String, (i32, i32, i32, i32)>
    > = std::cell::RefCell::new(std::collections::HashMap::new());

    static CONTAINER: RefCell<Option<gtk::Fixed>> =
        const { RefCell::new(None) };
}

pub fn install<R: Runtime>(ui: &Webview<R>) -> tauri::Result<()> {
    ui.with_webview(|platform| {
        let widget = platform.inner();
        let Some(parent) = widget.parent()
            .and_then(|p| p.downcast::<gtk::Box>().ok()) else {
            log::error!("UI の GtkBox が見つかりません");
            return;
        };

        let fixed = gtk::Fixed::new();
        fixed.set_hexpand(true);
        fixed.set_vexpand(true);

        parent.remove(&widget);
        widget.set_size_request(1, 1);
        fixed.put(&widget, 0, 0);

        fixed.connect_size_allocate(move |_, allocation| {
            widget.size_allocate(&gtk::Allocation::new(
                0, 0,
                allocation.width().max(1),
                allocation.height().max(1),
            ));
        });

        parent.pack_start(&fixed, true, true, 0);
        CONTAINER.with(|slot| {
            *slot.borrow_mut() = Some(fixed.clone());
        });
        fixed.show_all();
    })
}

pub fn set_bounds<R: Runtime>(
    view: &Webview<R>,
    rect: Rect,
) -> tauri::Result<()> {
    let position = rect.position.to_logical::<i32>(1.0);
    let size = rect.size.to_logical::<i32>(1.0);

    let label = view.label().to_owned();
    view.with_webview(move |platform| {
        CONTAINER.with(|slot| {
            let slot = slot.borrow();
            let Some(fixed) = slot.as_ref() else {
                log::error!("Linux 配置コンテナが未初期化です");
                return;
            };


            let bounds = (
                position.x, position.y,
                size.width.max(1), size.height.max(1),
            );
            let unchanged = LAST_BOUNDS.with(|cache| {
                cache.borrow().get(&label).copied() == Some(bounds)
            });
            if unchanged {
                return;
            }
            let widget = platform.inner();
            let in_fixed =
                widget.parent().as_ref() == Some(fixed.upcast_ref());

            if !in_fixed {
                if let Some(parent) = widget.parent()
                    .and_then(|p| p.downcast::<gtk::Container>().ok()) {
                    parent.remove(&widget);
                }
                fixed.put(&widget, position.x, position.y);
            }

            widget.set_size_request(
                size.width.max(1), size.height.max(1),
            );
            fixed.move_(&widget, position.x, position.y);
            LAST_BOUNDS.with(|cache| {
                cache.borrow_mut().insert(label.clone(), bounds);
            });

            widget.size_allocate(&gtk::Allocation::new(
                position.x, position.y,
                size.width.max(1), size.height.max(1),
            ));
        });
    })
}
