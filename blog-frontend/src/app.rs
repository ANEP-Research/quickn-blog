use crate::api::*;

use crate::footer::Footer;
use crate::navbar::Navbar;
use crate::pages::Accounts;
use crate::pages::Login;
use crate::pages::Main;
use crate::pages::Posts;
use crate::pages::Register;
use crate::route::AppRoute;
use crate::services::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::{route::Route, service::RouteService, Switch};

pub struct Model {
    route_service: RouteService<()>,
    cookie_service: CookieService,
    route: Route<()>,
    link: ComponentLink<Self>,
    info: FetchState<Info>,
    tmp_info: Info,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
    SetInfoFetchState(FetchState<Info>),
    GetInfo,
    GetLogout,
}

impl Model {
    fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
        self.link.callback(move |_| {
            let route = app_route.clone();
            Msg::ChangeRoute(route)
        })
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);
        Model {
            route_service,
            cookie_service: CookieService::new(),
            route,
            link,
            info: FetchState::NotFetching,
            tmp_info: Info::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInfoFetchState(state) => {
                self.info = state;
                match &self.info {
                    FetchState::Success(fetched) => {
                        self.tmp_info = fetched.clone();
                        true
                    }
                    _ => false,
                }
            }
            Msg::GetInfo => {
                let future = async {
                    match AuthService::new().get().await {
                        Ok(info) => Msg::SetInfoFetchState(FetchState::Success(info)),
                        Err(err) => Msg::SetInfoFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(Msg::SetInfoFetchState(FetchState::Fetching));
                false
            }
            Msg::GetLogout => {
                self.cookie_service.remove("token");
                self.link.send_message(Msg::GetInfo);
                self.link.send_message(Msg::ChangeRoute(AppRoute::Login));
                true
            }
            Msg::RouteChanged(route) => {
                self.route = route;
                true
            }
            Msg::ChangeRoute(route) => {
                self.link.send_message(Msg::GetInfo);
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.info {
            FetchState::NotFetching => {
                self.link.send_message(Msg::GetInfo);
            }
            _ => {}
        }
        html! {
            <>
            <Navbar info=self.tmp_info.clone() link_app=self.link.clone()/>
            <div style={"padding-top:30px;padding-bottom:30px;"}>
            <ybc::Container>
            {
                match AppRoute::switch(self.route.clone()) {
                    Some(AppRoute::Main) => html!{<Main info=self.tmp_info.clone()/>},
                    Some(AppRoute::Register) => html!{<Register info=self.tmp_info.clone() link_app=self.link.clone()/>},
                    Some(AppRoute::Login) => html!{<Login info=self.tmp_info.clone() link_app=self.link.clone()/>},
                    Some(AppRoute::Accounts) => html!{<Accounts info=self.tmp_info.clone() link_app=self.link.clone()/>},
                    Some(AppRoute::Posts) => html!{<Posts info=self.tmp_info.clone() link_app=self.link.clone()/>},
                    None => VNode::from("404"),
                }
            }
            </ybc::Container>
            </div>
            <Footer/>
            </>
        }
    }
}
