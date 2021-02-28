use std::error::Error;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use plankalkul::expr::Expr;
use plankalkul::expr_parse::expr;

pub struct Model {
    link: ComponentLink<Self>,
    expr: String,
}

pub enum Msg {
    GotInput(String),
}

impl Model {
    fn calc(&self) -> Result<Expr, Box<dyn Error + '_>> {
        let (rest, expr) = expr(&self.expr)?;
        if !rest.is_empty() {
            return Err(format!("unparsed {}", rest).into());
        }
        Ok(expr)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
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
        let expr = self.calc();
        let content = match expr {
            Ok(expr) => html! {
                <p>
                    {format!("{} = {} = {:?}", expr, expr.as_number(), expr.as_number().to_decimal())}
                </p>
            },
            Err(err) => html! {
                <p>{err}</p>
            },
        };

        html! {
            <div>
                <input type="text" inputmode="decimal" id="expr" value=&self.expr oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))/>
                {content}
            </div>
        }
    }
}
