use super::types::Action;

/// Filter actions based on CLI flags.
///
/// - `only`: If Some, only include actions from managers whose debug name matches one of the strings
/// - `skip`: If Some, exclude actions from managers whose debug name matches one of the strings
#[must_use]
pub fn filter_actions(
    actions: Vec<Action>,
    only: Option<&[String]>,
    skip: Option<&[String]>,
) -> Vec<Action> {
    let normalize_list = |items: &[String]| {
        let normalized = items
            .iter()
            .map(|item| item.trim().to_lowercase())
            .filter(|item| !item.is_empty())
            .collect::<Vec<_>>();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    };
    let only_list = only.and_then(normalize_list);
    let skip_list = skip.and_then(normalize_list);

    actions
        .into_iter()
        .filter(|action| {
            let manager_name = action.manager.as_ref().to_lowercase();

            // Filter by --only (whitelist)
            if let Some(ref only_list) = only_list {
                let matches = only_list.iter().any(|o| manager_name == *o);
                if !matches {
                    return false;
                }
            }

            // Filter by --skip (blacklist)
            if let Some(ref skip_list) = skip_list {
                let matches = skip_list.iter().any(|s| manager_name == *s);
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
            format!("{} update", manager),
            format!("Update {}", manager),
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

        let only = vec!["brew".to_string(), "rustup".to_string()];
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

    #[test]
    fn test_filter_case_insensitive_and_trimmed() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Rustup),
        ];

        let only = vec![" BREW ".to_string(), "RUSTUP".to_string()];
        let filtered = filter_actions(actions, Some(&only), None);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|a| a.manager == Manager::Brew));
        assert!(filtered.iter().any(|a| a.manager == Manager::Rustup));
    }

    #[test]
    fn test_filter_only_and_skip_combined() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Rustup),
        ];

        let only = vec!["brew".to_string(), "npm".to_string()];
        let skip = vec!["npm".to_string()];
        let filtered = filter_actions(actions, Some(&only), Some(&skip));

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].manager, Manager::Brew);
    }

    #[test]
    fn test_filter_empty_lists_noop() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Rustup),
        ];

        let only: Vec<String> = Vec::new();
        let skip: Vec<String> = Vec::new();
        let filtered = filter_actions(actions, Some(&only), Some(&skip));

        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().any(|a| a.manager == Manager::Brew));
        assert!(filtered.iter().any(|a| a.manager == Manager::Npm));
        assert!(filtered.iter().any(|a| a.manager == Manager::Rustup));
    }

    #[test]
    fn test_filter_whitespace_only_entries_noop() {
        let actions = vec![
            make_action(Manager::Brew),
            make_action(Manager::Npm),
            make_action(Manager::Rustup),
        ];

        let only = vec!["   ".to_string(), "\t".to_string()];
        let skip = vec!["\n".to_string()];
        let filtered = filter_actions(actions, Some(&only), Some(&skip));

        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().any(|a| a.manager == Manager::Brew));
        assert!(filtered.iter().any(|a| a.manager == Manager::Npm));
        assert!(filtered.iter().any(|a| a.manager == Manager::Rustup));
    }
}
