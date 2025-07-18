use crate::components::sections::section::Section;
use yew::prelude::*;

#[function_component]
pub fn Interests() -> Html {
    html! {
        <Section id="interests" title="Interests">

            // Use our own styled paragraphs. The .lead class from Bootstrap is gone.
            <p>
                {"Apart from being a software engineer, I spend a lot of my free time being active. Whether it's hitting a new personal record in powerlifting or exploring a hiking trail, I love the challenge of pushing my physical limits. It’s a great way to clear my head and approach problems with a fresh perspective."}
            </p>
            <p>
                {"When I'm indoors, I'm an avid follower of the sci-fi and fantasy genres in film and television. I'm also an aspiring chef, constantly experimenting with new recipes. I dedicate a significant amount of my time to exploring the latest technology advancements in the backend and systems programming world."}
            </p>

            // The <hr /> is no longer needed because our sections have distinct padding
            // that separates them visually.

        </Section>
    }
}
