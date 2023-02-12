use std::collections::HashMap;

pub struct Application {
    pub name: String,
    _module: Modules,
}

impl Application {
    pub fn new() -> Application {
        Application {
            name: String::from("EAI"),
            _module: Modules {
                _ticks: HashMap::new(),
            },
        }
    }

    pub fn run(&self) {
        println!("Hello World!");
    }

    pub fn _on(&self) -> usize {
        0
    }
    pub fn _close(&self, _id: usize) {}
    pub fn _exec(&self) {}

    fn _run_all_room(&self) {}
    fn _run_all_creep(&self) {}
    fn _run_all_pc(&self) {}
}

pub trait Tick {
    fn init(&self);
    fn start(&self);
    fn end(&self);
}

pub struct Modules {
    pub _ticks: HashMap<usize, Box<dyn Tick>>,
}
