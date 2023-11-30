use hyprland::prelude::HyprData;

const GENERAL_SELECT_AREA_SELECTOR: &str = "title:^(Select Area)(.*)$";

pub async fn set_rules() -> hyprland::Result<()> {
    unset_rules().await?;
    let monitors = hyprland::data::Monitors::get_async().await?;
    let tasks = monitors.iter().map(|monitor| {
        let monitor_selector = format!("title:^(Select Area {}x{})(.*)$", monitor.x, monitor.y);

        hyprland::keyword::Keyword::set_async("windowrule", format!("monitor {},{}", monitor.id, monitor_selector))
    });

    futures::future::join_all(tasks).await;

    hyprland::keyword::Keyword::set_async("windowrule", format!("noblur,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("fakefullscreen,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrule", format!("float,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrule", format!("size 100% 100%,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrule", format!("center,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrule", format!("noanim,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrule", format!("pin,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;

    Ok(())
}

pub async fn unset_rules() -> hyprland::Result<()> {
    hyprland::keyword::Keyword::set_async("windowrule", format!("unset,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;

    let monitors = hyprland::data::Monitors::get_async().await?;
    let tasks = monitors.iter().map(|monitor| {
        let monitor_selector = format!("title:^(Select Area {}x{})(.*)$", monitor.x, monitor.y);

        hyprland::keyword::Keyword::set_async("windowrule", format!("unset,{}", monitor_selector))
    });

    futures::future::join_all(tasks).await;

    Ok(())
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP") == Ok("Hyprland".to_string())
}
