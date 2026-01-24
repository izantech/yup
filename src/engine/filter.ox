//! Action filtering based on CLI flags

import super.types.Action

/// Filter actions based on CLI flags.
///
/// - `only`: If Some, only include actions from managers whose debug name matches one of the strings
/// - `skip`: If Some, exclude actions from managers whose debug name matches one of the strings
public fn filterActions(
    actions: [Action],
    only: [String]?,
    skip: [String]?,
): [Action] {
    let normalizeList = { items: [String] ->
        let normalized: [String] = items
            .iter()
            .map { it.trim().to_lowercase() }
            .filter { !it.is_empty() }
            .collect()
        if normalized.is_empty() {
            None
        } else {
            Some(normalized)
        }
    }
    let onlyList = only.and_then(normalizeList)
    let skipList = skip.and_then(normalizeList)

    actions
        .into_iter()
        .filter { action ->
            let managerName = action.manager.as_ref().to_lowercase()

            // Filter by --only (whitelist)
            if let onlyList = onlyList {
                let matches = onlyList.iter().any { managerName == *it }
                if !matches {
                    return false
                }
            }

            // Filter by --skip (blacklist)
            if let skipList = skipList {
                let matches = skipList.iter().any { managerName == *it }
                if matches {
                    return false
                }
            }

            true
        }
        .collect()
}

@[cfg(test)]
module tests {
    import super.*
    import crate.engine.types.Manager

    fn makeAction(manager: Manager): Action {
        Action.new(
            manager,
            format!("{} update", manager),
            format!("Update {}", manager),
            false,
        )
    }

    @[test]
    fn testFilterOnly() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.rustup),
        ]

        let only = ["brew".to_string(), "rustup".to_string()]
        let filtered = filterActions(actions, Some(only), None)

        assert_eq!(filtered.len(), 2)
        assert!(filtered.iter().any { it.manager == Manager.brew })
        assert!(filtered.iter().any { it.manager == Manager.rustup })
    }

    @[test]
    fn testFilterSkip() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.pipx),
        ]

        let skip = ["npm".to_string(), "pipx".to_string()]
        let filtered = filterActions(actions, None, Some(skip))

        assert_eq!(filtered.len(), 1)
        assert_eq!(filtered[0].manager, Manager.brew)
    }

    @[test]
    fn testFilterCaseInsensitiveAndTrimmed() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.rustup),
        ]

        let only = [" BREW ".to_string(), "RUSTUP".to_string()]
        let filtered = filterActions(actions, Some(only), None)

        assert_eq!(filtered.len(), 2)
        assert!(filtered.iter().any { it.manager == Manager.brew })
        assert!(filtered.iter().any { it.manager == Manager.rustup })
    }

    @[test]
    fn testFilterOnlyAndSkipCombined() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.rustup),
        ]

        let only = ["brew".to_string(), "npm".to_string()]
        let skip = ["npm".to_string()]
        let filtered = filterActions(actions, Some(only), Some(skip))

        assert_eq!(filtered.len(), 1)
        assert_eq!(filtered[0].manager, Manager.brew)
    }

    @[test]
    fn testFilterEmptyListsNoop() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.rustup),
        ]

        let only: [String] = []
        let skip: [String] = []
        let filtered = filterActions(actions, Some(only), Some(skip))

        assert_eq!(filtered.len(), 3)
        assert!(filtered.iter().any { it.manager == Manager.brew })
        assert!(filtered.iter().any { it.manager == Manager.npm })
        assert!(filtered.iter().any { it.manager == Manager.rustup })
    }

    @[test]
    fn testFilterWhitespaceOnlyEntriesNoop() {
        let actions = [
            makeAction(Manager.brew),
            makeAction(Manager.npm),
            makeAction(Manager.rustup),
        ]

        let only = ["   ".to_string(), "\t".to_string()]
        let skip = ["\n".to_string()]
        let filtered = filterActions(actions, Some(only), Some(skip))

        assert_eq!(filtered.len(), 3)
        assert!(filtered.iter().any { it.manager == Manager.brew })
        assert!(filtered.iter().any { it.manager == Manager.npm })
        assert!(filtered.iter().any { it.manager == Manager.rustup })
    }
}
