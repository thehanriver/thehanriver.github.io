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
                // This header section remains the same
                <header class="project-header">
                    <div class="project-icon"><i class="far fa-folder-open"></i></div>
                    <div class="project-links">
                        // ... your links ...
                    </div>
                </header>

                // The title and description also remain the same
                <h3 class="project-title"><a rel="noopener noreferrer" href={props.github_link.clone()}>{ &props.title }</a></h3>
                <div class="project-description">
                    <p>{ &props.description }</p>
                </div>

                // --- THE REFACTOR ---
                // We've moved the tags from a <footer> into the main content div.
                // This ensures they stay "attached" to the description.
                <ul class="project-tags">
                    { for props.tags.iter().map(|tag| {
                        // We add a class to each tag based on its name for custom styling
                        let tag_class = format!("tag-{}", tag.to_lowercase().replace(" ", "-"));
                        html!{ <li class={tag_class}>{tag}</li> }
                    }) }
                </ul>
            </div>
        </li>
    }
}
