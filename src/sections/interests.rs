use yew::prelude::*;

#[function_component]
pub fn Interests() -> Html {
    html! {
        <section class="resume-section" id="interests">
                <div class="resume-section-content">
                    <h2 class="mb-5 text-secondary">{"Interests"}<i class="fa fa-headphones"></i></h2>
                    <p>{"Apart from my search for my passion, I enjoy most of my time being active whether I am powerlifting
                    or going on a simple hike. When I'm not spending most of my time at the gym, I like to discover my
                    next new favorite coffee spots and always explore to find new books I can read."}</p>
                    <p class="mb-0">{"When forced indoors, I follow a number of horror movies as well as anime TV-shows. As a
                        side hobby, I'm currently learning how to cook and practice coding whenever I have time."}</p>
                </div>
            </section>
    }
}
