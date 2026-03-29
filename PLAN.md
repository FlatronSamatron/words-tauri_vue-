# 📋 Lexio — План разработки

> **Стек:** Tauri 2 · Vue 3 · TypeScript · Pinia · SQLite  
> **Правило:** один блок = один чат с Claude Code. Тесты запускаются после каждого блока перед переходом к следующему.

---

## Как работать с этим планом

| Символ | Значение |
|--------|----------|
| 🤖 | Задача для Claude Code |
| 👤 | Задача для разработчика |
| ✅ | Чекбокс выполнения |
| 🧪 | Тесты блока |

**Шаблон промпта для каждого блока:**
```
Стек: Tauri 2 + Vue 3 + TypeScript + Pinia + SQLite
Приложение: карточки для изучения слов (трей-приложение)
Задача: [название блока]

Контекст: [вставить только нужные типы/файлы]

Требования:
- [список]

Не трогай: [что уже готово]
```

---

## Блок 0 — Скаффолдинг проекта

### Задачи
- [ ] 👤 Создать новый Tauri 2 + Vue 3 TS проект (`npm create tauri-app`)
- [ ] 🤖 Настроить структуру папок
- [ ] 🤖 Установить и настроить Pinia
- [ ] 🤖 Установить и настроить Tailwind CSS
- [ ] 🤖 Настроить `tauri.conf.json` (название, идентификатор, размер окна)
- [ ] 🤖 Создать базовый роутинг Vue Router: `/` → `MainWindow`

### Ожидаемая структура
```
src/
  components/
  views/
    MainWindow.vue      # вкладки Words + Settings
  stores/
  types/
    index.ts            # общие типы
  router/
    index.ts
  App.vue
  main.ts
src-tauri/
  src/
    main.rs
    db.rs               # модуль базы данных
    commands.rs         # tauri commands
  tauri.conf.json
```

### 🧪 Тесты блока 0
```bash
# 1. Проект запускается без ошибок
npm run tauri dev

# 2. Проверить что Vue Router работает
# → Открыть http://localhost:1420, убедиться что MainWindow рендерится

# 3. Pinia подключена
# → В DevTools Vue → Pinia stores видны

# 4. Tailwind работает
# → Добавить временный класс text-red-500 в App.vue, убедиться что цвет применился
```

**✅ Блок 0 завершён если:** приложение запускается, роутер работает, нет ошибок в консоли.

---

## Блок 1 — База данных (Rust)

### Задачи
- [ ] 🤖 Установить `tauri-plugin-sql` с поддержкой SQLite
- [ ] 🤖 Создать `db.rs` — инициализация БД и миграции
- [ ] 🤖 Создать таблицы:

```sql
CREATE TABLE IF NOT EXISTS words (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  word TEXT NOT NULL,
  translate TEXT NOT NULL,
  correct INTEGER DEFAULT 0,
  total INTEGER DEFAULT 0,
  created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- Дефолтные настройки
INSERT OR IGNORE INTO settings VALUES ('interval_minutes', '5');
INSERT OR IGNORE INTO settings VALUES ('direction', 'native_to_foreign');
```

- [ ] 🤖 Создать `commands.rs` с командами:

```rust
// Слова
get_words() -> Vec<Word>
add_word(word: String, translate: String) -> Word
update_word(id: i64, word: String, translate: String) -> Word
delete_word(id: i64) -> bool

// Статистика
record_answer(id: i64, known: bool) -> Word

// Настройки
get_settings() -> Settings
save_settings(interval_minutes: u32, direction: String) -> bool
```

- [ ] 🤖 Зарегистрировать все команды в `main.rs`

### Контекст для Claude Code
```
Нужен только src-tauri/src/ (main.rs, db.rs, commands.rs)
+ Cargo.toml для зависимостей
```

### 🧪 Тесты блока 1
```bash
# 1. Rust компилируется без ошибок
cd src-tauri && cargo build

# 2. Тест команд через Tauri invoke (добавить временно в App.vue)
await invoke('add_word', { word: 'hello', translate: 'привет' })
await invoke('get_words')  // должен вернуть массив с 1 словом
await invoke('delete_word', { id: 1 })
await invoke('get_words')  // должен вернуть пустой массив

# 3. Тест настроек
await invoke('get_settings')  
// → { interval_minutes: 5, direction: 'native_to_foreign' }

# 4. Тест статистики
await invoke('add_word', { word: 'cat', translate: 'кот' })
await invoke('record_answer', { id: 1, known: true })
await invoke('get_words')  
// → слово с correct: 1, total: 1
```

