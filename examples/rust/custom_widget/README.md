# Custom Widget Example

This example demonstrates how to create custom widgets in wxDragon by implementing two different custom widgets:

1. **AniFillButton** - An animated button with a smooth fill effect on hover
2. **PieChart** - An interactive pie chart with hover animations

![Custom Widgets Demo](https://raw.githubusercontent.com/AllenDang/wxDragon/refs/heads/main/asset/custom_widget.gif)

## Custom Widgets Overview

### AniFillButton (`anim_fill_button.rs`)

A custom button widget that features:
- **Smooth fill animation** when the mouse hovers over it
- **Configurable colors** for border, background, and fill
- **Press state visual feedback** with darkening effect
- **Rounded corners** with customizable border radius
- **Timer-based animation** running at 60fps for smooth transitions

**Key Features:**
- Fully customizable appearance (colors, border, text)
- Smooth hover animations with configurable duration
- Proper event handling that works with external event listeners
- Flicker-free drawing using `AutoBufferedPaintDC`

### PieChart (`pie_chart.rs`)

An interactive pie chart widget that displays data with:
- **Dynamic data visualization** from labels and amounts
- **Hover animations** that move slices outward when highlighted
- **Automatic color generation** for pie slices
- **Mouse interaction** with precise slice detection
- **Smooth animations** when entering/leaving slices

**Key Features:**
- Supports any number of data points
- Automatic proportional slice calculation
- Interactive hover effects with smooth animations
- Customizable colors, borders, and animation timing

## Running the Example

```bash
cargo run -p custom_widget
```

The example creates a window with both custom widgets side by side, demonstrating their interactive features.

## Tutorial: Creating Custom Widgets in wxDragon

### Overview

wxDragon provides a powerful `custom_widget!` macro that simplifies creating custom widgets. This macro handles the boilerplate code for widget creation while allowing you to focus on the unique behavior and appearance of your widget.

### Step 1: Basic Structure

Every custom widget starts with the `custom_widget!` macro declaration:

```rust
use wxdragon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

custom_widget!(
    name: MyCustomWidget,
    fields: {
        // Widget configuration fields
        text: String = "Default Text".to_string(),
        background_color: Colour = Colour::new(240, 240, 240, 255),
        text_color: Colour = Colour::new(0, 0, 0, 255),
    },
    setup_impl: |config, panel| {
        // Widget initialization code goes here
    }
);
```

### Step 2: Widget Configuration Fields

The `fields` section defines the configurable properties of your widget:

```rust
fields: {
    // Basic appearance
    text: String = "Button".to_string(),
    background_color: Colour = Colour::new(240, 240, 240, 255),
    border_color: Colour = Colour::new(100, 100, 100, 255),
    border_width: i32 = 2,
    
    // Animation settings
    animation_duration: Duration = Duration::from_millis(300),
    
    // Data (for data visualization widgets)
    data_values: Vec<i32> = vec![10, 20, 30],
}
```

These fields automatically generate:
- Builder methods like `.with_text()`, `.with_background_color()`
- A configuration struct accessible in the setup implementation
- Type-safe initialization with default values

### Step 3: Setup Implementation

The `setup_impl` closure is where you define your widget's behavior:

```rust
setup_impl: |config, panel| {
    // 1. Configure the panel for custom drawing
    panel.set_background_style(BackgroundStyle::Paint);
    
    // 2. Set up event handlers
    panel.on_paint(move |event| {
        // Custom drawing code
        let dc = AutoBufferedPaintDC::new(&panel);
        // ... drawing implementation
        event.skip(true);
    });
    
    // 3. Add mouse/keyboard event handlers
    panel.on_mouse_left_down(|event| {
        println!("Widget clicked!");
        event.skip(true); // Important: allow event propagation
    });
}
```

### Step 4: Custom Drawing

For widgets that need custom appearance, implement drawing in the paint event:

```rust
panel.on_paint(move |event| {
    let dc = AutoBufferedPaintDC::new(&panel);
    let size = panel.get_size();
    
    // Clear background
    dc.set_brush(config.background_color, BrushStyle::Solid);
    dc.draw_rectangle(0, 0, size.width, size.height);
    
    // Draw custom content
    dc.set_text_foreground(config.text_color);
    dc.draw_text(&config.text, 10, 10);
    
    event.skip(true);
});
```

**Key Drawing Tips:**
- Always use `AutoBufferedPaintDC` for flicker-free drawing
- Set `BackgroundStyle::Paint` for custom-drawn widgets
- Call `event.skip(true)` to maintain proper event handling
- Use `panel.refresh(false, None)` to trigger redraws

### Step 5: Adding Animations

For animated widgets, use a timer-based approach:

```rust
// Create shared animation state
let animation_data = Rc::new(RefCell::new(AnimationState {
    progress: 0.0,
    is_animating: false,
    start_time: None,
}));

// Create timer
let timer = Rc::new(Timer::new(&panel));

// Mouse hover starts animation
let animation_data_hover = animation_data.clone();
let timer_hover = timer.clone();
panel.on_mouse_enter(move |event| {
    let mut anim = animation_data_hover.borrow_mut();
    anim.is_animating = true;
    anim.start_time = Some(Instant::now());
    drop(anim);
    
    timer_hover.start(16, false); // 60fps
    event.skip(true);
});

// Timer updates animation
let animation_data_timer = animation_data.clone();
timer.on_tick(move |event| {
    let mut anim = animation_data_timer.borrow_mut();
    
    if let Some(start) = anim.start_time {
        let elapsed = start.elapsed().as_millis() as f32;
        let duration = 300.0; // milliseconds
        anim.progress = (elapsed / duration).min(1.0);
        
        // Trigger redraw
        panel.refresh(false, None);
        
        // Stop when complete
        if anim.progress >= 1.0 {
            anim.is_animating = false;
            timer.stop();
        }
    }
    
    event.skip(true);
});
```

### Step 6: Event Handling Best Practices

**Always call `event.skip(true)`** to ensure proper event propagation:

```rust
panel.on_mouse_left_down(move |event| {
    // Your custom handling
    println!("Custom widget clicked!");
    
    // IMPORTANT: Let other handlers process the event too
    event.skip(true);
});
```

**Chain multiple event handlers** for complex interactions:

```rust
// Internal widget logic
panel.on_mouse_left_down(move |event| {
    // Widget's internal click handling
    handle_internal_click();
    event.skip(true);
});

// External usage can add more handlers
my_widget.on_mouse_left_down(|event| {
    // Application-specific click handling
    println!("Application received click!");
    event.skip(true);
});
```

### Step 7: Using Your Custom Widget

Once defined, use your custom widget like any built-in widget:

```rust
let my_widget = MyCustomWidget::builder(&parent)
    .with_text("Hello World!".to_string())
    .with_background_color(Colour::new(200, 200, 255, 255))
    .with_size(Size::new(200, 100))
    .build();

// Add to layout
sizer.add(&my_widget, 0, SizerFlag::All, 10);

// Bind external events
my_widget.on_mouse_left_down(|event| {
    println!("Widget was clicked!");
    event.skip(true);
});
```

### Advanced Patterns

#### State Management with RefCell and Rc

For widgets with complex internal state:

```rust
#[derive(Debug)]
struct WidgetState {
    current_value: f32,
    animation_progress: f32,
    is_pressed: bool,
}

let state = Rc::new(RefCell::new(WidgetState {
    current_value: 0.0,
    animation_progress: 0.0,
    is_pressed: false,
}));

// Share state between event handlers
let state_paint = state.clone();
let state_mouse = state.clone();
```

#### Data Visualization Widgets

For widgets that display data (like the PieChart):

```rust
fields: {
    labels: Vec<String> = vec!["A".to_string(), "B".to_string()],
    values: Vec<f32> = vec![30.0, 70.0],
    colors: Vec<Colour> = vec![],
},

setup_impl: |config, panel| {
    // Process data into drawable format
    let processed_data = process_chart_data(&config.labels, &config.values);
    
    // Store in shared state for drawing
    let chart_data = Rc::new(RefCell::new(processed_data));
    
    // Use in paint handler
    let chart_data_paint = chart_data.clone();
    panel.on_paint(move |event| {
        let data = chart_data_paint.borrow();
        draw_chart(&panel, &data);
        event.skip(true);
    });
}
```

#### Mouse Interaction and Hit Testing

For interactive widgets that respond to mouse position:

```rust
panel.on_mouse_motion(move |event| {
    if let WindowEventData::MouseMotion(ref motion_event) = event {
        if let Some(pos) = motion_event.get_position() {
            // Hit testing logic
            let hit_item = find_item_at_position(pos.x, pos.y);
            
            // Update hover state
            update_hover_state(hit_item);
            
            // Trigger redraw if needed
            panel.refresh(false, None);
        }
    }
    event.skip(true);
});
```

### Common Patterns and Tips

1. **Flicker-free Drawing**: Always use `AutoBufferedPaintDC` and `BackgroundStyle::Paint`

2. **Smooth Animations**: Use 60fps timers (`16ms` intervals) for smooth animations

3. **Event Propagation**: Always call `event.skip(true)` unless you specifically want to stop event propagation

4. **State Sharing**: Use `Rc<RefCell<T>>` for sharing mutable state between event handlers

5. **Performance**: Only call `panel.refresh()` when the visual state actually changes

6. **Color Management**: Use the `Colour::new(r, g, b, a)` constructor for consistent color handling

7. **Layout Integration**: Custom widgets work seamlessly with wxDragon's sizer-based layout system

### Example: Simple Custom Button

Here's a complete minimal example:

```rust
custom_widget!(
    name: SimpleButton,
    fields: {
        text: String = "Click Me".to_string(),
        is_pressed: bool = false,
    },
    setup_impl: |config, panel| {
        panel.set_background_style(BackgroundStyle::Paint);
        
        let pressed_state = Rc::new(RefCell::new(false));
        
        // Draw the button
        let config_paint = config.clone();
        let pressed_paint = pressed_state.clone();
        panel.on_paint(move |event| {
            let dc = AutoBufferedPaintDC::new(&panel);
            let size = panel.get_size();
            let is_pressed = *pressed_paint.borrow();
            
            // Choose colors based on state
            let bg_color = if is_pressed {
                Colour::new(180, 180, 180, 255)
            } else {
                Colour::new(220, 220, 220, 255)
            };
            
            // Draw background
            dc.set_brush(bg_color, BrushStyle::Solid);
            dc.draw_rectangle(0, 0, size.width, size.height);
            
            // Draw text
            dc.set_text_foreground(Colour::new(0, 0, 0, 255));
            let text_size = dc.get_text_extent(&config_paint.text);
            let x = (size.width - text_size.0) / 2;
            let y = (size.height - text_size.1) / 2;
            dc.draw_text(&config_paint.text, x, y);
            
            event.skip(true);
        });
        
        // Handle mouse press
        let pressed_down = pressed_state.clone();
        panel.on_mouse_left_down(move |event| {
            *pressed_down.borrow_mut() = true;
            panel.refresh(false, None);
            event.skip(true);
        });
        
        // Handle mouse release
        let pressed_up = pressed_state.clone();
        panel.on_mouse_left_up(move |event| {
            *pressed_up.borrow_mut() = false;
            panel.refresh(false, None);
            event.skip(true);
        });
    }
);
```

This example demonstrates all the core concepts needed to create effective custom widgets in wxDragon.

## Learning Resources

- Study the `AniFillButton` implementation for animation techniques
- Examine the `PieChart` for data visualization patterns
- Refer to the main wxDragon examples for layout and event handling
- Check the wxDragon widget source code for advanced patterns

The `custom_widget!` macro provides a powerful foundation for creating any type of custom widget, from simple controls to complex data visualizations. 