---
name: slint-expert
description: >-
  Expert guide for building native, reactive UIs using Slint.
  Use when designing or debugging Slint markup (.slint files), layouts,
  responsive grids, data models, theming, custom controls, and Rust-Slint integration.
---

# Slint UI Expert Guide

Comprehensive reference and best practices for developing native desktop applications with Slint and Rust.

---

## 1. Slint Layout Engine Principles

### Layout Containers
- **`VerticalLayout` / `HorizontalLayout`**:
  - Direct children share space along the layout axis based on their `vertical-stretch` / `horizontal-stretch` values.
  - Setting `alignment: start | center | end` disables automatic stretching. Children retain their intrinsic preferred or explicit sizes.
  - To prevent a child from expanding unnecessarily, set `vertical-stretch: 0;` or `horizontal-stretch: 0;`.

### Avoiding Circular Binding Loops
A common Slint error occurs when a child's width depends on its parent's width while the parent calculates its size from its children:
```slint
// ❌ ANTI-PATTERN: Circular binding loop
HorizontalLayout {
    StaticWallpaperItem {
        width: (parent.width - 40px) / 4; // Error: parent.width depends on item width!
    }
}

// ✅ BEST PRACTICE 1: Auto-stretching
HorizontalLayout {
    StaticWallpaperItem {
        horizontal-stretch: 1;
        min-width: 0px;
    }
}

// ✅ BEST PRACTICE 2: Direct Root-Derived Calculation
export component GalleryPage inherits Rectangle {
    // Calculated directly from root window width (no circular loop)
    property <length> item-width: max(160px, (self.width - 124px) / 4);

    VerticalLayout {
        for row in AppStore.rows : HorizontalLayout {
            alignment: start;
            spacing: 14px;

            for item in row.items : Rectangle {
                width: root.item-width;
                height: 185px;
                WallpaperCard { data: item; }
            }
        }
    }
}
```

---

## 2. Component Alignment & Sizing

### Image Components
- Slint's `Image` does **not** support `vertical-alignment`. It uses `image-fit: fill | contain | cover`.
- An `Image` inside a `HorizontalLayout` defaults to `y: 0`. To vertically center an icon or image alongside text:
  ```slint
  HorizontalLayout {
      height: 42px;

      Image {
          source: icon;
          width: 18px;
          height: 18px;
          image-fit: contain;
          y: (parent.height - self.height) / 2; // Perfect vertical centering
      }

      Text {
          text: label;
          vertical-alignment: center; // Text supports vertical-alignment
      }
  }
  ```

### Controls & Input Alignment
- When pairing labels with controls (`ComboBox`, `LineEdit`, `ToggleSwitch`, `Button`):
  ```slint
  HorizontalLayout {
      spacing: 10px;
      height: 34px;
      y: (parent.height - self.height) / 2;

      Text {
          text: "Category:";
          font-size: 13px;
          font-weight: 600;
          color: Theme.text-secondary;
          vertical-alignment: center;
      }

      ComboBox {
          width: 180px;
          height: 34px;
          model: AppStore.categories;
      }
  }
  ```

---

## 3. Responsive Grids & Scrollable Containers

### Flickable Viewport Calculation
When using `Flickable` for dynamic grids, compute the viewport height from row count:
```slint
Flickable {
    vertical-stretch: 1;
    viewport-width: self.width;
    viewport-height: (AppStore.rows.length * (item-height + gap)) + gap + 32px;

    VerticalLayout {
        width: parent.width;
        padding: 16px;
        spacing: gap;
        alignment: start;

        for row in AppStore.rows : HorizontalLayout {
            // Row items
        }
    }
}
```

---

## 4. Rust & Slint State Synchronization

### Data Models & Row Grouping
Slint lacks a native CSS grid. To display an $N$-column grid from a flat collection in Rust:
```rust
pub fn group_rows(items: &[WallpaperData], cols: usize) -> ModelRc<RowData> {
    let mut rows: Vec<RowData> = Vec::new();
    for chunk in items.chunks(cols) {
        let row_items: Vec<WallpaperData> = chunk.to_vec();
        rows.push(RowData {
            items: ModelRc::from(Rc::new(VecModel::from(row_items))),
        });
    }
    ModelRc::from(Rc::new(VecModel::from(rows)))
}
```

### Event Loop Dispatch
Always update Slint properties and models from the main UI thread:
```rust
let window_weak = main_window.as_weak();
std::thread::spawn(move || {
    let results = heavy_background_task();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window_weak.upgrade() {
            window.global::<AppStore>().set_items(results);
        }
    });
});
```

---

## 5. Theming Engine

Maintain theme tokens in a centralized `global Theme`:
```slint
export global Theme {
    in-out property <bool> dark: true;
    out property <color> bg: dark ? #12131a : #f5f6fa;
    out property <color> card: dark ? #1a1b24 : #ffffff;
    out property <color> sunken: dark ? #14151e : #eceef4;
    out property <color> accent: #6366f1;
    out property <color> text: dark ? #ffffff : #111827;
    out property <color> text-secondary: dark ? #9ca3af : #6b7280;
    out property <color> border: dark ? #2e303e : #e5e7eb;
}
```
