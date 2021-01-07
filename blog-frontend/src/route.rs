#[derive(yew_router::Switch, Debug, Clone)]
pub enum AppRoute {
    //#[to = "/post/{id}"]
    //ViewPost(i32),
    #[to = "/accounts"]
    Accounts,
    #[to = "/register"]
    Register,
    #[to = "/login"]
    Login,
    #[to = "/posts"]
    Posts,
    #[to = "/"]
    Main,
}

#[derive(yew_router::Switch, Debug)]
pub enum ApiRoute {
    #[to = "/app/info"]
    Info,
}
