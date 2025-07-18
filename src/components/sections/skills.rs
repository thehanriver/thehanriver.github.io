use crate::components::sections::section::Section;
use yew::prelude::*; // Import our Section wrapper

#[function_component]
pub fn Skills() -> Html {
    html! {
        <Section id="skills" title="Skills">
            <div class="skills-grid">
                <div class="skill-category">
                    <h3>{"Programming Languages"}</h3>
                    <ul>
                        <li>{"Rust"}</li>
                        <li>{"Python"}</li>
                        <li>{"JavaScript / TypeScript"}</li>
                        <li>{"C/C++"}</li>
                        <li>{"OCaml"}</li>
                        <li>{"Java"}</li>
                        <li>{"HTML/CSS"}</li>
                        <li>{"Verilog/VHDL"}</li>
                        <li>{"Matlab"}</li>
                    </ul>
                </div>

                <div class="skill-category">
                    <h3>{"Developer Tools"}</h3>
                    <ul>
                        <li>{"Git & GitHub"}</li>
                        <li>{"Docker"}</li>
                        <li>{"Visual Studio Code"}</li>
                        <li>{"Firebase"}</li>
                        <li>{"MongoDB"}</li>
                        <li>{"Cypress.io"}</li>
                        <li>{"Wireshark"}</li>
                    </ul>
                </div>

                <div class="skill-category">
                    <h3>{"Frameworks"}</h3>
                    <ul>
                        <li>{"Node.js"}</li>
                        <li>{"React"}</li>
                        <li>{"Yew.rs"}</li>
                    </ul>
                </div>

                 <div class="skill-category">
                    <h3>{"Languages"}</h3>
                    <ul>
                        <li>{"English (Native)"}</li>
                        <li>{"Korean (Conversational)"}</li>
                    </ul>
                </div>
            </div>
        </Section>
    }
}
