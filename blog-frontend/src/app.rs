use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::{route::Route, service::RouteService, Switch};
use crate::navbar::Navbar;
use crate::route::AppRoute;

pub struct Model {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
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
            route,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <Navbar/>
            {
                match AppRoute::switch(self.route.clone()) {
                    Some(AppRoute::Main) => VNode::from(""),
                    Some(AppRoute::Accounts) => VNode::from(""),
                    None => VNode::from("404"),
                }
            }
            </div>
        }
    }
}