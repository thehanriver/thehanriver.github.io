use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub github_link: String,
    // pub details_page_link: Option<String>,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    html! {
        <li class="project-card">
            <div class="card-content">
                <header class="project-header">
                    <div class="project-icon"><i class="far fa-folder-open"></i></div>
                    <div class="project-links">
                        // ... links ...
                    </div>
                </header>

                <h3 class="project-title"><a rel="noopener noreferrer" href={props.github_link.clone()}>{ &props.title }</a></h3>
                <div class="project-description">
                    <p>{ &props.description }</p>
                </div>

                <ul class="project-tags">
                    { for props.tags.iter().map(|tag| {
                        let tag_class = format!("tag-{}", tag.to_lowercase().replace(" ", "-"));
                        html!{ <li class={tag_class}>{tag}</li> }
                    }) }
                </ul>
            </div>
        </li>
    }
}
