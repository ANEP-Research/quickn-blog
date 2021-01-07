use yew::prelude::*;
use crate::app::Model;

pub struct ViewPost {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {

}

impl Component for ViewPost {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props.id != props.id {
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            
        }
    }
}