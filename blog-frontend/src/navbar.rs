use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::utils::host;

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
            <nav class = "uk-navbar-container uk-margin">
                <div class="uk-navbar-left">
                    <a href="#" class="uk-navbar-item uk-logo">
                        <div><img width="50" height="50" src=&format!("http://{}/static/images/mathion-logo.png", host().unwrap())></img></div>
                        <div id="blog-logo-text">{msg}</div>
                    </a>
                    <ul class="uk-navbar-nav">
                        <li>
                            <a href="/recent">{"Recent Posts"}</a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
