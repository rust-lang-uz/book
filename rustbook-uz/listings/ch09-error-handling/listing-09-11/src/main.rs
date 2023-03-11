// ANCHOR: here
fn birinchi_satrning_oxirgi_belgisi(matn: &str) -> Option<char> {
    matn.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        birinchi_satrning_oxirgi_belgisi("Salom Do'stim\n Ahvollaring qanday?"),
        Some('m')
    );

    assert_eq!(birinchi_satrning_oxirgi_belgisi(""), None);
    assert_eq!(birinchi_satrning_oxirgi_belgisi("\nhi"), None);
}
