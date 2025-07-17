use yew::prelude::*;

#[function_component]
pub fn Intro() -> Html {
    html! {
        <>
            <section class="resume-section" id="about">
                <div class="resume-section-content">
                    <h1 class="mb-0">
                        {"Mario"}
                        <span class="text-secondary">{" Han"}</span>
                    </h1>
                    <div class="subheading mb-5">
                        <span class="text-secondary">{"mariohan1234@gmail.com"}</span>
                    </div>
                    <p class="lead mb-5">{"Hi, I'm Mario Han — a Computer Engineering graduate from Boston University (Class
                        of 2021) with a passion for building resilient systems and user-focused software."}</p>
                    <p class="lead mb-5">{"I specialize in full-stack development, software engineering, and embedded systems.
                        At Wildseed, a fast-paced SaaS startup, I deepened my experience in full-stack engineering and
                        learned to thrive in agile environments. Later, at ClumL, I worked cross-functionally to deliver
                        innovative product features — improving system stability and accelerating release cycles by bridging
                        front-end and back-end engineering."}</p>
                    <p class="lead mb-5">{"Outside of work, I'm a part-time powerlifter and lifelong gamer. Whether debugging
                        code or hitting a new PR, I love tackling challenges I don't get on the first try — it is where I
                        grow most."}</p>
                    <p class="lead mb-5">
                        {"I'm currently seeking software engineering, full-stack, or embedded systems roles — ideally in the
                        Los Angeles area, but I'm open to relocating for the right opportunity."}</p>
                    <p class="lead mb-5">
                        {"Contact me at mariohan1234@gmail.com"}</p>
                    <div class="social-icons">
                        <a class="social-icon" href="https://www.linkedin.com/in/mario-h-642057122/"><i
                                class="fab fa-linkedin-in"></i></a>
                        <a class="social-icon" href="https://github.com/thehanriver"><i class="fab fa-github"></i></a>
                    </div>
                </div>
            </section>
            <hr class="m-0" />
        </>
    }
}
