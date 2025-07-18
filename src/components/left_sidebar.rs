use yew::prelude::*;

#[function_component(LeftSidebar)]
pub fn left_sidebar() -> Html {
    html! {
        // We'll also rename the primary CSS class for clarity
        <nav class="left-sidebar" id="sideNav">
            <div class="sidebar-header">
                <img class="profile-img" src="assets/img/profile.jpg" alt="A picture of Mario Han" />
                <h1 class="main-heading">{"Mario Han"}</h1>
                <p class="sub-heading">{"Your Subheading / Title Here"}</p>
            </div>

            <ul class="main-nav-list">
                <li><a class="nav-link" href="#about">{"About"}</a></li>
                <li><a class="nav-link" href="#experience">{"Experience"}</a></li>
                <li><a class="nav-link" href="#skills">{"Skills"}</a></li>
                <li><a class="nav-link" href="#projects">{"Projects"}</a></li>
            </ul>

            <ul class="social-links-footer">
                <li><a href="your_github_url" target="_blank"><i class="fab fa-github"></i></a></li>
                <li><a href="your_linkedin_url" target="_blank"><i class="fab fa-linkedin-in"></i></a></li>
            </ul>
        </nav>
    }
}
