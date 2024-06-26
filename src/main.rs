use std::collections::HashMap;

macro_rules! warn {
    ($($arg:tt)*) => {
        {
            eprint!("[LS-WATCH] ");
            eprintln!($($arg)*);
        }
    }
}

fn combine_short_args(short_args: &HashMap<char, &str>) -> (String, i32) {
    let mut no_value_block = String::new();
    let mut other_blocks = vec![];
    for (short_arg, value) in short_args {
        if value.is_empty() {
            no_value_block.push(*short_arg);
        } else {
            other_blocks.push(format!("{}{}", short_arg, value));
        }
    }
    let mut combined_short_args = String::from("-");
    combined_short_args.push_str(&no_value_block);
    let mut total_clusters = if no_value_block.is_empty() { 0 } else { 1 };
    match other_blocks.len() {
        0 => {}
        1 => {
            combined_short_args.push_str(&other_blocks[0]);
        }
        _ => {
            combined_short_args.push_str(&other_blocks[0]);
            combined_short_args.push_str(" -");
            combined_short_args.push_str(&other_blocks[1..].join(" -"));
            total_clusters += other_blocks.len() - 1;
        }
    }
    (combined_short_args, total_clusters as i32)
}

fn analyze_short_args(args: &Vec<String>) {
    let mut short_args = HashMap::new();
    let short_args_without_value: Vec<char> = vec![
        'a', 'A', 'b', 'B', 'c', 'C', 'd', 'D', 'f', 'F', 'g', 'G', 'h', 'H', 'i', 'k', 'l', 'L',
        'm', 'n', 'N', 'o', 'Q', 'r', 'R', 's', 'S', 't', 'u', 'U', 'v', 'x', 'Z', '1',
    ];
    let short_args_with_value = vec!['I', 'p', 'w', 'T'];
    let mut short_arg_clusters = 0;
    let mut iterator = args.iter();
    while let Some(arg) = iterator.next() {
        if arg.len() < 2
            || !arg.starts_with('-')
            || arg.chars().nth(1).expect("Failed to get short arg") == '-'
        {
            continue;
        }
        let mut empty_cluster = true;
        for (index, short_arg) in arg.chars().enumerate().skip(1) {
            if short_args_without_value.contains(&short_arg) {
                if short_args.contains_key(&short_arg) {
                    warn!("Duplicate argument: {}", short_arg);
                } else {
                    empty_cluster = false;
                    short_args.insert(short_arg, "");
                }
            } else if short_args_with_value.contains(&short_arg) {
                if short_args.contains_key(&short_arg) {
                    warn!("Duplicate argument: {}", short_arg);
                } else {
                    empty_cluster = false;
                    let arg_value = &arg[index + 1..];
                    if arg_value.is_empty() {
                        // The next argument is the value
                        let next_arg = iterator.next();
                        match next_arg {
                            Some(next_arg) => {
                                short_args.insert(short_arg, next_arg);
                            }
                            None => {
                                warn!("Missing value for argument: {}", short_arg);
                            }
                        }
                    } else {
                        short_args.insert(short_arg, arg_value);
                    }
                }
                break;
            } else {
                warn!("Unknown argument: {}", short_arg);
            }
        }
        if !empty_cluster {
            short_arg_clusters += 1;
        }
    }
    let (combined_short_args, total_clusters) = combine_short_args(&short_args);
    if short_arg_clusters > total_clusters {
        warn!(
            "Could have combined short arguments into {}",
            &combined_short_args,
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    analyze_short_args(&args);
}

// For reference, the man page:
// -a, --all
//     do not ignore entries starting with .
// -A, --almost-all
//     do not list implied . and ..
// --author
//     with -l, print the author of each file
// -b, --escape
//     print octal escapes for nongraphic characters
// --block-size=SIZE
//     use SIZE-byte blocks. See SIZE format below
// -B, --ignore-backups
//     do not list implied entries ending with ~
// -c
//     with -lt: sort by, and show, ctime (time of last modification of file status information) with -l: show ctime and sort by name otherwise: sort by ctime
// -C
//     list entries by columns
// --color[=WHEN]
//     colorize the output. WHEN defaults to 'always' or can be 'never' or 'auto'. More info below
// -d, --directory
//     list directory entries instead of contents, and do not dereference symbolic links
// -D, --dired
//     generate output designed for Emacs' dired mode
// -f
//     do not sort, enable -aU, disable -ls --color
// -F, --classify
//     append indicator (one of */=>@|) to entries
// --file-type
//     likewise, except do not append '*'
// --format=WORD
//     across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C
// --full-time
//     like -l --time-style=full-iso
// -g
//     like -l, but do not list owner
// --group-directories-first
//     group directories before files.
// augment with a --sort option, but any
//     use of --sort=none (-U) disables grouping
// -G, --no-group
//     in a long listing, don't print group names
// -h, --human-readable
//     with -l, print sizes in human readable format (e.g., 1K 234M 2G)
// --si
//     likewise, but use powers of 1000 not 1024
// -H, --dereference-command-line
//     follow symbolic links listed on the command line
// --dereference-command-line-symlink-to-dir
//     follow each command line symbolic link that points to a directory
// --hide=PATTERN
//     do not list implied entries matching shell PATTERN (overridden by -a or -A)
// --indicator-style=WORD
//     append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)
// -i, --inode
//     print the index number of each file
// -I, --ignore=PATTERN
//     do not list implied entries matching shell PATTERN
// -k
//     like --block-size=1K
// -l
//     use a long listing format
// -L, --dereference
//     when showing file information for a symbolic link, show information for the file the link references rather than for the link itself
// -m
//     fill width with a comma separated list of entries
// -n, --numeric-uid-gid
//     like -l, but list numeric user and group IDs
// -N, --literal
//     print raw entry names (don't treat e.g. control characters specially)
// -o
//     like -l, but do not list group information
// -p, --indicator-style=slash
//     append / indicator to directories
// -q, --hide-control-chars
//     print ? instead of non graphic characters
// --show-control-chars
//     show non graphic characters as-is (default unless program is 'ls' and output is a terminal)
// -Q, --quote-name
//     enclose entry names in double quotes
// --quoting-style=WORD
//     use quoting style WORD for entry names: literal, locale, shell, shell-always, c, escape
// -r, --reverse
//     reverse order while sorting
// -R, --recursive
//     list subdirectories recursively
// -s, --size
//     print the allocated size of each file, in blocks
// -S
//     sort by file size
// --sort=WORD
//     sort by WORD instead of name: none -U, extension -X, size -S, time -t, version -v
// --time=WORD
//     with -l, show time as WORD instead of modification time: atime -u, access -u, use -u, ctime -c, or status -c; use specified time as sort key if --sort=time
// --time-style=STYLE
//     with -l, show times using style STYLE: full-iso, long-iso, iso, locale, +FORMAT. FORMAT is interpreted like 'date'; if FORMAT is FORMAT1<newline>FORMAT2, FORMAT1 applies to non-recent files and FORMAT2 to recent files; if STYLE is prefixed with 'posix-', STYLE takes effect only outside the POSIX locale
// -t
//     sort by modification time
// -T, --tabsize=COLS
//     assume tab stops at each COLS instead of 8
// -u
//     with -lt: sort by, and show, access time with -l: show access time and sort by name otherwise: sort by access time
// -U
//     do not sort; list entries in directory order
// -v
//     natural sort of (version) numbers within text
// -w, --width=COLS
//     assume screen width instead of current value
// -x
//     list entries by lines instead of by columns
// -X
//     sort alphabetically by entry extension
// -1
//     list one file per line
//
// SELinux options:
//
// --lcontext
//     Display security context. Enable -l. Lines will probably be too wide for most displays.
// -Z, --context
//     Display security context so it fits on most displays. Displays only mode, user, group, security context and file name.
// --scontext
//     Display only security context and file name.
// --help
//     display this help and exit
// --version
//     output version information and exit
