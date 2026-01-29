use slint::ComponentHandle;
use crate::{App, NavigationState, ScreenId};
use crate::presenter::Presenter;

use crate::screens::home::HomePresenter;
use crate::screens::settings::SettingsPresenter;
use crate::screens::list::ListPresenter;
use crate::screens::detail::DetailPresenter;

/// Navigation stack entry
struct StackEntry {
    screen: ScreenId,
    presenter: Box<dyn Presenter>,
}

/// Stack-based Router managing Presenter lifecycle.
pub struct Router {
    app: slint::Weak<App>,
    stack: Vec<StackEntry>,
}

impl Router {
    pub fn new(app: &App) -> Self {
        let mut presenter = Self::create_presenter(ScreenId::Home);
        presenter.on_enter(app);

        let router = Self {
            app: app.as_weak(),
            stack: vec![StackEntry {
                screen: ScreenId::Home,
                presenter,
            }],
        };
        router.sync_to_view();
        router
    }

    pub fn go_to(&mut self, screen: ScreenId) {
        if let Some(app) = self.app.upgrade() {
            if let Some(current) = self.stack.last_mut() {
                current.presenter.on_suspend(&app);
            }

            let mut presenter = Self::create_presenter(screen);
            presenter.on_enter(&app);

            self.stack.push(StackEntry { screen, presenter });
            self.sync_to_view();
        }
    }

    pub fn go_back(&mut self) {
        if self.stack.len() > 1 {
            if let Some(app) = self.app.upgrade() {
                if let Some(mut exiting) = self.stack.pop() {
                    exiting.presenter.on_exit(&app);
                }
                if let Some(current) = self.stack.last_mut() {
                    current.presenter.on_resume(&app);
                }
                self.sync_to_view();
            }
        }
    }

    fn sync_to_view(&self) {
        if let Some(app) = self.app.upgrade() {
            let nav = app.global::<NavigationState>();
            if let Some(current) = self.stack.last() {
                nav.set_current_screen(current.screen);
            }
            nav.set_can_go_back(self.stack.len() > 1);
        }
    }

    fn create_presenter(screen: ScreenId) -> Box<dyn Presenter> {
        match screen {
            ScreenId::Home => Box::new(HomePresenter::new()),
            ScreenId::Settings => Box::new(SettingsPresenter::new()),
            ScreenId::List => Box::new(ListPresenter::new()),
            ScreenId::Detail => Box::new(DetailPresenter::new()),
        }
    }
}
