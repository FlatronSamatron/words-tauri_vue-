# Lexio (Words) - Vocabulary Learning App

A lightweight, distraction-free macOS menu bar application built with **Tauri v2** and **Vue 3** to help you learn and memorize new words using the spaced repetition concept.

## 🚀 Features
- **Menu Bar App**: Runs quietly in your macOS tray (menu bar).
- **Spaced Learning**: The tray icon notifies you when it's time to learn a new word based on your interval settings.
- **Quick Popup Game**: Review words directly from the tray without opening the main window.
- **Vocabulary Manager**: Add, edit, and manage your custom words via the main dashboard.
- **Local SQLite Database**: All your words and progress are stored securely and offline on your machine.

---

## 🛠 Tech Stack
- **Frontend**: Vue 3, TypeScript, Pinia (State Management), Tailwind CSS, Vite.
- **Backend & Core**: Rust, Tauri v2.
- **Database**: `rusqlite` (SQLite).

---

## 📦 Getting Started

### Prerequisites
Make sure you have the following installed on your system:
- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri v2 CLI prerequisites](https://v2.tauri.app/start/prerequisites/) 

### 1. Install Dependencies
Clone the repository and install the frontend dependencies:
```bash
npm install
```

### 2. Development Mode
To run the app in development mode with Hot Module Replacement (HMR) for both Vue and Rust:
```bash
npm run tauri dev
```
*Note: This will compile the Rust backend and spin up the Vite dev server.*

### 3. Build for Production
To create a standalone macOS `.app` bundle / `.dmg` installer:
```bash
npm run tauri build
```
Once the build concludes, you can find the generated `.app` and `.dmg` files in the `src-tauri/target/release/bundle/macos/` directory.

---

## 📖 Usage Guide
1. Launch the application. The main **Words Library** window will open.
2. Add your words and their translations using the left sidebar.
3. Configure your **learning interval** (e.g., every 5 minutes) and **translation direction** (Native → Foreign or vice-versa) in the Settings tab.
4. Once you close the main window, the application will remain active in your Mac's top menu bar (tray).
5. When the interval passes, the tray icon will light up with a red dot.
6. Click the tray icon to answer the flashcard. If you don't know the word, it will be shown more frequently!

To completely exit the app, right-click (or two-finger click) the tray icon and select **Quit**.
