// Embed the FinFlow logo as the native .exe icon.
// `embed-resource` uses rc.exe on MSVC and windres on GNU targets,
// so both local builds and GitHub Actions get the same icon.
fn main() {
    embed_resource::compile("assets/logo.rc", embed_resource::NONE);
}
