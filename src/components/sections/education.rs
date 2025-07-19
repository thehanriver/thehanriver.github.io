use crate::components::education_entry::EducationEntry;
use crate::components::sections::section::Section;
use yew::prelude::*;

#[function_component]
pub fn Education() -> Html {
    html! {
        <Section id="education" title="Education">
            <EducationEntry
                institution="Boston University"
                degree_or_title="Bachelor of Science in Computer Engineering"
                date_range="August 2017 - May 2021"
            >
                <p class="course-work-heading">{"Relevant Coursework:"}</p>
                <div class="course-lists">
                    <div class="course-category">
                        <h4>{"Programming"}</h4>
                        <ul>
                            <li>{"Applied Algorithms"}</li>
                            <li>{"Computer Organization"}</li>
                            <li>{"Operating Systems"}</li>
                            <li>{"Software Design"}</li>
                        </ul>
                    </div>
                    <div class="course-category">
                        <h4>{"Hardware"}</h4>
                        <ul>
                            <li>{"Electric Circuits"}</li>
                            <li>{"Logic Design"}</li>
                            <li>{"Mechanics"}</li>
                            <li>{"Signals & Systems"}</li>
                            <li>{"Smart & Connected Systems"}</li>
                            <li>{"Communication Systems"}</li>
                        </ul>
                    </div>
                    <div class="course-category">
                        <h4>{"Others"}</h4>
                        <ul>
                            <li>{"Probabilty & Statistics"}</li>
                            <li>{"Business Technology Innovation"}</li>
                            <li>{"Strategy in Tech Firms"}</li>
                        </ul>
                    </div>
                </div>
            </EducationEntry>

            <EducationEntry
                institution="Diamond Bar High School"
                degree_or_title="High School Diploma"
                date_range="August 2013 - May 2017"
            />

        </Section>
    }
}
