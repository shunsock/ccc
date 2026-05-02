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
                signature: "sqrt(x: int | float) -> float",
                description: "Square root",
            },
            BuiltinEntry {
                signature: "abs(x: int | float) -> int | float",
                description: "Absolute value (preserves input type)",
            },
            BuiltinEntry {
                signature: "sin(x: int | float) -> float",
                description: "Sine",
            },
            BuiltinEntry {
                signature: "cos(x: int | float) -> float",
                description: "Cosine",
            },
            BuiltinEntry {
                signature: "tan(x: int | float) -> float",
                description: "Tangent",
            },
            BuiltinEntry {
                signature: "arcsin(x: int | float) -> float",
                description: "Arcsine",
            },
            BuiltinEntry {
                signature: "arccos(x: int | float) -> float",
                description: "Arccosine",
            },
            BuiltinEntry {
                signature: "arctan(x: int | float) -> float",
                description: "Arctangent",
            },
            BuiltinEntry {
                signature: "log(x: int | float) -> float",
                description: "Natural logarithm",
            },
            BuiltinEntry {
                signature: "log2(x: int | float) -> float",
                description: "Base-2 logarithm",
            },
            BuiltinEntry {
                signature: "log10(x: int | float) -> float",
                description: "Base-10 logarithm",
            },
            BuiltinEntry {
                signature: "floor(x: int | float) -> float",
                description: "Floor",
            },
            BuiltinEntry {
                signature: "ceil(x: int | float) -> float",
                description: "Ceiling",
            },
            BuiltinEntry {
                signature: "round(x: int | float) -> float",
                description: "Round",
            },
        ],
    },
    BuiltinCategory {
        name: "Statistics",
        entries: &[
            BuiltinEntry {
                signature: "mean(l: list[int | float]) -> float",
                description: "Mean of numeric elements",
            },
            BuiltinEntry {
                signature: "mean(l: list[duration]) -> duration",
                description: "Mean of duration elements",
            },
            BuiltinEntry {
                signature: "variance(l: list[int | float]) -> float",
                description: "Variance of numeric elements",
            },
            BuiltinEntry {
                signature: "max(l: list[int | float | duration]) -> int | float | duration",
                description: "Maximum element",
            },
            BuiltinEntry {
                signature: "min(l: list[int | float | duration]) -> int | float | duration",
                description: "Minimum element",
            },
            BuiltinEntry {
                signature: "median(l: list[int | float]) -> float",
                description: "Median of numeric elements",
            },
            BuiltinEntry {
                signature: "median(l: list[duration]) -> duration",
                description: "Median of duration elements",
            },
        ],
    },
    BuiltinCategory {
        name: "List",
        entries: &[
            BuiltinEntry {
                signature: "len(l: list) -> int",
                description: "Length of list",
            },
            BuiltinEntry {
                signature: "sum(l: list[int | float]) -> int | float",
                description: "Sum of numeric elements",
            },
            BuiltinEntry {
                signature: "sum(l: list[duration]) -> duration",
                description: "Sum of duration elements",
            },
            BuiltinEntry {
                signature: "prod(l: list[int | float]) -> int | float",
                description: "Product of numeric elements",
            },
            BuiltinEntry {
                signature: "head(l: list) -> element type",
                description: "First element",
            },
            BuiltinEntry {
                signature: "tail(l: list) -> list",
                description: "All elements except first",
            },
        ],
    },
    BuiltinCategory {
        name: "Constructor",
        entries: &[
            BuiltinEntry {
                signature: "DurationTime(h, m, s: int) -> duration",
                description: "Create duration",
            },
            BuiltinEntry {
                signature: "DurationTime(d, h, m, s: int) -> duration",
                description: "Create duration with days",
            },
            BuiltinEntry {
                signature: "DateTime(y, mo, d, h, mi, s: int) -> datetime",
                description: "Create datetime (UTC)",
            },
            BuiltinEntry {
                signature: "Timestamp(n: int | float) -> timestamp",
                description: "Create timestamp",
            },
        ],
    },
    BuiltinCategory {
        name: "Type Cast (as)",
        entries: &[
            BuiltinEntry {
                signature: "<int | float> as int -> int",
                description: "Truncate toward zero",
            },
            BuiltinEntry {
                signature: "<int | float> as float -> float",
                description: "Widen to float",
            },
            BuiltinEntry {
                signature: "<datetime> as timestamp -> timestamp",
                description: "Convert to epoch seconds",
            },
            BuiltinEntry {
                signature: "<timestamp> as datetime -> datetime",
                description: "Convert to UTC datetime",
            },
        ],
    },
    BuiltinCategory {
        name: "Current Time",
        entries: &[
            BuiltinEntry {
                signature: "now() -> datetime",
                description: "Current datetime (UTC)",
            },
            BuiltinEntry {
                signature: "today() -> datetime",
                description: "Today at 00:00:00 (UTC)",
            },
            BuiltinEntry {
                signature: "current_timestamp() -> timestamp",
                description: "Current unix timestamp",
            },
        ],
    },
];

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
