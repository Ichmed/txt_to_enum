# txt_to_enum
Provides the `txt_to_enum!` macro that takes a relative path to a .txt file and creates an Enum from it.

This file
```
Name
VariantA
VariantB
VariantC
```
will be converted into
```rust
enum Name {
    VariantA,
    VariantB,
    VariantC
}
```
Tagged Unions are not supported