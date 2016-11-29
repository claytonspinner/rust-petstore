# Rust Pet Store App

## Running It

    cargo run

I also roll with traces and logging (only on main for now):

    RUST_BACKTRACE=1;RUST_LOG=main=debug cargo run

## Postman collection

`postman.json` can be run with newman. It adds a pet, gets the pet, deletes the pet, then gets the pet again.

One thing to note is that when you get a pet that doesn't exist it returns `null`. This probably should be different.