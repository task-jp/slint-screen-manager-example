use slint::ComponentHandle;
use crate::presenter::Presenter;
use crate::{App, HomeInterface, HomeModel};

pub struct HomePresenter;

impl HomePresenter {
    pub fn new() -> Self { Self }
}

impl Presenter for HomePresenter {
    fn on_enter(&mut self, app: &App) {
        app.global::<HomeInterface>().on_get_model(|| HomeModel {});
        let data = app.global::<HomeInterface>();
        data.set_tick(!data.get_tick());
    }
}
