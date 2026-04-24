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
use adw::prelude::*;
use std::collections::HashSet;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/vani_tty1/sector/window.ui")]
    pub struct SectorWindow {
        #[template_child]
        pub partition_list: TemplateChild<gtk::ListBox>,

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
            let partition_list = self.partition_list.clone();
            let lvlbar = self.lvlbar.clone();
            let percent_label = self.percent.clone();

            lvlbar.set_min_value(0.0);
            lvlbar.set_max_value(100.0);

            glib::spawn_future_local(async move {
                match udisks::list_block_devices().await {
                    Ok(_) => {
                        let disks = sysinfo::Disks::new_with_refreshed_list();
                        let mut total_space: u64 = 0;
                        let mut available_space: u64 = 0;
                        let mut seen_devices: HashSet<String> = HashSet::new(); // track already-shown devices

                        for disk in disks.list() {
                            let device_name = disk.name().to_string_lossy().to_string();
                            let mount_point = disk.mount_point().to_string_lossy().to_string();
                        
                            let is_real_partition = device_name.starts_with("/dev/sd")
                                || device_name.starts_with("/dev/nvme")
                                || device_name.starts_with("/dev/hd")
                                || device_name.starts_with("/dev/vd")
                                || device_name.starts_with("/dev/mmcblk");
                        
                            if !is_real_partition {
                                continue;
                            }
                        
                            if seen_devices.contains(&device_name) {
                                continue;
                            }
                            seen_devices.insert(device_name.clone());
                        
                            let total = disk.total_space();
                            let available = disk.available_space();
                            let used = total.saturating_sub(available);
                        
                            total_space += total;
                            available_space += available;
                        
                            let usage_frac = if total > 0 {
                                used as f64 / total as f64
                            } else {
                                0.0
                            };
                        
                            // Resolve human-readable label just like Baobab does
                            let display_name = udisks::get_partition_label(&device_name).await;
                        
                            let subtitle = format!(
                                "{} • {:.2} GB used of {:.2} GB ({:.0}%)",
                                mount_point,
                                used as f64 / 1_000_000_000.0,
                                total as f64 / 1_000_000_000.0,
                                usage_frac * 100.0,
                            );
                        
                            let row = adw::ActionRow::builder()
                                .title(&display_name)   // e.g. "Fedora Linux" or "Home" instead of /dev/sda3
                                .subtitle(&subtitle)
                                .build();
                        
                            let disk_lvlbar = gtk::LevelBar::builder()
                                .min_value(0.0)
                                .max_value(1.0)
                                .value(usage_frac)
                                .valign(gtk::Align::Center)
                                .width_request(120)
                                .margin_end(12)
                                .build();
                        
                            row.add_suffix(&disk_lvlbar);
                            partition_list.append(&row);
                        }

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
                        let error_row = adw::ActionRow::builder()
                            .title("Error reading devices")
                            .subtitle(&e.to_string())
                            .build();
                        partition_list.append(&error_row);
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
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SectorWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
