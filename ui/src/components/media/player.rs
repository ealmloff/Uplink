use std::rc::Weak;

use crate::{
    components::media::popout_player::PopoutPlayer,
    state::{Action, State},
    SETTINGS_ROUTE,
};

use dioxus::prelude::*;
use dioxus_desktop::use_window;
use dioxus_router::*;

use kit::{
    elements::{
        button::Button,
        tooltip::{ArrowPosition, Tooltip},
        Appearance,
    },
    icons::{Icon, IconElement},
    layout::topbar::Topbar,
};

#[derive(Eq, PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    larger: Option<bool>,
    settings_text: String,
    enable_camera_text: String,
    fullscreen_text: String,
    popout_player_text: String,
    screenshare_text: String,
    end_text: String,
}

#[allow(non_snake_case)]
pub fn MediaPlayer(cx: Scope<Props>) -> Element {
    let state = use_shared_state::<State>(cx)?;
    let window = use_window(cx);

    let silenced = state
        .read()
        .ui
        .current_call
        .clone()
        .map(|x| x.silenced)
        .unwrap_or(false);

    let silenced_str = silenced.to_string();

    cx.render(rsx!(div {
        id: "media-player",
        div {
            id: "handle",
            IconElement {
                icon: Icon::ChevronUpDown,
                size: 20,
            },
        },
        Topbar {
            controls: cx.render(
                rsx! (
                    Button {
                        icon: Icon::ArrowsPointingOut,
                        appearance: Appearance::Secondary,
                        tooltip: cx.render(rsx!(
                            Tooltip {
                                arrow_position: ArrowPosition::Top,
                                text: cx.props.fullscreen_text.clone(),
                            }
                        )),
                    },
                )
            )
        },
        div {
            id: "media-renderer",
            div {
                class: "video-wrap",
                Button {
                    icon: Icon::Square2Stack,
                    appearance: Appearance::Transparent,
                    tooltip: cx.render(rsx!(
                        Tooltip {
                            arrow_position: ArrowPosition::Right,
                            text: cx.props.popout_player_text.clone(),
                        }
                    )),
                    onpress: move |_| {
                        if state.read().ui.popout_player {
                            state.write().mutate(Action::ClearPopout);
                        } else {
                             let popout = VirtualDom::new_with_props(PopoutPlayer, ());
                            let window = window.new_window(popout, Default::default());
                            // for some reason using write() prevents the video from loading but using write_silent() works
                            state.write_silent().mutate(Action::SetPopout(Weak::upgrade(&window).unwrap()));
                        }
                    }
                },
                // don't render MediadPlayer if the video is popped out
                state.read().ui.popout_player.then(|| rsx!(
                    span {
                        class: "popped-out",
                        video {}
                    }
                )),
                (!state.read().ui.popout_player).then(|| rsx!(
                    video {
                        src: "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/Sintel.mp4",
                        autoplay: "true",
                        "loop": "true",
                        muted: "{silenced_str}"
                    }
                ))
            }
        },
        div {
            class: "media-controls",
            Button {
                icon: Icon::VideoCamera,
                appearance: Appearance::Secondary,
                tooltip: cx.render(rsx!(
                    Tooltip {
                        arrow_position: ArrowPosition::Bottom,
                        text: cx.props.enable_camera_text.clone(),
                    }
                )),
            },
            Button {
                icon: Icon::Window,
                appearance: Appearance::Secondary,
                tooltip: cx.render(rsx!(
                    Tooltip {
                        arrow_position: ArrowPosition::Bottom,
                        text: cx.props.screenshare_text.clone(),
                    }
                )),
                // TODO: https://github.com/quadrupleslap/scrap
            },
            Button {
                icon: Icon::PhoneXMark,
                appearance: Appearance::Danger,
                text: cx.props.end_text.clone(),
                onpress: move |_| {
                    state.write().mutate(Action::DisableMedia);
                }
            },
            Button {
                icon: Icon::Cog6Tooth,
                appearance: Appearance::Secondary,
                tooltip: cx.render(rsx!(
                    Tooltip {
                        arrow_position: ArrowPosition::Bottom,
                        text: cx.props.settings_text.clone(),
                    }
                )),
                // TODO: Navigate to media settings
                onpress: move |_| {
                    use_router(cx).replace_route(SETTINGS_ROUTE, None, None);
                }
            },
        }
    }))
}