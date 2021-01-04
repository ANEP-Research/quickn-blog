use crate::api::*;
use crate::errors::FetchError;
use crate::footer::Footer;
use crate::navbar::Navbar;
use crate::pages::Main;
use crate::pages::Register;
use crate::pages::Login;
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
    info: FetchState<BlogInfo>,
    user_info: FetchState<UserInfo>,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
    SetInfoFetchState(FetchState<BlogInfo>),
    SetUserInfoFetchState(FetchState<UserInfo>),
    GetInfo,
    GetUserInfo,
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
            user_info: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInfoFetchState(state) => {
                self.info = state;
                true
            },
            Msg::SetUserInfoFetchState(state) => {
                self.user_info = state;
                true
            },
            Msg::GetUserInfo => {
                let future = async {
                    match AuthService::new().get_user().await {
                        Ok(info) => Msg::SetUserInfoFetchState(FetchState::Success(info)),
                        Err(err) => Msg::SetUserInfoFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(Msg::SetUserInfoFetchState(FetchState::Fetching));
                false
            }
            Msg::GetInfo => {
                let future = async {
                    match BlogInfo::new().await {
                        Ok(info) => Msg::SetInfoFetchState(FetchState::Success(info)),
                        Err(err) => Msg::SetInfoFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(Msg::SetInfoFetchState(FetchState::Fetching));
                false
            },
            Msg::GetLogout => {
                self.cookie_service.remove("token");
                self.link.send_message(Msg::GetUserInfo);
                true
            },
            Msg::RouteChanged(route) => {
                self.route = route;
                true
            },
            Msg::ChangeRoute(route) => {
                self.link.send_message(Msg::GetUserInfo);
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
                true
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut blog_info = BlogInfo::default();
        let mut user_info = UserInfo::default();
        match &self.info {
            FetchState::NotFetching => {
                self.link.send_message(Msg::GetInfo);
            }
            FetchState::Success(info) => {
                blog_info = info.clone();
            }
            _ => {}
        }
        match &self.user_info {
            FetchState::NotFetching => {
                self.link.send_message(Msg::GetUserInfo);
            }
            FetchState::Success(info) => {
                user_info = info.clone();
            }
            _ => {}
        }
        html! {
            <>
            <Navbar info=blog_info.clone() user_info=user_info.clone() link_app=self.link.clone()/>
            <div style={"padding-top:30px;padding-bottom:30px;"}>
            <ybc::Container>
            {
                match AppRoute::switch(self.route.clone()) {
                    Some(AppRoute::Main) => html!{<Main info={blog_info.clone()}/>},
                    Some(AppRoute::Register) => html!{<Register link_app=self.link.clone()/>},
                    Some(AppRoute::Login) => html!{<Login link_app=self.link.clone()/>},
                    Some(AppRoute::Accounts) => VNode::from(""),
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
