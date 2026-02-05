# Release v0.8.0

## 🎉 New Features

### Formula Extraction from Spreadsheets
Extract formulas from Excel/ODS files instead of just cell values:

- **`--formulas`** flag: Extract formulas only (e.g., `=SUM(A1:A10)`, `=B1*2`)
- **`--formula-with-value`** flag: Show both cell values and formulas together in format: `VALUE<br/>fx: FORMULA`

```bash
# Extract only formulas
madato table --formulas workbook.xlsx

# Show both values and formulas
madato table --formula-with-value workbook.xlsx
```

Works with all output formats (Markdown, JSON, YAML, CSV)!

### Automatic File Type Detection
No more need to specify `-t` flag! File types are now auto-detected using:

- **Magic number detection** (file headers) - works even if files have wrong extensions
- **Content pattern analysis** - detects YAML, JSON, CSV by structure
- **Extension fallback** - uses file extension as final fallback

```bash
# Before (required -t flag)
madato table -t XLSX workbook.xlsx

# Now (auto-detected!)
madato table workbook.xlsx
```

The `-t` flag is still available to override auto-detection when needed.

## 🔧 Improvements

- Upgraded to **Rust 2024 Edition**
- Added comprehensive test suite (30 tests total)
  - 6 tests for formula extraction
  - 9 tests for file type auto-detection
  - Full integration test coverage

## 📦 Platform Support

Binaries available for:
- **Linux** (x86_64)
- **Windows** (x86_64)
- **macOS** (Intel x86_64)
- **macOS** (Apple Silicon ARM64)

## 📝 Technical Details

- Uses `infer` crate for robust file type detection via magic numbers
- Formula extraction powered by `calamine`'s `worksheet_formula()` API
- All existing features and options remain fully compatible

## 🐛 Bug Fixes

- Improved file type detection priority (JSON before YAML to avoid conflicts)
- Enhanced error messages when file type cannot be detected

## 📚 Documentation

- Updated README with new examples
- Added CLI help text for new flags
- Comprehensive test documentation

## 🙏 Breaking Changes

None! All changes are backward compatible. Existing scripts will continue to work.

---

**Full Changelog**: https://github.com/inosion/madato/compare/v0.7.1...v0.8.0
