use slint::ComponentHandle;
use crate::presenter::Presenter;
use crate::{App, SettingsInterface, SettingsModel, HeaderScreenModel};

pub struct SettingsPresenter;

impl SettingsPresenter {
    pub fn new() -> Self { Self }

    fn get_model(&self) -> SettingsModel {
        SettingsModel {
            header: HeaderScreenModel {
                title: "Settings".into(),
            },
        }
    }
}

impl Presenter for SettingsPresenter {
    fn on_enter(&mut self, app: &App) {
        let model = self.get_model();
        app.global::<SettingsInterface>().on_get_model(move || model.clone());
        let data = app.global::<SettingsInterface>();
        data.set_tick(!data.get_tick());
    }
}
