## quote
> https://crates.io/crates/quote

将Rust语法项展开为TokenStream的功能，以#开头的标识符将引用前文中已定义的标识符

```rust
let varname = format_ident!("_{}", ident);
quote! {
    let mut #varname = 0;
}
```

## proc-macro2
> https://crates.io/crates/proc-macro2

