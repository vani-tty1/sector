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
        pub label1: TemplateChild<gtk::Label>,

        #[template_child]
        pub label2: TemplateChild<gtk::Label>,

        #[template_child]
        pub lvlbar: TemplateChild<gtk::LevelBar>,

        #[template_child]
        pub percent: TemplateChild<gtk::Label>,
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
            let label = self.label2.clone();
            let lvlbar = self.lvlbar.clone();
            let percent_label = self.percent.clone();

            glib::spawn_future_local(async move {
                label.set_label("Scanning Disks");
                match udisks::list_block_devices().await {
                    Ok(mut devices) => {
                    //sort the devices according to the number
                        devices.sort_by(|a, b| {
                            let name_a = a.path.split('/').last().unwrap_or(&a.path);
                            let name_b = b.path.split('/').last().unwrap_or(&b.path);
                            match (name_a.parse::<u32>(), name_b.parse::<u32>()) {
                                (Ok(num_a), Ok(num_b)) => num_a.cmp(&num_b),
                                _ => name_a.cmp(name_b),
                            }
                        });
                        // this displays the devices in the box in which label2 is at
                        let mut result_text = format!("Found {} devices:\n\n", devices.len());
                        for dev in devices {
                            let size_gb =dev.size_bytes as f64 / 1_000_000_000.0;
                            let short_name = dev.path.split('/').last().unwrap_or(&dev.path);
                            result_text.push_str(&format!("{} — {:.2} GB\n", short_name.to_uppercase(), size_gb));
                        }
                        label.set_label(&result_text);
                        let disks = sysinfo::Disks::new_with_refreshed_list();
                        let mut total_space: u64 = 0;
                        let mut available_space: u64 = 0;
                        for disk in disks.list() {
                            total_space += disk.total_space();
                            available_space += disk.available_space();
                        }

                        // this where the percentage calculation is,
                        // god I love rust its much less cursed than C.
                        let used_space = total_space.saturating_sub(available_space);
                        let usage_percentage = if total_space > 0 {
                            (used_space as f64 / total_space as f64) * 100.0
                        } else {
                            0.0
                        };
                        lvlbar.set_value(usage_percentage);
                        percent_label.set_label(&format!("{:.0}%", usage_percentage));
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
