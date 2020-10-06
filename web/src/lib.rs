use eoautomata_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JSAutomata(Automata);

#[wasm_bindgen]
impl JSAutomata {
    pub fn gvz(&self) -> String {
        let mut out = String::new();
        self.0.write_graphviz(&mut out);
        out
    }
}

#[wasm_bindgen]
pub fn parse_eoa(str: &str) -> JSAutomata {
    JSAutomata(eoa::parse(str))
}

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
