use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use plankalkul::expr_parse::expr;

pub struct Model {
    link: ComponentLink<Self>,
    expr: String,
}

pub enum Msg {
    GotInput(String),
}

impl Model {
    fn calc(&self) -> String {
        let result = expr(self.expr.as_str());
        match result {
            Ok((_, expr)) => expr.as_number().to_decimal_periodic(),
            Err(_) => "???".to_string(),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link: link,
            expr: "2+2".to_string(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.expr = new_value;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <textarea value=&self.expr oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))/>
                <p>{self.calc()}</p>
            </div>
        }
    }
}
