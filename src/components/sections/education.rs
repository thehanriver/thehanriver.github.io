use crate::components::education_entry::EducationEntry;
use crate::components::sections::section::Section;
use yew::prelude::*;

#[function_component]
pub fn Education() -> Html {
    html! {
        <Section id="education" title="Education">
            <EducationEntry
                institution="Boston University" // Corrected the name
                degree_or_title="Bachelor of Science in Computer Engineering"
                date_range="August 2017 - May 2021"
            >
                // --- We pass the course lists in as children ---
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
                            // etc.
                        </ul>
                    </div>
                    <div class="course-category">
                        <h4>{"Others"}</h4>
                        <ul>
                            <li>{"Probabilty & Statistics"}</li>
                            // etc.
                        </ul>
                    </div>
                </div>
            </EducationEntry>

            <EducationEntry
                institution="Diamond Bar High School"
                degree_or_title="High School Diploma" // Be more descriptive
                date_range="August 2013 - May 2017"
            /> // This entry has no children, which is fine.

        </Section>
        // <section class="resume-section" id="education">
        //     <div class="resume-section-content">
        //         <h2 class="mb-5 text-secondary">{"Education"}<i class="fa fa-university"></i></h2>
        //         <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
        //             <div class="flex-grow-1">
        //                 <h3 class="mb-0">{"University of Boston University"}</h3>
        //                 <div class="subheading mb-3">{"Bachelor of Science"}</div>
        //                 <div>{"Major: Computer Engineering"}</div>
        //                 <p>
        //                 <div class="subheading mb-3">{"Relavant Coursework"}</div>
        //                 </p>
        //                 <div class="subheading mb-3">{"Programming"}</div>
        //                 <ul class="list-inline" type="disc">
        //                     <li class="list-inline">{"Applied Algorithms"}</li>
        //                     <li class="list-inline">{"Computer Organization"}</li>
        //                     <li class="list-inline">{"Operating Systems"}</li>
        //                     <li class="list-inline">{"Software Design"}</li>
        //                 </ul>
        //                 <div class="subheading mb-3">{"Hardware"}</div>
        //                 <ul class="list-inline" type="disc">
        //                     <li class="list-inline">{"Electric Circuits"}</li>
        //                     <li class="list-inline">{"Logic Design"}</li>
        //                     <li class="list-inline">{"Mechanics"}</li>
        //                     <li class="list-inline">{"Signals & Systems"}</li>
        //                     <li class="list-inline">{"Smart & Connected Systems"}</li>
        //                     <li class="list-inline">{"Communication Systems"}</li>
        //                 </ul>
        //                 <div class="subheading mb-3">{"Others"}</div>
        //                 <ul class="list-inline" type="disc">
        //                     <li class="list-inline">{"Probabilty & Statistics"}</li>
        //                     <li class="list-inline">{"Business Technology Innovation"}</li>
        //                     <li class="list-inline">{"Strategy in Tech Firms"}</li>

        //                 </ul>

        //             </div>
        //             <div class="flex-shrink-0"><span class="text-primary">{"August 2017 - May 2021"}</span></div>
        //         </div>
        //         <div class="d-flex flex-column flex-md-row justify-content-between">
        //             <div class="flex-grow-1">
        //                 <h3 class="mb-0">{"Diamond Bar High School"}</h3>
        //             </div>
        //             <div class="flex-shrink-0"><span class="text-primary">{"August 2013 - May 2017"}</span></div>
        //         </div>
        //     </div>
        // </section>
    }
}
