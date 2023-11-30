pub async fn set_rules() -> hyprland::Result<()> {
    hyprland::keyword::Keyword::set_async("windowrule", "noblur,title:^(Select Area)$").await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", "fakefullscreen,title:^(Select Area)$").await?;
    hyprland::keyword::Keyword::set_async("windowrule", "float,title:^(Select Area)$").await?;
    hyprland::keyword::Keyword::set_async("windowrule", "size 100% 100%,title:^(Select Area)$").await?;
    hyprland::keyword::Keyword::set_async("windowrule", "center,title:^(Select Area)$").await?;

    Ok(())
}

pub async fn unset_rules() -> hyprland::Result<()> {
    hyprland::keyword::Keyword::set_async("windowrule", "unset,title:^(Select Area)$").await?;
    hyprland::keyword::Keyword::set_async("windowrulev2", "unset,title:^(Select Area)$").await?;

    Ok(())
}

pub fn is_hyprland() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP") == Ok("Hyprland".to_string())
}
