# HuesLang Documentation

## Overview

HuesLang is a compiled systems programming language with switchable strictness modes. It transpiles to Nim, then compiles to C via Zig for maximum cross-platform compatibility.

**Key Features:**
- Multiple strictness modes (from beginner-friendly to bare-metal)
- Clean, readable syntax
- Zero-cost abstractions
- UEFI and embedded support
- Cross-compilation via Zig

---

## Compilation Modes

Place at the top of your file:

```
!#script-kiddy    // Forgiving mode, auto type inference, GC enabled
!#normal          // Balanced strictness and convenience
!#strict          // Maximum safety, unsafe blocks required for raw memory
!#unsafe          // Raw pointers allowed everywhere
!#web-backend     // Optimized for server applications
!#universal       // Cross-platform library mode
!#no-os           // Bare-metal, no GC, no std library (kernels, bootloaders)
```

---

## Data Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer numbers | `42`, `-17`, `0` |
| `float` | Floating-point numbers | `3.14`, `-0.5` |
| `bool` | Boolean values | `true`, `false` |
| `string` | Text strings | `"Hello, World!"` |

---

## Variables

### Immutable Variables (hui)

```
hui name = "Alice"
hui age = 25
hui pi = 3.14159

name = "Bob"    // ❌ Error: hui variables are immutable
```

### Mutable Variables (piz)

```
piz counter = 0
counter = counter + 1    // ✅ OK

piz message = "Hello"
message = "Goodbye"      // ✅ OK
```

### Constants (const)

```
const MAX_SIZE = 1024
const APP_NAME = "MyApp"
const PI = 3.14159
```

---

## Functions

### Basic Function

```
hue greet(name: string) {
    prt("Hello, " + name)
}

greet("World")    // Output: Hello, World
```

### Function with Return Value

```
hue add(a: int, b: int) int {
    ret a + b
}

hui result = add(5, 3)
prt(result)    // Output: 8
```

### Function with Multiple Parameters

```
hue createGreeting(name: string, age: int) string {
    ret "Name: " + name + ", Age: " + age
}
```

---

## Structures (construct)

### Definition

```
construct Player {
    Name: string,
    Health: int,
    Alive: bool,
    Stamina: int
}
```

### Instantiation

```
hui player = Player {
    Name: "Hero",
    Health: 100,
    Alive: true,
    Stamina: 100
}
```

### Field Access

```
prt(player.Name)       // Output: Hero
prt(player.Health)     // Output: 100
```

### Field Modification

```
piz enemy = Player {
    Name: "Goblin",
    Health: 50,
    Alive: true,
    Stamina: 30
}

enemy.Health = enemy.Health - 10
prt(enemy.Health)    // Output: 40
```

### Methods

```
construct Player {
    Name: string,
    Health: int,
    Alive: bool
}

hue Player.takeDamage(damage: int) {
    self.Health = self.Health - damage
    
    check self.Health <= 0 {
        self.Alive = false
    }
}

hue Player.isAlive() bool {
    ret self.Alive
}

hue Player.heal(amount: int) {
    self.Health = self.Health + amount
}

// Usage
piz player = Player {
    Name: "Hero",
    Health: 100,
    Alive: true
}

player.takeDamage(30)
prt(player.Health)       // Output: 70
prt(player.isAlive())    // Output: true
```

---

## Arrays

### Declaration

```
piz numbers = [1, 2, 3, 4, 5]
piz names = ["Alice", "Bob", "Charlie"]
hui constants = [10, 20, 30]
```

### Access (Zero-indexed)

```
prt(numbers[0])    // Output: 1
prt(numbers[4])    // Output: 5
prt(names[1])      // Output: Bob
```

### Modification

```
piz items = [1, 2, 3]
items[0] = 100
prt(items[0])    // Output: 100

items.add(4)     // Add element
prt(items[3])    // Output: 4
```

---

## Control Flow

### Conditional Statements

```
hui age = 18

check age < 18 {
    prt("Access denied")
}
then check age == 18 {
    prt("Just turned 18")
}
then {
    prt("Access granted")
}
```

