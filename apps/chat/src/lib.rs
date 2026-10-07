// SPDX-FileCopyrightText: 2026 Veydan Project
// SPDX-License-Identifier: LicenseRef-PolyForm-Perimeter-1.0.1

//! Veydan Chat: the product crate (internal/platform-spec.md 13.1). It holds
//! the Tauri config, the strings of the product and the list of its
//! modules — one, the adapter of the messenger; `veydan_shell` starts it,
//! and the plan of its sync is the shell's default over what the shell
//! registers (9.1): the messenger syncs nothing, its data lives in a
//! database of its own under its keys (7.4). The identifier
//! `net.veydan.chat` and the version (`VERSION`, written by
//! `scripts/set-version.sh chat`) are in the Tauri config.
//!
//! One library for every platform. On Android the module registers the
//! push plugin, and the handler of a push reaches the messenger through
//! `Java_net_veydan_push_Core_describe`, exported by the adapter: naming
//! `veydan_messenger_app::module()` here is what links it in (13.4).

use veydan_shell::{Module, Product};

/// What Chat is (13.1, 13.5).
fn product() -> Product {
    Product {
        id: "chat",
        name: "Veydan Chat",
        desktop_entry: "veydanchat",
        icon: "veydanchat",
        sync: veydan_shell::default_plan(&module_list()),
    }
}

/// The modules of Chat, as `products.json` lists them.
pub fn module_list() -> Vec<Module> {
    vec![veydan_messenger_app::module()]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    veydan_shell::run(tauri::generate_context!(), product(), module_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The modules of the crate are those `products.json` gives the
    /// product: the UI is built from that list, the app from this one.
    #[test]
    fn the_module_list_is_that_of_products_json() {
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../products.json")).unwrap();
        let listed: Vec<&str> = manifest["targets"]["chat"]["modules"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| id.as_str().unwrap())
            .collect();
        let ours: Vec<&str> = module_list().iter().map(|module| module.id).collect();
        assert_eq!(ours, listed);
    }

    /// `commands.golden.txt`: every command of the product on a line with
    /// the module that answers it and where it exists (`all` platforms,
    /// `desktop` or `mobile` only), the lines of Space's file that belong to the shell
    /// and to the messenger. A name neither appears nor disappears without
    /// a matching change in the UI.
    #[test]
    fn every_command_has_the_owner_the_golden_file_names() {
        let expected: Vec<(&str, &str)> = include_str!("commands.golden.txt")
            .lines()
            .map(|line| {
                let mut words = line.split(' ');
                let (command, module, platform) = (
                    words.next().unwrap(),
                    words.next().unwrap(),
                    words.next().unwrap(),
                );
                assert!(matches!(platform, "all" | "desktop" | "mobile"), "{line}");
                (command, module, platform)
            })
            .filter(|(_, _, platform)| match *platform {
                "desktop" => cfg!(desktop),
                "mobile" => cfg!(mobile),
                _ => true,
            })
            .map(|(command, module, _)| (command, module))
            .collect();
        let table = veydan_shell::command_table(product(), module_list()).unwrap();
        for line in &table {
            assert!(expected.contains(line), "not in the golden file: {line:?}");
        }
        for line in &expected {
            assert!(table.contains(line), "the router does not know {line:?}");
        }
        assert_eq!(table.len(), expected.len());
    }

    /// The messenger registers nothing for sync: Chat syncs the system
    /// entities alone — the lock, the settings of the shell on a computer,
    /// the labels — so a storage shared with Space gives it the PIN and the
    /// language (20.6).
    #[test]
    fn the_plan_syncs_the_system_entities_alone() {
        let registry = veydan_shell::sync_registry(product(), module_list()).unwrap();
        let (puts, _) = registry.apply_order();
        #[cfg(desktop)]
        let expected = ["password_vault", "setting", "label"];
        #[cfg(mobile)]
        let expected = ["password_vault", "label"];
        assert_eq!(puts, expected);
        assert!(registry.handler_names().is_empty());
    }

    /// The data file of Chat: the core's, the lock's and sync's tables and
    /// nothing else; the messenger keeps its data in a database of its own.
    #[test]
    fn the_schemas_are_the_cores_alone() {
        let mut names: Vec<_> = veydan_shell::schemas(&module_list())
            .iter()
            .map(|schema| schema.module)
            .collect();
        names.sort_unstable();
        assert_eq!(names, ["core", "lock", "sync"]);
    }
}
