use levenshtein::lev;
use seed::{*, prelude::*};

pub enum Msg {
    Words(Result<Vec<String>, FetchError>)
}

struct DemoLevenshtein {
    words: Vec<String>
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> DemoLevenshtein {
    orders.perform_cmd(async move {
        Msg::Words(Ok(Vec::new()))
    });
    DemoLevenshtein { words: Vec::new() }
}
fn update(msg: Msg, _model: &mut DemoLevenshtein, orders: &mut impl Orders<Msg>) {
    orders.skip();
    match msg {
        Msg::Words(Ok(data)) => {
            ::seed::log!(data);
        },
        Msg::Words(Err(e)) => {
            ::seed::error!(e);
        },
    }
}
fn view(_model: &DemoLevenshtein) -> Vec<Node<Msg>> {
    nodes![
        label!(
            attrs!["for" => "word"],
            "Enter a french word: "
        ),
        input!(
            id!("word"),
            attrs!["type" => "text"]
        )
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
