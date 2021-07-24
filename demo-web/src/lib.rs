use levenshtein::lev;
use seed::{*, prelude::*};

pub enum Msg {
    Words(Result<Vec<String>, FetchError>),
    Input(String)
}

struct DemoLevenshtein {
    words: Vec<String>,
    didyoumean: Option<String>
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> DemoLevenshtein {
    orders.perform_cmd(async move {
        let req = Request::new("/words.txt").method(Method::Get);
        let resp = req.fetch().await;
        let result = match resp.and_then(Response::check_status)
            .map(|r| async move { r.text().await } ) {
                Ok(future) => future.await,
                Err(e) => Err(e)
        }
        .map(|content| {
            let content = content.replace("\r", "");
            content.split("\n").map(ToOwned::to_owned).collect::<Vec<String>>()
        });
        Msg::Words(result)
    });
    DemoLevenshtein { words: Vec::new(), didyoumean: None }
}
fn update(msg: Msg, model: &mut DemoLevenshtein, orders: &mut impl Orders<Msg>) {
    orders.skip();
    match msg {
        Msg::Words(Ok(data)) => {
            model.words = data;
        },
        Msg::Words(Err(e)) => {
            ::seed::error!(e);
        },
        Msg::Input(input) => {
            let mut i_min = model.words.len();
            let mut min_dist = usize::MAX;
            for (i, w) in model.words.iter().enumerate() {
                let dist = lev(w, &input);
                if dist < min_dist {
                    min_dist = dist;
                    i_min = i;
                }
            }
            if min_dist == 0 {
                model.didyoumean = Some("The word is correctly spelled.".to_owned());
            } else {
                model.didyoumean = Some(format!("Did you mean '{}' ?", model.words[i_min]));
            }
            orders.render();
        }
    }
}
fn view(model: &DemoLevenshtein) -> Vec<Node<Msg>> {
    let mut didyoumean = nodes![];
    if let Some(s) = model.didyoumean.as_deref() {
        didyoumean.push(div![br!(), s.to_owned()]);
    }
    nodes![
        label!(
            attrs!["for" => "word"],
            "Enter a french word: "
        ),
        input!(
            id!("word"),
            input_ev(Ev::Input, |content| Msg::Input(content)),
            attrs!["type" => "text"]
        ),
        didyoumean
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
