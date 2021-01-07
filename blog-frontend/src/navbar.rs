use ybc::ImageSize;
use yew::prelude::*;
use yew::utils::host;

use crate::api::*;
use crate::app::{self, Model};
use crate::route::*;

use ybc::NavbarItemTag;

pub struct Navbar {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub info: Info,
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.info != props.info {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Navbar classes="has-shadow" navbrand={
                html!{
                    <ybc::NavbarItem href={&format!("http://{}", host().unwrap())}>
                        <img src={&format!("http://{}/static/images/mathion-logo.png", host().unwrap())} width="30" height="30"/>
                    </ybc::NavbarItem>
                }
            }
            navstart={
                html!{
                    <>
                            <a class="navbar-item" onclick=self.props.link_app.callback(|_| app::Msg::ChangeRoute(AppRoute::Main))>
                                {"Home"}
                            </a>
                            <a class="navbar-item" onclick=self.props.link_app.callback(|_| app::Msg::ChangeRoute(AppRoute::Posts))>
                                {"Posts"}
                            </a>
                    </>
                }
            }
            navend={
                if !self.props.info.account_info.success {
                    html!{
                        <>
                            <ybc::NavbarItem>
                                <ybc::Button onclick=self.props.link_app.callback(|_| app::Msg::ChangeRoute(AppRoute::Register))>
                                    {"Register"}
                                </ybc::Button>
                            </ybc::NavbarItem>
                            <ybc::NavbarItem>
                                <ybc::Button onclick=self.props.link_app.callback(|_| app::Msg::ChangeRoute(AppRoute::Login))>
                                    {"Login"}
                                </ybc::Button>
                            </ybc::NavbarItem>
                        </>
                    }
                } else {
                    html! {
                        <a class="navbar-item" onclick=self.props.link_app.callback(|_| app::Msg::ChangeRoute(AppRoute::Accounts))>
                            <ybc::Image size=ImageSize::Is32x32>
                                <img class="is-rounded" src={&format!("http://{}/static/images/account_circle-black-48dp.svg", host().unwrap())}/>
                            </ybc::Image>
                        </a>
                    }
                }
            }/>
        }
    }
}
