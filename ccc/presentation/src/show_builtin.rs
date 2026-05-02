/// A single entry in the built-in reference.
struct BuiltinEntry {
    signature: &'static str,
    return_type: &'static str,
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
                signature: "sqrt(x: number)",
                return_type: "float",
                description: "Square root",
            },
            BuiltinEntry {
                signature: "abs(x: number)",
                return_type: "same as input",
                description: "Absolute value",
            },
            BuiltinEntry {
                signature: "sin(x: number)",
                return_type: "float",
                description: "Sine",
            },
            BuiltinEntry {
                signature: "cos(x: number)",
                return_type: "float",
                description: "Cosine",
            },
            BuiltinEntry {
                signature: "tan(x: number)",
                return_type: "float",
                description: "Tangent",
            },
            BuiltinEntry {
                signature: "arcsin(x: number)",
                return_type: "float",
                description: "Arcsine",
            },
            BuiltinEntry {
                signature: "arccos(x: number)",
                return_type: "float",
                description: "Arccosine",
            },
            BuiltinEntry {
                signature: "arctan(x: number)",
                return_type: "float",
                description: "Arctangent",
            },
            BuiltinEntry {
                signature: "log(x: number)",
                return_type: "float",
                description: "Natural logarithm",
            },
            BuiltinEntry {
                signature: "log2(x: number)",
                return_type: "float",
                description: "Base-2 logarithm",
            },
            BuiltinEntry {
                signature: "log10(x: number)",
                return_type: "float",
                description: "Base-10 logarithm",
            },
            BuiltinEntry {
                signature: "floor(x: number)",
                return_type: "float",
                description: "Floor",
            },
            BuiltinEntry {
                signature: "ceil(x: number)",
                return_type: "float",
                description: "Ceiling",
            },
            BuiltinEntry {
                signature: "round(x: number)",
                return_type: "float",
                description: "Round",
            },
        ],
    },
    BuiltinCategory {
        name: "Statistics",
        entries: &[
            BuiltinEntry {
                signature: "mean(l: list)",
                return_type: "float | duration",
                description: "Mean of elements",
            },
            BuiltinEntry {
                signature: "var(l: list)",
                return_type: "float",
                description: "Variance of elements",
            },
            BuiltinEntry {
                signature: "max(l: list)",
                return_type: "same as element",
                description: "Maximum element",
            },
            BuiltinEntry {
                signature: "min(l: list)",
                return_type: "same as element",
                description: "Minimum element",
            },
            BuiltinEntry {
                signature: "median(l: list)",
                return_type: "float | duration",
                description: "Median of elements",
            },
        ],
    },
    BuiltinCategory {
        name: "List",
        entries: &[
            BuiltinEntry {
                signature: "len(l: list)",
                return_type: "int",
                description: "Length of list",
            },
            BuiltinEntry {
                signature: "sum(l: list)",
                return_type: "number | duration",
                description: "Sum of elements",
            },
            BuiltinEntry {
                signature: "prod(l: list)",
                return_type: "number",
                description: "Product of elements",
            },
            BuiltinEntry {
                signature: "head(l: list)",
                return_type: "element type",
                description: "First element",
            },
            BuiltinEntry {
                signature: "tail(l: list)",
                return_type: "list",
                description: "All elements except first",
            },
        ],
    },
    BuiltinCategory {
        name: "Constructor",
        entries: &[
            BuiltinEntry {
                signature: "DurationTime(h, m, s: int)",
                return_type: "duration",
                description: "Create duration",
            },
            BuiltinEntry {
                signature: "DurationTime(d, h, m, s: int)",
                return_type: "duration",
                description: "Create duration with days",
            },
            BuiltinEntry {
                signature: "DateTime(y, mo, d, h, mi, s: int)",
                return_type: "datetime",
                description: "Create datetime (UTC)",
            },
            BuiltinEntry {
                signature: "Timestamp(n: number)",
                return_type: "timestamp",
                description: "Create timestamp",
            },
        ],
    },
    BuiltinCategory {
        name: "Type Cast (as)",
        entries: &[
            BuiltinEntry {
                signature: "<number> as int",
                return_type: "int",
                description: "Convert to integer (truncate)",
            },
            BuiltinEntry {
                signature: "<number> as float",
                return_type: "float",
                description: "Convert to float",
            },
            BuiltinEntry {
                signature: "<datetime> as timestamp",
                return_type: "timestamp",
                description: "DateTime to timestamp",
            },
            BuiltinEntry {
                signature: "<timestamp> as datetime",
                return_type: "datetime",
                description: "Timestamp to datetime (UTC)",
            },
        ],
    },
    BuiltinCategory {
        name: "Current Time",
        entries: &[
            BuiltinEntry {
                signature: "now()",
                return_type: "datetime",
                description: "Current datetime (UTC)",
            },
            BuiltinEntry {
                signature: "today()",
                return_type: "datetime",
                description: "Today at 00:00:00 (UTC)",
            },
            BuiltinEntry {
                signature: "current_timestamp()",
                return_type: "timestamp",
                description: "Current unix timestamp",
            },
        ],
    },
];

/// Column width for aligning each column.
const SIGNATURE_COLUMN_WIDTH: usize = 38;
const RETURN_TYPE_COLUMN_WIDTH: usize = 18;

/// Print the built-in function reference to stdout.
pub fn print_builtin_list() {
    println!("Built-in Functions:");

    for category in BUILTIN_CATEGORIES {
        println!();
        println!("  {}", category.name);
        for entry in category.entries {
            let sig_pad = SIGNATURE_COLUMN_WIDTH.saturating_sub(entry.signature.len());
            let ret_pad = RETURN_TYPE_COLUMN_WIDTH.saturating_sub(entry.return_type.len());
            println!(
                "    {}{:sw$}-> {}{:rw$}{}",
                entry.signature,
                "",
                entry.return_type,
                "",
                entry.description,
                sw = sig_pad,
                rw = ret_pad,
            );
        }
    }
}
