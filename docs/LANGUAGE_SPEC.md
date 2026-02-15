# AeroFlow Language Specification v1.0

**Deterministic · Minimal · Backend + AI + App Native**

This is the formal specification of the AeroFlow programming language.

---

## Design Principles

1. **Deterministic execution** - Same input always produces same output
2. **No shared mutable state** - All state is isolated
3. **Message passing only** - Actors communicate via messages
4. **Explicit effects** - Side effects are declared, not hidden
5. **Finite grammar** - Language is simple and parseable
6. **Compile-time simplicity** - Fast compilation > syntactic sugar

**If a feature violates these principles, it does not exist in AeroFlow.**

---

## File Types

| Extension | Purpose |
|-----------|---------|
| `.af` | Source code |
| `.aefl` | Executable flow (bytecode) |
| `.afb` | Raw bytecode |
| `.afm` | Runtime manifest |
| `.afs` | Production snapshot (AOT) |

---

## Lexical Structure

### Tokens

- **Identifiers**: `[a-zA-Z_][a-zA-Z0-9_]*`
- **Numbers**: Integer (`123`) or Float (`123.45`)
- **Strings**: UTF-8, double-quoted (`"hello"`)
- **Keywords**: Fixed set (see below)

### Keywords

```
fn      actor   on      state   emit
send    recv    spawn   effect  return
if      else    for     while   loop
match   import  export  let     mut
```

### Operators

```
+  -  *  /  %     # Arithmetic
== != < > <= >=   # Comparison  
and or not        # Logical
=                 # Assignment
```

### Comments

```rust
# Single-line comment

/* Multi-line
   comment */
```

---

## Type System

### Primitive Types

```rust
Int       # 64-bit signed integer
Float     # 64-bit floating point
Bool      # true or false
String    # UTF-8 immutable string
```

### Collection Types

```rust
List[T]      # Ordered collection
Map[K, V]    # Key-value pairs
Tuple        # Fixed-size heterogeneous
```

### Special Types

```rust
ActorRef     # Reference to an actor
Bytes        # Raw byte array
Result[T, E] # Success or error
```

### Type Inference

```rust
x = 10          # Type inferred as Int
name = "Bob"    # Type inferred as String
nums = [1,2,3]  # Type inferred as List[Int]
```

**Once inferred, types are fixed (no dynamic typing).**

---

## Variables

### Declaration

```rust
x = 10
name = "AeroFlow"
active = true
```

### Mutability

All variables are **immutable by default**:

```rust
x = 10
x = 20  # Error: cannot reassign

# Explicit mutability
mut y = 10
y = 20  # OK
```

---

## Functions

### Basic Functions

```rust
fn add(a, b) {
    return a + b
}

result = add(10, 20)
```

### Type Annotations (Optional)

```rust
fn multiply(a: Int, b: Int) -> Int {
    return a * b
}
```

### Pure Functions

Functions are **pure by default** (no side effects):

```rust
fn calculate(x) {
    return x * 2  # Pure
}
```

### Effectful Functions

Side effects must be declared:

```rust
fn fetch_data() uses http {
    return http.get("/api/data")
}
```

---

## Control Flow

### If Statements

```rust
if x > 10 {
    print("Large")
} else {
    print("Small")
}
```

**No ternary operator.**

### Loops

```rust
# For loop
for i in range(0, 10) {
    print(i)
}

# While loop
while x < 100 {
    x = x + 1
}
```

**Infinite loops require explicit `yield` to prevent freezing.**

### Match (Pattern Matching)

```rust
match value {
    Ok(v) => print(v)
    Err(e) => print("Error:", e)
}
```

---

## Actors (Core Primitive)

Actors are the **fundamental unit of concurrency** in AeroFlow.

### Actor Definition

```rust
actor Counter {
    state count = 0

    on Increment {
        count = count + 1
        emit(count)
    }

    on Decrement {
        count = count - 1
        emit(count)
    }
}
```

### Spawning Actors

```rust
counter = spawn(Counter)
```

### Sending Messages

```rust
send(counter, Increment)
send(counter, Decrement)
```

### Receiving Results

```rust
result = recv()
print(result)  # Output: 1
```

### Actor Guarantees

✅ **One message at a time** - No race conditions  
✅ **No locks** - Message passing is lock-free  
✅ **Crash isolation** - One actor crash doesn't kill others  
✅ **Supervision** - Actors can be supervised and restarted

