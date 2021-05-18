use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use plankalkul::expr::Expr;
use plankalkul::expr_parse::expr;
use plankalkul::num::Number;

pub struct Model {
    link: ComponentLink<Self>,
    exprs: Vec<String>,
}

pub enum Msg {
    GotInput(usize, String),
    Add(),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            exprs: vec!["2+2".to_string()],
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(i, new_value) => {
                self.exprs[i] = new_value;
            }
            Msg::Add() => {
                self.exprs.push("".to_string());
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                { for self.exprs.iter().enumerate().map(|expr| self.view_expr(expr)) }
                <button onclick=self.link.callback(|_| Msg::Add())>{"+"}</button>
            </>
        }
    }
}

impl Model {
    fn calc(e: &str) -> Result<(Expr, Number), String> {
        if e.is_empty() {
            return Err("".to_string());
        }
        let (rest, expr) = expr(e).map_err(|e| format!("parsing error {}", e))?;
        if !rest.is_empty() {
            return Err(format!("unparsed {}", rest));
        }
        let num = expr.as_number();
        match num {
            Ok(num) => Ok((expr, num)),
            Err(e) => Err(e),
        }
    }

    fn view_expr(&self, (i, expr): (usize, &String)) -> Html {
        let content = match Model::calc(expr) {
            Ok((expr, number)) => html! {
                <p>
                    {format!("{} = {} = {:?}", expr, number, number.to_decimal(100))}
                </p>
            },
            Err(err) => html! {
                <p>{err}</p>
            },
        };

        html! {
            <div>
                <input type="text" inputmode="decimal" value=expr.to_string() oninput=self.link.callback(move |e: InputData| Msg::GotInput(i, e.value))/>
                {content}
            </div>
        }
    }
}
