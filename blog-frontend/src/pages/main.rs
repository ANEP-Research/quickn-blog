use crate::api::*;

use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Main {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub info: Info,
}

pub enum Msg {}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Hero classes="is-primary" body={
                html! {
                    <>
                        <ybc::Title>
                            {&format!("Welcome to the {}", self.props.info.blog_info.blog_name)}
                        </ybc::Title>
                        <ybc::Subtitle>
                            {"Coming soon"}
                        </ybc::Subtitle>
                    </>
                }
            }/>
        }
    }
}
