use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use std::sync::atomic::{AtomicUsize, Ordering};

static VIEW_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Msg {
    Click(usize),
}

pub struct Model {
    number: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { number: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(number) => self.number = number,
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        VIEW_COUNT.fetch_add(1, Ordering::SeqCst);
        let vc: usize = VIEW_COUNT.load(Ordering::SeqCst);
        html! {
            <div>
                <button onclick=|_| Msg::Click(vc),>{ format!("VIEW_COUNT: {}", vc) }</button>
                <p>{ format!("VIEW_COUNT: {}", vc) }</p>
                <p>{ format!("number: {}", self.number) }</p>
            </div>
        }
    }
}
