//! Action filtering based on CLI flags

import super.types.Action

/// - `skip`: If non-null, exclude actions from managers whose debug name matches one of the strings
public fn filterActions(actions: Array<Action>, consuming only: Array<String>?, consuming skip: Array<String>?): Array<Action> {
  let normalizeList = { items: Array<String> ->
    let normalized: Array<String> = items.iter().map { it.trim().toLowercase() }.filter { !it.isEmpty() }.collect()
    if normalized.isEmpty() { null } else { Some(normalized) }
  }
  let onlyList = only.andThen(normalizeList)
  let skipList = skip.andThen(normalizeList)

  actions
    .iter()
    .cloned()
    .filter { action ->
      let managerName = action.manager.asRef().toLowercase()
      // Filter by --only (whitelist)
      if let onlyList = onlyList.asRef() {
        let matches = onlyList.iter().any { managerName == *it }
        if !matches { return false }
      }
      // Filter by --skip (blacklist)
      if let skipList = skipList.asRef() {
        let matches = skipList.iter().any { managerName == *it }
        if matches { return false }
      }
      true
    }
    .collect()
}

module tests {
  import super.*
  import crate.engine.types.Manager
  fn makeAction(consuming manager: Manager): Action {
    Action(
      manager: manager,
      command: format!("{} update", manager),
      description: format!("Update {}", manager),
      requiresPrivilege: false,
    )
  }
  @[test]
  fn testFilterOnly() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Rustup),
    ]

    let only = ["brew", "rustup"]
    let filtered = filterActions(actions, only, null)

    assert_eq!(filtered.len(), 2)
    assert!(filtered.iter().any { it.manager == Manager.Brew })
    assert!(filtered.iter().any { it.manager == Manager.Rustup })
  }
  @[test]
  fn testFilterSkip() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Pipx),
    ]

    let skip = ["npm", "pipx"]
    let filtered = filterActions(actions, null, skip)

    assert_eq!(filtered.len(), 1)
    assert_eq!(filtered[0].manager, Manager.Brew)
  }
  @[test]
  fn testFilterCaseInsensitiveAndTrimmed() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Rustup),
    ]

    let only = [" BREW ", "RUSTUP"]
    let filtered = filterActions(actions, only, null)

    assert_eq!(filtered.len(), 2)
    assert!(filtered.iter().any { it.manager == Manager.Brew })
    assert!(filtered.iter().any { it.manager == Manager.Rustup })
  }
  @[test]
  fn testFilterOnlyAndSkipCombined() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Rustup),
    ]

    let only = ["brew", "npm"]
    let skip = ["npm"]
    let filtered = filterActions(actions, only, skip)

    assert_eq!(filtered.len(), 1)
    assert_eq!(filtered[0].manager, Manager.Brew)
  }
  @[test]
  fn testFilterEmptyListsNoop() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Rustup),
    ]

    let only: Array<String> = []
    let skip: Array<String> = []
    let filtered = filterActions(actions, only, skip)

    assert_eq!(filtered.len(), 3)
    assert!(filtered.iter().any { it.manager == Manager.Brew })
    assert!(filtered.iter().any { it.manager == Manager.Npm })
    assert!(filtered.iter().any { it.manager == Manager.Rustup })
  }
  @[test]
  fn testFilterWhitespaceOnlyEntriesNoop() {
    let actions = [
      makeAction(Manager.Brew),
      makeAction(Manager.Npm),
      makeAction(Manager.Rustup),
    ]

    let only = ["   ", "  "]
    let skip = [" "]
    let filtered = filterActions(actions, only, skip)

    assert_eq!(filtered.len(), 3)
    assert!(filtered.iter().any { it.manager == Manager.Brew })
    assert!(filtered.iter().any { it.manager == Manager.Npm })
    assert!(filtered.iter().any { it.manager == Manager.Rustup })
  }
}