**✅ Блок 1 завершён если:** все 4 теста пройдены, БД создаётся в app data directory.

---

## Блок 2 — TypeScript типы и Pinia stores

### Задачи
- [ ] 🤖 Создать `src/types/index.ts`:

```typescript
export interface Word {
  id: number
  word: string
  translate: string
  correct: number
  total: number
  created_at: string
  // вычисляемое
  percentage: number  // correct/total * 100 или 0
}

export interface Settings {
  interval_minutes: number
  direction: 'native_to_foreign' | 'foreign_to_native'
}
```

- [ ] 🤖 Создать `src/stores/words.ts` (wordsStore):
  - `words: Word[]` — список слов
  - `fetchWords()` — загрузить с бэка
  - `addWord(word, translate)` — добавить
  - `updateWord(id, word, translate)` — обновить
  - `deleteWord(id)` — удалить
  - `recordAnswer(id, known)` — записать результат

- [ ] 🤖 Создать `src/stores/settings.ts` (settingsStore):
  - `settings: Settings` — текущие настройки
  - `fetchSettings()` — загрузить
  - `saveSettings(settings)` — сохранить

- [ ] 🤖 Создать `src/stores/game.ts` (gameStore):
  - `currentWord: Word | null`
  - `nextWord()` — выбрать следующее слово (взвешенный рандом: слова с низким % показываются чаще)
  - `answer(known: bool)` — записать ответ + обновить статистику + взять следующее слово

### 🧪 Тесты блока 2
```typescript
// Добавить в App.vue временный тест-блок onMounted:

// 1. wordsStore CRUD
const wordsStore = useWordsStore()
await wordsStore.fetchWords()
await wordsStore.addWord('dog', 'собака')
console.assert(wordsStore.words.length > 0, 'words не пустой')

// 2. settingsStore
const settingsStore = useSettingsStore()
await settingsStore.fetchSettings()
console.assert(settingsStore.settings.interval_minutes === 5, 'дефолтный интервал 5')

// 3. gameStore — nextWord не падает при пустом списке слов
const gameStore = useGameStore()
gameStore.nextWord() // не должен падать — currentWord = null

// 4. Взвешенный рандом — слова с 0% должны попадаться чаще
// → Добавить 3 слова: первое correct=0/total=5, остальные correct=5/total=5
// → Запустить nextWord() 20 раз, первое слово должно появиться > 10 раз
```

**✅ Блок 2 завершён если:** все store методы работают, типы не ломают компиляцию TS.

---

## Блок 3 — UI: вкладка Words

### Задачи
- [x] 🤖 Создать `src/components/WordForm.vue`:
  - Два инпута: слово + перевод
  - Кнопка «Сохранить»
  - Валидация: оба поля обязательны
  - После сохранения — очистить форму

- [x] 🤖 Создать `src/components/WordsTable.vue`:
  - Колонки: Слово | Перевод | Правильно/Всего | % | Действия
  - Inline-редактирование (клик по строке → поля становятся input)
  - Кнопка удаления с подтверждением
  - Пустое состояние: иллюстрация + «Добавьте первое слово»
  - Сортировка по % (по возрастанию — слабые слова вверху)

- [x] 🤖 Создать `src/views/WordsView.vue`:
  - Слева (40%): `WordForm`
  - Справа (60%): `WordsTable`
  - Подключить к `wordsStore`

### Контекст для Claude Code
```typescript
// Передать тип Word из types/index.ts
// Передать сигнатуры методов wordsStore
```

### 🧪 Тесты блока 3
```
Ручное тестирование:

[ ] Форма: попытка сохранить пустую форму → показывается ошибка валидации
[ ] Форма: ввести слово + перевод → нажать Сохранить → слово появляется в таблице
[ ] Форма: после сохранения поля очищаются
[ ] Таблица: пустое состояние отображается при 0 словах
[ ] Таблица: клик по строке → поля становятся редактируемыми
[ ] Таблица: Сохранить edit → данные обновляются в таблице
[ ] Таблица: Отмена edit → данные не изменились
[ ] Таблица: кнопка удаления → подтверждение → слово удаляется
[ ] Таблица: % считается правильно (correct/total*100, 0 если total=0)
```

