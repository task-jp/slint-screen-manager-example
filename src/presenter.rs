use crate::App;

/// Presenter lifecycle:
///
/// Navigation forward (A → B):
///   A.on_suspend()  → B.on_enter()
///
/// Navigation back (B → A):
///   B.on_exit()     → A.on_resume()
///
/// The Presenter holds persistent state.
/// The Interface global is a transient communication channel.
pub trait Presenter {
    /// Called when this page becomes active.
    /// Set up callbacks and provide model data.
    fn on_enter(&mut self, app: &App);

    /// Called when navigating away (page stays on stack).
    fn on_suspend(&mut self, _app: &App) {}

    /// Called when returning via back navigation.
    fn on_resume(&mut self, app: &App) {
        self.on_enter(app);
    }

    /// Called when removed from stack.
    fn on_exit(&mut self, _app: &App) {}
}
