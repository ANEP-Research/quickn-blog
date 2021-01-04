use yew::prelude::*;
use yew::utils::host;

use crate::api::*;
use crate::app::{self, Model};
use crate::route::*;
use ybc::ImageSize;
use ybc::NavbarFixed::Top;
use ybc::NavbarItemTag;

pub struct Navbar {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    #[prop_or_default]
    pub info: BlogInfo,
    #[prop_or_default]
    pub user_info: UserInfo,
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.info != props.info || self.props.user_info != props.user_info {
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
                    <ybc::NavbarItem tag=NavbarItemTag::A href={&format!("http://{}", host().unwrap())}>
                        {"Home"}
                    </ybc::NavbarItem>
                }
            }
            navend={
                if !self.props.user_info.success {
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
                        <ybc::NavbarItem>
                            <ybc::Button onclick=self.props.link_app.callback(|_| app::Msg::GetLogout)>
                                {"Logout"}
                            </ybc::Button>
                        </ybc::NavbarItem>
                    }
                }
            }/>
        }
    }
}
