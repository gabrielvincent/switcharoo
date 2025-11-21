use relm4::adw::gtk;
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub struct Launcher {}

#[derive(Debug)]
pub enum LauncherMsg {}

#[derive(Debug)]
pub struct LauncherInit {}

#[derive(Debug)]
pub enum LauncherOutput {}

#[relm4::component(pub)]
impl SimpleComponent for Launcher {
    type Init = LauncherInit;
    type Input = LauncherMsg;
    type Output = LauncherOutput;

    view! {
        gtk::Box {

        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Launcher {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