**✅ Блок 3 завершён если:** все ручные тесты пройдены.

---

## Блок 4 — UI: вкладка Settings

### Задачи
- [x] 🤖 Создать `src/views/SettingsView.vue`:
  - Числовой инпут «Интервал напоминания (минут)» — мин. 1, макс. 60
  - Тоггл направления:
    - `native_to_foreign` → «Родной → Иностранный»
    - `foreign_to_native` → «Иностранный → Родной»
  - Кнопка «Сохранить» с индикатором успеха
  - Загрузка текущих значений при монтировании

- [x] 🤖 Обновить `src/views/MainWindow.vue`:
  - Две вкладки: Words / Settings
  - Активная вкладка подсвечена
  - Переключение без потери состояния (keep-alive)

### 🧪 Тесты блока 4
```
Ручное тестирование:

[ ] При открытии Settings — значения загружены из БД
[ ] Изменить интервал → Сохранить → перезапустить приложение → значение сохранилось
[ ] Изменить направление → Сохранить → перезапустить → значение сохранилось
[ ] Ввести интервал = 0 → кнопка Сохранить заблокирована или показана ошибка
[ ] Переключиться Words → Settings → обратно → форма добавления слова не сброшена (keep-alive)
```

**✅ Блок 4 завершён если:** настройки сохраняются между перезапусками.

---

## Блок 5 — Игровой попап

### Задачи
- [x] 🤖 Создать отдельное Tauri окно `game` в `tauri.conf.json`:
  ```json
  {
    "label": "game",
    "url": "/#/game",
    "title": "",
    "width": 320,
    "height": 200,
    "decorations": false,
    "alwaysOnTop": true,
    "visible": false
  }
  ```

- [x] 🤖 Создать `src/views/GameView.vue`:
  - Отображение слова (согласно direction из Settings)
  - Две кнопки: «✓ Знаю» (зелёная) / «✗ Не знаю» (красная)
  - После ответа — анимация смены слова (fade или slide)
  - Кнопка закрыть (X) — без записи в статистику
  - Если слов нет → сообщение «Добавьте слова в настройках»

- [x] 🤖 Добавить Tauri команды в `commands.rs`:
  ```rust
  open_game_window()   // показать окно игры
  close_game_window()  // скрыть окно игры
  ```

- [x] 🤖 В `gameStore` — загружать настройки direction и показывать нужную сторону слова

### 🧪 Тесты блока 5
```
Ручное тестирование:

[ ] Окно игры открывается через invoke('open_game_window')
[ ] Отображается слово (в зависимости от direction)
[ ] Нажать «Знаю» → слово меняется, анимация проигрывается
[ ] Нажать «Не знаю» → слово меняется
[ ] Проверить в таблице Words — correct и total обновились
[ ] Нажать X → окно закрывается, статистика не изменилась
[ ] Добавить только 1 слово → игра показывает его снова после ответа
[ ] Удалить все слова → открыть игру → сообщение «Добавьте слова»
[ ] direction = foreign_to_native → в игре показывается иностранное слово
```

**✅ Блок 5 завершён если:** статистика правильно пишется, окно открывается/закрывается.

---

## Блок 6 — Системный трей (Rust)

### Задачи
- [x] 🤖 Настроить system tray в `main.rs`:
  - Иконки: `icon-normal.png` и `icon-active.png` (32x32)
  - **ПКМ** → системное меню: `Settings` / `Quit`
  - **ЛКМ** → вызов `open_game_window()`

- [x] 👤 Создать две иконки (нормальная + активная с индикатором)
  - Рекомендуется: нормальная — серая, активная — цветная/с точкой

- [x] 🤖 Добавить Tauri команды:
  ```rust
  set_tray_active()    // сменить иконку на активную
  set_tray_normal()    // сменить иконку на обычную
  ```

- [x] 🤖 Обработчик ПКМ пункта `Settings`:
  - Если главное окно скрыто — показать и сфокусировать
  - Если уже видно — просто сфокусировать

### 🧪 Тесты блока 6
```
Ручное тестирование:

[ ] Иконка отображается в трее при запуске
[ ] ПКМ → меню с пунктами Settings и Quit
[ ] Settings → главное окно открывается / появляется на переднем плане
[ ] Quit → приложение полностью закрывается
[ ] ЛКМ → игровой попап открывается
[ ] invoke('set_tray_active') → иконка меняется
[ ] invoke('set_tray_normal') → иконка возвращается
[ ] Закрыть главное окно крестиком → приложение не закрывается (только скрывается)
```

