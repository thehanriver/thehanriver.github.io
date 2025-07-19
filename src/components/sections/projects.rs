use crate::components::project_card::ProjectCard;
use crate::components::sections::section::Section;
use yew::prelude::*;
#[function_component]
pub fn Projects() -> Html {
    html! {
        <Section id="projects" title="Things I've Built">
            <ul class="projects-grid">
                <ProjectCard
                    title="COVID-NET on Hybrid Cloud"
                    description="An exploration of deploying deep learning models for medical imaging on a hybrid cloud infrastructure."
                    tags={vec!["Python".to_string(), "Docker".to_string(), "Cloud".to_string(), "Shell".to_string()]}
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
                <ProjectCard
                    title="Let's Get This Bread App"
                    description="A class project my team and I built using Android studios for a competition."
                    tags={vec!["Android Studios".to_string(), "Java".to_string(), "Mobile Game".to_string()]}
                    github_link="https://github.com/thehanriver/Lets-Get-This-Bread"
                />
                <ProjectCard
                    title="ESP-32 Projects"
                    description="Class projects I did that involved ESP32 during my IoT class."
                    tags={vec!["ESP32".to_string(), "Hardware".to_string(), "C".to_string()]}
                    github_link="https://github.com/thehanriver/ESP-Team-Projects"
                />
                <ProjectCard
                    title="Operating Systmes Projects"
                    description="Operating Systems project I did for my OS class."
                    tags={vec!["Operating Systems".to_string(),"C++".to_string()]}
                    github_link="https://github.com/thehanriver/Operating-Systems"
                />
                <ProjectCard
                    title="VS Web Forms Projects"
                    description="Class project where I learned about Azure and web forms"
                    tags={vec!["Visual Studios".to_string(),"Azure".to_string(),"C#".to_string()]}
                    github_link="https://github.com/thehanriver/VS-Projects"
                />
                <ProjectCard
                    title="Software Radio"
                    description="Created a software radio with GNU radio and Matlab"
                    tags={vec!["GNU radio".to_string(),"Matlab".to_string()]}
                    github_link="https://github.com/thehanriver/Software-Radio"
                />
                <ProjectCard
                    title="VGA Board Lock"
                    description="A locking mechanism using a VGA Board and display"
                    tags={vec!["VGA".to_string(),"Verilog".to_string()]}
                    github_link="https://github.com/thehanriver/VGA-Board-Lock"
                />
            </ul>

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
