use slint::ComponentHandle;
use crate::presenter::Presenter;
use crate::{App, ListInterface, ListModel, ListItemModel, DetailInterface, HeaderScreenModel};

pub struct ListPresenter {
    labels: Vec<&'static str>,
    counts: Vec<i32>,
    selected: usize,
}

impl ListPresenter {
    pub fn new() -> Self {
        Self {
            labels: vec!["Apple", "Banana", "Cherry", "Dragonfruit", "Elderberry"],
            counts: vec![0; 5],
            selected: 0,
        }
    }

    fn get_model(&self) -> ListModel {
        use slint::{ModelRc, VecModel};

        let items: Vec<ListItemModel> = self.labels.iter().zip(&self.counts)
            .map(|(label, &count)| ListItemModel {
                label: (*label).into(),
                count,
            })
            .collect();

        ListModel {
            header: HeaderScreenModel {
                title: "Fruits".into(),
            },
            items: ModelRc::new(VecModel::from(items)),
        }
    }
}

impl Presenter for ListPresenter {
    fn on_enter(&mut self, app: &App) {
        let model = self.get_model();
        app.global::<ListInterface>().on_get_model(move || model.clone());
        let data = app.global::<ListInterface>();
        data.set_tick(!data.get_tick());
    }

    fn on_suspend(&mut self, app: &App) {
        self.selected = app.global::<ListInterface>().get_selected_index() as usize;
    }

    fn on_resume(&mut self, app: &App) {
        // Read back the updated count from Detail
        let detail_count = app.global::<DetailInterface>().invoke_get_model().count;
        if self.selected < self.counts.len() {
            self.counts[self.selected] = detail_count;
        }
        self.on_enter(app);
    }
}
