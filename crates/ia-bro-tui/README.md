# IA,BRO! TODO TUI

A beautiful, keyboard-driven terminal user interface (TUI) for managing tasks and todos.

## Features

✨ **Core Features**
- Create, read, update, delete tasks (CRUD)
- Mark tasks as complete/incomplete
- Set priority levels (Low, Medium, High)
- Add due dates and descriptions
- Tag your tasks
- Filter by status and priority

⚡ **User Experience**
- Smooth keyboard navigation (vim-like keys supported)
- Real-time task filtering
- Color-coded priorities
- Keyboard shortcuts for all actions
- Persistent storage (JSON)
- Responsive terminal UI

🎨 **Visual Design**
- Clean, minimal interface
- Status symbols (☐ ◐ ☑)
- Priority indicators (◇ ◆ ◈)
- Overdue task warnings (⚠)
- Color-coded status display

## Installation

### From Source

```bash
cd crates/ia-bro-tui
cargo build --release
./target/release/ia-bro-tui
```

### Via Cargo

```bash
cargo install --path crates/ia-bro-tui
ia-bro-tui
```

## Usage

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑`/`k` | Move up |
| `↓`/`j` | Move down |
| `n` | Create new task |
| `Enter` | Toggle task completion |
| `d` | Delete selected task |
| `f` | Cycle filter (All → Pending → In Progress → Completed → High Priority) |
| `Ctrl+c` | Clear all tasks |
| `Esc` | Cancel/Exit edit mode |
| `q` | Quit application |

### Creating Tasks

1. Press `n` to open task creation
2. Type task title
3. Press `Enter` to confirm
4. Task is created with default priority (Medium)

### Quick Operations

- **Toggle completion**: Press `Enter` on selected task
- **Change filter**: Press `f` to cycle through filters
- **Delete task**: Press `d` to remove selected task
- **Exit**: Press `q` to quit safely

## Data Storage

Tasks are stored in:
- **Default**: `~/.ia-bro/tasks.json`
- **Custom**: Pass `--storage /path/to/storage` flag

### Task Structure

```json
{
  "id": "uuid",
  "title": "Task title",
  "description": "Optional description",
  "status": "pending",
  "priority": "medium",
  "created_at": "2026-07-03T12:00:00Z",
  "due_date": "2026-07-10T23:59:59Z",
  "completed_at": null,
  "tags": ["tag1", "tag2"]
}
```

## Configuration

Custom storage location:

```bash
ia-bro-tui --storage ~/my-tasks/tasks.json
```

## Task Statuses

- **☐ Pending**: Not started
- **◐ In Progress**: Currently working on
- **☑ Completed**: Done!

## Priority Levels

- **◇ Low** (Cyan): Nice to have
- **◆ Medium** (Yellow): Important
- **◈ High** (Red): Critical/Urgent

## Keyboard Tips

- Use `j`/`k` for navigation if arrow keys don't work
- Use `Ctrl+c` carefully (clears ALL tasks)
- Tasks auto-save after each action

## Troubleshooting

### Terminal display issues

Try resizing your terminal or using a different terminal emulator.

### Data not saving

Check that `~/.ia-bro/` directory has write permissions:

```bash
ls -la ~/.ia-bro/
chmod 755 ~/.ia-bro/
```

### Port conflicts

Not applicable - this is a TUI application with no network component.

## Development

### Project Structure

```
crates/ia-bro-tui/
├── src/
│   ├── lib.rs       # Library root
│   ├── main.rs      # Binary entry point
│   ├── app.rs       # Application state
│   ├── model.rs     # Data models (Task, Priority, Status)
│   ├── storage.rs   # File persistence
│   ├── event.rs     # Event handling
│   ├── handler.rs   # Key bindings & input handling
│   └── ui.rs        # Terminal rendering
├── Cargo.toml       # Dependencies
└── README.md        # This file
```

### Dependencies

- **ratatui**: Terminal UI framework
- **tokio**: Async runtime
- **crossterm**: Terminal manipulation
- **serde/serde_json**: Serialization
- **chrono**: Date/time handling
- **uuid**: Unique identifiers
- **clap**: CLI argument parsing

### Building

```bash
# Development build
cargo build -p ia-bro-tui

# Release build (optimized)
cargo build --release -p ia-bro-tui

# Run directly
cargo run -p ia-bro-tui

# Run tests
cargo test -p ia-bro-tui
```

## Contributing

Contributions welcome! Areas for improvement:

- [ ] Task editing (not just creation)
- [ ] Task search/filter by keywords
- [ ] Export to CSV/PDF
- [ ] Task categories/projects
- [ ] Recurring tasks
- [ ] Task statistics
- [ ] Dark/light theme toggle
- [ ] Undo/redo functionality
- [ ] Sync with cloud services

## License

Apache 2.0 OR MIT

## Author

Created by **Vibecodingchile** for the IA,BRO! project.

---

**Happy task managing!** 🚀
