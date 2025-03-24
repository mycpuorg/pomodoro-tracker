# Pomodoro Tracker

A minimalist desktop Pomodoro timer and activity tracking application built with Tauri, Svelte, and SQLite. This app helps you maintain focus and track how you spend your time by implementing the Pomodoro Technique.

I only have cursory understanding of this stack. I vibe-coded a lot of the boilerplate.

![Pomodoro Tracker Screenshot](screenshots/pomodoro-tracker.png)

## Features

- **Pomodoro Timer**: 25-minute work intervals followed by 5-minute breaks
- **Task Tracking**: Assign a task name and category to each Pomodoro session
- **Daily View**: See your completed Pomodoro sessions organized by day
- **Analytics**: Visualize your time distribution with charts:
  - Time spent per category (pie chart)
  - Time spent per day of the week (bar chart)
- **Data Persistence**: All your sessions are saved locally in an SQLite database
- **Mobile-Friendly UI**: Responsive design that works well on all screen sizes
- **Offline-First**: Works completely offline as a desktop application

## Installation

### Prerequisites

To build and run the application, you'll need:

- [Node.js](https://nodejs.org/) (v16 or newer)
- [Rust](https://www.rust-lang.org/tools/install) (stable channel)
- Linux system dependencies (for Tauri):

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### Building from Source

Clone the repository:

```
git clone https://github.com/your-username/pomodoro-tracker.git
cd pomodoro-tracker
```

#### Install NPM dependencies:

```bash
npm install
```

#### Run in development mode:

```bash
npm run tauri dev
```

#### Build for production:

```bash
npm run tauri build
```

The compiled application will be available in src-tauri/target/release/ and packaged installers in src-tauri/target/release/bundle/.
Usage

Timer: Set your task name and category, then start the timer
Daily View: Browse through your completed Pomodoro sessions
Analytics: View your productivity patterns over time

Technology Stack

Frontend: Svelte
Backend: Rust with Tauri
Database: SQLite (local)
Charts: Chart.js

Project Structure
```
pomodoro-tracker/
├── src/                  # Svelte frontend
│   ├── lib/              
│   │   ├── components/   # UI components
│   │   ├── stores/       # Svelte stores for state
│   └── App.svelte        # Main app component
├── src-tauri/            # Tauri/Rust backend
│   ├── src/              # Rust code
│   │   ├── main.rs       # App entry point
│   │   └── db.rs         # Database operations
│   └── Cargo.toml        # Rust dependencies
└── package.json          # NPM dependencies
```
