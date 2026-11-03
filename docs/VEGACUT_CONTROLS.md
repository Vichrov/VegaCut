# VegaCut v0.3 controls / Управление

Choose **Edit → Keyboard Shortcuts → VegaCut**. New installations select it automatically. Upgrades retain any previously saved profile. Saving a custom copy also saves its mouse behavior. Choosing another built-in profile restores that profile's gestures.

Выберите **Правка → Горячие клавиши → VegaCut**. При первом запуске профиль включён автоматически; ранее сохранённые настройки не заменяются. Пользовательская копия профиля сохраняет и поведение мыши.

| Input / Ввод | Action / Действие |
|---|---|
| S | Split selected clips at playhead; without a selection at the playhead, split targeted tracks / Разрезать выбранные клипы; иначе — целевые дорожки |
| G / U | Group / ungroup selection / Сгруппировать / разгруппировать |
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
