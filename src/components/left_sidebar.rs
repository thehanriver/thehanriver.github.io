use yew::prelude::*;

#[function_component(LeftSidebar)]
pub fn left_sidebar() -> Html {
    html! {
        // We'll also rename the primary CSS class for clarity
        <nav class="left-sidebar" id="sideNav">
            <div class="sidebar-header">
                <img class="profile-img" src="assets/img/profile.jpg" alt="A picture of Mario Han" />
                <h1 class="main-heading">{"Mario Han"}</h1>
                <p class="sub-heading">{"Software Engineer whose brother isn't Luigi"}</p>
            </div>

            <ul class="main-nav-list">
                <li><a class="nav-link" href="#about">{"About"}</a></li>
                <li><a class="nav-link" href="#experience">{"Experience"}</a></li>
                <li><a class="nav-link" href="#education">{"Education"}</a></li>
                <li><a class="nav-link" href="#skills">{"Skills"}</a></li>
                <li><a class="nav-link" href="#interests">{"Interests"}</a></li>
                <li><a class="nav-link" href="#projects">{"Things I Did"}</a></li>
            </ul>

            <ul class="social-links-footer">
                <li><a href="https://github.com/thehanriver" target="_blank" rel="noopener noreferrer"><i class="fab fa-github"></i></a></li>
                <li><a href="https://www.linkedin.com/in/mario-h-642057122/" target="_blank" rel="noopener noreferrer"><i class="fab fa-linkedin-in"></i></a></li>
            </ul>
        </nav>
    }
}
