//! Cœur de PokéLauncher, indépendant de l'interface (testable sans Tauri).

pub mod catalogue;
pub mod config;
pub mod entete;
pub mod scan;

pub use catalogue::Catalogue;
