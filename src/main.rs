mod presenter;
mod router;
mod screens;

use std::cell::RefCell;
use std::rc::Rc;

use slint::Model;
use router::Router;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let router = Rc::new(RefCell::new(Router::new(&app)));

    let app_weak = app.as_weak();
    let nav = app.global::<Navigator>();

    let router_clone = router.clone();
    nav.on_back(move || {
        router_clone.borrow_mut().go_back();
    });

    let router_clone = router.clone();
    nav.on_navigate(move |screen| {
        let Some(app) = app_weak.upgrade() else { return };
        if screen == ScreenId::Detail {
            // Read selected item from List, set Detail model
            let list_data = app.global::<ListInterface>();
            let selected = list_data.get_selected_index();
            let list_model = list_data.invoke_get_model();
            let item = list_model.items.row_data(selected as usize);
            let count = item.as_ref().map(|i| i.count).unwrap_or(0);
            let name = item.as_ref().map(|i| i.label.clone()).unwrap_or_default();
            let model = DetailModel {
                header: HeaderScreenModel { title: name },
                count,
            };
            let detail_data = app.global::<DetailInterface>();
            detail_data.on_get_model(move || model.clone());
        }
        router_clone.borrow_mut().go_to(screen);
    });

    app.run()
}
