use yew::prelude::*;

#[function_component]
pub fn Projects() -> Html {
    html! {
        <section class="resume-section" id="projects">
            <div class="resume-section-content">
                <h2 class="mb-5 text-secondary">{"Projects and Achievements"}<i class="fa fa-rocket"></i></h2>
                <div class="subheading mb-3">{"Projects"}</div>
                <ul class="list-inline" type="circle">
                    <div>
                        <a href="https://github.com/thehanriver/topo_covidnet">{"COVID-NET on Hybrid Cloud"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/COVID-Symptoms-Site">{"COVID Symptoms Web App"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/Lets-Get-This-Bread">{"Let's Get This Bread App"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/ESP-Team-Projects">{"ESP-32 Projects"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/VS-Projects">{"VS Web Forms Projects"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/Operating-Systems">{"Operating Systems Projects"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/Software-Radio">{"Software Radio"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/RISC-V-CPU">{"RISCV-CPU"}</a>
                    </div>
                    <div>
                        <a href="https://github.com/thehanriver/VGA-Board-Lock">{"VGA Board Lock"}</a>
                    </div>
                </ul>
                <div class="subheading mb-3">{"Achievements"}</div>
                <ul class="fa-ul mb-0">
                    <li>
                        <span class="fa-li"><i class="fas fa-trophy text-warning"></i></span>
                        {"Eagle Scout - Boy Scouts of America, Troop 278"}
                    </li>
                </ul>
            </div>
        </section>
    }
}
