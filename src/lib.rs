#![recursion_limit="500"]

mod components;

use stdweb::web::{Location, window};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::services::ConsoleService;
use yew_router::{route::Route, service::RouteService, components::RouterAnchor, Switch, router::Router};
use components::navbar::*;


struct Model {
    link: ComponentLink<Self>,
    route: Route<()>,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/page1/"]
    Page1,
    #[to = "/page2/"]
    Page2,
    #[to = "/page3/"]
    Page3,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        Self {
            link,
            route,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="navbar navbar-expand-sm bg-light justify-content-center">
                    <a class="navbar-brand" href="#">{"YEW Demo"}</a>
                    <ul class="navbar-nav mr-auto">
                    <li class="nav-item">
                        <RouterAnchor<AppRoute> route=AppRoute::Page1> {"Page 1"} </RouterAnchor<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <RouterAnchor<AppRoute> route=AppRoute::Page2> {"Page 2"} </RouterAnchor<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <RouterAnchor<AppRoute> route=AppRoute::Page3> {"Page 3"} </RouterAnchor<AppRoute>>
                    </li>
                    </ul>
                </nav>
                <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Page1 => html!{ "Page 1" },
                                AppRoute::Page2 => html!{"Page 2"},
                                AppRoute::Page3 => html!{"Page 3"},
                            }
                        })
                    />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}