**✅ Блок 6 завершён если:** трей работает, меню открывается, окна управляются правильно.

---

## Блок 7 — Таймер и смена иконки (Rust)

### Задачи
- [ ] 🤖 Добавить background таймер в `main.rs` (tokio):
  - Читать интервал из settings БД при старте
  - Каждые N минут → вызывать `set_tray_active()`
  - После хотя бы одного ответа в игре → `set_tray_normal()` + сбросить таймер

- [ ] 🤖 Добавить Tauri event `answer-recorded` — фронтенд шлёт его после каждого ответа:
  ```typescript
  // gameStore.answer()
  await emit('answer-recorded')
  ```

- [ ] 🤖 При изменении интервала в Settings — перезапустить таймер:
  - Добавить команду `update_timer_interval(minutes: u32)`
  - Вызывать при сохранении настроек

### 🧪 Тесты блока 7
```
Ручное тестирование:

[ ] Установить интервал = 1 минута в Settings
[ ] Подождать 1 минуту → иконка стала активной
[ ] Открыть игру → ответить на слово → иконка вернулась в нормальную
[ ] Подождать ещё 1 минуту → иконка снова стала активной (таймер сбросился)
[ ] Изменить интервал = 2 минуты → сохранить → старый таймер не срабатывает через 1 минуту
[ ] Закрыть и открыть приложение → таймер запускается с актуальным интервалом из БД

# Stress test
[ ] Ответить на 10 слов подряд → статистика правильно записана для всех
```

**✅ Блок 7 завершён если:** таймер работает, иконка сбрасывается после игры.

---

## Блок 8 — Полировка и edge cases

### Задачи
- [ ] 🤖 Edge cases:
  - 0 слов: трей ЛКМ → попап с сообщением вместо игры
  - 1 слово: игра показывает его снова после ответа (не падает)
  - Пустые поля в форме: сохранение заблокировано

- [ ] 🤖 UX улучшения:
  - Горячая клавиша в игре: `←` = Не знаю, `→` = Знаю, `Esc` = закрыть
  - Автофокус на первый инпут при открытии формы
  - Toast-уведомление после сохранения слова

- [ ] 🤖 Сохранение позиции игрового попапа (запомнить где пользователь его оставил)

- [ ] 👤 Финальная проверка на Windows / macOS

### 🧪 Тесты блока 8
```
[ ] 0 слов → ЛКМ трей → "Добавьте слова" (не крашится)
[ ] 1 слово → игра работает бесконечно
[ ] Горячие клавиши работают в игровом попапе
[ ] Toast появляется после добавления слова и исчезает через 2с
[ ] Позиция попапа сохраняется между открытиями
[ ] Нет утечек памяти: открыть/закрыть игру 50 раз
```

**✅ Блок 8 завершён если:** нет edge case багов, UX отполирован.

---

## Итоговый чеклист релиза

```
[ ] cargo build --release компилируется без warning'ов
[ ] npm run build компилируется без ошибок TypeScript
[ ] Протестировано на целевой ОС
[ ] Иконка трея отображается корректно
[ ] БД создаётся в правильной директории (app data)
[ ] Приложение запускается при логине (опционально)
```

---

## Блок 9 — Группы слов

> Слова привязываются к группе. В настройках можно выбрать конкретную группу или «All» (все слова). Игра использует только слова выбранной группы.

### Задачи

#### 9.1 — База данных: миграция v2

- [ ] 🤖 Добавить **Migration v2** в `db.rs`:

```sql
-- Таблица групп
CREATE TABLE IF NOT EXISTS groups (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  created_at TEXT DEFAULT (datetime('now'))
);

-- Группа по умолчанию
INSERT OR IGNORE INTO groups (id, name) VALUES (1, 'Default');

-- Добавить столбец group_id к words
ALTER TABLE words ADD COLUMN group_id INTEGER DEFAULT 1 REFERENCES groups(id) ON DELETE CASCADE;

-- Настройка активной группы (null = All)
INSERT OR IGNORE INTO settings VALUES ('active_group_id', 'all');
```

