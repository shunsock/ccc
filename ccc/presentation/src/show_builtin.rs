/// Print the built-in function reference to stdout.
pub fn print_builtin_list() {
    print!(
        "\
Built-in Functions:

  Math
    sqrt(x)          Square root
    abs(x)           Absolute value
    sin(x)           Sine
    cos(x)           Cosine
    tan(x)           Tangent
    arcsin(x)        Arcsine
    arccos(x)        Arccosine
    arctan(x)        Arctangent
    log(x)           Natural logarithm
    log2(x)          Base-2 logarithm
    log10(x)         Base-10 logarithm
    floor(x)         Floor
    ceil(x)          Ceiling
    round(x)         Round

  Statistics
    mean(l)          Mean of elements
    var(l)           Variance of elements
    max(l)           Maximum element
    min(l)           Minimum element
    median(l)        Median of elements

  List
    len(l)           Length of list
    sum(l)           Sum of elements
    prod(l)          Product of elements
    head(l)          First element
    tail(l)          All elements except first

  Constructor
    DurationTime(h, m, s)           Create duration
    DurationTime(d, h, m, s)        Create duration with days
    DateTime(y, mo, d, h, mi, s)    Create datetime (UTC)
    Timestamp(n)                    Create timestamp

  Type Cast (as)
    <expr> as int                   Convert to integer (truncate)
    <expr> as float                 Convert to float
    <expr> as timestamp             DateTime to timestamp
    <expr> as datetime              Timestamp to datetime (UTC)

  Current Time
    now()                 Current datetime (UTC)
    today()               Today at 00:00:00 (UTC)
    current_timestamp()   Current unix timestamp
"
    );
}
