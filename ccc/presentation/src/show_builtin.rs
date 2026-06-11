use crate::builtin_reference::BUILTIN_CATEGORIES;

/// Column width for aligning signatures and descriptions.
const SIGNATURE_COLUMN_WIDTH: usize = 64;

/// Print the built-in function reference to stdout.
pub fn print_builtin_list() {
    println!("Built-in Functions:");

    for category in BUILTIN_CATEGORIES {
        println!();
        println!("  {}", category.name);
        for entry in category.entries {
            let padding = SIGNATURE_COLUMN_WIDTH.saturating_sub(entry.signature.len());
            println!(
                "    {}{:w$}{}",
                entry.signature,
                "",
                entry.description,
                w = padding,
            );
        }
    }
}
