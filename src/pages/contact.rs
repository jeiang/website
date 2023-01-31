use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <h2> {"Contact Me!"} </h2>
            <p>
                {"You can contact me (or see what I'm up to) via:"}
                <ul>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://anilist.co/user/jeiang">
                            {"AniList"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://discordapp.com/users/343517984222085134">
                            {"Discord"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="mailto:aidan@aidanpinard.co">
                        {"Email"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://github.com/jeiang/">
                            {"Github"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://app.roll20.net/users/6153276/jeiang">
                            {"Roll20"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://twitter.com/jeiang_">
                            {"Twitter"}
                        </a>
                    </li>
                    <li>
                        <a target="_blank" rel="noopener noreferrer" href="https://www.youtube.com/@jeiang">
                            {"YouTube"}
                        </a>
                    </li>
                </ul>
            </p>
        </>
    }
}
