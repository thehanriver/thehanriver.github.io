use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub id: String,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section id={props.id.clone()} class="content-section">
            <div class="section-heading">
                <h2 class="heading-title">{ &props.title }</h2>
            </div>

            <div class="section-content">
                { for props.children.iter() }
            </div>
        </section>
    }
}
