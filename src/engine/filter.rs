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
    actions
        .into_iter()
        .filter(|action| {
            let manager_name = format!("{:?}", action.manager).to_lowercase();

            // Filter by --only (whitelist)
            if let Some(only_list) = only {
                let matches = only_list
                    .iter()
                    .any(|o| manager_name.contains(&o.to_lowercase()));
                if !matches {
                    return false;
                }
            }

            // Filter by --skip (blacklist)
            if let Some(skip_list) = skip {
                let matches = skip_list
                    .iter()
                    .any(|s| manager_name.contains(&s.to_lowercase()));
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
    use crate::engine::types::{ActionKind, Manager};

    fn make_action(manager: Manager) -> Action {
        Action {
            manager,
            kind: ActionKind::Update,
            command: format!("{:?} update", manager),
            description: format!("Update {:?}", manager),
        }
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
            make_action(Manager::Pip),
        ];

        let skip = vec!["npm".to_string(), "pip".to_string()];
        let filtered = filter_actions(actions, None, Some(&skip));

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].manager, Manager::Brew);
    }
}
