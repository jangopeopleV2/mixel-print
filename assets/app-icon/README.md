# MIXEL Print app icon

This revision uses the exact user-supplied `mixel_core_clean_master.png` as its MIXEL brand base, with an independently drawn **printer and page** product symbol and the uppercase product name **PRINT**. The supplied brand has not been vectorized or claimed as newly authored open-source art.

## Editable composition and provenance

The supplied PNG is 1024 × 1024 RGB, SHA-256 `d968314eb375bfc4441e73fd6b4ace5db157128a0b591e10c5187532edc5b516`. Its exact bytes are embedded as a PNG data URI in the `supplied-mixel-master` image element. The `product-symbol` group adds original vectors in the 1024-space box `x=400, y=292, width=224, height=175`. Product-name outlines are centered at baseline `714`, using Inter SemiBold at size `68` with tracking `3.5`; they are paths, not a runtime font dependency.

The 512 × 512 canonical source has the exact root header `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">`, uses uniform half-scale composition, and clips only the outer white corners to `rx=104`. Keep the embedded master unchanged. Edit the independent product symbol and name-outline groups without redrawing, recoloring or replacing the supplied brand. No five-point-M geometry or old per-product jewel-tone palette is used. Current symbol strokes use `#242121`, accents `#BB181A` and white fills.

The independently editable glyph sources are delivered separately with the icon artwork. The committed canonical SVG contains the base and all vector overlays, so the existing repository generation script can regenerate its derived assets without the builder or any new font file. Changing the text outlines requires the existing licensed Inter SemiBold source and its notice; keep the original supplied PNG bytes intact.

## Files and consumers

- `pdfcraft.svg`, `pdfcraft-small.svg`: self-contained production compositions; the compact file is byte-identical.
- `pdfcraft-1024.png`: the repository’s padded macOS composition.
- `pdfcraft.icns`: macOS bundle icon; MAS packaging copies it to `Contents/Resources/MIXEL Print.icns`.
- `pdfcraft.ico`: Windows app icon.
- `hicolor/<size>/apps/ch.mixel.print.png` (16, 24, 32, 48, 64, 128, 256, 512) and `hicolor/scalable/apps/ch.mixel.print.svg`: Linux/shared runtime assets.

The separate App Store delivery is a full-square, opaque RGB 1024 PNG with a matching editable SVG. Keep this delivery outside the repository until deliberately integrated; it is distinct from a padded macOS icon. Actual dimensions and opacity must be checked after rendering. Production MAS packaging continues to consume the ICNS file.

The separate Windows PDF document icons, `pdfcraft-document.svg` and `pdfcraft-document.ico`, are unchanged. Their existing Lucide outline, PDF badge, outlined lettering, authors, `LICENSE-document.txt` terms and resource-2 association remain intact. Do not apply the app-icon brand metadata or hashes to them.

## Regenerate

Edit `pdfcraft.svg`, mirror it byte-for-byte to `pdfcraft-small.svg`, then run `packaging/icons.sh`. Preserve the exact SVG root header: existing macOS wrapper transforms, especially Effects, depend on it. The script keeps the established output paths, sizes, `resvg` rendering and macOS `iconutil` ICNS generation. ICO packing uses the script’s existing Python standard-library implementation. The approximately 824-in-1024 macOS body and transparent outer margin remain the script’s responsibility.

Render the separately delivered full-square App Store SVG directly to a 1024 PNG; do not substitute the macOS-padded image. Update current attribution metadata alongside actual generated assets. Rebind only the fourteen changed app-icon SHA-256 entries in `ATTRIBUTION.toml` from the actual delivered bytes, then regenerate `ATTRIBUTION.md` with `cargo xtask assets --write` and run `cargo xtask assets`.

## Rights and earlier artwork

The supplied MIXEL master and branded composite icons have reserved brand rights under `LICENSE-MIXEL-brand.txt` (`LicenseRef-MIXEL-Brand`). Their use for these seven applications was requested by the project owner; no general public right to reuse or relicense the brand is asserted. The independently drawn product-symbol vector layers alone are original MIXEL contributor work under MIT OR Apache-2.0; see the scoped addendum in `LICENSE.txt` and the repository licence texts. Product-name paths come from the existing Inter SemiBold font; the complete `OFL-Inter.txt` is included. No new font binary is distributed by this icon revision.

These app-icon filenames previously held the upstream lion artwork. The complete earlier licence and attribution are preserved at the start of `LICENSE.txt` and in historical metadata. Those notices remain active for any unchanged assets or retained derivatives; this app-icon replacement does not remove upstream root LICENSE or NOTICE text.

<details>
<summary>Historical attribution rows for the replaced upstream app-icon artwork</summary>

```text
| `assets/app-icon/pdfcraft.svg` | PdfCraft app icon (master vector) | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Master artwork; every other app-icon file is rendered from it (packaging/icons.sh) |
| `assets/app-icon/pdfcraft-small.svg` | PdfCraft app icon (light vector) | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | README header and assets/app-icon/README.md |
| `assets/app-icon/pdfcraft-1024.png` | PdfCraft app icon, 1024 px on Apple's icon grid | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Runtime window/Dock icon on macOS (apps/pdfcraft/src/main.rs) |
| `assets/app-icon/pdfcraft.icns` | PdfCraft app icon (macOS .icns) | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | macOS app bundle icon |
| `assets/app-icon/pdfcraft.ico` | PdfCraft app icon (Windows .ico, 16-256 px) | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Embedded in pdfcraft.exe by apps/pdfcraft/build.rs (Explorer, taskbar, Start menu, Alt-Tab) |
| `assets/app-icon/hicolor/16x16/apps/ch.mixel.print.png` | PdfCraft app icon, 16 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/24x24/apps/ch.mixel.print.png` | PdfCraft app icon, 24 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/32x32/apps/ch.mixel.print.png` | PdfCraft app icon, 32 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/48x48/apps/ch.mixel.print.png` | PdfCraft app icon, 48 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/64x64/apps/ch.mixel.print.png` | PdfCraft app icon, 64 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/128x128/apps/ch.mixel.print.png` | PdfCraft app icon, 128 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/256x256/apps/ch.mixel.print.png` | PdfCraft app icon, 256 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop); runtime window icon on Windows and Linux (apps/pdfcraft/src/main.rs) |
| `assets/app-icon/hicolor/512x512/apps/ch.mixel.print.png` | PdfCraft app icon, 512 px | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
| `assets/app-icon/hicolor/scalable/apps/ch.mixel.print.svg` | PdfCraft app icon (Linux scalable) | PdfCraft contributors (drawn by the project owner in ArtCraft; vectorised with craftrules vectorize_tile.py) | MIT OR Apache-2.0 | Contributor-original (see assets/app-icon/README.md) | Linux hicolor theme icon (packaging/linux/ch.mixel.print.desktop) |
```

</details>
