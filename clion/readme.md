Configuring CLion
===

 * Install Rust Plugin
 * Open the project
 * Configure `Custom Build Tools` via `Settings->Build, Execution & Deployment`
   * Use `Cargo-Release` name
   * Clean tool
 ![Clean Tool](screenshots/custom-tool-clean.png)
   * Build Tool
 ![Build Tool](screenshots/custom-tool-release.png)
 * Configure cross-toolchain via `Settings->Build, Execution & Deployment -> Toolchains`
  ![Toolchains](screenshots/toolchain.png)
 * Configure `Run Configuration`
   * Setup Target `Cargo-Release`
   * Setup GDB server
  ![Run Configuration 1](screenshots/run-config1.png)
   * Select target binary
  ![Run Configuration 2](screenshots/run-config2.png)
  * Set reasonable breakpoint
  * Start Debugger
  