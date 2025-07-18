use crate::components::project_card::ProjectCard;
use crate::components::sections::section::Section; // Our main section wrapper
use yew::prelude::*; // Our new reusable card
#[function_component]
pub fn Projects() -> Html {
    html! {
        <Section id="projects" title="Things I've Built">
            // This grid will hold all the project cards
            <ul class="projects-grid">
                <ProjectCard
                    title="COVID-NET on Hybrid Cloud"
                    description="An exploration of deploying deep learning models for medical imaging on a hybrid cloud infrastructure."
                    tags={vec!["Python".to_string(), "Terraform".to_string(), "Cloud".to_string()]}
                    github_link="https://github.com/thehanriver/topo_covidnet"
                />
                <ProjectCard
                    title="COVID Symptoms Web App"
                    description="A full-stack web application allowing users to log symptoms, built with a focus on data visualization."
                    tags={vec!["JavaScript".to_string(), "React".to_string(), "Firebase".to_string()]}
                    github_link="https://github.com/thehanriver/COVID-Symptoms-Site"
                />
                 <ProjectCard
                    title="RISC-V CPU"
                    description="A 32-bit, 5-stage pipelined RISC-V CPU implementation in Verilog, exploring low-level computer architecture."
                    tags={vec!["Verilog".to_string(), "Hardware".to_string(), "Architecture".to_string()]}
                    github_link="https://github.com/thehanriver/RISC-V-CPU"
                />
                // --- Add your other top projects here ---
            </ul>

            // --- Separate sub-section for Achievements ---
            <div class="achievements-section">
                <h3 class="achievements-title">{"Achievements"}</h3>
                <ul class="achievement-list">
                    <li>
                        <i class="fas fa-trophy"></i>
                        <div class="achievement-text">
                            <strong>{"Eagle Scout - "}</strong>
                            {"Boy Scouts of America, Troop 278"}
                        </div>
                    </li>
                </ul>
            </div>
        </Section>
    }
}
// <section class="resume-section" id="projects">
//     <div class="resume-section-content">
//         <h2 class="mb-5 text-secondary">{"Projects and Achievements"}<i class="fa fa-rocket"></i></h2>
//         <div class="subheading mb-3">{"Projects"}</div>
//         <ul class="list-inline" type="circle">
//             <div>
//                 <a href="https://github.com/thehanriver/topo_covidnet">{"COVID-NET on Hybrid Cloud"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/COVID-Symptoms-Site">{"COVID Symptoms Web App"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/Lets-Get-This-Bread">{"Let's Get This Bread App"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/ESP-Team-Projects">{"ESP-32 Projects"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/VS-Projects">{"VS Web Forms Projects"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/Operating-Systems">{"Operating Systems Projects"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/Software-Radio">{"Software Radio"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/RISC-V-CPU">{"RISCV-CPU"}</a>
//             </div>
//             <div>
//                 <a href="https://github.com/thehanriver/VGA-Board-Lock">{"VGA Board Lock"}</a>
//             </div>
//         </ul>
//         <div class="subheading mb-3">{"Achievements"}</div>
//         <ul class="fa-ul mb-0">
//             <li>
//                 <span class="fa-li"><i class="fas fa-trophy text-warning"></i></span>
//                 {"Eagle Scout - Boy Scouts of America, Troop 278"}
//             </li>
//         </ul>
//     </div>
// </section>
