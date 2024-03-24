# Unicode Separated Values (USV)

Unicode separated values (USV) is a data format that uses Unicode symbol characters between data parts.

The USV repo is <https://github.com/sixarm/usv>.

## USV characters

Separators:

* File Separator (FS) is U+001C or U+241C ␜

* Group Separator (GS) is U+001D or U+241D ␝

* Record Separator (RS) is U+001E or U+241E ␞

* Unit Separator (US) is U+001F or U+241F ␟

Modifiers:

* Escape (ESC) is U+001B or U+241B ␛

* End of Transmission (EOT) is U+0004 or U+2404 ␄

Liners:

* Carriage Return (CR) is U+000D

* Line Feed (LF) is U+000A

## Units

```rust
use usv::*;
let input = "a␟b␟";
let units: Units = input.units().collect();
assert_eq!(units, ["a", "b"]);
```

## Records

```rust
use usv::*;
let input = "a␟b␟␞c␟d␟␞";
let records: Records = input.records().collect();
assert_eq!(records, [["a", "b"],["c", "d"]]);
```

## Groups

```rust
use usv::*;
let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
let groups: Groups = input.groups().collect();
assert_eq!(groups, [[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]]);
```

## Files

```rust
use usv::*;
let input = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
let files: Files = input.files().collect();
assert_eq!(files, [[[["a", "b"],["c", "d"]],[["e", "f"],["g", "h"]]],[[["i", "j"],["k", "l"]],[["m", "n"],["o", "p"]]]]);
```
