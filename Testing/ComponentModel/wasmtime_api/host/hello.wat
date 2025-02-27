(module
  (func $imported (import "" "imported_from_rust"))
  (func (export "answer") (result i32)
    call $imported
  )
)