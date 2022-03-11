use web_sys::InputEvent;
use yew::{html, Component, Context, Html};

use plankalkul::expr::Expr;
use plankalkul::expr_parse::expr;
use plankalkul::num::Number;

pub struct Model {
    exprs: Vec<String>,
}

pub enum Msg {
    GotInput(usize, String),
    Add(),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model {
            exprs: vec!["2+2".to_string()],
        }
    }

    //    fn changed(&mut self, _: Self::Properties) -> bool {
    //        false
    //    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { for self.exprs.iter().enumerate().map(|expr| self.view_expr(expr, ctx)) }
                <button onclick={ctx.link().callback(|_| Msg::Add())}>{"+"}</button>
            </>
        }
    }
}

impl Model {
    fn calc(e: &str) -> Result<(Box<Expr>, Number), String> {
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

    fn view_expr(&self, (i, expr): (usize, &String), ctx: &Context<Self>) -> Html {
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
                <input type="text" inputmode="decimal" value={expr.to_string()} oninput={ctx.link().callback(move |e: InputEvent| Msg::GotInput(i, e.data().unwrap()))}/>
                {content}
            </div>
        }
    }
}
