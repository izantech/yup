//! Action filtering based on CLI flags

import super.types.Action

/// - `skip`: If non-null, exclude actions from managers whose debug name matches one of the strings
public fn filterActions(actions: [Action], only: [String]?, skip: [String]?): [Action] {
  let normalizeList = { items: [String] ->
    let normalized: [String] = items.iter().map {
      it.trim().toLowercase()
    }.filter {
      !it.isEmpty()
    }.collect()
    if normalized.isEmpty() { null } else { normalized }
  }
  let onlyList = only.andThen(normalizeList)
  let skipList = skip.andThen(normalizeList)

  actions.intoIter().filter { action ->
    let managerName = action.manager.asRef().toLowercase()
    // Filter by --only (whitelist)
    if let onlyList = onlyList {
      let matches = onlyList.iter().any {
        managerName == *it
      }
      if !matches { return false }
    }
    // Filter by --skip (blacklist)
    if let skipList = skipList {
      let matches = skipList.iter().any {
        managerName == *it
      }
      if matches { return false }
    }
    true
  }.collect()
}

module tests {
  import super.*
  import crate.engine.types.Manager
  fn makeAction(manager: Manager): Action {
    Action(manager: manager, command: format!("{} update", manager), description: format!("Update {}", manager), requiresPrivilege: false)
  }
  @[test]
  fn testFilterOnly() {
    let actions = [
      makeAction(Manager.brew),
      makeAction(Manager.npm),
      makeAction(Manager.rustup),
    ]

    let only = ["brew", "rustup"]
    let filtered = filterActions(actions, only, null)

    assert_eq!(filtered.len(), 2)
    assert!(filtered.iter().any {
      it.manager == Manager.brew
    })
    assert!(filtered.iter().any {
      it.manager == Manager.rustup
    })
  }
  @[test]
  fn testFilterSkip() {
    let actions = [
      makeAction(Manager.brew),
      makeAction(Manager.npm),
      makeAction(Manager.pipx),
    ]

    let skip = ["npm", "pipx"]
    let filtered = filterActions(actions, null, skip)

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

    let only = [" BREW ", "RUSTUP"]
    let filtered = filterActions(actions, only, null)

    assert_eq!(filtered.len(), 2)
    assert!(filtered.iter().any {
      it.manager == Manager.brew
    })
    assert!(filtered.iter().any {
      it.manager == Manager.rustup
    })
  }
  @[test]
  fn testFilterOnlyAndSkipCombined() {
    let actions = [
      makeAction(Manager.brew),
      makeAction(Manager.npm),
      makeAction(Manager.rustup),
    ]

    let only = ["brew", "npm"]
    let skip = ["npm"]
    let filtered = filterActions(actions, only, skip)

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
    let filtered = filterActions(actions, only, skip)

    assert_eq!(filtered.len(), 3)
    assert!(filtered.iter().any {
      it.manager == Manager.brew
    })
    assert!(filtered.iter().any {
      it.manager == Manager.npm
    })
    assert!(filtered.iter().any {
      it.manager == Manager.rustup
    })
  }
  @[test]
  fn testFilterWhitespaceOnlyEntriesNoop() {
    let actions = [
      makeAction(Manager.brew),
      makeAction(Manager.npm),
      makeAction(Manager.rustup),
    ]

    let only = ["   ", "\t"]
    let skip = ["\n"]
    let filtered = filterActions(actions, only, skip)

    assert_eq!(filtered.len(), 3)
    assert!(filtered.iter().any {
      it.manager == Manager.brew
    })
    assert!(filtered.iter().any {
      it.manager == Manager.npm
    })
    assert!(filtered.iter().any {
      it.manager == Manager.rustup
    })
  }
}
