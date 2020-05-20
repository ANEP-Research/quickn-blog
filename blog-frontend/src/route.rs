#[derive(yew_router::Switch, Debug)]
pub enum AppRoute {
    #[to = "/"]
    Main,
    #[to = "/accounts"]
    Accounts,
}

#[derive(yew_router::Switch, Debug)]
pub enum ApiRoute {
    #[to = "/app/info"]
    Info,
}