use adw::gtk::Orientation;
use adw::prelude::*;
use config_lib::style::Theme;
use relm4::abstractions::Toaster;
use relm4::factory::*;
use relm4::gtk::gdk_pixbuf::{InterpType, Pixbuf};
use relm4::gtk::{Align, IconPaintable, gio, glib};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, gtk};
use std::fs::File;
use std::path::PathBuf;
use tracing::{debug, trace, warn};

#[derive(Debug)]
struct ThemeCarousel {
    theme: Theme,
}

#[derive(Debug, Clone, Copy)]
enum ThemeCarouselInput {}

#[derive(Debug)]
enum ThemeCarouselOutput {}

#[relm4::factory]
impl FactoryComponent for ThemeCarousel {
    type Init = Theme;
    type Input = ThemeCarouselInput;
    type Output = ThemeCarouselOutput;
    type CommandOutput = ();
    type ParentWidget = adw::Carousel;

    view! {
        gtk::Box {
            // set_description: Some(&self.theme.path.display().to_string()),
            set_orientation: Orientation::Vertical,
            set_css_classes: &["theme"],
            // set_hexpand: true,
            set_halign: Align::Fill,
            set_valign: Align::Fill,
            gtk::Label {
                set_text: &self.theme.name,
                set_css_classes: &["big-text"],
                set_margin_bottom: 15,
            },
            gtk::Picture {
                set_file:  self.theme.image_path.as_ref().map(|path| gio::File::for_path(path)).as_ref(),
                set_css_classes: &["theme-image"],
                set_vexpand: true,
                set_hexpand: false,
                set_valign: Align::Center,
                set_halign: Align::Center,
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { theme: init }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {};
    }
}

#[derive(Debug)]
pub struct Style {
    err: Option<String>,
    themes_list: FactoryVecDeque<ThemeCarousel>,
    toaster: Toaster,
}

#[derive(Debug)]
pub enum StyleInput {}

#[derive(Debug)]
pub struct StyleInit {}

#[derive(Debug)]
pub enum StyleOutput {}

#[relm4::component(pub)]
impl SimpleComponent for Style {
    type Init = StyleInit;
    type Input = StyleInput;
    type Output = StyleOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,
            set_margin_all: 10,
            // gtk::Label {
            //     set_text: "Themes",
            //     set_css_classes: &["big-text"],
            //     set_margin_top: 5,
            //     set_margin_bottom: 10,
            // },
            gtk::Label {
                #[watch]
                set_visible: model.err.is_some(),
                #[watch]
                set_text: match &model.err {
                    Some(err) => err,
                    None => "",
                }
            },
            #[local_ref]
            toast_overlay -> adw::ToastOverlay {
                set_vexpand: true,
                #[local_ref]
                themes_carousel -> adw::Carousel {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 5,
                    set_css_classes: &["theme-carousel"],
                    set_vexpand: true,
                    set_vexpand_set: true,
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut themes_list = FactoryVecDeque::builder()
            .launch(adw::Carousel::builder().build())
            .forward(sender.input_sender(), |output| match output {});

        let model = match load_themes() {
            Ok((themes, errors)) => {
                let mut v = themes_list.guard();
                for theme in themes {
                    v.push_back(theme);
                }
                drop(v);
                let toaster = Toaster::default();
                for err in errors {
                    toaster.add_toast(adw::Toast::builder().title(err).timeout(0).build());
                }
                Style {
                    toaster,
                    err: None,
                    themes_list,
                }
            }
            Err(err) => {
                warn!("Failed to load themes: {err}");
                Style {
                    toaster: Toaster::default(),
                    err: Some(err),
                    themes_list,
                }
            }
        };

        let themes_carousel = model.themes_list.widget();
        let toast_overlay = model.toaster.overlay_widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        trace!("update: {message:?}");
        match message {}
    }
}

fn load_themes() -> Result<(Vec<Theme>, Vec<String>), String> {
    let path = core_lib::path::get_default_system_data_dir().join("themes");
    let themes = config_lib::style::list_themes(path);
    trace!("Loaded themes: {:?}", themes);
    match themes {
        Ok((themes, errors)) => {
            debug!("Loaded {} themes, {} errors", themes.len(), errors.len());
            Ok((themes, errors.iter().map(|e| e.to_string()).collect()))
        }
        Err(err) => {
            warn!("Failed to load themes: {err}");
            Err(err.to_string())
        }
    }
}
