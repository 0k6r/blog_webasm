use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <template>
                <div class="wrapper">
                    <header>
                        <h1 class="logo">
                            <RouterAnchor<AppRoute> route=AppRoute::Home classes="navbar-brand">
                                { "KONSTANTIN PETROV" }
                            </RouterAnchor<AppRoute>>
                        </h1>
                        <input class="switcher" type="checkbox" id="menu" />
                        <label class="open" for="menu">
                            <h2>{ "MENU" }</h2>
                        </label>
                        <nav>
                            <div class="nav-menu">
                                <ul>
                                    <li>
                                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                            { "Home" }
                                        </RouterAnchor<AppRoute>>
                                    </li>
                                    <li>
                                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                            { "About" }
                                        </RouterAnchor<AppRoute>>
                                    </li>
                                    <li>
                                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                            { "Blog" }
                                        </RouterAnchor<AppRoute>>
                                    </li>
                                    <li>
                                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                                            { "Contact" }
                                        </RouterAnchor<AppRoute>>
                                    </li>
                                </ul>
                            </div>
                            <label class="close" for="menu">
                                <i class="fa fa-times"></i>
                            </label>
                        </nav>
                    </header>
                </div>
            </template>
        }
    }
}
