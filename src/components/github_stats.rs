use yew::prelude::*;
use yew::Properties;

fn image_html(light_url: &str, dark_url: &str, alt_text: &str) -> Html {
    let light_url = light_url.to_owned();
    let dark_url = dark_url.to_owned();
    let alt_text = alt_text.to_owned();
    
    html! {
        <a target="_blank" rel="noopener noreferrer" href="https://github.com/jeiang">
            <picture>
                <source srcset={dark_url} media="(prefers-color-scheme: dark)"/>
                <img src={light_url} alt={alt_text} />
            </picture>
        </a>
    }
}

#[function_component(GHStatsOverview)]
pub fn overview() -> Html {
    const LIGHT_URL: &str =
        "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/overview.svg#gh-light-mode-only";
    const DARK_URL: &str =
        "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/overview.svg#gh-dark-mode-only";
    const ALT_TEXT: &str = 
        "Github Statistics";
    let stats_image = use_memo(|_| {
        image_html(LIGHT_URL, DARK_URL, ALT_TEXT)
    }, ());
    (*stats_image).clone()
}

#[function_component(GHStatsLanguages)]
pub fn languages() -> Html {
    const LIGHT_URL: &str = 
        "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/languages.svg#gh-light-mode-only";
    const DARK_URL: &str = 
        "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/languages.svg#gh-dark-mode-only";
    const ALT_TEXT: &str =
        "Github Language Usage Statistics";
    let stats_image = use_memo(|_| {
        image_html(LIGHT_URL, DARK_URL, ALT_TEXT)
    }, ());
    (*stats_image).clone()
}



