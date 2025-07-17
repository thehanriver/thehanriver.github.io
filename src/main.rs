use crate::sections::{
    education::Education, experience::Experience, interests::Interests, intro::Intro,
    navbar::NavBar, projects::Projects, skills::Skills,
};
use yew::prelude::*;

pub mod sections;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <NavBar />
            <div class="container-fluid p-0">
                <Intro />
                <Experience />
                <Education />
                <Skills />
                <Interests />
                <Projects />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
