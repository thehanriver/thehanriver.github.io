use crate::components::{
    left_sidebar::LeftSidebar,
    right_sidebar::RightSidebar,
    sections::{
        education::Education, experience::Experience, interests::Interests, intro::Intro,
        projects::Projects, skills::Skills,
    },
};

use yew::prelude::*;
pub mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app-container">
            <LeftSidebar />
            <RightSidebar />

            <main class="main-content">
                <Intro />
                <Experience />
                <Education />
                <Skills />
                <Interests />
                <Projects />
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
