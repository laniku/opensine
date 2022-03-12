#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
use std::process::Command;
use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Box, ListBox, Orientation};
#[repr(transparent)]
pub struct FileChooserDialog;

fn main() {
    let application = Application::builder()
        .application_id("cf.krnl286.lucas.opensine")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        let row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Open audio file")
            .build();
        row.connect_activated(|_| {
            Command::new("/usr/bin/aplay")
                    .arg("/dev/urandom")
                    .output()
                    .expect("failed to execute process");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        let content = Box::new(Orientation::Vertical, 0);
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("OpenSine", ""))
                .build(),
        );
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
