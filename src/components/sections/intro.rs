use yew::prelude::*;
// We no longer need to import the Section component here

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <section class="content-section" id="intro">
            // The title or main heading of the whole page
            <h1 class="intro-name">
                {"Mario"}
                // Use a <span> with a class so we can color it with our accent
                <span class="intro-name-accent">{" Han"}</span>
            </h1>

            // Paragraphs can now use a simpler class if needed, or none at all
            <p class="intro-paragraph">{"Hi, I'm Mario Han — a Computer Engineering graduate from Boston University (Class
                of 2021) with a passion for building resilient systems and user-focused software."}</p>
            <p class="intro-paragraph">{"I specialize in full-stack development, software engineering, and embedded systems.
                At Wildseed, a fast-paced SaaS startup, I deepened my experience in full-stack engineering and
                learned to thrive in agile environments. Later, at ClumL, I worked cross-functionally to deliver
                innovative product features — improving system stability and accelerating release cycles by bridging
                front-end and back-end engineering."}</p>
            <p class="intro-paragraph">{"Outside of work, I'm a part-time powerlifter and lifelong gamer. Whether debugging
                code or hitting a new PR, I love tackling challenges I don't get on the first try — it is where I
                grow most."}</p>
            <p class="intro-paragraph">
                {"I'm currently seeking software engineering, full-stack, or embedded systems roles — ideally in the
                Los Angeles area, but I'm open to relocating for the right opportunity."}</p>
        </section>
    }
}
