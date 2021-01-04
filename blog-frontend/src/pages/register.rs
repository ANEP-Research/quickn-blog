use crate::api::*;
use crate::app::{self, Model};
use crate::route::AppRoute;
use ybc::InputType;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Register {
    username: String,
    pass: String,
    email: String,
    term: bool,
    info: FetchState<AccountResult>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePass(String),
    UpdateEmail(String),
    UpdateTerm(bool),
    GetRegister,
    SetInfoFetchState(FetchState<AccountResult>),
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            username: String::new(),
            pass: String::new(),
            email: String::new(),
            term: false,
            info: FetchState::NotFetching,
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
            Msg::UpdatePass(p) => {
                self.pass = p;
                false
            }
            Msg::UpdateEmail(e) => {
                self.email = e;
                false
            }
            Msg::UpdateTerm(t) => {
                self.term = t;
                true
            }
            Msg::GetRegister => {
                if !mailchecker::is_valid(&self.email.clone()) {
                    self.info = FetchState::Success(AccountResult {
                        success: false,
                        error_msg: Some(AccountError::NotValidEmail),
                        token: None,
                    });
                    true
                } else {
                    let register_form = RegisterForm {
                        username: self.username.clone(),
                        pass: self.pass.clone(),
                        email: self.email.clone(),
                    };
                    let future = async move {
                        match register_form.send().await {
                            Ok(info) => Msg::SetInfoFetchState(FetchState::Success(info)),
                            Err(err) => Msg::SetInfoFetchState(FetchState::Failed(err)),
                        }
                    };
                    send_future(self.link.clone(), future);
                    false
                }
            }
            Msg::SetInfoFetchState(state) => {
                self.info = state;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        //self.props.neq_assign(props)
        false
    }

    fn view(&self) -> Html {
        let state = match &self.info {
            FetchState::Failed(e) => Some(AccountError::NetworkError(format!("{}", e))),
            FetchState::Success(res) => {
                if res.success {
                    self.props
                        .link_app
                        .send_message(app::Msg::ChangeRoute(AppRoute::Login));
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
                    <label class="label">{"Email"}</label>
                    <ybc::Control classes="has-icons-left">
                        <ybc::Input name="email" value=self.email.clone() update=self.link.callback(|e| Msg::UpdateEmail(e)) r#type=InputType::Email placeholder="Email"></ybc::Input>
                        <ybc::Icon classes="is-small is-left">
                            <i class="fas fa-envelope"></i>
                        </ybc::Icon>
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field>
                    <ybc::Control>
                        <ybc::Checkbox name="term" checked=self.term update=self.link.callback(|t| Msg::UpdateTerm(t))>
                            {"I agree to the terms and conditions"}
                        </ybc::Checkbox>
                    </ybc::Control>
                </ybc::Field>
                <ybc::Field>
                    <ybc::Control>
                        <ybc::Button classes="is-success" disabled=!self.term onclick=self.link.callback(|_| Msg::GetRegister)>
                            {"Submit"}
                        </ybc::Button>
                    </ybc::Control>
                </ybc::Field>
            </ybc::Box>
        }
    }
}
