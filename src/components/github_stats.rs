use dark_light::{detect, Mode};

use yew::prelude::*;
use yew::Properties;

const OVERVIEW_LIGHT_URL: &str =
    "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/overview.svg#gh-light-mode-only";
const OVERVIEW_DARK_URL: &str =
    "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/overview.svg#gh-dark-mode-only";

const LANGUAGES_LIGHT_URL: &str = 
    "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/languages.svg#gh-light-mode-only";
const LANGUAGES_DARK_URL: &str = 
    "https://raw.githubusercontent.com/jeiang/github-stats/master/generated/languages.svg#gh-dark-mode-only";

#[derive(Properties, PartialEq, Eq, PartialOrd, Ord)]
pub struct GHStatsProps {
    #[prop_or(matches!(detect(), Mode::Dark))]
    pub is_dark_mode: bool
}

#[function_component(GHStatsOverview)]
pub fn overview(props: &GHStatsProps) -> Html {
    let url = if props.is_dark_mode {
        OVERVIEW_DARK_URL
    } else {
        OVERVIEW_LIGHT_URL
    };
    html! {
        <a target="_blank" rel="noopener noreferrer" href="https://github.com/jeiang">
            <img src={url} alt="github statistics" />
        </a>
    }
}

#[function_component(GHStatsLanguages)]
pub fn languages(props: &GHStatsProps) -> Html {
    let url = if props.is_dark_mode {
        LANGUAGES_DARK_URL
    } else {
        LANGUAGES_LIGHT_URL
    };
    html! {
        <a target="_blank" rel="noopener noreferrer" href="https://github.com/jeiang">
            <img src={url} alt="github statistics" />
        </a>
    }
}



