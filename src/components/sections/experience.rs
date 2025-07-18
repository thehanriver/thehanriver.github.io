use crate::components::job::Job;
use crate::components::sections::section::Section;
use yew::prelude::*;

#[function_component]
pub fn Experience() -> Html {
    html! {
        <Section id="experience" title="Experience">
            <Job
                title="Software Developer"
                company="ClumL"
                date_range="March 2022 - Present"
                location="Remote"
                description_points={vec![
                    "Support cross-functional collaboration in delivering new product features, ensuring seamless integration between front-end and back-end systems to enhance system stability and release efficiency.".to_string(),
                    "Research and evaluate new technologies with the team to identify performace optimization opportunities, leveraging tools like Wireshark to enhance system efficiency.".to_string(),
                    "Resolve critical bugs and develop new front-end and backe-end features, improving system stability and user experience. Engage in team-led brainstorming to drive technical and performace optimizations.".to_string(),
                    "Exploring new tech stack in order to create and edit UI main product effectively and efficiently.".to_string(),
                ]}
                tags={vec!["Rust".to_string(),"GraphQL".to_string(),"Yew".to_string(),"WebAssembly".to_string()]}
            />
            <Job
                title="Freelancer/Owner"
                company="Tekhan LLC"
                date_range="January 2022 - Present"
                location="Remote"
                description_points={vec![
                    "Created my own LLC in order to help students and young adults to make websites or website profolios of their own.".to_string(),
                    "Using GoDaddy and WordPress in order to build simple, fast, and effective portfolios unless specified".to_string(),
                    "Teaching close acquintences JavaScript ,CSS ,HTML ,and basic Web Dev".to_string(),
                ]}
                tags={vec!["JavaScript/CSS/HTML".to_string(),"GoDaddy/WebPress".to_string()]}
            />
            <Job
                title="Software Engineering Intern"
                company="Wildseed Tech"
                date_range="May 2021 - December 2021"
                location="Remote"
                description_points={vec![
                    "Developed and launched front-end and back-end features to streamline client user manangement, attracting and additional client to the platform.".to_string(),
                    "Resolved an average of 2 complex front-end tickets weekly, enhancing stability and performace for smoother user interactions".to_string(),
                    "Collaborated with team members once a week to create tickets and aid in development operations".to_string(),
                    "Conducted automated end-to-end testing with Cypress, streamlining the development cycle and reducing the need for manual testing.".to_string(),
                    "Tech Stack: TypeScript(JavaScript), React(Redux-Rematch, Recoil), MongoDB".to_string()
                ]}
                tags={vec!["TypeScript/JavaScript".to_string(),"React(Redux-Rematch, Recoil), MongoDB".to_string()]}
            />
            <Job
                title="IT Specialist"
                company="Learning and Events Technology Services (LETS)"
                date_range="March 2019 - January 2021"
                location="Boston, Massachusetts"
                description_points={vec![
                    "Maintained upkeep of all projector/computer systems and classroom equipment across campus".to_string(),
                    "Assessed and troubleshot technical problems brought by professors, students, and faculty by assisting in 2 calls every shift".to_string(),
                    "Cooperated with team to expedite troubleshooting process and to recognize faster ways of recognizing the problem presented".to_string()
                ]}
            />
            <Job
                title="Engineering Intern"
                company="Worthington Industries"
                date_range="May 2019 - August 2019"
                location="Pomona, California"
                description_points={vec![
                    "Created asset lists for machine parts and motors for upkeep of all working machines every week.".to_string(),
                    "Conducted daily maintenance checks on the factory floor to gurantee all machines are in working order.".to_string(),
                    "Tackled IT-related problems both on the office floor and factory floor.".to_string()
                ]}
            />
            <Job
                title="Elective Clerkship"
                company="Seoul National University Hospital"
                date_range="May 2018 - June 2018"
                location="Seoul, South Korea"
                description_points={vec![
                    "Shadowed surgeons and doctors in the Urology Department as well as General Surgery.".to_string(),
                    "Translated research papers from Korean to English for the Urology Department.".to_string(),
                    "Gained exposure to medicine and procurment/management of medical equipment.".to_string()
                ]}
            />
        </Section>
    }
}
