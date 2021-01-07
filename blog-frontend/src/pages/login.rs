use crate::api::*;
use crate::app::{self, Model};
use crate::route::AppRoute;
use crate::services::CookieService;
use ybc::InputType;
use yew::prelude::*;

pub struct Login {
    username: String,
    pass: String,
    info: FetchState<AccountResult>,
    cookie: CookieService,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub info: Info,
    pub link_app: ComponentLink<Model>,
}

#[derive(Clone)]
pub enum Msg {
    UpdateUsername(String),
    UpdatePass(String),
    GetLogin,
    UpdateFetchState(FetchState<AccountResult>),
}

impl Component for Login {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            username: String::new(),
            pass: String::new(),
            info: FetchState::NotFetching,
            cookie: CookieService::new(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(s) => {
                self.username = s;
                false
            }
            Msg::UpdatePass(s) => {
                self.pass = s;
                false
            }
            Msg::GetLogin => {
                let login_form = LoginForm {
                    username: self.username.clone(),
                    pass: self.pass.clone(),
                };
                let future = async move {
                    match login_form.send().await {
                        Ok(info) => Msg::UpdateFetchState(FetchState::Success(info)),
                        Err(err) => Msg::UpdateFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::UpdateFetchState(state) => {
                self.info = state;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        //self.props.neq_assign(props)
        if self.props.info != props.info {
            self.props.info = props.info;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.info.account_info.success {
            self.props
                .link_app
                .send_message(app::Msg::ChangeRoute(AppRoute::Accounts));
        }
        let state = match self.info.clone() {
            FetchState::Failed(e) => Some(AccountError::NetworkError(format!("{}", e))),
            FetchState::Success(res) => {
                if res.success {
                    self.cookie.set("token", &res.token.unwrap());
                    self.props
                        .link_app
                        .send_message(app::Msg::ChangeRoute(AppRoute::Main));
                }
                res.error_msg.clone()
            }
            _ => None,
        };
        html! {
            <ybc::Box>
                {
                if let Some(s) = state {
                    html! {
                    <ybc::Notification classes="is-danger">
                        { &format!("Sorry, An error occurred: {}",s)  }
                    </ybc::Notification>
                    }
                } else {
                    html!{}
                }
                }
                <ybc::Field>
                    <label class="label">{"Username"}</label>
                    <ybc::Control classes="has-icons-left">
                        <ybc::Input name="username" value=self.username.clone() update=self.link.callback(|s| Msg::UpdateUsername(s)) placeholder="Username"></ybc::Input>
                        <ybc::Icon classes="is-small is-left">
                            <i class="fas fa-user"></i>
                        </ybc::Icon>
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field>
                    <label class="label">{"Password"}</label>
                    <ybc::Control classes="has-icons-left">
                        <ybc::Input name="pass" value=self.pass.clone() update=self.link.callback(|p| Msg::UpdatePass(p)) r#type=InputType::Password placeholder="Password"></ybc::Input>
                        <ybc::Icon classes="is-small is-left">
                            <i class="fas fa-lock"></i>
                        </ybc::Icon>
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field>
                    <ybc::Control>
                        <ybc::Button classes="is-success" onclick=self.link.callback(|_| Msg::GetLogin)>
                            {"Submit"}
                        </ybc::Button>
                    </ybc::Control>
                </ybc::Field>
            </ybc::Box>
        }
    }
}
