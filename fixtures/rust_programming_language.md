**Rust** is a [general-purpose programming language](https://en.wikipedia.org/wiki/general-purpose_programming_language) that emphasizes [performance](https://en.wikipedia.org/wiki/Computer_performance), [type safety](https://en.wikipedia.org/wiki/type_safety), [concurrency](https://en.wikipedia.org/wiki/Concurrency_(computer_science)), and [memory safety](https://en.wikipedia.org/wiki/memory_safety).

Rust supports multiple [programming paradigms](https://en.wikipedia.org/wiki/programming_paradigm). It was influenced by ideas from [functional programming](https://en.wikipedia.org/wiki/functional_programming), including [immutability](https://en.wikipedia.org/wiki/immutability), [higher-order functions](https://en.wikipedia.org/wiki/higher-order_function), [algebraic data types](https://en.wikipedia.org/wiki/algebraic_data_type), and [pattern matching](https://en.wikipedia.org/wiki/pattern_matching). It also supports [object-oriented programming](https://en.wikipedia.org/wiki/object-oriented_programming) via structs, [enums](https://en.wikipedia.org/wiki/Union_type), traits, and methods. Rust enforces memory safety (i.e., that all [references](https://en.wikipedia.org/wiki/Reference_(computer_science)) point to valid memory) without a conventional [garbage collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)); instead, memory safety errors and [data races](https://en.wikipedia.org/wiki/data_race) are prevented by the "borrow checker", which tracks the [object lifetime](https://en.wikipedia.org/wiki/object_lifetime) of references [at compile time](https://en.wikipedia.org/wiki/Compiler).

Software developer Graydon Hoare created Rust in 2006 while working at [Mozilla](https://en.wikipedia.org/wiki/Mozilla), which officially sponsored the project in 2009. The first stable release, Rust 1.0, was published in May 2015. The project has been sponsored by the [Rust Foundation](https://en.wikipedia.org/wiki/#Rust_Foundation) since February 2021.

Rust has been adopted by many software projects, especially [web services](https://en.wikipedia.org/wiki/web_service) and [system software](https://en.wikipedia.org/wiki/system_software). It has been studied academically and has a growing community of developers.

## History

### 2006–2009: Early years

 In 2006, Rust began as a personal project by [Mozilla](https://en.wikipedia.org/wiki/Mozilla) employee Graydon Hoare. According to *MIT Technology Review*, he started the project due to his frustration with a broken elevator in his apartment building whose software had crashed, and named the language after the [fungi of the same name](https://en.wikipedia.org/wiki/Rust_(fungus)) that is "over-engineered for survival". Between 2006 and 2009, Rust was not publicized to others at Mozilla and was written in Hoare's free time. Hoare began speaking about the language around 2009 after a small group at Mozilla became interested.

Early contributors have agreed that the language was based primarily on existing ideas. Hoare cited languages from the 1970s, 1980s, and 1990s as influences — including [CLU](https://en.wikipedia.org/wiki/CLU_(programming_language)), [BETA](https://en.wikipedia.org/wiki/BETA_(programming_language)), [Mesa](https://en.wikipedia.org/wiki/Mesa_(programming_language)), NIL, [Erlang](https://en.wikipedia.org/wiki/Erlang_(programming_language)), [Newsqueak](https://en.wikipedia.org/wiki/Newsqueak), [Napier](https://en.wikipedia.org/wiki/Napier88), [Hermes](https://en.wikipedia.org/wiki/Hermes_(programming_language)), [Sather](https://en.wikipedia.org/wiki/Sather), [Alef](https://en.wikipedia.org/wiki/Alef_(programming_language)), and [Limbo](https://en.wikipedia.org/wiki/Limbo_(programming_language)). He described the language as "technology from the past come to save the future from itself." Early contributor Manish Goregaokar similarly described it as based on "mostly decades-old research."

The first [compiler](https://en.wikipedia.org/wiki/compiler) was written in about 38,000 lines of [OCaml](https://en.wikipedia.org/wiki/OCaml). It supported a number of features no longer present today, including explicit [object-oriented programming](https://en.wikipedia.org/wiki/object-oriented_programming) via an keyword and a [typestate analysis](https://en.wikipedia.org/wiki/typestate_analysis) system tracking variable state such as going from uninitialized to initialized.

### 2009–2012: Mozilla sponsorship

In 2009, Mozilla officially sponsored the project. [Brendan Eich](https://en.wikipedia.org/wiki/Brendan_Eich) and other executives, intrigued by the possibility of using Rust for a safe [web browser](https://en.wikipedia.org/wiki/web_browser) engine, placed engineers on the project including Patrick Walton, Niko Matsakis, Felix Klock, and Manish Goregaokar. A conference room taken by the project developers was dubbed "the nerd cave," with a sign placed outside the door.

Work shifted from the OCaml compiler to a [self-hosting compiler](https://en.wikipedia.org/wiki/self-hosting_compiler) (*i.e.*, written in Rust) targeting [LLVM](https://en.wikipedia.org/wiki/LLVM). The ownership system was in place by 2010. The Rust logo was developed in 2011 based on a bicycle chainring. The early 2010s witnessed increasing involvement from full-time engineers at Mozilla and open source contributors.

On January 20, 2012, Rust 0.1 became the first public release for Windows, Linux, and MacOS.

### 2012–2015: Evolution

The following years were marked by substantial changes to the Rust [type system](https://en.wikipedia.org/wiki/type_system). Memory management through the ownership system was gradually consolidated and expanded; by 2013, the [garbage collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)) was rarely used, and was removed. Also removed were typestates, the keyword, various specialized pointer types, and syntax support for [channels](https://en.wikipedia.org/wiki/channel_(programming)).

According to Steve Klabnik, the language was influenced during this period by developers coming from [C++](https://en.wikipedia.org/wiki/C++) (e.g., low-level performance of features), [scripting languages](https://en.wikipedia.org/wiki/scripting_language) (e.g., Cargo and package management), and [functional programming](https://en.wikipedia.org/wiki/functional_programming) (e.g., the type system).

In 2013, Graydon Hoare stepped down from Rust. After Hoare's departure, it evolved organically under a federated governance structure, with a "core team" of initially six people, and around 30-40 developers total across various other teams. In March 2014, a [Request for Comments](https://en.wikipedia.org/wiki/Request_for_Comments) (RFC) process for new language features was added. By 2016, the core team would grow to nine people with over 1600 RFCs.

Development focused on finalizing features for version 1.0 (which would begin providing [backward compatibility](https://en.wikipedia.org/wiki/backward_compatibility)). According to Andrew Binstock writing for *[Dr. Dobb's Journal](https://en.wikipedia.org/wiki/Dr._Dobb's_Journal)* in January 2014, while Rust was "widely viewed as a remarkably elegant language", adoption slowed because it radically changed from version to version.

On May 15, 2015, Rust 1.0 became the first [stable release](https://en.wikipedia.org/wiki/stable_release). One year later, the compiler had accumulated at least 1,400 contributors, and there were at least 5,000 third-party libraries published on the package management website Crates.io.

### 2015–2020: Servo and early adoption

 The development of the [Servo browser engine](https://en.wikipedia.org/wiki/Servo_browser_engine) continued in parallel with Rust, funded by Mozilla and [Samsung](https://en.wikipedia.org/wiki/Samsung). New features in Rust were tested out by the Servo team, and new features in Servo were used to give feedback back to the Rust team. The first version of Servo was released in 2016. The [Firefox](https://en.wikipedia.org/wiki/Firefox) web browser shipped with Rust code as of 2016 (version 45), but components of Servo did not appear in Firefox until September 2017 (version 57) as part of the [Gecko](https://en.wikipedia.org/wiki/Gecko_(software)) and [Quantum](https://en.wikipedia.org/wiki/Gecko_(software)#Quantum) projects.

In the years following 1.0, improvements were made to the toolchain ecosystem including [Rustfmt](https://en.wikipedia.org/wiki/#Rustfmt), [integrated development environment](https://en.wikipedia.org/wiki/integrated_development_environment) (IDE) support, and a regular compiler testing and release cycle. Rust's community gained a [code of conduct](https://en.wikipedia.org/wiki/code_of_conduct) and an [IRC](https://en.wikipedia.org/wiki/IRC) chat for discussion.

The earliest known adoption outside of Mozilla was by individual projects at Samsung, [Facebook](https://en.wikipedia.org/wiki/Facebook) (now [Meta Platforms](https://en.wikipedia.org/wiki/Meta_Platforms)), [Dropbox](https://en.wikipedia.org/wiki/Dropbox), and Tilde, Inc., the company behind [ember.js](https://en.wikipedia.org/wiki/ember.js). [Amazon Web Services](https://en.wikipedia.org/wiki/Amazon_Web_Services) followed in 2020. Engineers cited performance, lack of a garbage collector, safety, and developer experience as reasons for the adoption. Amazon developers cited a 2017 research finding that Rust code was more [energy efficient](https://en.wikipedia.org/wiki/Energy_efficiency_in_computing) compared to similar [Java](https://en.wikipedia.org/wiki/Java_(programming_language)) code.

### 2020–present: Mozilla layoffs and Rust Foundation

In August 2020, Mozilla laid off 250 of its 1,000 employees as part of a corporate restructuring during the [COVID-19 pandemic](https://en.wikipedia.org/wiki/COVID-19_pandemic). The Servo team was disbanded. The event raised concerns about the future of Rust. The Rust Core Team acknowledged the impact of the layoffs and announced plans for an independent foundation. Its first goal would be to own and take financial responsibility for [trademarks](https://en.wikipedia.org/wiki/trademark) and [domain names](https://en.wikipedia.org/wiki/domain_name).

On February 8, 2021, the [Rust Foundation](https://en.wikipedia.org/wiki/#Rust_Foundation) was formed by founding companies [Amazon Web Services](https://en.wikipedia.org/wiki/Amazon_Web_Services), [Google](https://en.wikipedia.org/wiki/Google), [Huawei](https://en.wikipedia.org/wiki/Huawei), [Microsoft](https://en.wikipedia.org/wiki/Microsoft), and [Mozilla](https://en.wikipedia.org/wiki/Mozilla). The foundation would support developers financially through grants and server funding. In an April 6, 2021 blog post, Google announced support for Rust within the [Android Open Source Project](https://en.wikipedia.org/wiki/Android_Open_Source_Project). Facebook ([Meta Platforms](https://en.wikipedia.org/wiki/Meta_Platforms)) joined the foundation in April 2021, followed by ARM ([Arm Holdings](https://en.wikipedia.org/wiki/Arm_Holdings)) in November.

On November 22, 2021, the Moderation Team, responsible for enforcing the community code of conduct, resigned "in protest of the Core Team placing themselves unaccountable to anyone but themselves". In May 2022, the Rust leadership council published a response to the incident.

The Rust Foundation posted a draft for a new trademark policy on April 6, 2023 that resulted in widespread negative reactions. The trademark policy included rules for how the Rust logo and name could be used.

In June 2026, [OpenAI](https://en.wikipedia.org/wiki/OpenAI) joined the Rust Foundation.

## Syntax and features

 Rust's [syntax](https://en.wikipedia.org/wiki/Syntax_(programming_languages)) is similar to that of [C](https://en.wikipedia.org/wiki/C_(programming_language)) and [C++](https://en.wikipedia.org/wiki/C++), although many of its features were influenced by [functional programming](https://en.wikipedia.org/wiki/functional_programming) languages such as [OCaml](https://en.wikipedia.org/wiki/OCaml). Hoare has described Rust as targeted at frustrated C++ developers.

### Hello World program

Below is a ["Hello, World!" program](https://en.wikipedia.org/wiki/%22Hello,_World!%22_program) in Rust. The keyword denotes a [function](https://en.wikipedia.org/wiki/Function_(computer_programming)), and the [macro](https://en.wikipedia.org/wiki/Macro_(computer_science)) (see ) prints the message to [standard output](https://en.wikipedia.org/wiki/standard_output). [Statements](https://en.wikipedia.org/wiki/Statement_(computer_science)) in Rust are separated by [semicolons](https://en.wikipedia.org/wiki/Semicolon#Programming).
```

fn main() {
    println!("Hello, World!");
}
```

### Variables

[Variables](https://en.wikipedia.org/wiki/Variable_(computer_science)) in Rust are defined through the keyword. The example below assigns a value to the variable with name of type and outputs its value; the type annotation can be omitted.
```

fn main() {
    let foo: i32 = 10;
    println!("The value of foo is {foo}");
}
```

Variables are [immutable](https://en.wikipedia.org/wiki/immutable) by default, unless the keyword is added. The following example uses , which denotes the start of a [comment](https://en.wikipedia.org/wiki/Comment_(computer_programming)).
```

fn main() {
    // This code would not compile without adding "mut".
    let mut foo = 10;
    println!("The value of foo is {foo}");
    foo = 20;
    println!("The value of foo is {foo}");
}
```

Multiple expressions can define multiple variables with the same name, known as [variable shadowing](https://en.wikipedia.org/wiki/variable_shadowing). Variable shadowing allows transforming variables without having to name the variables differently. The example below declares a new variable with the same name that is double the original value:
```

fn main() {
    let foo = 10;
    // This will output "The value of foo is 10"
    println!("The value of foo is {foo}");
    let foo = foo * 2;
    // This will output "The value of foo is 20"
    println!("The value of foo is {foo}");
}
```

Variable shadowing is also possible for values of different types. For example, going from a string to its length:
```

fn main() {
    let letters = "abc";
    let letters = letters.len();
}
```

### Block expressions and control flow

A *block expression* is delimited by [curly brackets](https://en.wikipedia.org/wiki/Bracket#Curly_brackets). When the last expression inside a block does not end with a semicolon, the block evaluates to the value of that trailing expression:
```

fn main() {
    let x = {
        println!("this is inside the block");
        1 + 2
    };
    println!("1 + 2 = {x}");
}
```

Trailing expressions of function bodies are used as the return value:
```

fn add_two(x: i32) -> i32 {
    x + 2
}
```

#### expressions

An [conditional expression](https://en.wikipedia.org/wiki/conditional_expression) executes code based on whether the given value is . can be used for when the value evaluates to , and can be used for combining multiple expressions.
```

fn main() {
    let x = 10;
    if x > 5 {
        println!("value is greater than five");
    }

    if x % 7 == 0 {
        println!("value is divisible by 7");
    } else if x % 5 == 0 {
        println!("value is divisible by 5");
    } else {
        println!("value is not divisible by 7 or 5");
    }
}
```

 and blocks can evaluate to a value, which can then be assigned to a variable:
```

fn main() {
    let x = 10;
    let new_x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    println!("{new_x}");
}
```

#### loops

`[while](https://en.wikipedia.org/wiki/While_loop)` can be used to repeat a block of code while a condition is met.
```

fn main() {
    // Iterate over all integers from 4 to 10
    let mut value = 4;
    while value <= 10 {
         println!("value = {value}");
         value += 1;
    }
}
```

#### loops and iterators

[For loops](https://en.wikipedia.org/wiki/For_loop) in Rust loop over elements of a collection. expressions work over any [iterator](https://en.wikipedia.org/wiki/iterator) type.
```

fn main() {
    // Using `for` with range syntax for the same functionality as above
    // The syntax 4..=10 means the range from 4 to 10, up to and including 10.
    for value in 4..=10 {
        println!("value = {value}");
    }
}
```

In the above code, is a value of type that implements the trait. The code within the curly braces is applied to each element returned by the iterator.

Iterators can be combined with functions over iterators like , , and . For example, the following adds up all numbers between 1 and 100 that are multiples of 3:
```

(1..=100).filter(|x| x % 3 == 0).sum()
```

#### and statements

More generally, the keyword allows repeating a portion of code until a occurs. may optionally exit the loop with a value. In the case of nested loops, labels denoted by can be used to break an outer loop rather than the innermost loop.
```

fn main() {
    let value = 456;
    let mut x = 1;
    let y = loop {
        x *= 10;
        if x > value {
            break x / 10;
        }
    };
    println!("largest power of ten that is smaller than or equal to value: {y}");

    let mut up = 1;
    'outer: loop {
        let mut down = 120;
        loop {
            if up > 100 {
                break 'outer;
            }

            if down < 4 {
                break;
            }

            down /= 2;
            up += 1;
            println!("up: {up}, down: {down}");
        }
        up *= 2;
    }
}
```

### Pattern matching

The and expressions can be used for [pattern matching](https://en.wikipedia.org/wiki/pattern_matching). For example, can be used to double an optional integer value if present, and return zero otherwise:
```

fn double(x: Option<u64>) -> u64 {
    match x {
        Some(value) => value * 2,
        None => 0,
    }
}
```

Equivalently, this can be written with and :
```

fn double(x: Option<u64>) -> u64 {
    if let Some(value) = x {
        value * 2
    } else {
        0
    }
}
```

### Types

Rust is [strongly typed](https://en.wikipedia.org/wiki/strongly_typed) and [statically typed](https://en.wikipedia.org/wiki/statically_typed), meaning that the types of all variables must be known at compilation time. Assigning a value of a particular type to a differently typed variable causes a [compilation error](https://en.wikipedia.org/wiki/compilation_error). [Type inference](https://en.wikipedia.org/wiki/Type_inference) is used to determine the type of variables if unspecified.

The [unit type](https://en.wikipedia.org/wiki/unit_type), notated `()` in Rust, is a concrete type that has exactly one value. It occupies no memory (as it represents the absence of value). All functions that do not have an indicated return type implicitly return `()`. It is similar to in other C-style languages; however, denotes the absence of a type and cannot have any value.

The default integer type is , and the default [floating point](https://en.wikipedia.org/wiki/floating_point) type is . If the type of a [literal](https://en.wikipedia.org/wiki/Literal_(computer_programming)) number is not explicitly provided, it is either inferred from the context or the default type is used.

#### Primitive types

[Integer types](https://en.wikipedia.org/wiki/Integer_type) in Rust are named based on the [signedness](https://en.wikipedia.org/wiki/signedness) and the number of bits the type takes. For example, is a signed integer that takes 32 bits of storage, whereas is unsigned and only takes 8 bits of storage. and take storage depending on the [memory address bus width](https://en.wikipedia.org/wiki/Bus_(computing)#Address_bus) of the compilation target. For example, when building for [32-bit targets](https://en.wikipedia.org/wiki/32-bit_computing)s, both types will take up 32 bits of space.

By default, integer literals are in base-10, but different [radices](https://en.wikipedia.org/wiki/radix) are supported with prefixes, for example, for [binary numbers](https://en.wikipedia.org/wiki/binary_number), for [octals](https://en.wikipedia.org/wiki/octal), and for [hexadecimals](https://en.wikipedia.org/wiki/hexadecimal). By default, integer literals default to as its type. Suffixes such as can be used to explicitly set the type of a literal. Byte literals such as are available to represent the [ASCII](https://en.wikipedia.org/wiki/ASCII) value (as a ) of a specific character.

The [Boolean type](https://en.wikipedia.org/wiki/Boolean_type) is referred to as which can take a value of either or . A takes up 32 bits of space and represents a Unicode scalar value: a [Unicode codepoint](https://en.wikipedia.org/wiki/Unicode_codepoint) that is not a [surrogate](https://en.wikipedia.org/wiki/Universal_Character_Set_characters#Surrogates). [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floating point numbers are supported with for [single precision floats](https://en.wikipedia.org/wiki/single_precision_float) and for [double precision floats](https://en.wikipedia.org/wiki/double_precision_float).

#### Compound types

Compound types can contain multiple values. Tuples are fixed-size lists that can contain values whose types can be different. Arrays are fixed-size lists whose values are of the same type. Expressions of the tuple and array types can be written through listing the values, and can be accessed with (with tuples) or (with arrays):
```

let tuple: (u32, bool) = (3, true);
let array: [i8; 5] = [1, 2, 3, 4, 5];
let value = tuple.1; // true
let value = array[2]; // 3
```

Arrays can also be constructed through copying a single value a number of times:
```

let array2: [char; 10] = [' '; 10];
```

### Ownership and references

Rust's ownership system consists of rules that ensure memory safety without using a garbage collector. At compile time, each value must be attached to a variable called the *owner* of that value, and every value must have exactly one owner. Values are moved between different owners through assignment or passing a value as a function parameter. Values can also be *borrowed,* meaning they are temporarily passed to a different function before being returned to the owner. With these rules, Rust can prevent the creation and use of [dangling pointers](https://en.wikipedia.org/wiki/dangling_pointers):
```

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(s); // s consumed by print_string
    // s has been moved, so cannot be used any more
    // another print_string(s); would result in a compile error
}
```

The function takes ownership over the value passed in; Alternatively, can be used to indicate a [reference](https://en.wikipedia.org/wiki/Reference_(computer_science)) type (in ) and to create a reference (in ):
```

fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(&s); // s borrowed by print_string
    print_string(&s); // s has not been consumed; we can call the function many times
}
```

Because of these ownership rules, Rust types are known as *[affine types](https://en.wikipedia.org/wiki/affine_type)*, meaning each value may be used at most once. This enforces a form of [software fault isolation](https://en.wikipedia.org/wiki/software_fault_isolation) as the owner of a value is solely responsible for its correctness and deallocation.

When a value goes out of scope, it is *dropped* by running its [destructor](https://en.wikipedia.org/wiki/Destructor_(computer_programming)). The destructor may be programmatically defined through implementing the [trait](https://en.wikipedia.org/wiki/#Traits). This helps manage resources such as file handles, network sockets, and [locks](https://en.wikipedia.org/wiki/Lock_(computer_science)), since when objects are dropped, the resources associated with them are closed or released automatically.

#### Lifetimes

[Object lifetime](https://en.wikipedia.org/wiki/Object_lifetime) refers to the period of time during which a reference is valid; that is, the time between the object creation and destruction. These *lifetimes* are implicitly associated with all Rust reference types. While often inferred, they can also be indicated explicitly with named lifetime parameters (often denoted , , and so on).

A value's lifetime in Rust is inferred from the set of locations in the source code (i.e., function, line, and column numbers) for which a variable is valid. For example, a reference to a local variable has a lifetime from the expression it is declared in up until the last use of it.
```

fn main() {
    let mut x = 5;            // ------------------+- Lifetime 'a
                              //                   |
    let r = &x;               // -+-- Lifetime 'b  |
                              //  |                |
    println!("r: {}", r);     // -+                |
    // Since r is no longer used,                  |
    // its lifetime ends                           |
    let r2 = &mut x;          // -+-- Lifetime 'c  |
}                             // ------------------+
```

The borrow checker in the Rust compiler then enforces that references are only used in the locations of the source code where the associated lifetime is valid. In the example above, storing a reference to variable in is valid, as variable has a longer lifetime () than variable (). However, when has a shorter lifetime, the borrow checker would reject the program:
```

fn main() {
    let r;                    // ------------------+- Lifetime 'a
                              //                   |
    {                         //                   |
        let x = 5;            // -+-- Lifetime 'b  |
        r = &x; // ERROR: x does  |                |
    }           // not live long -|                |
                // enough                          |
    println!("r: {}", r);     //                   |
}                             // ------------------+
```

Since the lifetime of the referenced variable () is shorter than the lifetime of the variable holding the reference (), the borrow checker errors, preventing from being used from outside its scope.

Lifetimes can be indicated using explicit *lifetime parameters* on function arguments. For example, the following code specifies that the reference returned by the function has the same lifetime as (and *not* necessarily the same lifetime as ):
```

fn remove_prefix<'a>(mut original: &'a str, prefix: &str) -> &'a str {
    if original.starts_with(prefix) {
        original = original[prefix.len()..];
    }
    original
}
```

In the compiler, ownership and lifetimes work together to prevent memory safety issues such as dangling pointers.

### User-defined types

User-defined types are created with the or keywords. The keyword is used to denote a [record type](https://en.wikipedia.org/wiki/Record_(computer_science)) that groups multiple related values. s can take on different variants at runtime, with their capabilities similar to [algebraic data types](https://en.wikipedia.org/wiki/algebraic_data_types) found in functional programming languages. Both records and enum variants can contain [fields](https://en.wikipedia.org/wiki/Field_(computer_science)) with different types. Alternative names, or aliases, for the same type can be defined with the keyword.

The keyword can define methods for a user-defined type. Data and functions are defined separately. Implementations fulfill a role similar to that of [classes](https://en.wikipedia.org/wiki/Class_(programming)) within other languages.

#### Standard library

 The Rust [standard library](https://en.wikipedia.org/wiki/standard_library) defines and implements many widely used custom data types, including core data structures such as , , and , as well as [smart pointer](https://en.wikipedia.org/wiki/smart_pointer) types. Rust provides a way to exclude most of the standard library using the attribute , for applications such as embedded devices. Internally, the standard library is divided into three parts, , , and , where and are excluded by .

Rust uses the [option type](https://en.wikipedia.org/wiki/option_type) `Option<T>` to define optional values, which can be matched using `if let` or `match` to access the inner value:
```

fn main() {
    let name1: Option<&str> = None;
    // In this case, nothing will be printed out
    if let Some(name) = name1 {
        println!("{name}");
    }

    let name2: Option<&str> = Some("Matthew");
    // In this case, the word "Matthew" will be printed out
    if let Some(name) = name2 {
        println!("{name}");
    }
}
```

Similarly, Rust's [result type](https://en.wikipedia.org/wiki/result_type) `Result<T, E>` holds either a successfully computed value (the `Ok` variant) or an error (the `Err` variant). Like `Option`, the use of `Result` means that the inner value cannot be used directly; programmers must use a `match` expression, syntactic sugar such as `?` (the "try" operator), or an explicit `unwrap` assertion to access it. Both `Option` and `Result` are used throughout the standard library and are a fundamental part of Rust's explicit approach to handling errors and missing data.

### Pointers

The `&` and reference types are guaranteed to not be null and point to valid memory. The raw pointer types and opt out of the safety guarantees, thus they may be null or invalid; however, it is impossible to dereference them unless the code is explicitly declared unsafe through the use of an block. Unlike dereferencing, the creation of raw pointers is allowed inside safe Rust code.

### Type conversion

### Polymorphism

Rust supports [polymorphism](https://en.wikipedia.org/wiki/Polymorphism_(computer_science)) through [traits](https://en.wikipedia.org/wiki/Trait_(computer_programming)), [generic functions](https://en.wikipedia.org/wiki/generic_function), and [trait objects](https://en.wikipedia.org/wiki/Trait_object_(Rust)).

#### Traits

Common behavior between types is declared using traits and blocks:
```

trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Zero::zero()
    }
}

impl Zero for u32 {
    fn zero() -> u32 { 0 }
}

impl Zero for f32 {
    fn zero() -> Self { 0.0 }
}
```

The example above includes a method that provides a default implementation that may be overridden when implementing the trait.

#### Generic functions

A function can be made generic by adding type parameters inside angle brackets (), which only allow types that implement the trait:
```

// zero is a generic function with one type parameter, Num
fn zero<Num: Zero>() -> Num {
    Num::zero()
}

fn main() {
    let a: u32 = zero();
    let b: f32 = zero();
    assert!(a.is_zero() && b.is_zero());
}
```

In the examples above, as well as are trait bounds that constrain the type to only allow types that implement or . Within a trait or impl, refers to the type that the code is implementing.

Generics can be used in functions to allow implementing a behavior for different types without repeating the same code (see [bounded parametric polymorphism](https://en.wikipedia.org/wiki/bounded_parametric_polymorphism)). Generic functions can be written in relation to other generics, without knowing the actual type.

#### Trait objects

By default, traits use [static dispatch](https://en.wikipedia.org/wiki/static_dispatch): the compiler [monomorphizes](https://en.wikipedia.org/wiki/Monomorphization) the function for each concrete type instance, yielding performance equivalent to type-specific code at the cost of longer compile times and larger binaries.

When the exact type is not known at compile time, Rust provides [trait objects](https://en.wikipedia.org/wiki/Trait_object_(Rust)) `&dyn Trait` and `Box<dyn Trait>`. Trait object calls use [dynamic dispatch](https://en.wikipedia.org/wiki/dynamic_dispatch) via a lookup table; a trait object is a "fat pointer" carrying both a data pointer and a method table pointer. This indirection adds a small runtime cost, but it keeps a single copy of the code and reduces binary size. Only "object-safe" traits are eligible to be used as trait objects.

This approach is similar to [duck typing](https://en.wikipedia.org/wiki/duck_typing), where all data types that implement a given trait can be treated as functionally interchangeable. The following example creates a list of objects where each object implements the `Display` trait:
```

use std::fmt::Display;

let v: Vec<Box<dyn Display>> = vec![
    Box::new(3),
    Box::new(5.0),
    Box::new("hi"),
];

for x in v {
    println!("{x}");
}
```

If an element in the list does not implement the `Display` trait, it will cause a compile-time error.

### Memory management

Rust does not use [garbage collection](https://en.wikipedia.org/wiki/garbage_collection_(computer_science)). Memory and other resources are instead managed through the "resource acquisition is initialization" convention, with optional [reference counting](https://en.wikipedia.org/wiki/reference_counting). Rust provides deterministic management of resources, with very low [overhead](https://en.wikipedia.org/wiki/Overhead_(computing)). Values are [allocated on the stack](https://en.wikipedia.org/wiki/Stack-based_memory_allocation) by default, and all [dynamic allocations](https://en.wikipedia.org/wiki/dynamic_allocation) must be explicit.

The built-in reference types using the `&` symbol do not involve run-time reference counting. The safety and validity of the underlying pointers are verified at compile time, preventing [dangling pointers](https://en.wikipedia.org/wiki/dangling_pointers) and other forms of [undefined behavior](https://en.wikipedia.org/wiki/undefined_behavior). Rust's type system separates shared, [immutable](https://en.wikipedia.org/wiki/immutable) references of the form `&T` from unique, mutable references of the form `&mut T`. A mutable reference can be coerced to an immutable reference, but not vice versa.

### Unsafe

Rust's memory safety checks (See [#Safety](https://en.wikipedia.org/wiki/#Safety)) may be circumvented through the use of blocks. This allows programmers to dereference arbitrary raw pointers, call external code, or perform other low-level functionality not allowed by safe Rust. Some low-level functionality enabled in this way includes [volatile memory access](https://en.wikipedia.org/wiki/Volatile_(computer_programming)), architecture-specific intrinsics, [type punning](https://en.wikipedia.org/wiki/type_punning), and inline assembly.

Unsafe code is needed, for example, in the implementation of data structures. A frequently cited example is that it is difficult or impossible to implement [doubly linked lists](https://en.wikipedia.org/wiki/doubly_linked_list) in safe Rust.

Programmers using unsafe Rust are considered responsible for upholding Rust's memory and type safety requirements, for example, that no two mutable references exist pointing to the same location. If programmers write code that violates these requirements, this results in [undefined behavior](https://en.wikipedia.org/wiki/undefined_behavior). The Rust documentation includes a list of behaviors considered undefined, including accessing dangling or misaligned pointers, or breaking the aliasing rules for references.

### Macros

Macros allow the generation and transformation of Rust code to reduce repetition. Macros come in two forms, with *declarative macros* defined through `macro_rules!`, and *procedural macros*, which are defined in separate crates.

#### Declarative macros

A declarative macro (also called a "macro by example") is a macro, defined using the `macro_rules!` keyword, that uses pattern matching to determine its expansion. Below is an example that sums over all its arguments:
```
macro_rules! sum {
    ( $initial:expr $(, $expr:expr )* $(,)? ) => {
        $initial $(+ $expr)*
    }
}

fn main() {
    let x = sum!(1, 2, 3);
    println!("{x}"); // prints 6
}
```

 In this example, the macro named `sum` is defined using the form `macro_rules! sum {` `(...) => { ... } }`. The first part inside the parentheses of the definition, the macro pattern `( $initial:expr $(, $expr:expr )* $(,)? )` specifies the structure of input it can take. Here, `$initial:expr` represents the first expression, while `$(, $expr:expr )*` means there can be zero or more additional comma-separated expressions after it. The trailing `$(,)?` allows the caller to optionally include a final comma without causing an error. The second part, after the arrow `=>`, describes what code will be generated when the macro is invoked. In this case, `$initial $(+ $expr)*` means that the generated code will start with the first expression, followed by a `+` and each of the additional expressions in sequence. The `*` again means "repeat this pattern zero or more times". This means, when the macro is later called in line 8, as `sum!(1, 2, 3)`, the macro will resolve to `1 + 2 + 3` representing the addition of all of the passed expressions.

#### Procedural macros

Procedural macros are Rust functions that run and modify the compiler's input [token](https://en.wikipedia.org/wiki/Token_(parser)) stream, before any other components are compiled. They are generally more flexible than declarative macros, but are more difficult to maintain due to their complexity.

Procedural macros come in three flavors:
- Function-like macros `custom!(...)`
- Derive macros `#[derive(CustomDerive)]`
- Attribute macros `#[custom_attribute]`

### Interface with C and C++

Rust supports the creation of [foreign function interfaces](https://en.wikipedia.org/wiki/foreign_function_interface) (FFI) through the keyword. A function that uses the C [calling convention](https://en.wikipedia.org/wiki/calling_convention) can be written using . Symbols can be exported from Rust to other languages through the attribute, and symbols can be imported into Rust through blocks:
```

#[unsafe(no_mangle)]
pub extern "C" fn exported_from_rust(x: i32) -> i32 { x + 1 }
unsafe extern "C" {
    fn imported_into_rust(x: i32) -> i32;
}
```

The attribute enables deterministic memory layouts for s and s for use across FFI boundaries. External libraries such as and can generate Rust bindings for C/C++.

## Safety

[Safety properties](https://en.wikipedia.org/wiki/Safety_properties) guaranteed by Rust include [memory safety](https://en.wikipedia.org/wiki/memory_safety), [type safety](https://en.wikipedia.org/wiki/type_safety), and [data race](https://en.wikipedia.org/wiki/data_race) freedom. As described above, these guarantees can be circumvented by using the keyword.

Memory safety includes the absence of dereferences to [null](https://en.wikipedia.org/wiki/null_pointer), [dangling](https://en.wikipedia.org/wiki/dangling_pointer), and misaligned [pointers](https://en.wikipedia.org/wiki/Pointer_(computer_programming)), and the absence of [buffer overflows](https://en.wikipedia.org/wiki/buffer_overflow) and [double free](https://en.wikipedia.org/wiki/double_free) errors.

[Memory leaks](https://en.wikipedia.org/wiki/Memory_leak) are possible in safe Rust. Memory leaks may occur as a result of creating reference-counted pointers that point at each other (a reference cycle) or can be deliberately created through calling .

## Ecosystem

The Rust ecosystem includes its compiler, [standard library](https://en.wikipedia.org/wiki/#Standard_library), and other software development [toolchain](https://en.wikipedia.org/wiki/toolchain) components. Component installation is typically managed by , a Rust [toolchain](https://en.wikipedia.org/wiki/toolchain) installer.

### Compiler

The [Rust compiler](https://en.wikipedia.org/wiki/Rust_compiler), , compiles Rust code into [executables](https://en.wikipedia.org/wiki/executable). Source code is parsed as an [abstract syntax tree](https://en.wikipedia.org/wiki/abstract_syntax_tree) (AST) and lowered through multiple [intermediate representations](https://en.wikipedia.org/wiki/intermediate_representation) (IRs). Next, a compiler backend is invoked to apply [optimizations](https://en.wikipedia.org/wiki/Optimizing_compiler) and produce [object code](https://en.wikipedia.org/wiki/object_code), and a [linker](https://en.wikipedia.org/wiki/Linker_(computing)) combines the object(s) into a single binary executable.

The compiler uses [LLVM](https://en.wikipedia.org/wiki/LLVM) as its default backend, and supports alternatives such as [GCC](https://en.wikipedia.org/wiki/GNU_Compiler_Collection) and [Cranelift](https://en.wikipedia.org/wiki/Cranelift). The intention of those alternative backends is to increase platform coverage of Rust or to improve compilation times.

### Cargo

 Cargo is Rust's [build system](https://en.wikipedia.org/wiki/Build_system_(software_development)) and [package manager](https://en.wikipedia.org/wiki/package_manager). It downloads, compiles, distributes, and uploads packages—called *crates*—that are maintained in an official registry. It also acts as a front-end for Clippy and other Rust components.

By default, Cargo sources its dependencies from the user-contributed registry *crates.io*. [Git](https://en.wikipedia.org/wiki/Git) repositories, folders on the local filesystem, and other external sources can also be specified as dependencies.

Cargo supports reproducible builds through two metadata files: Cargo.toml and Cargo.lock. Cargo.toml declares each package used and its version requirements. Cargo.lock is generated automatically during dependency resolution and records exact versions of all dependencies, including [transitive dependencies](https://en.wikipedia.org/wiki/Transitive_dependency).

### Rustfmt

Rustfmt is a [code formatter](https://en.wikipedia.org/wiki/code_formatter) for Rust. It formats whitespace and [indentation](https://en.wikipedia.org/wiki/Indentation_style) to produce code in accordance with a common [programming style](https://en.wikipedia.org/wiki/programming_style). It can be invoked as a standalone program or through Cargo.

### Clippy

 Clippy is Rust's built-in [linting](https://en.wikipedia.org/wiki/linting) tool to improve the correctness, performance, and readability of Rust code. , it has over 800 rules.

### Versioning system

Following Rust 1.0, new features are developed in *nightly* versions, released daily. During each six-week release cycle, changes to nightly versions are released to beta, while changes from the previous beta version are released to a new stable version.

Every two or three years, a new "edition" is produced. Editions are released to allow making limited [breaking changes](https://en.wikipedia.org/wiki/breaking_changes), such as promoting to a keyword to support [async/await](https://en.wikipedia.org/wiki/async/await) features. Crates targeting different editions can interoperate with each other, so a crate can upgrade to a new edition even if its callers or its dependencies still target older editions. Migration to a new edition can be assisted with automated tooling.

### IDE support

*rust-analyzer* is a set of [utilities](https://en.wikipedia.org/wiki/Utility_software) that provides [integrated development environments](https://en.wikipedia.org/wiki/integrated_development_environment) (IDEs) and [text editors](https://en.wikipedia.org/wiki/text_editor) with information about a Rust project through the [Language Server Protocol](https://en.wikipedia.org/wiki/Language_Server_Protocol). Among other features, this is used for [autocomplete](https://en.wikipedia.org/wiki/autocomplete) and displaying [compilation errors](https://en.wikipedia.org/wiki/compilation_error) during editing.

## Performance

Lacking garbage collection, Rust is often faster than other memory-safe languages. Most of Rust's memory safety guarantees impose no runtime overhead, with the notable exception of [array indexing](https://en.wikipedia.org/wiki/Array_(data_structure)), which is checked at runtime by default. The performance impact of array indexing bounds checks varies, but can be significant in some cases.

Many of Rust's features are *zero-cost abstractions*, meaning they are optimized away at compile time and incur no runtime overhead. The ownership and borrowing system permits [zero-copy](https://en.wikipedia.org/wiki/zero-copy) implementations for some performance-sensitive tasks, such as [parsing](https://en.wikipedia.org/wiki/parsing). [Static dispatch](https://en.wikipedia.org/wiki/Static_dispatch) is used by default to eliminate [method calls](https://en.wikipedia.org/wiki/method_call), except for methods called on dynamic trait objects. The compiler uses [inline expansion](https://en.wikipedia.org/wiki/inline_expansion) to eliminate [function calls](https://en.wikipedia.org/wiki/function_call) and statically dispatched method invocations.

Unlike in C and C++, the Rust compiler may reorder struct and enum elements unless a `#[repr(C)]` representation attribute is applied. This can produce more efficient code in some cases.

Performance improvements in [LLVM](https://en.wikipedia.org/wiki/LLVM) carry over to Rust using the default backend.

## Adoption

    In [web services](https://en.wikipedia.org/wiki/web_service), [OpenDNS](https://en.wikipedia.org/wiki/OpenDNS), a [DNS](https://en.wikipedia.org/wiki/DNS) resolution service owned by [Cisco](https://en.wikipedia.org/wiki/Cisco), uses Rust internally. [Amazon Web Services](https://en.wikipedia.org/wiki/Amazon_Web_Services) uses Rust in "performance-sensitive components" of services. In 2019, AWS converted [Firecracker](https://en.wikipedia.org/wiki/Firecracker_(software)), a virtualization solution primarily written in Rust, to [open source](https://en.wikipedia.org/wiki/open_sourced). [Microsoft Azure](https://en.wikipedia.org/wiki/Microsoft_Azure) IoT Edge, a platform used to run Azure services on [IoT](https://en.wikipedia.org/wiki/IoT) devices, has components implemented in Rust. Microsoft also uses Rust to run containerized modules with [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) and [Kubernetes](https://en.wikipedia.org/wiki/Kubernetes). [Cloudflare](https://en.wikipedia.org/wiki/Cloudflare), a company providing [content delivery network](https://en.wikipedia.org/wiki/content_delivery_network) services, used Rust to build a new [web proxy](https://en.wikipedia.org/wiki/web_proxy) named Pingora for increased performance and efficiency. The [npm package manager](https://en.wikipedia.org/wiki/npm) used Rust for its production authentication service in 2019.

In operating systems, the Linux kernel began introducing experimental support for Rust code in Version 6.1 in late 2022, as part of the [Rust for Linux](https://en.wikipedia.org/wiki/Rust_for_Linux) project. The first drivers written in Rust were included in version 6.8. In 2025, kernel developers at the [Linux Kernel Developers Summit](https://en.wikipedia.org/wiki/Linux_Kernel_Developers_Summit) determined the project to be a success, and Rust usage for kernel code will no longer be considered experimental. The [Android](https://en.wikipedia.org/wiki/Android_(operating_system)) developers used Rust in 2021 to rewrite existing components. [Microsoft](https://en.wikipedia.org/wiki/Microsoft) has rewritten parts of [Windows](https://en.wikipedia.org/wiki/Windows) in Rust. The r9 project aims to re-implement [Plan 9 from Bell Labs](https://en.wikipedia.org/wiki/Plan_9_from_Bell_Labs) in Rust. Rust has also been used in the development of new operating systems such as [Redox](https://en.wikipedia.org/wiki/Redox_(operating_system)), a "Unix-like" operating system and [microkernel](https://en.wikipedia.org/wiki/microkernel), Theseus, an experimental operating system with modular state management, and most of [Fuchsia](https://en.wikipedia.org/wiki/Fuchsia_(operating_system)). Rust is used for command-line tools and operating system components such as [stratisd](https://en.wikipedia.org/wiki/Stratis_(configuration_daemon)), a [file system](https://en.wikipedia.org/wiki/file_system) manager and COSMIC, a [desktop environment](https://en.wikipedia.org/wiki/desktop_environment) by [System76](https://en.wikipedia.org/wiki/System76).

In web development, [Deno](https://en.wikipedia.org/wiki/Deno_(software)), a secure runtime for [JavaScript](https://en.wikipedia.org/wiki/JavaScript) and [TypeScript](https://en.wikipedia.org/wiki/TypeScript), is built on top of [V8](https://en.wikipedia.org/wiki/V8_(JavaScript_engine)) using Rust and Tokio. Other notable adoptions in this space include [Ruffle](https://en.wikipedia.org/wiki/Ruffle_(software)), an open-source [SWF](https://en.wikipedia.org/wiki/SWF) emulator, and [Polkadot](https://en.wikipedia.org/wiki/Polkadot_(cryptocurrency)), an open source [blockchain](https://en.wikipedia.org/wiki/blockchain) and [cryptocurrency](https://en.wikipedia.org/wiki/cryptocurrency) platform. Components from the Servo browser engine (funded by [Mozilla](https://en.wikipedia.org/wiki/Mozilla) and [Samsung](https://en.wikipedia.org/wiki/Samsung)) were incorporated in the [Gecko](https://en.wikipedia.org/wiki/Gecko_(software)) browser engine underlying [Firefox](https://en.wikipedia.org/wiki/Firefox). In January 2023, Google ([Alphabet](https://en.wikipedia.org/wiki/Alphabet_Inc.)) announced support for using third party Rust libraries in [Chromium](https://en.wikipedia.org/wiki/Chromium_(web_browser)).

In other uses, [Discord](https://en.wikipedia.org/wiki/Discord), an [instant messaging](https://en.wikipedia.org/wiki/instant_messaging) software company, rewrote parts of its system in Rust for increased performance in 2020. In the same year, Dropbox announced that its [file synchronization](https://en.wikipedia.org/wiki/file_synchronization) had been rewritten in Rust. [Facebook](https://en.wikipedia.org/wiki/Facebook) ([Meta](https://en.wikipedia.org/wiki/Meta_Platforms)) used Rust to redesign its system that manages source code for internal projects.

In the 2025 [Stack Overflow](https://en.wikipedia.org/wiki/Stack_Overflow) Developer Survey, 14.8% of respondents had recently done extensive development in Rust. The survey named Rust the "most admired programming language" annually from 2016 to 2025 (inclusive), as measured by the number of existing developers interested in continuing to work in the language. In 2025, 29.2% of developers not currently working in Rust expressed an interest in doing so.

In the [TIOBE programming community index](https://en.wikipedia.org/wiki/TIOBE_programming_community_index), which measures search-engine-derived interest in programming languages, Rust ranked in the top 10 for the first time in July 2026, with a rating of 1.34%.

## In academic research

Rust's safety and performance have been the topic of programming languages research.

Outside of computer science, Rust has been investigated for its use in scientific software. A journal article published to *[Proceedings of the International Astronomical Union](https://en.wikipedia.org/wiki/Proceedings_of_the_International_Astronomical_Union)* reported on using Rust to simulate multi-planet systems. An article published in *[Nature](https://en.wikipedia.org/wiki/Nature_(journal))* shared stories of bioinformaticians using Rust.

[Formal verification](https://en.wikipedia.org/wiki/Formal_verification) tools have been developed for Rust, the most notable of which are Aeneas, Creusot, Flux, Kani, Prusti, RustHornBelt, and [Verus](https://en.wikipedia.org/wiki/Verus_(proof_system)).

The 2025 [DARPA](https://en.wikipedia.org/wiki/DARPA) project TRACTOR aims to automatically translate C to Rust using techniques such as static analysis, dynamic analysis, and large language models.

## Community

According to the *[MIT Technology Review](https://en.wikipedia.org/wiki/MIT_Technology_Review)*, the Rust community has been seen as "unusually friendly" to newcomers and particularly attracted people from the [queer community](https://en.wikipedia.org/wiki/queer_community), partly due to its [code of conduct](https://en.wikipedia.org/wiki/code_of_conduct). Inclusiveness has been cited as an important factor for some developers. The official blog collects and publishes demographic data each year.

### Rust Foundation

The **Rust Foundation** is a non-profit [membership organization](https://en.wikipedia.org/wiki/membership_organization) incorporated in [United States](https://en.wikipedia.org/wiki/United_States); it manages the Rust trademark, infrastructure, and assets.

It was established on February 8, 2021. The foundation's board was chaired by Shane Miller, with Ashley Williams as interim executive director. In late 2021, Rebecca Rumbul became executive director and CEO.

The foundation's website lists ARM, Amazon, Google, Huawei, Meta, Microsoft, and OpenAI as platinum members. OpenAI is the latest platinum member, having joined in June 2026.

### Governance teams

The Rust project is maintained by 8 top-level *teams* : the leadership council, compiler team, dev tools team, infrastructure team, language team, launching pad, library team, and moderation team. The leadership council oversees the project and is formed by representatives among the other teams.

## See also

- [List of programming languages](https://en.wikipedia.org/wiki/List_of_programming_languages)
- [History of programming languages](https://en.wikipedia.org/wiki/History_of_programming_languages)
- [Outline of the Rust programming language](https://en.wikipedia.org/wiki/Outline_of_the_Rust_programming_language)

## Notes

## References

### Book sources

### Others

### Further reading

## External links

- [Documentation](https://doc.rust-lang.org/stable/)
