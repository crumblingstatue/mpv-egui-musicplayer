use std::sync::Mutex;

mod app;
mod bool_ext;
mod config;
mod ipc;
mod mpv_handler;
mod rect_math;
mod runner;

static MODAL: Mutex<Option<egui_modal::Modal>> = Mutex::new(None);

fn warn_dialog(title: &str, desc: &str) {
    let Some(modal) = &mut *MODAL.lock().unwrap() else {
        eprintln!("Dialog not yet init. warn: {title}: {desc}");
        return;
    };
    modal
        .dialog()
        .with_title(title)
        .with_icon(egui_modal::Icon::Warning)
        .with_body(desc)
        .open();
}

fn main() {
    runner::run(740, 500, "mpv-egui-musicplayer");
}
