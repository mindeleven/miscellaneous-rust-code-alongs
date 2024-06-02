# Rust Project: Building web APIs with Rust (beginners)

This is a code along with the Udemy Couse `Building web APIs with Rust` to get a better understanding of how to use the Rust programming language.

The purpose of this repository is to document my learning journey of the Rust programming language.

## Building web APIs with Rocket and Diesel

Coding along with Section 4 of the Udemy Couse [Building web APIs with Rust](https://www.udemy.com/course/web-dev-with-rust-rocket-diesel/) by Paris Liakos.

Create Sqlite database with `diesel setup --database-url ./database.sqlite`

Create Rustaceans table with `diesel migration generate create_rustaceans`

Running migrations with `diesel migration run --database-url=database.sqlite`

You can roll back the migration with `diesel migration revert --database-url=database.sqlite`


## Disclaimer

Code examples and comments are borrowed from the Udemy course [Building web APIs with Rust](https://www.udemy.com/course/web-dev-with-rust-rocket-diesel/).
