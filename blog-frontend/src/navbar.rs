use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::BlogInfo;
use crate::errors::*;

pub struct Navbar {
    info: FetchState<BlogInfo>,
    link: ComponentLink<Self>,
}

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

pub enum Msg {
    SetInfoFetchState(FetchState<BlogInfo>),
    GetInfo,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            info: FetchState::NotFetching,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetInfoFetchState(state) => {
                self.info = state;
                true
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
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut msg: &str = "";
        match &self.info {
            FetchState::NotFetching => {
                self.link.send_message(Msg::GetInfo);
            }
            FetchState::Success(info) => {
                msg = &info.blog_name;
            }
            _ => {}
        }
        html! {
            <nav>
                <div class="nav-wrapper">
                    <a href="#" class="brand-logo">{ msg }</a>
                    <a href="#" data-target="mobile-demo" class="sidenav-trigger"><i class="material-icons">{"menu"}</i></a>
                    <ul class="right hide-on-med-and-down">
                        <li><a href="/recent">{"Recent Posts"}</a></li>
                    </ul>
                </div>
                <ul class="sidenav" id="mobile-nav">
                    <li><a href="/recent">{"Recent Posts"}</a></li>
                </ul>
            </nav>
        }
    }
}
