extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("build-assets/icon.rc", embed_resource::NONE).manifest_optional().unwrap();
    }
}