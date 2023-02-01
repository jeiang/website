use yew::prelude::*;

use crate::components::github_stats::{GHStatsLanguages, GHStatsOverview};

#[function_component(About)]
pub fn about() -> Html {
    // TODO: Add github stats
    html! {
        <>
            <h2>{ "About Me" }</h2>
            <p>
                { "I am from " }
                <a href="https://en.wikipedia.org/wiki/Trinidad_and_Tobago">{ "Trinidad" }</a>
                { " and studying Electrical and Computer Engineering at " }
                <a href="https://sta.uwi.edu/">{ "UWI St. Augustine" }</a>
                { ". I starting getting interested in computers, etc., around 14.
                    Some of the more interesting things I've tried are:" }
                <ul>
                    <li>
                        <a href="https://nixos.org/">{ "Nix/NixOS" }</a>
                        { ": I jumped on to NixOS after a bit of distro-jumping. It had a sort of 'it
                            just works' vibe, and if you mess up, you can just go from 0 to 100 using 
                            a few files. Impermanence is great, since you get most of the benefits of 
                            resetting your computer each time you restart, but not having to waste 
                            time getting everything setup right. (I am severely understating many 
                            other cool benefits but check it out!)" }
                    </li>
                    <li>
                        <a href="https://vlang.io/">{ "V" }</a>
                        { ": Language I happened upon right before learning Rust. It's in progress,
                            but has some cool ideas. Syntax is nice (very much like Go, but I haven't 
                            used Go) and is simple to understand. It does overpromise a lot, and there
                            are some controversies around it." }
                    </li>
                    <li>
                        <a href="https://rust-lang.org/">{ "Rust" }</a>
                        { ": Not much needs to be said about Rust. My favorite Rust YouTube channel is " }
                        <a href="https://www.youtube.com/@NoBoilerplate">{ "No Boilerplate" }</a>
                        { " because every video is concise and talks about why Rust is amazing." }
                    </li>
                    <li>
                        <a href="https://fsharp.org/">{ "F#" }</a>
                        { ": Introduced me to Functional Programming. When I later needed to learn
                            C# for an internship, my knowledge of F# helped me to not write funky code." }
                    </li>
                </ul>
                <br />
                { "I also like playing Skyrim (specifically modded Skyrim: Special Edition), and use " }
                <a href="https://www.wabbajack.org/">{ "Wabbajack" }</a>
                { " for easily modding." }
            </p>
            <h3>{ "My GitHub Stats" }</h3>
            <div>
                <GHStatsOverview />
                <GHStatsLanguages />
            </div>
            <h2>{ "About this site" }</h2>
            <p>{ "This site was built using the following:" }</p>
            <ul>
                <li>
                    <a href="https://www.rust-lang.org/">{ "Rust" }</a>
                    { ": Rust is a language for cool kids." }
                </li>
                <li>
                    <a href="https://yew.rs/">{ "Yew" }</a>
                    { ": Rust framework used to create the page." }
                </li>
                <li>
                    <a href="https://trunkrs.dev/">{ "Trunk" }</a>
                    { ": Used to bundle the app so I can host on Github." }
                </li>
                <li>
                    <a href="https://concrete.style/">{ "concrete.css" }</a>
                    { ": Styling for the site. I am not good at styling/designing frontends,
                        so this made things super easy." }
                </li>
                <li>
                    <a href="https://necolas.github.io/normalize.css/">{ "Normalize.css" }</a>
                    { ": Recommended by concrete.css." }
                </li>
                <li>
                    <a href="https://fonts.google.com/specimen/Merriweather">{ "Merriweather" }</a>
                    { " & " }
                    <a href="https://fonts.google.com/specimen/Raleway">{ "Raleway" }</a>
                    { ": Fonts used for the page. Merriweather for headings, Raleway for everything else." }
                </li>
            </ul>
            <p>
                { "Hosted by Porkbun (static hosting). You can find the code on " }
                <a href="https://github.com/jeiang/website">{ "Github" }</a>
                { "." }
            </p>
        </>
    }
}
