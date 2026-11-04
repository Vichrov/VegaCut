# VegaCut v0.4 controls / Управление

Choose **Edit → Keyboard Shortcuts → VegaCut**. New installations select it automatically. Upgrades retain any previously saved profile. Saving a custom copy also saves its mouse behavior. Choosing another built-in profile restores that profile's gestures.

Выберите **Правка → Горячие клавиши → VegaCut**. При первом запуске профиль включён автоматически; ранее сохранённые настройки не заменяются. Пользовательская копия профиля сохраняет и поведение мыши.

| Input / Ввод | Action / Действие |
|---|---|
| S | Split selected clips at playhead; without a selection at the playhead, split targeted tracks / Разрезать выбранные клипы; иначе — целевые дорожки |
| G / U | Group / detach audio-video and ungroup / Сгруппировать / отделить звук и разгруппировать |
| F8 | Toggle timeline snapping / Привязка |
| Q | Toggle loop playback / Циклическое воспроизведение |
| J / K / L, Space | Existing shuttle and play/stop commands / Перемотка и воспроизведение |
| Alt+Left / Right | Previous / next frame / Предыдущий / следующий кадр |
| Ctrl+Home / End | Sequence start / end / Начало / конец секвенции |
| Up / Down, timeline focused | Zoom in / out / Масштаб таймлайна |
| M, I, O | Marker, in, out / Маркер, вход, выход |
| Ctrl+D | Selection tool / Инструмент выделения |
| Wheel | Zoom around pointer / Масштаб вокруг указателя |
| Ctrl+wheel | Scroll tracks vertically / Прокрутка дорожек |
| Shift+wheel | Scroll time horizontally / Прокрутка по времени |
| Ctrl+drag clip body | Duplicate selected clips; one Undo / Копировать клипы; одна отмена |
| Ctrl+click clip body | Toggle selection / Добавить или убрать из выделения |

Ctrl means the primary modifier (Command on macOS). Pinch and horizontal trackpad gestures remain supported. Mouse rules apply over the timeline, not unrelated panels. The legacy Timeline Mouse Scrolling preference applies to other profiles; VegaCut uses the rules above.

This is an initial workflow subset. The original trim tools remain available; Vegas Alt/Shift edge modifiers, range selection, numpad editing and return-to-start transport behavior are not asserted. Copying onto occupied time follows the existing edit engine's overwrite behavior. Crossfade/ripple changes belong to 0.4.

Functional reference: [official VEGAS keyboard/mouse documentation](https://help.magix-hub.com/video/vegas/22/en/content/topics/13-appendix/shortcutkeys.htm). No proprietary preset or artwork is included.

## v0.4 corrections / Исправления

A plain click on a clip body or empty track time sets the edit cursor to the clicked frame. It does not require dragging the cursor handle. Modifier selection and clip drags do not seek. U removes both ordinary groups and A/V links; after U, click the individual half to select it. G regroups the selected events. Custom modified presets keep their bindings: reselect the built-in VegaCut profile to adopt the corrected U command.

Обычный щелчок по клипу или свободному месту дорожки устанавливает курсор монтажа. U отделяет звук от видео; затем щёлкните по нужной части для её отдельного перемещения. G снова группирует выделенные клипы. Если профиль клавиш изменялся вручную, выберите встроенный VegaCut заново для обновлённой команды U.

Red badges show the source alignment offset in frames (`f`). Positive means this event is delayed relative to its partner, negative means advanced. Both move position and source In matter; a correctly trimmed clip therefore has no false offset. The badge updates during moves, trims and slip-tool drags and disappears when alignment is restored or undone. It is diagnostic only and does not change media or export audio.

Красная метка показывает смещение в кадрах: плюс — эта часть запаздывает, минус — опережает. Разная скорость помечается отдельно. Сравниваются аудио/видео одного исходника: внутри связи/группы — с наибольшим перекрытием (либо ближайшие), без группы — только перекрывающиеся события. Это не анализ звуковой волны и не синхронизация разных камер. Стоп-кадры и анимированное изменение скорости пока не получают числовую метку.

Reference: [VEGAS synchronization detection](https://help.magix-hub.com/video/vegas/22/en/content/topics/9-audio/audio_video_synchronization_detection_and_repair.htm).
