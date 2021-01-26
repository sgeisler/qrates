//! Report information about traits and their implementations.

use super::utils::DefPathResolver;
use super::utils::GroupByIterator;
use crate::write_csv;
use corpus_database::tables::Loader;
use log::info;
use std::collections::{HashMap, BTreeMap};
use std::path::Path;
use corpus_database::types::Name;
use corpus_database::types::Item;

pub fn query(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let type_trait_impls = loader.load_type_defs()
        .iter()
        .cloned()
        .filter_map(|(item, typ, def_path, interned_string, visibility, ty_def_kind)| {
            let path = def_path_resolver.resolve(def_path);
            if path.0 != "bitcoin" {
                return None;
            }

            let name = path.4.to_string();
            let traits = loader.load_trait_impls().iter().cloned().filter_map(|(i, t, p)| {
                if t == typ {
                    Some(def_path_resolver.resolve(p).4.to_string())
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            Some((name, traits))
        }).collect::<BTreeMap<_, _>>();

    dbg!(type_trait_impls);
}
