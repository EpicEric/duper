use duper::DuperValue;
use godot::prelude::*;

struct DuperExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DuperExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot::classes::Engine::singleton()
                .register_singleton(&Duper::class_id().to_string_name(), &Duper::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = godot::classes::Engine::singleton();
            let singleton_name = &Duper::class_id().to_string_name();
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                godot_error!("Failed to get Duper singleton");
            }
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Object)]
struct Duper {
    base: Base<Object>,
}

#[godot_api]
impl Duper {
    #[func]
    fn new(&self) -> Gd<DuperParser> {
        DuperParser::new_gd()
    }

    #[func]
    fn stringify(&self, data: Variant) -> String {
        duper::serde::ser::to_string(&data).unwrap()
    }
}

#[derive(GodotClass)]
#[class(init)]
struct DuperParser {
    value: Option<Variant>,
}

#[godot_api]
impl DuperParser {
    #[func]
    fn parse(&mut self, input: String) -> godot::global::Error {
        match duper::DuperParser::parse_duper_value(&input) {
            Ok(value) => {
                todo!();
                godot::global::Error::OK
            }
            Err(errors) => {
                if let Ok(msg) = duper::DuperParser::prettify_error(&input, &errors, None) {
                    godot_error!("{msg}");
                }
                godot::global::Error::FAILED
            }
        }
    }
}
