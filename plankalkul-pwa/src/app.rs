use std::error::Error;

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use plankalkul::expr::Expr;
use plankalkul::expr_parse::expr;

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
    fn calc(e: &str) -> Result<Expr, Box<dyn Error + '_>> {
        let (rest, expr) = expr(e)?;
        if !rest.is_empty() {
            return Err(format!("unparsed {}", rest).into());
        }
        Ok(expr)
    }

    fn view_expr(&self, (i, expr): (usize, &String)) -> Html {
        let value = Model::calc(expr);
        let content = match value {
            Ok(value) => html! {
                <p>
                    {format!("{} = {} = {:?}", value, value.as_number(), value.as_number().to_decimal(100))}
                </p>
            },
            Err(err) => html! {
                <p>{format!("{:?}", err)}</p>
            },
        };

        html! {
            <div>
                <input type="text" inputmode="decimal" value=expr oninput=self.link.callback(move |e: InputData| Msg::GotInput(i, e.value))/>
                {content}
            </div>
        }
    }
}
