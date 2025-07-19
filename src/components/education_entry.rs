use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EducationEntryProps {
    pub institution: String,
    pub degree_or_title: String,
    pub date_range: String,

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

            <div class="education-body">
                { for props.children.iter() }
            </div>
        </div>
    }
}
