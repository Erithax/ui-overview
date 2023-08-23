
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize)]

pub struct Langbridge {}

impl Blockify for Langbridge {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        (
            "Erithaxlangbridge",
                Info {
                    name: "Erithax Langbridge",
                    owner: Owner::Erithax,
                    description: "TODO",
                    code_openness: SourceOpenness::Copyleft,
                    impl_langs: vec![Lang::Rust],
                    website: "https://erithax.com",
                },
                ExtraInfo::Langbridge {
                    bind_langs: vec![Lang::C],
                },
                parsetree2!{
                    $
                }
                // vec![vec!["Erithaxlangbridge"]],
        ),
        (
        "Gtk3rs", 
            Info {
                name: "Gtk3-rs",
                owner: Owner::Gtkrs,
                description: "TODO",
                code_openness: SourceOpenness::Copyleft,
                impl_langs: vec![Lang::Rust],
                website: "https://gtk-rs.org",
            },
            ExtraInfo::Langbridge {
                bind_langs: vec![Lang::Rust],
            },
            vec![vec!["Gtk3rs", "Gtk"]],
        ),
        ] 
    }
}


