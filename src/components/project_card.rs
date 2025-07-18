// in src/components/project_card.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub github_link: String,
    // We'll add this later for the separate pages you mentioned
    // pub details_page_link: Option<String>,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <li class="project-card">
            <div class="card-content">
                <header class="project-header">
                    <div class="project-icon">
                        <i class="far fa-folder-open"></i>
                    </div>
                    <div class="project-links">
                        <a href={props.github_link.clone()} target="_blank" rel="noopener noreferrer" aria-label="GitHub Link">
                            <i class="fab fa-github"></i>
                        </a>
                        // We can add an external link icon here later if a project is live
                    </div>
                </header>

                <h3 class="project-title">
                    // For now, the title links to GitHub. Later, it can link to the details page.
                    <a href={props.github_link.clone()}>{ &props.title }</a>
                </h3>

                <div class="project-description">
                    <p>{ &props.description }</p>
                </div>

                <footer class="project-tags">
                    <ul>
                        { for props.tags.iter().map(|tag| html!{ <li>{tag}</li> }) }
                    </ul>
                </footer>
            </div>
        </li>
    }
}
