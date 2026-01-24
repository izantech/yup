//! Engine module - core scanning and action logic

external module types
external module scan
external module filter
external module managers

// Re-export key types
public import types.{Action, Manager, ScanReport}
public import scan.{scan, getActionsForScan, getCheckActionsForScan}
public import filter.filterActions
public import managers.{PackageManager, createManager}
