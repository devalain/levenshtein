use seed::{*, prelude::*};

pub enum Msg {}

struct DemoLevenshtein {}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> DemoLevenshtein {
    DemoLevenshtein {}
}
fn update(_msg: Msg, _model: &mut DemoLevenshtein, orders: &mut impl Orders<Msg>) {
    orders.skip();
}
fn view(_model: &DemoLevenshtein) -> Vec<Node<Msg>> {
    nodes![
        input!(attrs![
            "type" => "text"
        ])
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
