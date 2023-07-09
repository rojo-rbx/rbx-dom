@ECHO OFF

IF "%1" EQU "--dry-run" (
    cargo run --bin rbx_reflector -- generate --patches patches
) ELSE (
    cargo run --bin rbx_reflector -- generate rbx_reflection_database/database.msgpack rbx_dom_lua/src/database.json --patches patches
    cargo run --bin rbx_reflector -- values rbx_dom_lua/src/allValues.json
)