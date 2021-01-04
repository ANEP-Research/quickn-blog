use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Footer {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}
pub struct Msg {}

impl Component for Footer {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Footer>
                <ybc::Content classes="has-text-centered">
                    <p>
                        <strong>{"quickn-blog"}</strong> {" by "} <a href="https://quickn.tech">{"quickn.tech"}</a>
                    </p>
                </ybc::Content>
            </ybc::Footer>
        }
    }
}
