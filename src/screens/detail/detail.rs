use slint::ComponentHandle;
use crate::presenter::Presenter;
use crate::{App, DetailInterface, DetailModel};

pub struct DetailPresenter;

impl DetailPresenter {
    pub fn new() -> Self { Self }
}

impl Presenter for DetailPresenter {
    fn on_enter(&mut self, app: &App) {
        let data = app.global::<DetailInterface>();

        let app_weak = app.as_weak();
        data.on_increment(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let data = app.global::<DetailInterface>();
            let current = data.invoke_get_model();
            let model = DetailModel {
                header: current.header,
                count: current.count + 1,
            };
            data.on_get_model(move || model.clone());
            data.set_tick(!data.get_tick());
        });

        let app_weak = app.as_weak();
        data.on_decrement(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let data = app.global::<DetailInterface>();
            let current = data.invoke_get_model();
            let model = DetailModel {
                header: current.header,
                count: current.count - 1,
            };
            data.on_get_model(move || model.clone());
            data.set_tick(!data.get_tick());
        });

        let app_weak = app.as_weak();
        data.on_reset(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let data = app.global::<DetailInterface>();
            let current = data.invoke_get_model();
            let model = DetailModel {
                header: current.header,
                count: 0,
            };
            data.on_get_model(move || model.clone());
            data.set_tick(!data.get_tick());
        });

        data.set_tick(!data.get_tick());
    }
}
