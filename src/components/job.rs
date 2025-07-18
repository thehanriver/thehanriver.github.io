use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JobProps {
    pub title: String,
    pub company: String,
    pub date_range: String,
    pub location: String,
    #[prop_or_default]
    pub description_points: Vec<String>, // A list of bullet points
    #[prop_or_default]
    pub tags: Vec<String>,
}

#[function_component(Job)]
pub fn job(props: &JobProps) -> Html {
    html! {
        <div class="job-entry">
            <div class="job-header">
                <div class="job-title-company">
                    <h3 class="job-title">{&props.title}</h3>
                    <div class="job-company">{&props.company}</div>
                </div>
                <div class="job-date-location">
                    <span class="job-date">{&props.date_range}</span>
                    <p class="job-location">{&props.location}</p>
                </div>
            </div>

            <ul class="job-description">
                { for props.description_points.iter().map(|point| html!{ <li>{point}</li> }) }
            </ul>
            if !props.tags.is_empty() {
                <ul class="job-tags">
                    {
                        for props.tags.iter().map(|tag| {
                            html!{<li>{tag}</li>}
                        })
                    }
                </ul>
            }
        </div>
    }
}
