use yew::prelude::*;

fn create_contact_link(visible_name: &str, url: &str) -> Html {
    let visible_name = visible_name.to_owned();
    let url = url.to_owned();
    html! {
        <a target="_blank" rel="noopener noreferrer" href={url} key={visible_name.clone()}>
            {visible_name}
        </a>
    }
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let contact_page = use_memo(
        |_| {
            let contact_links = [
                ("https://anilist.co/user/jeiang", "AniList"),
                ("https://discordapp.com/users/343517984222085134", "Discord"),
                ("mailto:aidan@aidanpinard.co", "Email"),
                ("https://github.com/jeiang/", "Github"),
                ("https://app.roll20.net/users/6153276/jeiang", "Roll20"),
                ("https://twitter.com/jeiang_", "Twitter"),
                ("https://www.youtube.com/@jeiang", "YouTube"),
            ]
            .map(|(link, name)| create_contact_link(name, link))
            .into_iter()
            .collect::<Html>();
            html! {
                <>
                    <h2>{ "Contact Me!" }</h2>
                    <p>
                        { "You can contact me (or see what I'm up to) via:" }
                        <ul>
                            { contact_links }
                        </ul>
                    </p>
                </>
            }
        },
        (),
    );

    (*contact_page).clone()
}
