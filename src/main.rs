#[macro_use]
extern crate yew;
extern crate rand;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use rand::distributions::{IndependentSample, Range};

struct Context {
    console: ConsoleService,
}

struct Model {
    roll: u64,
}

enum Msg {
    Roll,
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            roll: 0
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Roll => {
                let range = Range::new(1, 100);
                let mut rng = rand::thread_rng();

                self.roll = range.ind_sample(&mut rng);

                context.console.log("Rolled the dice!");
            }
        }

        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <p>{"You rolled a: "} { self.roll }</p>
                <button onclick=|_| Msg::Roll,>{ "Roll" }</button>
            </div>
        }
    }
}

fn main() {
    yew::initialize();

    let context = Context {
        console: ConsoleService,
    };

    let app: App<_, Model> = App::new(context);

    app.mount_to_body();
    yew::run_loop();
}