### Pattern

```
check condition {
    // if block
}
then check another_condition {
    // else if block
}
then {
    // else block
}
```

---

## Loops

### Infinite Loop (zalyping)

```
zalyping {
    prt("Forever")
    // Use nahui to exit
}
```

### While Loop (poka)

```
piz x = 0
poka x < 10 {
    prt(x)
    x = x + 1
}
```

### For Loop

```
for i in 0..10 {
    prt(i)
}

for item in items {
    prt(item)
}
```

### Loop Control

```
piz i = 0
poka i < 100 {
    check i == 50 {
        nahui    // break - exit loop
    }
    
    check i % 2 == 0 {
        i = i + 1
        ebat     // continue - skip iteration
    }
    
    prt(i)
    i = i + 1
}
```

---

## Comments

### Single-line

```
// This is a comment
hui x = 42    // Inline comment
```

### Multi-line

```
/[
    This is a multi-line comment.
    It can span multiple lines.
    Use it for documentation.
/]
```

---

## Imports

```
give @std/io
give @std/math
give @std/strings
give @myproject/utils
```

---

## Unsafe Blocks

In `!#strict` mode, raw memory operations require unsafe blocks:

```
!#strict

hui x = 42

// Raw memory access requires unsafe
unsafe {
    piz ptr = 0xB8000
    ptr[0] = 0xFF
}

// Outside unsafe - safe code only
prt(x)
```

---

## Complete Example

```
!#normal

give @std/io

construct Task {
    Title: string,
    Done: bool,
    Priority: int
}

hue Task.complete() {
    self.Done = true
}

hue Task.display() {
    check self.Done {
        prt("[X] " + self.Title)
    }
    then {
        prt("[ ] " + self.Title)
    }
}

hue main() {
    piz tasks = [
        Task { Title: "Learn HuesLang", Done: false, Priority: 1 },
        Task { Title: "Build compiler", Done: false, Priority: 2 },
        Task { Title: "Take over world", Done: false, Priority: 3 }
    ]
    
    // Complete first task
    tasks[0].complete()
    
    // Display all tasks
    for task in tasks {
        task.display()
    }
}
```

**Output:**
```
[X] Learn HuesLang
[ ] Build compiler
[ ] Take over world
```

---

## Bare-Metal Example (UEFI)

```
!#no-os

hue efi_main(handle: EFI_HANDLE, table: ptr EFI_SYSTEM_TABLE) EFI_STATUS {
    
    unsafe {
        hui gop = table.BootServices.LocateProtocol(EFI_GOP_GUID)
        
        piz framebuffer = gop.Mode.FrameBufferBase
        
        // Draw red pixel at (0, 0)
        framebuffer[0] = 0xFF
        framebuffer[1] = 0x00
        framebuffer[2] = 0x00
    }
    
    ret EFI_SUCCESS
}
```

---

## Keyword Reference

| Keyword | Description |
|---------|-------------|
| `hui` | Immutable variable declaration |
| `piz` | Mutable variable declaration |
| `const` | Constant declaration |
| `hue` | Function declaration |
| `construct` | Structure definition |
| `self` | Reference to current instance |
| `check` | If statement |
| `then` | Else / else-if statement |
| `zalyping` | Infinite loop |
| `poka` | While loop |
| `for` | For loop |
| `nahui` | Break from loop |
| `ebat` | Continue to next iteration |
| `ret` | Return from function |
| `prt` | Print to stdout |
| `give` | Import module |
| `unsafe` | Unsafe code block |

---

## Compilation

```bash
# Standard compilation
huesc main.hues -o main

# With specific mode (overrides file directive)
huesc main.hues -o main --strict

# Cross-compilation
huesc main.hues -o main.exe --target x86_64-windows
huesc main.hues -o main.efi --target x86_64-uefi

# Debug build
huesc main.hues -o main --debug

# Release build (optimized)
huesc main.hues -o main --release
```

---

## License

**Compiler:** GPL-2.0

**Your code:** No restrictions. Use HuesLang for open source, closed source, commercial projects — anything you want.
