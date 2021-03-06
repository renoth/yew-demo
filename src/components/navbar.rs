use yew::prelude::*;

pub struct Navbar {
    link: ComponentLink<Self>,
}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            {"suuuuuuuuuuuuuuuuper"}
        }
    }
}