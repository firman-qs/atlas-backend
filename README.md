# TODO LIST:

## For the import process:

since this managed by admin, for right now we dont implement it and force the
correctness of the TOML files to be imported. But in the future we should
implement:

- Validation before import (duplicate codes, missing references, malformed packages).
- Better error reporting (e.g., "Question Q023 in concept electric_field failed because...").
- Import result statistics (57 questions imported, 114 options imported, etc.).
- Admin UI to upload the two TOML files and display the preview/import results.


TODO: 
- [importer] change stem to question_text 
- [importer] remove difficulty
- [domain] add code column to question table

