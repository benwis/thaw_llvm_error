mod theme;
use crate::{theme::*, utils::mount_style::mount_style};
use leptos::*;
use stylers::style_sheet_str;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone)]
pub enum ButtonType {
    #[default]
    PRIMARY,
    SOLID,
    TEXT,
}

#[derive(Default, Clone)]
pub enum ButtonColor {
    #[default]
    PRIMARY,
    SUCCESS,
    WARNING,
    Error,
}

impl ButtonColor {
    pub fn theme_color(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::PRIMARY => theme.common.color_primary.clone(),
            ButtonColor::SUCCESS => theme.common.color_success.clone(),
            ButtonColor::WARNING => theme.common.color_warning.clone(),
            ButtonColor::Error => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional, into)] type_: MaybeSignal<ButtonType>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = color.get().theme_color(&theme);
        css_vars.push_str(&format!("--background-color: {bg_color}"));
        css_vars
    });
    let class_name = mount_style("button", || style_sheet_str!("./src/button/button.css"));

    view! {cx, class=class_name,
        <button 
            class:melt-button=true 
            class=("melt-button--text", move || type_.get() == ButtonType::TEXT) 
            style=move || css_vars.get()
            >
            {children(cx)}
        </button>
    }
}