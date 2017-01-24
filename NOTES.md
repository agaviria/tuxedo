Rust related notes:
==================
###Extern Crate declarations
A `crate` is simply put a package, which typically is used by other rust programs.  When using `extern crate` you are declaring there is a crate `linkage dependency` your program requires to compile.  To use extern crate declarations in Rust, you need to be aware of `cargo` the rust packaging system.  Declaring extern crate in a rust file is done at the top of the file:

```
extern crate chrono;
extern crate chrono::NaiveDate as date; // linking to NaiveDate as another name, 'date'
```

###Use declarations
The `use` declaration creates one or more local name bindings synonomous with some other path.  These declarations may appear in modules or blocks, usually at the top and are private by default.  They can however be made public by using the prefix keyword `pub`.

```
use std::option::Option::{Some, None};
use std::collections::hash_map::{self, HashMap};
```

* Simultaneously binding a list of paths differing only in their final element and their immediate parent module, using the `self` keyword, such as use a::b::{self, c, d};

An example of re-exporting:

```
\\ In this example, the module quux re-exports two public names defined in foo.
mod quux {
    pub use quux::foo::{bar, baz};

    pub mod foo {
        pub fn bar() { }
        pub fn baz() { }
    }
}
```

###Attributes
Attributes can be summarized as a form of metadatum that is interpreted according to:

* name e.g. `lib`
* identifier e.g. `=` or `(foo="bar")`

Attributes with a `!` after the hash `#` apply to the item that is declared within.

```
// General metadata applied to the enclosing module or crate.
#![crate_type = "lib"]
```

Attributes that do not have a bang after the hash apply to the item that follows the attribute. By the way `unimplemented!()` lets the compiler know that the function is unimplemented and ignores warnings.

```
// A function marked as unit test
#[test]
fn test_foo() {
    unimplemented!()
}
```

###Enums
Data types that represent one of several possible variants.  Each variant can optionally have data associated withit. Variants can also optionally have syntax names.  Moreover, the scope of variants are only to their enum names and can be used like constructors or functions by using `::` see last line in following example.

```
pub enum Frequency {
    Daily(bool, u16)
    Weekly(bool, u16)
}

let f = Frequency::Daily(true, 1)
```


