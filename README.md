# Slint Screen Manager Example

A multi-screen application built with [Slint](https://slint.dev/) and Rust, demonstrating the **MVP (Model-View-Presenter)** pattern with stack-based navigation.

## Architecture

```
View (.slint)  ←→  Interface (global)  ←→  Presenter (.rs)
                                              ↑
                                           Router (stack-based navigation)
```

| Layer       | Role                                         |
|-------------|----------------------------------------------|
| **View**    | Slint components (`*View`) — UI rendering    |
| **Interface** | Slint globals (`*Interface`) — View↔Presenter contract |
| **Model**   | Slint structs (`*Model`) — data passed to View |
| **Presenter** | Rust structs (`*Presenter`) — business logic |
| **Router**  | Manages navigation stack and Presenter lifecycle |

### Presenter Lifecycle

```
Navigate forward (A → B):  A.on_suspend() → B.on_enter()
Navigate back   (B → A):  B.on_exit()    → A.on_resume()
```

### Screen Co-location

Each screen lives in its own directory with both `.slint` and `.rs` files:

```
src/screens/
├── mod.rs
├── home/
│   ├── home.slint      # HomeView
│   └── home.rs         # HomePresenter
├── list/
│   ├── list.slint      # ListView
│   └── list.rs         # ListPresenter
├── detail/
│   ├── detail.slint    # DetailView
│   └── detail.rs       # DetailPresenter
└── settings/
    ├── settings.slint  # SettingsView
    └── settings.rs     # SettingsPresenter
```

## Design Principles

### Interface as Stateless Channel

Slint requires a `global` per screen for View-Presenter communication. This project minimizes each global to a stateless channel: just a `tick` property and a `pure callback get-model()`, plus screen-specific action callbacks. No application state is stored in the Slint layer.

```slint
export global DetailInterface {
    in-out property <bool> tick: false;
    pure callback get-model() -> DetailModel;
    callback increment();
}
```

### Pull-based Data Flow via tick

Slint has no built-in mechanism to signal "re-call this pure callback." This project uses a `tick` boolean toggle to force re-evaluation:

```slint
property <DetailModel> model:
    DetailInterface.tick ? DetailInterface.get-model() : DetailInterface.get-model();
```

Both branches call `get-model()`, so toggling `tick` triggers a fresh pull from the Presenter. This enables a Passive View pattern where the View never holds or mutates state — it only renders what the Presenter provides.

### Presenter as Single Source of Truth

All persistent state lives in Rust Presenters, not in Slint globals. The Interface global is re-wired on every `on_enter`, making it a transient communication channel. When the Router pops a screen from the stack, its Presenter is dropped and the memory is freed. Only the screens in the current navigation stack consume memory.

### Memory Efficiency

- **Interface globals**: always exist but are minimal (a bool + callback pointers)
- **Presenters**: only exist while on the navigation stack, dropped on `go_back()`
- **View components**: Slint instantiates only the active screen via `if` conditional rendering

This means memory usage scales with navigation depth, not total screen count.

## Screens

- **Home** — Navigation hub with buttons to List and Settings
- **List** — Displays a list of fruits with per-item count
- **Detail** — Counter with increment/decrement/reset (count is stored back in the list)
- **Settings** — Placeholder settings page

## Building

```sh
cargo build
```

## Running

```sh
cargo run
```

## Dependencies

- [Slint](https://slint.dev/) 1.14.1
- Rust Edition 2024
