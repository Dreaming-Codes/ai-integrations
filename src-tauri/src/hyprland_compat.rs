use hyprland::prelude::HyprData;

const GENERAL_SELECT_AREA_SELECTOR: &str = "title:^(Select Area)(.*)$,class:(ai-integration)";
const STATUS_SELECTOR: &str = "title:(Status),class:(ai-integration)";

const SHARED_RULES: [&str; 9] = [
    "noblur",
    "float",
    "center",
    "noanim",
    "pin",
    "noinitialfocus",
    "noborder",
    "nodim",
    "noshadow",
];

pub async fn set_rules() -> hyprland::Result<()> {
    set_area_selector_rules().await?;
    set_status_rules().await?;
    set_shared_rules().await?;

    Ok(())
}

pub async fn unset_rules() -> hyprland::Result<()> {
    unset_area_selector_monitor_dependant_rules().await?;

    hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", STATUS_SELECTOR)).await?;

    Ok(())
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP") == Ok("Hyprland".to_string())
}

pub async fn set_area_selector_monitor_dependant_rules() -> hyprland::Result<()> {
    let monitors = hyprland::data::Monitors::get_async().await?;

    // This is an attempt to fix multiple monitors support
    for monitor in monitors {
        let monitor_selector = format!("title:Select Area {}x{}", monitor.x, monitor.y);
        hyprland::keyword::Keyword::set_async("windowrulev2", format!("monitor {},{}", monitor.id, monitor_selector)).await?;
    }

    Ok(())
}

async fn set_shared_rules() -> hyprland::Result<()> {
    for rule in &SHARED_RULES {
        hyprland::keyword::Keyword::set_async("windowrulev2", format!("{},{}", rule, GENERAL_SELECT_AREA_SELECTOR)).await?;
        hyprland::keyword::Keyword::set_async("windowrulev2", format!("{},{}", rule, STATUS_SELECTOR)).await?;
    }

    Ok(())
}

async fn unset_area_selector_monitor_dependant_rules() -> hyprland::Result<()> {
    let monitors = hyprland::data::Monitors::get_async().await?;

    for monitor in monitors {
        let monitor_selector = format!("title:Select Area {}x{}", monitor.x, monitor.y);
        hyprland::keyword::Keyword::set_async("windowrulev2", format!("unset,{}", monitor_selector)).await?;
    }

    Ok(())
}

async fn set_area_selector_rules() -> hyprland::Result<()> {
    set_area_selector_monitor_dependant_rules().await?;

    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nofullscreenrequest,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nomaximizerequest,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("size 100% 100%,{}", GENERAL_SELECT_AREA_SELECTOR)).await?;

    Ok(())
}

async fn set_status_rules() -> hyprland::Result<()> {
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("monitor 0,{}", STATUS_SELECTOR)).await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", format!("nofocus,{}", STATUS_SELECTOR)).await?;

    Ok(())
}
