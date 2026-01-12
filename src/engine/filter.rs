use super::types::Action;

/// Filter actions based on CLI flags.
///
/// - `only`: If Some, only include actions from managers whose debug name contains one of the strings
/// - `skip`: If Some, exclude actions from managers whose debug name contains one of the strings
pub fn filter_actions(
    actions: Vec<Action>,
    only: Option<&[String]>,
    skip: Option<&[String]>,
) -> Vec<Action> {
    let only_list = only.map(|items| items.iter().map(|o| o.to_lowercase()).collect::<Vec<_>>());
    let skip_list = skip.map(|items| items.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>());

    actions
        .into_iter()
        .filter(|action| {
            let manager_name = action.manager.as_str();

            // Filter by --only (whitelist)
            if let Some(ref only_list) = only_list {
                let matches = only_list.iter().any(|o| manager_name.contains(o));
                if !matches {
                    return false;
                }
            }

            // Filter by --skip (blacklist)
            if let Some(ref skip_list) = skip_list {
                let matches = skip_list.iter().any(|s| manager_name.contains(s));
                if matches {
                    return false;
                }
            }

            true
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::types::Manager;

    fn make_action(manager: Manager) -> Action {
        Action::new(
            manager,
            format!("{:?} update", manager),
            format!("Update {:?}", manager),
            false,
        )
    }

    #[test]
    fn test_filter_only() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Rustup),
        ];

        let only = vec!["brew".to_string(), "rust".to_string()];
        let filtered = filter_actions(actions, Some(&only), None);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|a| a.manager == Manager::Brew));
        assert!(filtered.iter().any(|a| a.manager == Manager::Rustup));
    }

    #[test]
    fn test_filter_skip() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Pipx),
        ];

        let skip = vec!["npm".to_string(), "pipx".to_string()];
        let filtered = filter_actions(actions, None, Some(&skip));

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].manager, Manager::Brew);
    }
}
