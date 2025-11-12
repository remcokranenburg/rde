use gtk4::{
    gio,
    glib::{self, Variant},
    prelude::*,
};
use relm4::{
    actions::{ActionablePlus, RelmAction, RelmActionGroup},
    new_action_group, new_stateful_action, new_stateless_action,
    prelude::*,
};

const WIDGET_HEIGHT: i32 = 28;
const WIDGET_SPACING: u8 = 8;

#[derive(Clone, Debug, Default, Variant)]
enum CalculatorView {
    #[default]
    Standard,
    Scientific,
}

#[derive(Clone, Debug, Default, Variant)]
enum NumberBase {
    Hex,
    #[default]
    Dec,
    Oct,
    Bin,
}

#[derive(Debug, Default)]
struct App {
    view: CalculatorView,
    buffer: gtk::EntryBuffer,
    number_base: NumberBase,
}

#[derive(Debug)]
enum Msg {
    UpdateView(CalculatorView),
    UpdateNumberBase(NumberBase),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Calculator"),
            set_default_size: (1, 1),
            set_resizable: false,
            set_show_menubar: true,
            // set_valign: gtk::Align::End,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 10,
                set_spacing: WIDGET_SPACING.into(),

                gtk::Separator {
                    set_orientation: gtk::Orientation::Horizontal,
                },

                gtk::Entry::with_buffer(&model.buffer) {
                    gtk4::prelude::EntryExt::set_alignment: 1.0,
                    set_height_request: WIDGET_HEIGHT,
                },

                gtk::Stack {
                    set_transition_type: gtk::StackTransitionType::SlideUpDown,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_interpolate_size: true,
                    set_hhomogeneous: false,
                    set_vhomogeneous: false,

                    // Standard Calculator View
                    add_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: WIDGET_SPACING.into(),
                            set_homogeneous: true,

                            gtk::Entry {
                                set_editable: false,
                                set_sensitive: false,
                            },

                            gtk::Button {
                                set_label: "MC",
                            },

                            gtk::Button {
                                set_label: "MR",
                            },

                            gtk::Button {
                                set_label: "MS",
                            },

                            gtk::Button {
                                set_label: "M+",
                            },

                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: WIDGET_SPACING.into(),
                            set_margin_start: 18,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: WIDGET_SPACING.into(),
                                set_homogeneous: true,
                                set_halign: gtk::Align::End,

                                gtk::Button {
                                    set_label: "Back",
                                    set_height_request: WIDGET_HEIGHT,
                                },

                                gtk::Button {
                                    set_label: "CE",
                                },

                                gtk::Button {
                                    set_label: "C",
                                },
                            },

