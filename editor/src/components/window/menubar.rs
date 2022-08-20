/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use super::EchidnaWindow;
use crate::prelude::*;
use gio::{MenuModel, SimpleAction};
use glib::clone;
use gtk::AboutDialog;

pub trait MenubarImplementedEditor {
    fn setup_menubar(&self);
}

impl MenubarImplementedEditor for EchidnaWindow {
    fn setup_menubar(&self) {
        let app = self
            .application()
            .expect("self does not have an application set.");
        let menubuilder =
            gtk::Builder::from_resource("/io/fortressia/Echidna/components/window/menu.ui");
        let menubar: MenuModel = menubuilder
            .object("menu")
            .expect("Could not get object 'menu' from builder.");
        app.set_menubar(Some(&menubar));
        self.set_show_menubar(true);
        {
            let act_exit: SimpleAction = SimpleAction::new("exit", None);
            app.add_action(&act_exit);

            act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                    app.quit();
                }
            ));
        }
        {
            let act_about: SimpleAction = SimpleAction::new("about", None);
            app.add_action(&act_about);
            act_about.connect_activate(|_action, _value| {
                let about_dialog: AboutDialog = AboutDialog::new();

                about_dialog.set_license_type(gtk::License::Mpl20);
                about_dialog.set_program_name(Some("Echidna Code Editor"));
                about_dialog.set_website(Some("https://gitlab.com/EchidnaHQ/Echidna"));
                about_dialog.set_authors(&["FortressValkriye"]);
                about_dialog.set_copyright(Some("Made with by ❤️ Echidna contributors"));
                about_dialog.set_visible(true);
            });
        }
        {
            //app.notebook = Some(Rc::new(RefCell::new(notebook)));
            let act_exit: SimpleAction = SimpleAction::new("exit", None);
            app.add_action(&act_exit);

            act_exit.connect_activate(clone!(@weak app =>
                move |_action, _value| {
                    app.quit();
                }
            ));
        }
        {
            let act_about: SimpleAction = SimpleAction::new("about", None);
            app.add_action(&act_about);
            act_about.connect_activate(clone!(@weak app => 
                move |_action, _value| {
                AboutDialog::builder()
                .license_type(gtk::License::Mpl20)
                .program_name("Echidna Code")
                .website("https://github.com/EchidnaHQ/Echidna")
                .authors(vec!["Nefo Fortressia".to_string()])
                .copyright("Made with by ❤️ Echidna contributors")
                .logo_icon_name(&app.application_id().unwrap())
                .version(crate::config::VERSION)
                .visible(true)
                .build();
            }));
        }
        {
            let act_report_issue = SimpleAction::new("report-issue", None);

            app.add_action(&act_report_issue);

            act_report_issue.connect_activate(clone!(@weak self as win =>
                move |_action, _variant| {
                gtk::show_uri(Some(&win), "https://github.com/EchidnaHQ/Echidna/components/issues/new", gdk::CURRENT_TIME);
            }));
        }
        {
            let act_search_feature_requests = SimpleAction::new("search-feature-requests", None);

            app.add_action(&act_search_feature_requests);

            act_search_feature_requests.connect_activate(clone!(@weak self as win =>
                move |_action, _variant| {
                gtk::show_uri(Some(&win), "https://github.com/EchidnaHQ/Echidna/components/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement", gdk::CURRENT_TIME);
            }));
        }
        {
            let act_window_close = SimpleAction::new("close", None);

            self.add_action(&act_window_close);

            act_window_close.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                window.close();
            }));
        }
        {
            let action_open_file: SimpleAction = SimpleAction::new("open-file", None);

            self.add_action(&action_open_file);
            action_open_file.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                    window.action_open_file();
            }));
        }
        {
            let action_save_file_as = SimpleAction::new("save-file-as", None);

            self.add_action(&action_save_file_as);

            action_save_file_as.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                    window.action_save_file_as();
            }));
        }
        {
            let action_new_file = SimpleAction::new("new-file", None);

            self.add_action(&action_new_file);

            action_new_file.connect_activate(clone!(@weak self as window =>
                move |_action, _variant| {
                    window.action_new_file();
            }));
        }
        {
            let action_save = SimpleAction::new("save", None);

            self.add_action(&action_save);

            action_save.connect_activate(clone!(@weak self as window =>
                move |_, _| {
                    window.action_save_file();
                }
            ));
        }
    }
}
