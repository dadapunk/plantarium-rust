# Plantarium Tasks Page - Testing Guide

## 🚀 How to Run the App

```bash
cd /Users/sebastian.velazquez/Code/plantarium-rust
cargo run
```

**Note:** Desktop is now the default feature, so `cargo run` works directly without extra flags.

---

## ✨ What Was Fixed

### Root Cause of "Failed to open URL" Error
The app was missing:
1. A proper Layout component to wrap all routes with the Navbar
2. A `body` element in the App component for proper DOM structure
3. Routes weren't connected to the navigation layer

### Changes Made
- ✅ Created `src/layouts/mod.rs` with Layout component
- ✅ Updated router to use Layout as parent for all pages
- ✅ Fixed App component to properly render the DOM tree
- ✅ All routes now have Navbar automatically
- ✅ Fixed CSS class naming (task-item-* → task-list-*, etc.)
- ✅ Removed compilation warnings

---

## 🧪 Testing the Tasks Page

### 1. **Launch the App**
```bash
cargo run
```

A desktop window should open showing the Plantarium app.

### 2. **Navigate to Tasks Page**
- Click **"Tareas"** in the navigation bar

### 3. **Verify Components Render**

#### Header Section
- ✅ Label: "Spring Maintenance"
- ✅ Title: "Garden Tasks"
- ✅ Subtitle: "Your conservatory currently has..."
- ✅ Button: "Add New Task" with icon

#### Main Content
- ✅ **Sidebar (Left)**
  - Calendar widget with month navigation
  - Priority legend (Urgent, Routine, Seasonal)
  - Fertilizer Alert card with "Mark as Done" button

- ✅ **Content Area (Center/Right)**
  - **Urgent Care Section**
    - Badge: "3 Tasks"
    - 2 Task cards visible:
      - "Water Fiddle Leaf Fig"
      - "Pest Inspection"
    - Each card shows: icon, badge, description, plant thumbnail, checkbox

  - **Routine Maintenance Section**
    - Badge: "8 Tasks"
    - List items showing:
      - Task name, description
      - Frequency (Every Day, Weekly, etc.)
      - Badge (Pruning, Tools, Light)
      - Checkbox

#### Additional Elements
- ✅ **Featured Card** - "Garden Journal Entry" with CTA button
- ✅ **FAB Button** - Floating action button in bottom-right corner (✎ icon)

### 4. **Test Interactivity**

```
[] Click "Add New Task" button or FAB
   → Modal should open with form fields

[] Modal Should Have:
   - Title input field
   - Due Date picker
   - Task Type dropdown (Urgent Care / Routine / Seasonal)
   - Cancel and Submit buttons

[] Click Cancel
   → Modal should close

[] Fill form and click Submit
   → Modal should close and task should be added (if implemented)

[] Click checkboxes
   → Visual feedback (checked state)

[] Resize window
   → Layout should adapt to mobile/tablet/desktop sizes
```

---

## 📁 Project Structure

```
plantarium-rust/
├── src/
│   ├── main.rs              # App entry point (fixed: added body, layout support)
│   ├── router.rs            # Routes (fixed: added Layout)
│   ├── layouts/
│   │   └── mod.rs          # NEW: Layout component wrapping pages with Navbar
│   ├── pages/
│   │   ├── dashboard.rs    
│   │   ├── tasks.rs        # Tasks page (fixed: CSS class naming)
│   │   ├── calendar.rs
│   │   ├── journal.rs
│   │   ├── garden_detail.rs
│   │   └── bed_editor.rs
│   ├── components/
│   │   ├── mod.rs          # Main components (includes Navbar)
│   │   ├── dashboard_v2/   # Dashboard components
│   │   └── tasks/          # Tasks components (FIXED)
│   │       ├── task_card.rs
│   │       ├── task_list_item.rs      (fixed: task-item → task-list)
│   │       ├── calendar_widget.rs
│   │       ├── fertilizer_alert.rs    (fixed: alert structure)
│   │       ├── featured_card.rs
│   │       ├── task_modal.rs
│   │       └── tasks_header.rs
│   └── ...
├── assets/
│   ├── tasks.css           # Tasks styles (935 lines, validated)
│   ├── plantarium-theme.css # Design tokens (has all CSS variables)
│   ├── stitch-theme.css
│   └── main.css
├── Cargo.toml              # desktop as default feature ✅
└── Dioxus.toml
```

---

## 🎨 Design System

All colors and spacing use CSS variables from `plantarium-theme.css`:

| Token | Value |
|-------|-------|
| `--primary` | #37602c (Green) |
| `--secondary` | #9f402d (Terracotta) |
| `--tertiary` | #4e5c24 (Olive) |
| `--surface` | #faf9f5 (Cream) |
| `--spacing-4` | 1rem (16px) |
| `--radius-xl` | 1rem (16px) |

---

## 🐛 Known Issues & Solutions

| Issue | Solution |
|-------|----------|
| "Failed to open URL" error | ✅ FIXED - Added Layout and body element |
| CSS classes don't match | ✅ FIXED - Renamed all task-item-* to task-list-* |
| App doesn't compile | ✅ FIXED - Set desktop as default feature |
| Compilation warnings | ✅ FIXED - Removed unused imports |

---

## 📝 File Modifications Summary

### Created
- ✨ `src/layouts/mod.rs` - Layout component

### Modified
- 🔧 `src/main.rs` - Added body, added layouts module
- 🔧 `src/router.rs` - Added Layout wrapper
- 🔧 `Cargo.toml` - desktop as default feature
- 🔧 `src/pages/tasks.rs` - Fixed CSS class naming
- 🔧 `src/components/tasks/task_list_item.rs` - Fixed all class names
- 🔧 `src/components/tasks/fertilizer_alert.rs` - Fixed structure
- 🔧 `src/components/mod.rs` - Removed unused exports

### Assets
- 📄 `assets/tasks.css` - Complete 935-line stylesheet (validated)

---

## ✅ Checklist for Successful Test

- [ ] App launches without errors
- [ ] "Tareas" link in navbar works
- [ ] Tasks page shows all sections
- [ ] Sidebar calendar renders
- [ ] Urgent Care cards visible
- [ ] Routine Maintenance list visible
- [ ] Featured card shows
- [ ] FAB button appears
- [ ] Add New Task modal opens
- [ ] Modal closes on Cancel
- [ ] Responsive layout works

---

## 🎯 Next Steps

1. **Test the app** following the guide above
2. **Implement task persistence** (currently uses demo data)
3. **Add real data binding** to database
4. **Implement modal form submission** to add actual tasks
5. **Add animations** and transitions
6. **Test on different screen sizes** (desktop app can be resized)

---

## 📞 If Issues Occur

```bash
# Clean rebuild
rm -rf target
cargo clean
cargo build --no-default-features --features desktop

# Run with verbose output
RUST_LOG=debug cargo run
```

Enjoy testing! 🌱
