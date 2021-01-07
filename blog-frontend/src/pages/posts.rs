use crate::api::{self, *};
use crate::app::Model;
use crate::constants::*;
use std::cmp::min;
use yew::prelude::*;

pub struct Posts {
    cur: i64,
    list: Vec<(api::Posts, api::Author)>,
    previous_enabled: bool,
    next_enabled: bool,
    posts: FetchState<PostsResponse>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub info: Info,
    pub link_app: ComponentLink<Model>,
}

pub enum Msg {
    GetPosts,
    SetPostsFetchState(FetchState<PostsResponse>),
    Next,
    Previous,
}

impl Component for Posts {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            cur: 0,
            list: vec![],
            previous_enabled: false,
            next_enabled: false,
            posts: FetchState::NotFetching,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::GetPosts => {
                let get_posts = GetPosts {
                    start: self.cur * MAX_POSTS,
                    limit: MAX_POSTS * 2,
                };
                let future = async move {
                    match get_posts.get().await {
                        Ok(info) => Msg::SetPostsFetchState(FetchState::Success(info)),
                        Err(err) => Msg::SetPostsFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(Msg::SetPostsFetchState(FetchState::Fetching));
                false
            }
            Msg::SetPostsFetchState(state) => {
                self.posts = state.clone();
                match state {
                    FetchState::Success(response) => {
                        if let Some(posts) = response.body {
                            if posts.len() > (MAX_POSTS as usize) {
                                self.next_enabled = true;
                            } else {
                                self.next_enabled = false;
                            }
                            self.list = posts;
                        }
                        true
                    }
                    _ => false,
                }
            }
            Msg::Next => {
                self.cur += 1;
                self.previous_enabled = true;
                self.link.send_message(Msg::GetPosts);
                false
            }
            Msg::Previous => {
                self.cur -= 1;
                if self.cur == 0 {
                    self.previous_enabled = false;
                }
                self.link.send_message(Msg::GetPosts);
                false
            }
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
        match &self.posts {
            FetchState::NotFetching => {
                self.link.send_message(Msg::GetPosts);
            }
            _ => {}
        }
        html! {
            <>
                {
                    self.list.get(0..min(MAX_POSTS as usize, self.list.len())).unwrap().iter().map(|post| html! {
                        <div style={"padding-top:15px;padding-bottom:15px;"}>
                        <ybc::Card>
                            <ybc::CardHeader>
                            </ybc::CardHeader>
                            <ybc::CardContent>
                                <ybc::Media>
                                    <ybc::MediaContent>
                                        <ybc::Title classes="is-4">
                                            { post.0.title.clone() }
                                        </ybc::Title>
                                        <ybc::Subtitle classes="is-6">
                                            { &format!("@{}", post.1.name) }
                                        </ybc::Subtitle>
                                    </ybc::MediaContent>
                                </ybc::Media>
                                <ybc::Content>
                                    { &format!("{}...", post.0.body.clone().get(0..min(post.0.body.len(), PREVIEW_BODY_LENGHT)).unwrap()) }
                                    //<br/>

                                </ybc::Content>
                            </ybc::CardContent>
                            <ybc::CardFooter>
                                <a href="#" class="card-footer-item"> {"Read more"} </a>
                            </ybc::CardFooter>
                        </ybc::Card>
                        </div>
                    }).collect::<Html>()
                }
                <ybc::Buttons>
                    <ybc::Button disabled=!self.previous_enabled onclick=self.link.callback(|_| Msg::Previous)>
                        {"Previous"}
                    </ybc::Button>
                    <ybc::Button disabled=!self.next_enabled onclick=self.link.callback(|_| Msg::Next)>
                        {"Next"}
                    </ybc::Button>
                </ybc::Buttons>
            </>
        }
    }
}
