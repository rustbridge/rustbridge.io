# Summary
[summary]: #summary

This RFC describes a restructuring of the project into a library with separate modules for errors, forms,
models and schemas following the project layout [reference](https://doc.rust-lang.org/cargo/reference/manifest.html#the-project-layout).

# Motivation
[motivation]: #motivation

## Why Restructure the Project?
This project is the website for the rustbridge project and aims to make
organizing and creating new rustbridge workshops easy.  Right now the website
content is displayed and a bare-bones implementation of the submission form for
new workshops is available but doesn't actually work.  

Right now adding new features or changing existing features is a messy process
and changing or adding one feature impacts the implementation the others.  

## Whats the Expected Outcome?
The expected outcome of the proposed changes is a well structured project that
is easy to navigate, build on and document without needing to change too much of
whats already there.  

# Detailed Explanation
[detailed-explaination]: #detail-explaination

This RFC proposes the following project structure:
```
src
├── error             # error handling
│   └── mod.rs
├── form              # input form controllers
│   ├── login.rs
│   ├── mod.rs
│   └── workshop.rs
├── model             # data models
│   ├── mod.rs
│   └── user.rs
├── schema            # database schemas (not entirely sure if we need this module)
│   ├── mod.rs
│   └── user.rs
├── lib.rs
├── main.rs
└── tests.rs
```

## Directory Structure Explanation
Using the proposed project structure we achieve high cohesion within each module
and loose coupling within the project between each module.  This lets us add or
remove modules as needed without drastically affecting the existing modules
already present.  

## `bin/`
The `bin` directory separates the actual executable from the rest of the
project.   

#### Consequences
+ How we run the server and how we implement the server can now vary
  independently.

## `error/`, `form/`, `model/`, `schema/`
The `error`, `form`, `model` and `schema` directories isolate the modules
implementation from the modules usage within the project.  

#### Consequences
+ We can make changes to the underlying business logic, while keeping the
  implementation clean.  

## `tests/`
The tests directory isolates the implementation of unit tests from the
implementation of the module.

#### Consequences
+ The unit tests stay organized.
+ We know very quickly if a change made breaks an existing module.  
