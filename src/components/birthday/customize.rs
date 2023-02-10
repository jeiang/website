use yew::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BirthdayError {
    reason: String,
}

impl BirthdayError {
    pub fn new(reason: &str) -> Self {
        let reason = reason.to_owned();
        Self { reason }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct BirthdayCustomizerProps {
    pub error: Option<BirthdayError>,
}

// TODO: Default page should provide a creator
// TODO: Allow an error option at the top, if error, use query info as field data, and erase query
#[function_component(BirthdayCustomizer)]
pub fn create_birthday_customizer(params: &BirthdayCustomizerProps) -> Html {
    html! {}
}