---

## Effects System

All side effects are **explicit** and **trackable**.

### Effect Declaration

```rust
effect http
effect db
effect io
effect gpu
```

### Using Effects

```rust
fn fetch_user(id) uses db {
    return db.query("SELECT * FROM users WHERE id = ?", id)
}
```

### Why This Matters

- **Static analysis** - Know what a function can do
- **Sandboxing** - Restrict effects in production
- **Replay** - Reproduce bugs deterministically
- **AI reasoning** - LLMs can understand code behavior

---

## Error Handling

**No exceptions.** All errors are explicit.

### Result Type

```rust
fn divide(a, b) -> Result[Float, String] {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}
```

### Handling Errors

```rust
result = divide(10, 0)

match result {
    Ok(value) => print("Result:", value)
    Err(error) => print("Error:", error)
}
```

---

## Modules

### Importing

```rust
import net.http
import ai.model
import utils.math
```

### Exporting

```rust
export fn add(a, b) {
    return a + b
}
```

### Module Rules

- **Static imports** - No dynamic `require()`
- **No circular dependencies** - Enforced at compile time
- **Compiled once** - Modules are cached

---

## Concurrency Model

### Fibers (Invisible)

You **never manage fibers directly**. The runtime handles them.

**Rules:**
- Every HTTP request = 1 fiber
- Every actor = 1 fiber
- I/O operations yield automatically

### Message Passing

```rust
send(actor_id, message)
result = recv()
```

**Guarantees:**
- **Ordered** - Messages arrive in send order
- **Non-blocking** - Sender doesn't wait for receiver
- **Zero-copy** - When possible

---

## Performance Guarantees

These are **part of the language specification**:

| Feature | Guarantee |
|---------|-----------|
| Compilation | O(n) complexity |
| Startup | <10µs |
| Actor switch | <100ns |
| Message send | O(1) |
| I/O | Non-blocking |
| GC | Optional (arena-based default) |

---

## Security Model

### Capabilities

All permissions are declared in `manifest.afm`:

```toml
[permissions]
network = ["http", "https"]
filesystem = ["read:/app/data"]
gpu = true
```

### Runtime Enforcement

Permissions are checked at **instruction level**, not function level.

```rust
# This fails if "http" not in permissions
http.get("/api")
```

---

## Memory Model

| Component | Strategy |
|-----------|----------|
| Variables | Stack / Arena |
| Objects | Region-based allocation |
| Strings | Immutable slices |
| Collections | Copy-on-write |
| AI Tensors | Pinned memory |

**No garbage collector by default.**

---

## Standard Library (Minimal)

### Built-in Functions

```rust
print(...)       # Output to console
range(start, end) # Generate range
spawn(actor)     # Create actor
send(ref, msg)   # Send message
recv()           # Receive message
```

### Standard Modules

```rust
import math      # Mathematical functions
import string    # String operations
import list      # List utilities
import http      # HTTP client/server
import json      # JSON parsing
import ai        # AI model loading
```

---

## Future Reserved Keywords

These are **reserved** but not yet implemented:

```
async   await   trait   impl    struct
enum    union   unsafe  pub     priv
```

---

## Grammar (EBNF)

```ebnf
program        ::= { declaration }

declaration    ::= function
                 | actor
                 | effect_decl
                 | import

function       ::= "fn" identifier "(" [params] ")" [effect_list] block

actor          ::= "actor" identifier "{" actor_body "}"

actor_body     ::= { state_decl | handler }

state_decl     ::= "state" identifier "=" expression

handler        ::= "on" identifier block

effect_list    ::= "uses" identifier { "," identifier }

block          ::= "{" { statement } "}"

statement      ::= let_stmt
                 | return_stmt
                 | if_stmt
                 | for_stmt
                 | while_stmt
                 | expr_stmt

expression     ::= literal
                 | identifier  
                 | binary_op
                 | call

literal        ::= number | string | boolean

binary_op      ::= expression operator expression

operator       ::= "+" | "-" | "*" | "/" | "==" | "!=" | ">" | "<"
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-02-15 | Initial stable release |
| 0.9.0 | 2026-02-01 | Beta preview |

---

## Compliance

An implementation is **AeroFlow-compliant** if it:

1. Passes the official test suite
2. Meets all performance guarantees above
3. Implements the full grammar
4. Enforces the security model

---

**This specification is maintained by the AeroFlow Steering Committee.**
