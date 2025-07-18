// in src/components/education_entry.rs
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EducationEntryProps {
    pub institution: String,
    pub degree_or_title: String,
    pub date_range: String,

    // `children` will let us pass the course lists or other details
    // into the component from the outside.
    #[prop_or_default]
    pub children: Children,
}

#[function_component(EducationEntry)]
pub fn education_entry(props: &EducationEntryProps) -> Html {
    html! {
        <div class="education-entry">
            <div class="education-header">
                <div class="education-institution-degree">
                    <h3 class="institution-name">{&props.institution}</h3>
                    <div class="degree-title">{&props.degree_or_title}</div>
                </div>
                <div class="education-date">
                    <span>{&props.date_range}</span>
                </div>
            </div>

            // The main content of the entry, like the course list
            <div class="education-body">
                { for props.children.iter() }
            </div>
        </div>
    }
}
