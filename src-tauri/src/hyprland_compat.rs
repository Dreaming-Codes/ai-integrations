use hyprland::prelude::HyprData;

const GENERAL_SELECT_AREA_SELECTOR: &str = "title:^(Select Area)(.*)$,class:(ai-integration)";
const STATUS_SELECTOR: &str = "title:(Status),class:(ai-integration)";

pub async fn set_rules() -> hyprland::Result<()> {
    unset_rules().await?;
    let monitors = hyprland::data::Monitors::get_async().await?;
    let tasks = monitors.iter().map(|monitor| {
        let monitor_selector = format!("title:^(Select Area {}x{})(.*)$", monitor.x, monitor.y);

        hyprland::keyword::Keyword::set_async("windowrulev2", format!("monitor {},{}", monitor.id, monitor_selector))
    });

    futures::future::join_all(tasks).await;

    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noblur,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nofullscreenrequest,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nomaximizerequest,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("float,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("size 100% 100%,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("center,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noanim,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("pin,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noborder,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nodim,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noinitialfocus,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noshadow,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("forceinput,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nofocus,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;

    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noblur,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("float,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("center,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noanim,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("pin,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("monitor 0,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nofocus,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noinitialfocus,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noborder,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nodim,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("noshadow,{}", STATUS_SELECTOR)).await?;

    Ok(())
}

pub async fn unset_rules() -> hyprland::Result<()> {
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", STATUS_SELECTOR)).await?;


    let monitors = hyprland::data::Monitors::get_async().await?;
    let tasks = monitors.iter().map(|monitor| {
        let monitor_selector = format!("title:^(Select Area {}x{})(.*)$", monitor.x, monitor.y);

        hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", monitor_selector))
    });

    futures::future::join_all(tasks).await;

    Ok(())
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP") == Ok("Hyprland".to_string())
}
