/// A single entry in the built-in reference.
struct BuiltinEntry {
    signature: &'static str,
    description: &'static str,
}

/// A named group of related built-in entries.
struct BuiltinCategory {
    name: &'static str,
    entries: &'static [BuiltinEntry],
}

const BUILTIN_CATEGORIES: &[BuiltinCategory] = &[
    BuiltinCategory {
        name: "Math",
        entries: &[
            BuiltinEntry {
                signature: "sqrt(x)",
                description: "Square root",
            },
            BuiltinEntry {
                signature: "abs(x)",
                description: "Absolute value",
            },
            BuiltinEntry {
                signature: "sin(x)",
                description: "Sine",
            },
            BuiltinEntry {
                signature: "cos(x)",
                description: "Cosine",
            },
            BuiltinEntry {
                signature: "tan(x)",
                description: "Tangent",
            },
            BuiltinEntry {
                signature: "arcsin(x)",
                description: "Arcsine",
            },
            BuiltinEntry {
                signature: "arccos(x)",
                description: "Arccosine",
            },
            BuiltinEntry {
                signature: "arctan(x)",
                description: "Arctangent",
            },
            BuiltinEntry {
                signature: "log(x)",
                description: "Natural logarithm",
            },
            BuiltinEntry {
                signature: "log2(x)",
                description: "Base-2 logarithm",
            },
            BuiltinEntry {
                signature: "log10(x)",
                description: "Base-10 logarithm",
            },
            BuiltinEntry {
                signature: "floor(x)",
                description: "Floor",
            },
            BuiltinEntry {
                signature: "ceil(x)",
                description: "Ceiling",
            },
            BuiltinEntry {
                signature: "round(x)",
                description: "Round",
            },
        ],
    },
    BuiltinCategory {
        name: "Statistics",
        entries: &[
            BuiltinEntry {
                signature: "mean(l)",
                description: "Mean of elements",
            },
            BuiltinEntry {
                signature: "var(l)",
                description: "Variance of elements",
            },
            BuiltinEntry {
                signature: "max(l)",
                description: "Maximum element",
            },
            BuiltinEntry {
                signature: "min(l)",
                description: "Minimum element",
            },
            BuiltinEntry {
                signature: "median(l)",
                description: "Median of elements",
            },
        ],
    },
    BuiltinCategory {
        name: "List",
        entries: &[
            BuiltinEntry {
                signature: "len(l)",
                description: "Length of list",
            },
            BuiltinEntry {
                signature: "sum(l)",
                description: "Sum of elements",
            },
            BuiltinEntry {
                signature: "prod(l)",
                description: "Product of elements",
            },
            BuiltinEntry {
                signature: "head(l)",
                description: "First element",
            },
            BuiltinEntry {
                signature: "tail(l)",
                description: "All elements except first",
            },
        ],
    },
    BuiltinCategory {
        name: "Constructor",
        entries: &[
            BuiltinEntry {
                signature: "DurationTime(h, m, s)",
                description: "Create duration",
            },
            BuiltinEntry {
                signature: "DurationTime(d, h, m, s)",
                description: "Create duration with days",
            },
            BuiltinEntry {
                signature: "DateTime(y, mo, d, h, mi, s)",
                description: "Create datetime (UTC)",
            },
            BuiltinEntry {
                signature: "Timestamp(n)",
                description: "Create timestamp",
            },
        ],
    },
    BuiltinCategory {
        name: "Type Cast (as)",
        entries: &[
            BuiltinEntry {
                signature: "<expr> as int",
                description: "Convert to integer (truncate)",
            },
            BuiltinEntry {
                signature: "<expr> as float",
                description: "Convert to float",
            },
            BuiltinEntry {
                signature: "<expr> as timestamp",
                description: "DateTime to timestamp",
            },
            BuiltinEntry {
                signature: "<expr> as datetime",
                description: "Timestamp to datetime (UTC)",
            },
        ],
    },
    BuiltinCategory {
        name: "Current Time",
        entries: &[
            BuiltinEntry {
                signature: "now()",
                description: "Current datetime (UTC)",
            },
            BuiltinEntry {
                signature: "today()",
                description: "Today at 00:00:00 (UTC)",
            },
            BuiltinEntry {
                signature: "current_timestamp()",
                description: "Current unix timestamp",
            },
        ],
    },
];

/// Column width for aligning signatures and descriptions.
const SIGNATURE_COLUMN_WIDTH: usize = 32;

/// Print the built-in function reference to stdout.
pub fn print_builtin_list() {
    println!("Built-in Functions:");

    for category in BUILTIN_CATEGORIES {
        println!();
        println!("  {}", category.name);
        for entry in category.entries {
            let padding = SIGNATURE_COLUMN_WIDTH.saturating_sub(entry.signature.len());
            println!(
                "    {}{:width$}{}",
                entry.signature,
                "",
                entry.description,
                width = padding
            );
        }
    }
}
