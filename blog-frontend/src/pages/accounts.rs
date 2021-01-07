use crate::api::*;
use crate::app::{self, Model};
use crate::route::AppRoute;
use crate::services::CookieService;
use crate::errors::FetchError;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Accounts {
    title: String,
    body: String,
    props: Props,
    cookie: CookieService,
    res: FetchState<PostsResponse>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub info: Info,
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {
    UpdateTitle(String),
    UpdateBody(String),
    GetSumbit,
    UpdateFetchState(FetchState<PostsResponse>)
}

impl Component for Accounts {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self { 
            title: String::new(), 
            body: String::new(), 
            props, 
            cookie: CookieService::new(),
            res: FetchState::NotFetching,
            link 
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::UpdateTitle(title) => {
                self.title = title;
                false
            },
            Msg::UpdateBody(body) => {
                self.body = body;
                false
            },
            Msg::GetSumbit => {
                if let Ok(token) = self.cookie.get("token") {
                    let create_post = CreatePost {
                        token,
                        title: self.title.clone(),
                        body: self.body.clone(),
                    };
                    let future = async move {
                        match create_post.send().await {
                            Ok(info) => Msg::UpdateFetchState(FetchState::Success(info)),
                            Err(err) => Msg::UpdateFetchState(FetchState::Failed(FetchError::from(JsValue::from_str(&format!("{}", err))))),
                        }
                    };
                    send_future(self.link.clone(), future);
                    false
                } else {
                    self.link.send_message(Msg::UpdateFetchState(FetchState::Failed(FetchError::from(JsValue::from_str("You aren't logined yet.")))));
                    true
                }
            },
            Msg::UpdateFetchState(state) => {
                self.res = state.clone();
                true
            },
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if self.props.info != props.info {
            self.props.info = props.info;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        //info!("{:?}", self.props.info);
        if !self.props.info.account_info.success {
            self.props
                .link_app
                .send_message(app::Msg::ChangeRoute(AppRoute::Login));
            html! {}
        } else {
            html! {
                <>
                    {
                        match &self.res {
                            FetchState::Success(_) => {
                                html! {
                                    <ybc::Notification classes="is-success">
                                        { "Success!" }
                                    </ybc::Notification>
                                }
                            },
                            FetchState::Failed(err) => {
                                html! {
                                    <ybc::Notification classes="is-success">
                                        { &format!("Sorry, An error occurred: {}", err)  }
                                    </ybc::Notification>
                                }
                            },
                            _ => { html! {} }
                        }
                    }
                    <ybc::Title>
                        {&format!("Welcome, {}", self.props.info.account_info.username.clone().unwrap())}
                    </ybc::Title>
                    <ybc::Button onclick=self.props.link_app.callback(|_| app::Msg::GetLogout)>
                        {"Logout"}
                    </ybc::Button>
                    <ybc::Container>
                    <ybc::Field>
                        <label class="label">{"Title"}</label>
                        <ybc::Control>
                            <ybc::Input name="title" value=self.title.clone() update=self.link.callback(|s| Msg::UpdateTitle(s))/>
                        </ybc::Control>
                    </ybc::Field>
                    <ybc::Field>
                        <label class="label">{"Body"}</label>
                        <ybc::Control>
                            <ybc::TextArea name="body" value=self.body.clone() update=self.link.callback(|s| Msg::UpdateBody(s))/>
                        </ybc::Control>
                    </ybc::Field>
                    <ybc::Field>
                        <ybc::Control>
                            <ybc::Button classes="is-success" onclick=self.link.callback(|_| Msg::GetSumbit)>
                                {"Sumbit"}
                            </ybc::Button>
                        </ybc::Control>
                    </ybc::Field>
                    </ybc::Container>
                </>
            }
        }
    }
}
