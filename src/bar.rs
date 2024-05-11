use crate::{BAR_HEIGHT_PX, BLACK, FONT, GREY, ORANGE, WHITE};
use penrose::{x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, battery_summary, current_date_and_time, wifi_network, CurrentLayout,
            Workspaces,Spacer,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
// use penrose_ui::bar::widgets::debug::StateSummary;

// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = ORANGE.into();
    let empty_ws: Color = GREY.into();

    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (3, 3),
    };

    let padded_style = TextStyle {
        padding: (4, 2),
        ..style
    };

    let orange_style = TextStyle {
        bg: Some(BLACK.into()),
        fg: ORANGE.into(),
        padding: (4, 2),
        ..style
    };
    let spacer_style = TextStyle {
        padding: (0, 0),
        fg: ORANGE.into(),
        bg: Some(BLACK.into()),
    };

    StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        FONT,
        24,
        vec![
            Box::new(Workspaces::new(style, highlight, empty_ws)),
            Box::new(CurrentLayout::new(style)),
            Box::new(Spacer::new(spacer_style)),
            Box::new(current_date_and_time(TextStyle {
                fg: ORANGE.into(),
                bg: Some(BLACK.into()),
                padding: (2, 2),
            })),
            // Box::new(StateSummary::new(style)),
            // Box::new(ActiveWindowName::new(
            //     MAX_ACTIVE_WINDOW_CHARS,
            //     TextStyle {
            //         bg: Some(highlight),
            //         padding: (6, 4),
            //         ..style
            //     },
            //     true,
            //     false,
            // )),
            Box::new(Spacer::new(spacer_style)),
            Box::new(wifi_network(padded_style)),
            Box::new(battery_summary("BAT1", orange_style)),
            Box::new(amixer_volume("Master", orange_style)),
        ],
    )
}
