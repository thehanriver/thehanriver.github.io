use yew::prelude::*;

#[function_component(RightSidebar)]
pub fn right_sidebar() -> Html {
    html! {
        <aside class="right-sidebar">
            <div class="email-wrapper">
                <a rel="noopener noreferrer" href="mailto:mariohan1234@gmail.com">{"mariohan1234@gmail.com"}</a>
            </div>
        </aside>
    }
}
