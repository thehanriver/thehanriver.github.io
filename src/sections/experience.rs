use yew::prelude::*;

#[function_component]
pub fn Experience() -> Html {
    html! {
    <>
        <section class="resume-section" id="experience">
            <div class="resume-section-content">
                <h2 class="mb-5 text-secondary">{"Experience"}<i class="fa fa-cubes"></i></h2>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0">{"Software Developer"}</h3>
                        <div class="subheading mb-3">{"ClumL"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Support cross-functional collaboration in delivering new product
                                features, ensuring seamless integration between front-end and back-end systems to
                                enhance system stability and release efficiency."}</li>
                            <li class="list-inline">{"Research and evaluate new technologies with the team to identify
                                performance optimization opportunities, leveraging tools like Wireshark to enhance
                                system efficiency."}</li>
                            <li class="list-inline"> {"Resolve critical bugs and develop new front-end and back-end
                                features, improving system stability and user experience. Engage in team-led
                                brainstorming to drive technical and performance optimizations.
                                render instead of client side"}</li>
                            <li class="list-inline">{"Exploring new tech stack in order to create and edit UI main product
                                effectively and efficiently"}</li>
                            <li class="list-inline">{"Current Tech Stack: JS/HTML/CSS and Node, Rust, GraphQL, NoSQL,
                                PostGresSQL"}</li>

                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"March 2022 - Present"}
                            <p style="text-align:right">{"Remote"}</p>
                        </span></div>

                </div>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0">{"Freelancer/Owner"}</h3>
                        <div class="subheading mb-3">{"Tekhan LLC"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Created my own LLC in order to help students and young adults to
                                make website or website protfolios of their own"}</li>
                            <li class="list-inline">{"Using Godaddy and Wordpress in order to build simple, fast, and
                                effective portfolios unless specified"}</li>
                            <li class="list-inline">{"Teaching close acquintences JavaScript ,CSS ,HTML ,and basic Web Dev"}
                            </li>
                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"January 2022 -
                            Present"}
                            <p style="text-align:right">{"Remote"}</p>
                        </span></div>

                </div>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0">{"Software Engineering Intern"}</h3>
                        <div class="subheading mb-3">{"Wildseed Tech"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Developed and launched front-end and back-end features to streamline
                                client user management, attracting an additional client to the platform."}</li>
                            <li class="list-inline">{"Resolved an average of 2 complex front-end tickets weekly, enhancing
                                stability and performance for smoother user interactions."}</li>
                            <li class="list-inline">{"Collaborated with team members once a week to create tickets and aid
                                in development operations"}</li>
                            <li class="list-inline">{"Conducted automated end-to-end testing with Cypress, streamlining
                                the development cycle and reducing the need for manual testing."}</li>
                            <li class="list-inline">{"Current Tech Stack: TypeScript(JavaScript), React (Redux-Rematch,
                                Recoil), MongoDB"}</li>

                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"May 2021 - December
                            2021"}
                            <p style="text-align:right">{"Remote"}</p>
                        </span></div>

                </div>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0">{"IT Specialist"}</h3>
                        <div class="subheading mb-3">{"Learning Events Technology Services"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Maintained upkeep of all projector/computer systems and classroom
                                equipment across campus"}</li>
                            <li class="list-inline">{"Assessed and troubleshot technical problems brought by professors,
                                students, and faculty by assisting in 2 calls every shift"}</li>
                            <li class="list-inline">{"Cooperated with team to expedite troubleshooting process and to
                                recognize faster ways of recognizing the problems presented"}
                            </li>

                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"March 2019 - January
                            2021"}
                            <p>{"Boston, Massachusetts"}</p>
                        </span></div>

                </div>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0">{"Engineering Intern"}</h3>
                        <div class="subheading mb-3">{"Worthington Industries"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Created asset lists for machine parts and motors for upkeep of all
                                working machines every week"}</li>
                            <li class="list-inline">{"Conducted daily maintenance checks on the factory floor to gurantee
                                all machines are in working order"}</li>
                            <li class="list-inline">{"Tackled IT-related problems both on the office floor and factory
                                floor"}</li>

                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"May 2019 - August
                            2019"}
                            <p>{"Pomona, California"}</p>
                        </span></div>

                </div>
                <div class="d-flex flex-column flex-md-row justify-content-between mb-5">
                    <div class="flex-grow-1">
                        <h3 class="mb-0 text">{"Elective Clerkship"}</h3>
                        <div class="subheading mb-3">{"Seoul National University Hospital"}</div>
                        <ul class="list-inline" type="disc">
                            <li class="list-inline">{"Shadowed surgeons and doctors in the Urology Department as well as
                                General Surgery"}</li>
                            <li class="list-inline">{"Translated research papers from Korean to English for the Urology
                                Department"}</li>
                            <li class="list-inline">{"Gained exposure to medicine and procurement/management of medical
                                equipment"}</li>

                        </ul>
                    </div>
                    <div class="flex-shrink-0"><span class="text-primary" style="text-align:right">{"May 2018 - June 2018"}
                            <p>{"Seoul, South Korea"}</p>
                        </span></div>

                </div>
            </div>
        </section>
        <hr class="m-0" />
        </>
    }
}
