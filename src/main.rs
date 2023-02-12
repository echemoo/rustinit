mod app;

fn main() {
    let mut app = app::Application::new();

    app.on(Box::new(app::ModuleA{}));
    let id = app.on(Box::new(app::ModuleB{}));
    app.on(Box::new(app::ModuleC{}));

    app.run();

    app.close(id);

    app.run();
}
