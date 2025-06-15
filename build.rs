extern crate cargo_metadata;
extern crate winres;

// use std::path::Path;

fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let version = cargo_metadata::MetadataCommand::new()
            .exec()
            .unwrap()
            .root_package()
            .unwrap()
            .version.to_string()
            .to_owned();

        let mut res = winres::WindowsResource::new();
        res
           .set_icon("green.ico")
           .set_language(0x0416) // Pt-Br
           .set("InternalName", "Cobra Criada - Snake Game")
           .set("OriginalFilename", "cobra.exe")
           .set("FileVersion", &version)
           .set("ProductVersion", &version)
           .compile().unwrap();
    }
}
