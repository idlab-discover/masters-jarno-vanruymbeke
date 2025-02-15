Dit document zal uitleg bevatten over hoe bepaalde onderwerpen werken (zoals cargo-component, wac) om referenties hiernaar sneller te maken (i.p.v. de docs van deze tools te bekijken)

# Cargo Component
Cargo component is een subcommand voor cargo om WASM components te maken

## Dependencies
- Rust
- OpenSSL
- C toolchain

## Installatie
```shell
cargo install cargo-component --locked
```

## Wat doet het
Maakt WIT bindings (`bindings.rs`) zodat je de interfaces gespecifieerd in de WIT bestanden moet naleven (anders is rustc boos)

## Cargo.toml
Cargo component maakt bindings gebaseerd op de locatie van de wit bestanden in Cargo.toml, standaard kijkt deze in `./wit`

Indien je een ander pad wilt (vb: projectstructuur met wit, guest en host op dezelfde hoogte) kan je via deze tables (`[i.am.a.table]`)
```TOML
# Package van de API
[package.metadata.component]
package = "package:package"

# Locatie van de WIT en de naam van de world
[package.metadata.component.target]
path = "pad naar WIT file" # als het pad wit/example/world.wit is dan is de value wit/example
world = "example_world" # Naam van de world die je wilt gebruiken, want meerdere worlds mogelijk per file

# Dependency files (imports uit andere WIT files)
[package.metadata.component.target.dependencies]
"dependency:package" = { path = "pad naar dependency world folder" }