> Все существующие слова автоматически попадают в группу «Default» (через `DEFAULT 1`).

---

#### 9.2 — Rust Backend: команды для групп

- [ ] 🤖 Добавить структуру `Group` в `commands.rs`:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub word_count: i64,
}
```

- [ ] 🤖 Добавить новые команды:

| Команда | Параметры | Описание |
|---------|-----------|----------|
| `get_groups` | — | Список всех групп с количеством слов |
| `add_group` | `name: String` | Создать группу |
| `rename_group` | `id: i64, name: String` | Переименовать группу |
| `delete_group` | `id: i64` | Удалить группу (CASCADE — слова удаляются) |

- [ ] 🤖 Обновить существующие команды:

| Команда | Изменение |
|---------|-----------|
| `Word` struct | + `group_id: i64` |
| `Settings` struct | + `active_group_id: String` (`"all"` или числовой ID) |
| `add_word` | + параметр `group_id: i64` |
| `get_words` | + `group_id` в SELECT |
| `get_settings` / `save_settings` | + чтение/запись `active_group_id` |

- [ ] 🤖 Зарегистрировать новые команды в `lib.rs` → `invoke_handler`

---

#### 9.3 — Frontend: типы и stores

- [ ] 🤖 Обновить `src/types/index.ts`:

```typescript
export interface Group {
  id: number
  name: string
  created_at: string
  word_count: number
}

// Word → + group_id: number
// Settings → + active_group_id: string  // "all" | "<number>"
```

- [ ] 🤖 Создать `src/stores/groups.ts`:
  - `groups: Group[]`
  - `fetchGroups()` — загрузить список
  - `addGroup(name)` — создать
  - `renameGroup(id, name)` — переименовать
  - `deleteGroup(id)` — удалить

- [ ] 🤖 Обновить `src/stores/words.ts`:
  - `addWord()` — + параметр `groupId: number`
  - `mapWord()` — маппить `group_id`

- [ ] 🤖 Обновить `src/stores/settings.ts`:
  - Добавить `active_group_id` в default state, `fetchSettings`, `saveSettings`

- [ ] 🤖 Обновить `src/stores/game.ts`:
  - `nextWord()` — фильтровать слова по `settingsStore.settings.active_group_id` (если не `"all"`)

---

#### 9.4 — Frontend: UI

- [ ] 🤖 `SettingsView.vue` — новая секция **«Active Group»**:
  - Dropdown: `All` + список групп из `groupsStore`
  - Блок «Manage Groups» с CRUD: создание, переименование, удаление групп

- [ ] 🤖 `WordForm.vue` — dropdown-селектор группы:
  - Значение по умолчанию = текущая активная группа из настроек

- [ ] 🤖 `WordsTable.vue` — столбец «Group» в таблице

- [ ] 🤖 `WordsView.vue` — фильтр по группам:
  - Chip-selector вверху: `All | Group1 | Group2 | ...`
  - Quick Stats считать по текущему фильтру

- [ ] 🤖 `GameView.vue`:
  - Показывать название активной группы мелким текстом
  - Адаптировать empty state: «No words in this group» vs «No words at all»

---

### 🧪 Тесты блока 9
```
Ручное тестирование:

[ ] Миграция: при запуске создаётся группа «Default», все слова привязаны к ней
[ ] Создать новую группу в настройках
[ ] Добавить слово в новую группу
[ ] Переключить активную группу → слова фильтруются в таблице и в игре
[ ] Выбрать «All» → все слова видны
[ ] Удалить группу → слова удалены вместе с ней
[ ] Игра корректно показывает слова только из выбранной группы
[ ] Игра: пустая группа → сообщение «No words in this group»
[ ] Quick Stats пересчитываются при смене фильтра группы
[ ] Группу «Default» нельзя удалить (но можно переименовать)
```

**✅ Блок 9 завершён если:** группы работают end-to-end, фильтрация корректна в таблице и в игре.

---

## Зависимости (Cargo.toml)

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
```

## Зависимости (package.json)

```json
{
  "dependencies": {
    "vue": "^3.4",
    "vue-router": "^4",
    "pinia": "^2",
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-sql": "^2"
  },
  "devDependencies": {
    "typescript": "^5",
    "vite": "^5",
    "@vitejs/plugin-vue": "^5",
    "tailwindcss": "^3",
    "vue-tsc": "^2"
  }
}
```
