use gtk4::{glib, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    prelude::*,
};

struct App {
    menu_open: bool,
    counter: u8,
    formatted_time: glib::GString,
}

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
    UpdateStartMenuState(bool),
    UpdateTime, // Add new message for updating time
}

fn format_now(format: &str) -> glib::GString {
    let now = glib::DateTime::now_local().unwrap();
    now.format(format).unwrap()
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = (bool, u8, glib::GString);
    type Input = Msg;
    type Output = ();

    view! {
        #[root]
        taskbar_window = gtk::Window {
            // Do gtk4_layer_shell stuff first
            init_layer_shell: (),
            set_layer: Layer::Top,
            auto_exclusive_zone_enable: (),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Top, true),

            // Now the normal gtk stuff
            set_title: Some("RDE Panel"),
            set_default_size: (1024, 32),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 5,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: true,

                    gtk::MenuButton {
                        set_label: "Start",
                        #[wrap(Some)]
                        set_popover = &gtk::PopoverMenu::from_model(Some(&start_menu)) {
                            add_child: (&popover_child, "my_widget"),
                            set_has_arrow: false,

                            connect_map => Msg::UpdateStartMenuState(true),
                            connect_unmap => Msg::UpdateStartMenuState(false),
                        },
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("Menu is open: {}, counter is: {}", model.menu_open, model.counter),
                        set_margin_all: 5,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Label {
                        #[watch]
                        set_label: &model.formatted_time,
                    },
                }
            }
        },
        popover_child = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            gtk::Label {
                set_label: "Redmond Desktop Environment"
            },

        },
    }

    menu! {
        start_menu: {
            "Firefox" => ExampleAction,
            section!{
                "Programs" => ExampleAction,
                "Documents" => ExampleAction,
                "Settings" => ExampleAction,
                "Search" => ExampleAction,
                "Help" => ExampleAction,
                "Run..." => ExampleAction,
            },
            section! {
                &format!("Log Off {}...", glib::user_name().to_string_lossy()) => ExitAction,
                "Shut Down..." => ShutDownAction,
            },
        }
    }

    // Initialize the component.
    fn init(
        init_values: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            menu_open: init_values.0,
            counter: init_values.1,
            formatted_time: init_values.2,
        };
        let widgets = view_output!();

        let app = relm4::main_application();
        app.set_accelerators_for_action::<ExampleAction>(&["<primary>W"]);

        let sender_clone = sender.clone();
        let action: RelmAction<ExampleAction> = {
            RelmAction::new_stateless(move |_| {
                println!("Statelesss action!");
                sender_clone.input(Msg::Increment);
            })
        };

        let action2: RelmAction<ExampleU8Action> =
            RelmAction::new_stateful_with_target_value(&0, |_, state, _value| {
                *state ^= 1;
                dbg!(state);
            });

        let exit_action: RelmAction<ExitAction> = RelmAction::new_stateless(move |_| {
            println!("Logging off...");
            std::process::Command::new("labwc")
                .arg("--exit")
                .spawn()
                .expect("Failed to execute labwc --exit, is it installed?");
        });

        let shutdown_action: RelmAction<ShutDownAction> = RelmAction::new_stateless(move |_| {
            println!("Shutting down...");
        });

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(action);
        group.add_action(action2);
        group.add_action(exit_action);
        group.add_action(shutdown_action);
        group.register_for_widget(&widgets.taskbar_window);

        // Set up a timer to send UpdateTime message every second
        let sender_clone = sender.clone();
        glib::timeout_add_seconds_local(1, move || {
            sender_clone.input(Msg::UpdateTime);
            glib::ControlFlow::Continue
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::UpdateStartMenuState(is_open) => {
                self.menu_open = is_open;
            }
            Msg::Increment => {
                self.counter += 1;
            }
            Msg::Decrement => {
                self.counter -= 1;
            }
            Msg::UpdateTime => {
                self.formatted_time = format_now("%H:%M");
            }
        }
    }
}

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ExampleAction, WindowActionGroup, "example");
relm4::new_stateless_action!(ExitAction, WindowActionGroup, "exit");
relm4::new_stateless_action!(ShutDownAction, WindowActionGroup, "shutdown");
relm4::new_stateful_action!(ExampleU8Action, WindowActionGroup, "example2", u8, u8);

fn main() {
    let app = RelmApp::new("com.remcokranenburg.RdePanel");
    app.run::<App>((false, 0, format_now("%H:%M")));
}
