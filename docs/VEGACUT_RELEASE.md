# VegaCut v0.4.0

Fixes three timeline problems reported after v0.3:

- **U** now detaches audio from video as well as removing the ordinary group. **G** groups the selected events again. Undo restores the relationship.
- A plain click on a clip body or empty track time places the edit cursor there; dragging its handle is no longer necessary.
- Red **A/V offset** badges show signed frame displacement, including live move/trim/slip previews. Restoring synchronization clears the badge. Different speeds get a separate warning.

Исправлены отделение звука клавишей **U**, установка курсора обычным щелчком и метки рассинхронизации аудио/видео. **G** снова группирует выделенные клипы.

Select the built-in **VegaCut** keyboard profile to use these controls. Existing modified custom profiles are preserved; reselect VegaCut to adopt the corrected U assignment.

Extract the Windows x64 portable ZIP into a writable folder and run `VegaCut.exe`. Preserve the adjacent `data` folder when upgrading. Interface: English/Russian. Projects remain `.fcproj`.

This release prioritizes the reported editing problems; automatic overlap crossfades and new ripple modes are deferred. Sync badges compare opposite media types from the same source, not independent recordings; frame holds and animated retiming have no numeric offset badge. No `.veg`, VST3 or OpenFX compatibility is promised. Manual Windows GUI testing is still unverified.

Publication requires the complete Linux validation gate and Windows tests/build/package. See `docs/VEGACUT_CONTROLS.md` and `docs/VEGACUT_VALIDATION.md` for scope and checks.
