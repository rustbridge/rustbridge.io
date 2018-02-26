# Summary
[summary]: #summary

This RFC describes a restructuring of the project into a library with separate modules for errors, forms,
models and schemas.   

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
├── bin
│   └── main.rs
├── error
│   ├── test
│   │   └── mod.rs
│   └── mod.rs
├── form
│   ├── test
│   │   ├── mod.rs
│   │   ├── login.rs
│   │   └── workshop.rs
│   ├── mod.rs
│   ├── login.rs
│   └── workshop.rs
├── model
│   ├── test
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── mod.rs
│   └── user.rs
├── schema
│   ├── test
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── mod.rs
│   └── user.rs
└── lib.rs

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
+ We can make changes to individual modules without needing to make changes
  anywhere else.

## `error/test/`, `form/test/`, `model/test/`, `schema/test/` 
The `error/test/`, `form/test/`, `model/test/` and `schema/test/` directories
isolate the implementation of unit tests from the implementation of the
module.

#### Consequences
+ The unit tests stay organized
+ We know very quickly if a change made breaks an existing module.  