                            gtk::Grid {
                                set_row_spacing: WIDGET_SPACING.into(),
                                set_column_spacing: WIDGET_SPACING.into(),
                                set_row_homogeneous: true,
                                set_column_homogeneous: true,

                                attach[0,0,1,1] = &gtk::Button {
                                    set_label: "7",
                                    set_height_request: WIDGET_HEIGHT,
                                },

                                attach[1,0,1,1] = &gtk::Button {
                                    set_label: "8",
                                },

                                attach[2,0,1,1] = &gtk::Button {
                                    set_label: "9",
                                },

                                attach[3,0,1,1] = &gtk::Button {
                                    set_label: "/",
                                },

                                attach[4,0,1,1] = &gtk::Button {
                                    set_label: "√",
                                },

                                attach[0,1,1,1] = &gtk::Button {
                                    set_label: "4",
                                },

                                attach[1,1,1,1] = &gtk::Button {
                                    set_label: "5",
                                },

                                attach[2,1,1,1] = &gtk::Button {
                                    set_label: "6",
                                },

                                attach[3,1,1,1] = &gtk::Button {
                                    set_label: "*",
                                },

                                attach[4,1,1,1] = &gtk::Button {
                                    set_label: "%",
                                },

                                attach[0,2,1,1] = &gtk::Button {
                                    set_label: "1",
                                },

                                attach[1,2,1,1] = &gtk::Button {
                                    set_label: "2",
                                },

                                attach[2,2,1,1] = &gtk::Button {
                                    set_label: "3",
                                },

                                attach[3,2,1,1] = &gtk::Button {
                                    set_label: "-",
                                },

                                attach[4,2,1,1] = &gtk::Button {
                                    set_label: "1/x",
                                },

                                attach[0,3,1,1] = &gtk::Button {
                                    set_label: "0",
                                },

                                attach[1,3,1,1] = &gtk::Button {
                                    set_label: "±",
                                },

                                attach[2,3,1,1] = &gtk::Button {
                                    set_label: ".",
                                },

                                attach[3,3,1,1] = &gtk::Button {
                                    set_label: "+",
                                },

                                attach[4,3,1,1] = &gtk::Button {
                                    set_label: "=",
                                },
                            },
                        },
                    } -> {
                        set_name: "standard_view",
                    },

                    add_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: WIDGET_SPACING.into(),

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: WIDGET_SPACING.into(),
                            set_homogeneous: true,

                            gtk::Frame {
                                set_height_request: WIDGET_HEIGHT,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_homogeneous: true,

                                    gtk::CheckButton {
                                        set_label: Some("Hex"),
                                        ActionablePlus::set_action::<NumberBaseAction>: NumberBase::Hex,
                                    },
                                    gtk::CheckButton {
                                        set_label: Some("Dec"),
                                        ActionablePlus::set_action::<NumberBaseAction>: NumberBase::Dec,
                                    },
                                    gtk::CheckButton {
                                        set_label: Some("Oct"),
                                        ActionablePlus::set_action::<NumberBaseAction>: NumberBase::Oct,
                                    },
                                    gtk::CheckButton {
                                        set_label: Some("Bin"),
                                        ActionablePlus::set_action::<NumberBaseAction>: NumberBase::Bin,
                                    },
                                },
                            },

                            gtk::Frame {
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_homogeneous: true,

                                    gtk::CheckButton {
                                        set_label: Some("Deg"),
                                    },
                                    gtk::CheckButton {
                                        set_label: Some("Rad"),
                                    },
                                    gtk::CheckButton {
                                        set_label: Some("Grad"),
                                    },
                                },
                            },
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: WIDGET_SPACING.into(),

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: WIDGET_SPACING.into(),

                                    gtk::Frame {
                                        gtk::Box {
                                            set_orientation: gtk::Orientation::Horizontal,
                                            set_homogeneous: true,

                                            gtk::CheckButton {
                                                set_label: Some("Inv"),
                                            },
                                            gtk::CheckButton {
                                                set_label: Some("Hyp"),
                                            },
                                        },
                                    },

                                    gtk::Entry {
                                        set_editable: false,
                                        set_sensitive: false,
                                        set_height_request: WIDGET_HEIGHT,
                                    },
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,

                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_spacing: WIDGET_SPACING.into(),
                                        set_homogeneous: true,

                                        gtk::Button {
                                            set_label: "Sta",
                                        },

                                        gtk::Button {
                                            set_label: "Ave",
                                        },

                                        gtk::Button {
                                            set_label: "Sum",
                                        },

                                        gtk::Button {
                                            set_label: "s",
                                        },

                                        gtk::Button {
                                            set_label: "Dat",
                                        },
                                    },

                                    gtk::Grid {
                                        set_row_spacing: WIDGET_SPACING.into(),
                                        set_column_spacing: WIDGET_SPACING.into(),
                                        set_row_homogeneous: true,
                                        set_column_homogeneous: true,
                                        set_margin_start: 18,

                                        attach[0,0,1,1] = &gtk::Button {
                                            set_label: "F-E",
                                            set_height_request: WIDGET_HEIGHT,
                                        },

                                        attach[1,0,1,1] = &gtk::Button {
                                            set_label: "(",
                                        },

                                        attach[2,0,1,1] = &gtk::Button {
                                            set_label: ")",
                                        },

                                        attach[0,1,1,1] = &gtk::Button {
                                            set_label: "dms",
                                        },

                                        attach[1,1,1,1] = &gtk::Button {
                                            set_label: "Exp",
                                        },

                                        attach[2,1,1,1] = &gtk::Button {
                                            set_label: "ln",
                                        },

                                        attach[0,2,1,1] = &gtk::Button {
                                            set_label: "sin",
                                        },

                                        attach[1,2,1,1] = &gtk::Button {
                                            set_label: "x^y",
                                        },

                                        attach[2,2,1,1] = &gtk::Button {
                                            set_label: "log",
                                        },

                                        attach[0,3,1,1] = &gtk::Button {
                                            set_label: "cos",
                                        },

                                        attach[1,3,1,1] = &gtk::Button {
                                            set_label: "x^3",
                                        },

                                        attach[2,3,1,1] = &gtk::Button {
                                            set_label: "n!",
                                        },

                                        attach[0,4,1,1] = &gtk::Button {
                                            set_label: "tan",
                                        },

                                        attach[1,4,1,1] = &gtk::Button {
                                            set_label: "x^2",
                                        },

                                        attach[2,4,1,1] = &gtk::Button {
                                            set_label: "1/x",
                                        },

                                    }
                                }
                            },

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: WIDGET_SPACING.into(),
                                set_margin_start: 18,
                                set_homogeneous: true,

                                gtk::Entry {
                                    set_editable: false,
                                    set_sensitive: false,
                                },

                                gtk::Button {
                                    set_label: "MC",
                                },

                                gtk::Button {
                                    set_label: "MR",
                                },

                                gtk::Button {
                                    set_label: "MS",
                                },

                                gtk::Button {
                                    set_label: "M+",
                                },

                                gtk::Button {
                                    set_label: "PI",
                                },
                            },

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: WIDGET_SPACING.into(),
                                set_margin_start: 18,

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: WIDGET_SPACING.into(),
                                    set_homogeneous: true,
                                    set_halign: gtk::Align::End,

                                    gtk::Button {
                                        set_label: "Back",
                                        set_height_request: WIDGET_HEIGHT,
                                    },

                                    gtk::Button {
                                        set_label: "CE",
                                    },

                                    gtk::Button {
                                        set_label: "C",
                                    },
                                },

                                gtk::Grid {
                                    set_row_spacing: WIDGET_SPACING.into(),
                                    set_column_spacing: WIDGET_SPACING.into(),
                                    set_row_homogeneous: true,
                                    set_column_homogeneous: true,

                                    attach[0,0,1,1] = &gtk::Button {
                                        set_label: "7",
                                        set_height_request: WIDGET_HEIGHT,
                                    },

                                    attach[1,0,1,1] = &gtk::Button {
                                        set_label: "8",
                                    },

                                    attach[2,0,1,1] = &gtk::Button {
                                        set_label: "9",
                                    },

                                    attach[3,0,1,1] = &gtk::Button {
                                        set_label: "/",
                                    },

                                    attach[4,0,1,1] = &gtk::Button {
                                        set_label: "Mod",
                                    },

                                    attach[5,0,1,1] = &gtk::Button {
                                        set_label: "And",
                                    },

                                    attach[0,1,1,1] = &gtk::Button {
                                        set_label: "4",
                                    },

                                    attach[1,1,1,1] = &gtk::Button {
                                        set_label: "5",
                                    },

                                    attach[2,1,1,1] = &gtk::Button {
                                        set_label: "6",
                                    },

                                    attach[3,1,1,1] = &gtk::Button {
                                        set_label: "*",
                                    },

                                    attach[4,1,1,1] = &gtk::Button {
                                        set_label: "Or",
                                    },

                                    attach[5,1,1,1] = &gtk::Button {
                                        set_label: "Xor",
                                    },

                                    attach[0,2,1,1] = &gtk::Button {
                                        set_label: "1",
                                    },

                                    attach[1,2,1,1] = &gtk::Button {
                                        set_label: "2",
                                    },

                                    attach[2,2,1,1] = &gtk::Button {
                                        set_label: "3",
                                    },

                                    attach[3,2,1,1] = &gtk::Button {
                                        set_label: "-",
                                    },

                                    attach[4,2,1,1] = &gtk::Button {
                                        set_label: "Lsh",
                                    },

                                    attach[5,2,1,1] = &gtk::Button {
                                        set_label: "Not",
                                    },

                                    attach[0,3,1,1] = &gtk::Button {
                                        set_label: "0",
                                    },

                                    attach[1,3,1,1] = &gtk::Button {
                                        set_label: "±",
                                    },

                                    attach[2,3,1,1] = &gtk::Button {
                                        set_label: ".",
                                    },

                                    attach[3,3,1,1] = &gtk::Button {
                                        set_label: "+",
                                    },

                                    attach[4,3,1,1] = &gtk::Button {
                                        set_label: "=",
                                    },

                                    attach[5,3,1,1] = &gtk::Button {
                                        set_label: "Int",
                                    },

                                    attach[0,4,1,1] = &gtk::Button {
                                        set_label: "A",
                                    },

                                    attach[1,4,1,1] = &gtk::Button {
                                        set_label: "B",
                                    },

                                    attach[2,4,1,1] = &gtk::Button {
                                        set_label: "C",
                                    },

                                    attach[3,4,1,1] = &gtk::Button {
                                        set_label: "D",
                                    },

                                    attach[4,4,1,1] = &gtk::Button {
                                        set_label: "E",
                                    },

                                    attach[5,4,1,1] = &gtk::Button {
                                        set_label: "F",
                                    },
                                },
                            },
                        },
                    } -> {
                        set_name: "scientific_view",
                    },

                    #[watch]
                    set_visible_child_name: {
                        match model.view {
                            CalculatorView::Standard => "standard_view",
                            CalculatorView::Scientific => "scientific_view",
                        }
                    },
                },
            },
        }
    }

    // Initialize the component.
    fn init(
        _init_values: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        relm4::menu! {
            main_menu: {
                "_Edit" {
                    "Copy" => DummyAction,
                    "Paste" => DummyAction,
                },
                "_View" {
                    "Scientific" => ViewAction(CalculatorView::Scientific),
                    "Standard" => ViewAction(CalculatorView::Standard),
                },
                "_Help" {
                    "Help Topics" => DummyAction,
                    "About Calculator" => DummyAction,
                },
            }
        }

        let model = App {
            view: CalculatorView::Standard, // TODO: Remember mode
            buffer: gtk::EntryBuffer::new(Some("0.")),
            number_base: NumberBase::Dec,
        };

        let widgets = view_output!();

        let sender_clone = sender.clone();

        let view_action = RelmAction::<ViewAction>::new_stateful_with_target_value(
            &CalculatorView::Standard,
            move |_, state, value: CalculatorView| {
                *state = value.clone();
                sender.input(Msg::UpdateView(value.clone()));
            },
        );

        let number_base_action = RelmAction::<NumberBaseAction>::new_stateful_with_target_value(
            &NumberBase::Dec,
            move |_, state, value: NumberBase| {
                *state = value.clone();
                sender_clone.input(Msg::UpdateNumberBase(value.clone()));
            },
        );

        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        group.add_action(view_action);
        group.add_action(number_base_action);
        group.register_for_widget(&widgets.main_window);

        let app = relm4::main_application();
        app.set_menubar(Some(&gio::MenuModel::from(main_menu)));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::UpdateView(view) => {
                self.view = view;
            }
            Msg::UpdateNumberBase(base) => {
                self.number_base = base;
            }
        }
    }
}

new_action_group!(WindowActionGroup, "win");
new_stateless_action!(DummyAction, WindowActionGroup, "dummy");
new_stateful_action!(
    ViewAction,
    WindowActionGroup,
    "view",
    CalculatorView,
    CalculatorView
);
new_stateful_action!(
    NumberBaseAction,
    WindowActionGroup,
    "base",
    NumberBase,
    NumberBase
);

fn main() {
    gtk4::init().expect("Failed to initialize GTK4");

    // settings

    let settings = gtk4::Settings::default().expect("No default GtkSettings");
    settings.set_gtk_primary_button_warps_slider(false);
    settings.set_gtk_overlay_scrolling(false);
    settings.set_gtk_cursor_theme_size(16);
    settings.set_gtk_decoration_layout(Some("icon:minimize,maximize,close"));

    // stylesheet

    let provider = gtk4::CssProvider::new();
    provider.load_from_path("stylesheet/gtk.css");
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("No display found"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // app

    let app = RelmApp::new("com.remcokranenburg.RdeCalc");
    app.run::<App>(());
}
