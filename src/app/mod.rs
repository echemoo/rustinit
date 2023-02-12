use std::collections::HashMap;

pub struct Application {
    pub name: String,
    modules: Modules,
    index: usize,
}

impl Application {
    pub fn new() -> Application {
        Application {
            name: String::from("EAI"),
            modules: Modules {
                ticks: HashMap::new(),
            },
            index: 0,
        }
    }

    pub fn run(&self) {
        println!("Hello World!");

        self.modules.ticks.values().for_each(|module| {
            module.init();
        });
        self.modules.ticks.values().for_each(|module| {
            module.start();
        });
        self.modules.ticks.values().for_each(|module| {
            module.end();
        });
    }

    pub fn on(&mut self, module: Box<dyn Tick>) -> usize {
        self.index += 1;
        self.modules.ticks.insert(self.index, module);
        self.index
    }
    pub fn close(&mut self, id: usize) {
        self.modules.ticks.remove(&id);
    }
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
    pub ticks: HashMap<usize, Box<dyn Tick>>,
}

pub struct ModuleA;
impl Tick for ModuleA {
    fn init(&self) {
        println!("ModuleA init");
    }

    fn start(&self) {
        println!("ModuleA start");
    }

    fn end(&self) {
        println!("ModuleA end");
    }
}

pub struct ModuleB;
impl Tick for ModuleB {
    fn init(&self) {
        println!("ModuleB init");
    }

    fn start(&self) {
        println!("ModuleB start");
    }

    fn end(&self) {
        println!("ModuleB end");
    }
}

pub struct ModuleC;
impl Tick for ModuleC {
    fn init(&self) {
        println!("ModuleC init");
    }

    fn start(&self) {
        println!("ModuleC start");
    }

    fn end(&self) {
        println!("ModuleC end");
    }
}
