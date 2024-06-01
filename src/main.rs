use log::debug;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

enum Msg {
    StartBreed,
    SetPlants(String),
}

struct App {
    input: Option<String>,
    breed_result: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            input: None,
            breed_result: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartBreed => {
                self.breed_result = Some(rust_pow2::breed_plants(self.input.clone().unwrap()));
                debug!("Bred! {:?}", &self.breed_result);
                true
            }
            Msg::SetPlants(plants) => {
                self.input = Some(plants);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::SetPlants(input.value())
        });

        html! {
            <div>
                <textarea id="plants_input" rows="30" cols="10" type="text" {oninput}></textarea>
                <label for="plants_input"></label>
                <button onclick={ctx.link().callback(|_| Msg::StartBreed)}>{ "Start breed" }</button>
                <pre>{ &self.breed_result }</pre>
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    debug!("Init!");

    yew::Renderer::<App>::new().render();
}
