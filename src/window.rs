/* window.rs
 *
 * Copyright 2026 Giovanni
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use crate::udisks;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/vani_tty1/sector/window.ui")]
    pub struct SectorWindow {
        // Template widgets
        #[template_child]
        pub label: TemplateChild<gtk::Label>,

        #[template_child]
        pub lvlbar: TemplateChild<gtk::LevelBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SectorWindow {
        const NAME: &'static str = "SectorWindow";
        type Type = super::SectorWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }


    impl ObjectImpl for SectorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let label = self.label.clone();
            glib::spawn_future_local(async move {
                label.set_label("Scanning Disks");
                match udisks::list_block_devices().await {
                    Ok(devices) => {
                    let mut result_text = format!("Found {} devices:\n\n", devices.len());
                    for dev in devices {
                            let size_gb =dev.size_bytes as f64 / 1_000_000_000.0;
                            let short_name = dev.path.split('/').last().unwrap_or(&dev.path);
                            result_text.push_str(&format!("{} — {:.2} GB\n", short_name, size_gb));
                        }
                        label.set_label(&result_text);
                    }
                    Err(e) => {
                        label.set_label(&format!("Error: {}", e));
                    }
                }
            });
        }
    }
    impl WidgetImpl for SectorWindow {}
    impl WindowImpl for SectorWindow {}
    impl ApplicationWindowImpl for SectorWindow {}
    impl AdwApplicationWindowImpl for SectorWindow {}
}

glib::wrapper! {
    pub struct SectorWindow(ObjectSubclass<imp::SectorWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl SectorWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
