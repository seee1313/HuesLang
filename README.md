# HuesLang 
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

it is currently a toy system programming lang, but the ultimate goal is to make it production-ready. I'll be glad to receive any help

## Features
1. variables and functions
2. Returns
3. Compiles to LLVM

## Restrictions
1. Syntax unstable
2. there is no standard library
3. there may be bugs

## Code Example 
```HuesLang
extern hue puts(s: *i8) -> void;

hue main() -> i32 {
    let x = 10;

    if x > 5 {
        puts("x is big");
    } else {
        puts("x is small");
    }
}

```

## How to compile?
 ```Bash
git clone https://github.com/seee1313/HuesLang.git
cd HuesLang
cargo build --release
```
## How to run? 
 ```Bash
./target/release/huesc program.hues
 ```

## License
This project is licensed under the **Mozilla Public License Version 2.0** (MPL-2.0). 
See the [LICENSE](LICENSE) file for more details.

**WARNING: HuesLang is still unstable so DO NOT USE IT IN PRODUCTION**
