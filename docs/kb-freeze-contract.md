# hope-kb Freeze Contract

Freeze before implementation:

- table names
- required columns
- Chinese token field naming
- source metadata minimum set
- snapshot version metadata
- hash generation strategy
- seed import format

## Chinese token rule

All token fields default to Chinese and carry no language suffix.

## Dual-layer model

- Content fields use Chinese main fields.
- `machine_id` stays stable and ASCII for joins, references, and import order.
- Do not add new director fields unless Day 2-3 explicitly requires them.

## Frozen metadata fields

- `snapshot_version`
- `snapshot_name`
- `content_hash`
- `hash_algo`
- `seed_import_format`
- `imported_at`

## Frozen source metadata fields

All v0.1 content records that are not pure system metadata should be able to carry:

- `source_type`
- `source_notes`
- `confidence_level`
- `last_reviewed_at`

The seed bundle may start with placeholder values for some rows, but the field set itself is frozen.

## Source register rule

- `seed/v0.1/source_register.json` is the bundle-level source registry.
- It records which external or internal source families feed which tables.
- Table-level rows may later carry more granular source metadata, but must not contradict the source register.

## Hash strategy

- Use the ordered seed bundle as the hash source.
- Keep `seed/v0.1/manifest.json` first in the import chain, but exclude the manifest itself from the bundle hash.
- Compute one bundle-level SHA-256 over the stable UTF-8 content of the seed files.
