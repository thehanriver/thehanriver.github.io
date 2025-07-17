use yew::prelude::*;

#[function_component]
pub fn Skills() -> Html {
    html! {
        <section class="resume-section" id="skills">
            <div class="resume-section-content">
                <h2 class="mb-5 text-secondary">{"Skills"}<i class="fa fa-desktop"></i></h2>
                <div class="subheading mb-3">{"Programming Languages"}</div>
                <ul class="list-inline" type="disc">
                    <li class="list-inline">{"Rust - Most Proficient"}</li>
                    <li class="list-inline">{"Matlab"}</li>
                    <li class="list-inline">{"OCaml"}</li>
                    <li class="list-inline">{"C/C#/C++"}</li>
                    <li class="list-inline">{"Verilog/VHDL"}</li>
                    <li class="list-inline">{"Python"}</li>
                    <li class="list-inline">{"Java/JavaScript"}</li>
                    <li class="list-inline">{"HTML/CSS"}</li>
                </ul>
                <div class="subheading mb-3">{"Developer Tools"}</div>
                <ul class="list-inline" type="circle">
                    <li class="list-inline">{"Git"}</li>
                    <li class="list-inline">{"Docker"}</li>
                    <li class="list-inline">{"Visual Studio"}</li>
                    <li class="list-inline">{"Firebase"}</li>
                    <li class="list-inline">{"MongoDB"}</li>
                    <li class="list-inline">{"Cypress.io"}</li>
                </ul>

                <div class="subheading mb-3">{"Frameworks"}</div>
                <ul class="list-inline" type="circle">
                    <li class="list-inline">{"React"}</li>
                    <li class="list-inline">{"Node.js"}</li>
                </ul>

                <div class="subheading mb-3">{"Languages"}</div>
                <ul class="list-inline" type="circle">
                    <li class="list-inline">{"English"}</li>
                    <li class="list-inline">{"Korean"}</li>
                </ul>
            </div>
        </section>
    }
}
