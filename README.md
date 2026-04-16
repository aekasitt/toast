# Toast

![Toast banner](static/toast-banner.svg)

### Supported languages

* [English 🇬🇧](./README.md)
* [Thai 🇹🇭](./README.th.md)

### Prerequisites

- [cURL](https://curl.se) - command line tool and library for transferring data with URLs 
    <details>
      <summary> Install using Homebrew (Darwin) </summary>
      
      brew install curl
    </details>
    <details>
      <summary> Install using Advancted Package Tool (Linux and Windows Subsystem for Linux [WSL]) </summary>
    
      apt update && apt upgrade
      apt install curl
    </details>

- [Rust](https://rust-lang.org) - A language empowering everyone to build reliable and
  efficient software. 
    <details>
      <summary> Install using standalone installer (Darwin, Linux, and Windows Subsystem for Linux [WSL]) </summary>
      
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    </details>

- [Just](https://just.systems) - A command runner
    <details>
      <summary> Install using Cargo provided by Rustup </summary>
      
      cargo install just
    </details>

- [Jq](https://jqlang.org) - a lightweight and flexible command-line JSON processor. 
    <details>
      <summary> Install using Homebrew (Darwin) </summary>
      
      brew install jq
    </details>
    <details>
      <summary> Install using Advancted Package Tool (Linux and Windows Subsystem for Linux [WSL]) </summary>
    
      apt update && apt upgrade
      apt install jq
    </details>

### Getting started

We're building a CRUD application to manage a Post-It like Todo List
In order to complete the C-Create, R-Read, U-Update and D-Delete plan
for this application, users must be able to: 

- Create a new Todo that persists
- Read a list of persisted Todos
- Update the completed status of Todos
- Delete Todo from list of available Todos

## Acknowledgements

* [Essay] [Build a Single-File Rust Web API with SQLite](https://hamy.xyz/blog/2026-03_rust-webapi-db)
  by [Hamilton Greene](https://hamy.xyz)
* [Toasty, an async ORM for Rust, is now on crates.io](https://tokio.rs/blog/2026-04-03-toasty-released)
  by [tokio team](https://tokio.rs) on April 3rd, 2026.
* [EDMuzashi](https://www.f0nt.com/release/edmuzazhi) typeface
  by [อาทรเกติ์ แสงเพชร](https://www.facebook.com/ed.crub)
  nicknamed [ed_crub](https://www.f0nt.com/by/ed_crub)

## License

This project is licensed under the MIT License.
