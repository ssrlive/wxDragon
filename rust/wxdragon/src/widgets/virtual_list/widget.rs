use crate::prelude::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use super::{
    adaptive_pool::AdaptiveItemPool,
    error::{IntoVirtualListError, VirtualListError, VirtualListResult},
    item_context::ItemContext,
};

// Global registry for panel-to-context mapping removed
// TASK 3.1: Global registry removed - replaced with instance-owned registry for memory safety
// PHASE 3: ItemContext moved to item_context.rs module for better code organization

/// Trait for providing data to the virtual list
pub trait VirtualListDataSource: Send + Sync {
    /// Get the total number of items
    fn get_item_count(&self) -> usize;

    /// Get the data for a specific item
    fn get_item_data(&self, index: usize) -> Box<dyn Any + Send + Sync>;
}

/// Trait for rendering items in the virtual list
pub trait VirtualListItemRenderer: Send + Sync {
    /// Create a new panel for rendering an item
    fn create_item(&self, parent: &Panel) -> Panel;

    /// Update the content of a panel for a specific item with its data
    fn update_item(&self, panel: &Panel, index: usize, data: &dyn Any);
}

/// Layout modes for the virtual list
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VirtualListLayoutMode {
    /// Items arranged vertically (rows), each item stretches to container width
    Vertical,
    /// Items arranged horizontally (columns), each item stretches to container height
    Horizontal,
}

/// Item sizing behavior for cache invalidation strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemSizingMode {
    /// Items have fixed size regardless of container width (images, fixed text, etc.)
    /// Cache entries are preserved when width changes for better performance
    FixedSize,
    /// Items resize based on container width (text wrapping, responsive layouts, etc.)
    /// Cache entries are invalidated when width changes to ensure correct layout
    DynamicSize,
}

/// Internal state for the virtual list
pub struct VirtualListState {
    // Configuration
    layout_mode: VirtualListLayoutMode,
    internal_params: super::config::VirtualListInternalParams,

    // Content sources
    data_source: Option<Rc<dyn VirtualListDataSource>>,
    item_renderer: Option<Rc<dyn VirtualListItemRenderer>>,

    // Pooling and layout
    item_pool: AdaptiveItemPool,

    // Currently visible items (index -> panel)
    item_to_panel: HashMap<usize, Panel>,

    // Size caching for performance with selective invalidation
    item_size_cache: HashMap<usize, Size>,
    item_sizing_mode: ItemSizingMode,
    cache_generation: u64,

    // TASK 2.4: Measurement deduplication tracking
    current_update_cycle: u64,
    measured_in_current_cycle: HashSet<usize>,

    // Viewport tracking
    viewport_size: Size,
    scroll_position: Point,
    total_content_size: Size,
    visible_range: std::ops::Range<usize>,

    // Track viewport width changes to force content re-layout when needed
    previous_viewport_width: i32,

    // TASK 3.1: Owned panel context registry (replaces global static)
    panel_context_registry: HashMap<i32, ItemContext>,
}

impl std::fmt::Debug for VirtualListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualListState")
            .field("layout_mode", &self.layout_mode)
            .field("data_source", &self.data_source.is_some())
            .field("item_renderer", &self.item_renderer.is_some())
            .field("item_pool", &self.item_pool)
            .field(
                "item_to_panel",
                &format!("{} items", self.item_to_panel.len()),
            )
            .field(
                "item_size_cache",
                &format!("{} cached sizes", self.item_size_cache.len()),
            )
            .field("cache_generation", &self.cache_generation)
            .field("viewport_size", &self.viewport_size)
            .field("scroll_position", &self.scroll_position)
            .field("total_content_size", &self.total_content_size)
            .field("visible_range", &self.visible_range)
            .finish()
    }
}

impl VirtualListState {
    fn new(
        layout_mode: VirtualListLayoutMode,
        scrollbar_size: Option<i32>,
        keyboard_scroll_amount: Option<i32>,
        mouse_wheel_multiplier: f32,
    ) -> Self {
        // Create intelligent internal params that auto-configure based on default viewport
        let internal_params = super::config::VirtualListInternalParams::new_for_viewport(
            (800, 600), // Default viewport for initialization
            scrollbar_size,
            keyboard_scroll_amount,
            mouse_wheel_multiplier,
        );

        Self {
            layout_mode,
            internal_params: internal_params.clone(),
            data_source: None,
            item_renderer: None,
            item_pool: AdaptiveItemPool::new(layout_mode, &internal_params),
            item_to_panel: HashMap::new(),
            item_size_cache: HashMap::new(),
            item_sizing_mode: ItemSizingMode::DynamicSize, // Default to dynamic sizing for safety
            cache_generation: 0,

            // TASK 2.4: Initialize measurement deduplication tracking
            current_update_cycle: 0,
            measured_in_current_cycle: HashSet::new(),

            viewport_size: Size::new(0, 0),
            scroll_position: Point::new(0, 0),
            total_content_size: Size::new(0, 0),
            visible_range: 0..0,
            previous_viewport_width: 0,

            // TASK 3.1: Initialize owned panel context registry
            panel_context_registry: HashMap::new(),
        }
    }

    fn set_data_source(&mut self, data_source: Rc<dyn VirtualListDataSource>) {
        self.data_source = Some(data_source);
        self.clear_size_cache();
    }

    fn set_item_renderer(&mut self, item_renderer: Rc<dyn VirtualListItemRenderer>) {
        self.item_renderer = Some(item_renderer);
        self.clear_size_cache();
    }

    fn invalidate_layout(&mut self) {
        // Clear current visible items
        self.hide_all_items();

        // Clear size cache since layout has changed
        self.clear_size_cache();

        // Total content size will be recalculated when items are actually rendered
        // in update_visible_items where we have access to the parent panel
    }

    fn hide_all_items(&mut self) {
        // Return all panels to the pool and clean up their context
        let panels_to_clean: Vec<Panel> =
            self.item_to_panel.drain().map(|(_, panel)| panel).collect();

        for panel in panels_to_clean {
            self.remove_panel_context(&panel);
            self.item_pool.return_item(panel);
        }
        self.visible_range = 0..0;
    }

    fn update_visible_items(&mut self, parent: &Panel) {
        // TASK 2.4: Start new measurement cycle to track deduplication
        self.current_update_cycle = self.current_update_cycle.wrapping_add(1);
        self.measured_in_current_cycle.clear();

        if let (Some(ref data_source), Some(ref item_renderer)) =
            (&self.data_source, &self.item_renderer)
        {
            // Clone the references to avoid borrowing conflicts
            let data_source = data_source.clone();
            let item_renderer = item_renderer.clone();

            let total_items = data_source.get_item_count();

            if total_items == 0 {
                return;
            }

            // Track viewport width changes to detect when we need to force content re-layout
            let current_viewport_width = self.viewport_size.width;
            // Only trigger width change handling for significant changes (adaptive threshold)
            // This prevents excessive cache clearing and re-measurement during minor resize operations
            let width_change_threshold = self.internal_params.width_change_threshold;
            let width_changed = self.previous_viewport_width != 0
                && (current_viewport_width - self.previous_viewport_width).abs()
                    >= width_change_threshold;

            // PHASE 2 OPTIMIZATION: Selective Cache Invalidation
            // Instead of clearing ALL cache, only invalidate items that actually need re-measurement
            if width_changed {
                self.selective_cache_invalidation();
            }

            // CRITICAL: Always update previous width, regardless of change detection
            self.previous_viewport_width = current_viewport_width;

            // PROGRESSIVE MEASUREMENT: Only measure items as they become visible

            // Step 1: Calculate visible range using estimated heights first
            let estimated_item_height = self.internal_params.estimated_item_height;
            let viewport_height = self.viewport_size.height;
            let scroll_y = self.scroll_position.y;

            // Estimate which items might be visible
            let estimated_start_index = if estimated_item_height > 0 {
                (scroll_y / estimated_item_height).max(0) as usize
            } else {
                0
            }
            .min(total_items.saturating_sub(1));

            let estimated_visible_count = if estimated_item_height > 0 {
                (viewport_height / estimated_item_height + 2) as usize // +2 for buffer
            } else {
                10 // fallback
            }
            .min(total_items.saturating_sub(estimated_start_index));

            let estimated_end_index =
                (estimated_start_index + estimated_visible_count).min(total_items);

            // Step 2: Find actual visible items by checking a reasonable range
            let mut current_y = 0;
            let mut items_to_show: Vec<(usize, i32, i32)> = Vec::new();

            // Calculate positions using mix of actual measurements and estimates
            for index in 0..total_items {
                let item_height = if let Some(cached_size) = self.item_size_cache.get(&index) {
                    // Use actual measurement if available
                    match self.layout_mode {
                        VirtualListLayoutMode::Vertical => cached_size.height,
                        VirtualListLayoutMode::Horizontal => cached_size.width,
                    }
                } else if index >= estimated_start_index && index < estimated_end_index {
                    // Measure items in the estimated visible range
                    let item_size = self.measure_item_size_for_visible(
                        index,
                        data_source.as_ref(),
                        item_renderer.as_ref(),
                        parent,
                    );
                    match self.layout_mode {
                        VirtualListLayoutMode::Vertical => item_size.height,
                        VirtualListLayoutMode::Horizontal => item_size.width,
                    }
                } else {
                    // Use estimate for items outside visible range
                    estimated_item_height
                };

                // Check if this item is visible
                let item_top = current_y;
                let item_bottom = current_y + item_height;

                if item_bottom > scroll_y && item_top < scroll_y + viewport_height {
                    items_to_show.push((index, item_top, item_height));
                }

                current_y += item_height;

                // Early termination: stop well past the visible area
                if item_top
                    > scroll_y + viewport_height + self.internal_params.early_termination_threshold
                {
                    break;
                }
            }

            // Step 3: Update total content size using mix of actual + estimated
            self.total_content_size =
                self.calculate_total_content_size_progressive(total_items, estimated_item_height);

            // Check if the visible items have actually changed
            let current_visible_items: HashSet<usize> =
                self.item_to_panel.keys().cloned().collect();
            let new_visible_items: HashSet<usize> =
                items_to_show.iter().map(|(idx, _, _)| *idx).collect();

            // Only update if the set of visible items has changed
            if current_visible_items != new_visible_items {
                // Hide panels for items that are no longer visible
                let items_to_hide: Vec<usize> = current_visible_items
                    .difference(&new_visible_items)
                    .cloned()
                    .collect();
                for item_index in items_to_hide {
                    if let Some(panel) = self.item_to_panel.remove(&item_index) {
                        self.remove_panel_context(&panel);
                        // Return panel to pool for reuse
                        self.item_pool.return_item(panel);
                    }
                }

                // Show/create panels for new visible items
                for (data_index, _, _) in &items_to_show {
                    if !self.item_to_panel.contains_key(data_index) {
                        // Need to show this item - get from pool or create new one
                        let item_panel = self
                            .item_pool
                            .get_or_create_item(parent, || item_renderer.create_item(parent));

                        // CRITICAL: Set proper width BEFORE updating content
                        item_panel.set_size(Size::new(
                            self.viewport_size.width,
                            self.internal_params.temporary_panel_height,
                        )); // Temporary height

                        // Get item data and update panel content
                        let item_data = data_source.get_item_data(*data_index);
                        item_renderer.update_item(&item_panel, *data_index, item_data.as_ref());

                        // Force layout so sizers can do their work and we get real size
                        item_panel.layout();

                        // Store context for safe event handling
                        self.store_item_context(&item_panel, *data_index, item_data.as_ref());

                        // Track this panel for this data index
                        self.item_to_panel.insert(*data_index, item_panel);
                    }
                }
            }

            // Handle width changes by forcing complete re-layout with BATCHED OPERATIONS
            if width_changed {
                // PHASE 2 OPTIMIZATION: Batched Layout Operations
                // OLD: 6 operations per item (set_size -> update_item -> layout -> get_best_size -> set_size -> move_window)
                // NEW: 2 operations per item (batch_prepare -> batch_apply)

                let mut batch_operations: Vec<(usize, Panel, Box<dyn Any + Send + Sync>)> =
                    Vec::new();

                // BATCH PHASE 1: Collect all items and prepare for batch processing
                for (data_index, _, _) in &items_to_show {
                    if let Some(panel) = self.item_to_panel.get(data_index) {
                        if let (Some(ref data_source), Some(ref _item_renderer)) =
                            (&self.data_source, &self.item_renderer)
                        {
                            let item_data = data_source.get_item_data(*data_index);
                            batch_operations.push((*data_index, panel.clone(), item_data));
                        }
                    }
                }

                // BATCH PHASE 2: Apply all size changes BEFORE any layout operations
                for (data_index, panel, item_data) in &batch_operations {
                    // Set width for all panels first (but keep temporary height)
                    panel.set_size(Size::new(
                        self.viewport_size.width,
                        self.internal_params.temporary_panel_height,
                    ));
                    // Update content with new width (applies text wrapping)
                    if let (Some(_data_source), Some(ref item_renderer)) =
                        (&self.data_source, &self.item_renderer)
                    {
                        item_renderer.update_item(panel, *data_index, item_data.as_ref());
                    }
                }

                // BATCH PHASE 3: Single layout pass for all panels
                for (_, panel, _) in &batch_operations {
                    panel.layout(); // This is now the ONLY layout call per item
                }

                // BATCH PHASE 4: Measure actual sizes and update cache
                let mut item_positions: Vec<(usize, i32, i32)> = Vec::new();
                let mut current_y = items_to_show.first().map(|(_, top, _)| *top).unwrap_or(0);

                for (data_index, panel, _) in &batch_operations {
                    let new_best_size = panel.get_best_size();
                    let actual_height_needed = new_best_size.height;

                    // Update cache with new measurements
                    self.item_size_cache.insert(
                        *data_index,
                        Size::new(self.viewport_size.width, actual_height_needed),
                    );

                    // Record position for this item
                    item_positions.push((*data_index, current_y, actual_height_needed));
                    current_y += actual_height_needed;
                }

                // BATCH PHASE 5: Apply final sizes and positions in one pass
                for (data_index, item_top, actual_height) in &item_positions {
                    if let Some(panel) = self.item_to_panel.get(data_index) {
                        let item_y_position = item_top - scroll_y;
                        // Final size and position - this is the ONLY positioning call per item
                        panel.set_size(Size::new(self.viewport_size.width, *actual_height));
                        panel.move_window(0, item_y_position);
                        panel.show(true);
                    }
                }

                // TASK 2.5: Simplified additional item detection
                self.handle_additional_visible_items(
                    &data_source,
                    &item_renderer,
                    parent,
                    current_y,
                    scroll_y,
                );

                // Update total content size after all changes
                self.total_content_size = self.calculate_total_content_size_progressive(
                    total_items,
                    self.internal_params.estimated_item_height,
                );
            } else {
                // TASK 2.5: Simplified normal case - unified positioning logic
                self.position_items_with_optimal_sizing(items_to_show, scroll_y);
            }
        }
    }

    /// Measure the actual size of an item by creating, updating, and laying out a temporary panel
    /// PHASE 2B: Enhanced measurement with deduplication tracking
    fn measure_item_size(
        &mut self,
        index: usize,
        data_source: &dyn VirtualListDataSource,
        item_renderer: &dyn VirtualListItemRenderer,
        parent: &Panel,
    ) -> Size {
        // TASK 2.4: Check if already measured in current cycle
        if self.measured_in_current_cycle.contains(&index) {
            // Already measured in this cycle - check cache
            if let Some(cached_size) = self.item_size_cache.get(&index) {
                return *cached_size;
            }
        }

        // Check cache first (normal cache check)
        if let Some(cached_size) = self.item_size_cache.get(&index) {
            // Mark as measured in current cycle even for cache hits
            self.measured_in_current_cycle.insert(index);
            return *cached_size;
        }

        // Mark as measured in current cycle before performing expensive measurement
        self.measured_in_current_cycle.insert(index);

        // Create temporary panel for measurement
        let temp_panel = item_renderer.create_item(parent);

        // CRITICAL: Set proper width BEFORE updating content for accurate measurement
        temp_panel.set_size(Size::new(self.viewport_size.width, 100)); // Temporary height

        let item_data = data_source.get_item_data(index);
        item_renderer.update_item(&temp_panel, index, item_data.as_ref());

        // CRITICAL: Force layout so sizers can do their work
        temp_panel.layout();
        let measured_size = temp_panel.get_best_size();

        // Hide temp panel (cleanup)
        temp_panel.show(false);

        // Cache the result with current viewport width
        let cached_size = Size::new(self.viewport_size.width, measured_size.height);
        self.item_size_cache.insert(index, cached_size);

        cached_size
    }

    /// PHASE 2B: Measure an item that is about to become visible with deduplication  
    fn measure_item_size_for_visible(
        &mut self,
        index: usize,
        data_source: &dyn VirtualListDataSource,
        item_renderer: &dyn VirtualListItemRenderer,
        parent: &Panel,
    ) -> Size {
        // TASK 2.4: Check if already measured in current cycle
        if self.measured_in_current_cycle.contains(&index) {
            // Already measured in this cycle - return cached result
            if let Some(cached_size) = self.item_size_cache.get(&index) {
                return *cached_size;
            }
        }

        // Mark as measured in current cycle before measuring
        self.measured_in_current_cycle.insert(index);

        // Perform measurement (same as measure_item_size but with deduplication tracking)
        self.measure_item_size(index, data_source, item_renderer, parent)
    }

    /// Calculate total content size using progressive measurement approach
    fn calculate_total_content_size_progressive(
        &self,
        total_items: usize,
        estimated_item_height: i32,
    ) -> Size {
        let mut total_height = 0;

        for index in 0..total_items {
            let item_height = if let Some(cached_size) = self.item_size_cache.get(&index) {
                // Use actual measurement if we have it
                match self.layout_mode {
                    VirtualListLayoutMode::Vertical => cached_size.height,
                    VirtualListLayoutMode::Horizontal => cached_size.width,
                }
            } else {
                // Use estimate for non-measured items
                estimated_item_height
            };

            total_height += item_height;
        }

        match self.layout_mode {
            VirtualListLayoutMode::Vertical => Size::new(self.viewport_size.width, total_height),
            VirtualListLayoutMode::Horizontal => Size::new(total_height, self.viewport_size.height),
        }
    }

    /// Clear the entire size cache and increment generation
    fn clear_size_cache(&mut self) {
        self.item_size_cache.clear();
        self.cache_generation += 1;
    }

    /// PHASE 2 OPTIMIZATION: Selective Cache Invalidation
    /// Only invalidate cache entries when items have dynamic sizing behavior
    fn selective_cache_invalidation(&mut self) {
        match self.item_sizing_mode {
            ItemSizingMode::DynamicSize => {
                // Items resize based on container width - invalidate all cache entries
                self.item_size_cache.clear();
            }
            ItemSizingMode::FixedSize => {
                // Items have fixed size - preserve cache entries for better performance
                // Do nothing - all cached measurements remain valid
            }
        }

        self.cache_generation = self.cache_generation.wrapping_add(1);
    }

    // Content type detection removed - replaced with user-controlled item_sizing_mode
    // Users now explicitly specify whether their items have dynamic sizing

    /// PHASE 3: Store item context using owned registry (replaces global registry)
    fn store_item_context(&mut self, panel: &Panel, index: usize, data: &dyn Any) {
        // Store current item index and data in panel context registry
        // This ensures event handlers always get the current item context
        let panel_id = panel.get_id();

        // For now, we'll store a simplified version of the data
        // In a real implementation, data sources would provide Clone + Send + Sync data
        let stored_data: Arc<dyn Any + Send + Sync> =
            if let Some(string_data) = data.downcast_ref::<String>() {
                Arc::new(string_data.clone())
            } else {
                Arc::new(format!("Item {index}"))
            };

        let context = ItemContext {
            index,
            data: stored_data,
        };

        // TASK 3.1: Use owned registry instead of global static
        self.panel_context_registry.insert(panel_id, context);
    }

    /// PHASE 3: Remove panel context using owned registry
    fn remove_panel_context(&mut self, panel: &Panel) {
        let panel_id = panel.get_id();
        // TASK 3.1: Use owned registry instead of global static
        self.panel_context_registry.remove(&panel_id);
    }

    fn hit_test(&self, point: Point) -> Option<usize> {
        // Simple hit test based on currently visible items
        for (&index, panel) in &self.item_to_panel {
            let panel_pos = panel.get_position();
            let panel_size = panel.get_size();

            if point.x >= panel_pos.x
                && point.x < panel_pos.x + panel_size.width
                && point.y >= panel_pos.y
                && point.y < panel_pos.y + panel_size.height
            {
                return Some(index);
            }
        }

        None
    }

    fn clear_all_items(&mut self) {
        // Clear all items and return panels to pool
        self.hide_all_items();
        self.item_pool.clear_all();
    }

    /// TASK 2.5: Simplified additional item detection with batched operations
    fn handle_additional_visible_items(
        &mut self,
        data_source: &Rc<dyn VirtualListDataSource>,
        item_renderer: &Rc<dyn VirtualListItemRenderer>,
        parent: &Panel,
        last_item_bottom: i32,
        scroll_y: i32,
    ) {
        let total_items = data_source.get_item_count();
        let viewport_bottom = scroll_y + self.viewport_size.height;

        // Early exit if no room for additional items
        if last_item_bottom >= viewport_bottom {
            return;
        }

        // Find last visible item to continue from there
        let last_visible_item = self.item_to_panel.keys().max().copied().unwrap_or(0);

        // Simple loop to add items that fit in viewport
        let mut current_y = last_item_bottom;
        for next_item_index in (last_visible_item + 1)..total_items {
            if current_y >= viewport_bottom {
                break;
            }

            // Create panel if needed
            if !self.item_to_panel.contains_key(&next_item_index) {
                let item_panel = self
                    .item_pool
                    .get_or_create_item(parent, || item_renderer.create_item(parent));

                // Setup content
                let item_data = data_source.get_item_data(next_item_index);
                item_panel.set_size(Size::new(
                    self.viewport_size.width,
                    self.internal_params.temporary_panel_height,
                ));
                item_renderer.update_item(&item_panel, next_item_index, item_data.as_ref());
                item_panel.layout();

                // Position and show
                let actual_size = item_panel.get_best_size();
                let actual_height = actual_size.height;
                item_panel.set_size(Size::new(self.viewport_size.width, actual_height));
                item_panel.move_window(0, current_y - scroll_y);
                item_panel.show(true);

                // Update tracking
                self.store_item_context(&item_panel, next_item_index, item_data.as_ref());
                self.item_to_panel.insert(next_item_index, item_panel);
                self.item_size_cache.insert(
                    next_item_index,
                    Size::new(self.viewport_size.width, actual_height),
                );

                current_y += actual_height;
            }
        }
    }

    /// TASK 2.5: Simplified positioning logic with optimal sizing
    fn position_items_with_optimal_sizing(
        &mut self,
        items_to_show: Vec<(usize, i32, i32)>,
        scroll_y: i32,
    ) {
        // Simple, unified approach: ensure each item has correct size and position
        for (data_index, item_top, estimated_height) in items_to_show {
            if let Some(panel) = self.item_to_panel.get(&data_index) {
                let current_size = panel.get_size();
                let mut needs_content_refresh = false;
                let mut actual_height = estimated_height;

                // For DynamicSize mode, always refresh content for newly visible items during rapid scrolling
                // This ensures text wrapping is recalculated correctly after scrollbar drags
                if self.item_sizing_mode == ItemSizingMode::DynamicSize {
                    // Check if this item might have stale content (cache miss or size mismatch)
                    let has_cached_size = self.item_size_cache.contains_key(&data_index);
                    let size_mismatch = current_size.width != self.viewport_size.width;

                    if !has_cached_size || size_mismatch {
                        needs_content_refresh = true;
                    }
                }

                // Get cached height or prepare for measurement
                if let Some(cached_size) = self.item_size_cache.get(&data_index) {
                    actual_height = cached_size.height;
                } else if self.item_sizing_mode == ItemSizingMode::DynamicSize {
                    // No cache entry for dynamic sizing - must measure
                    needs_content_refresh = true;
                }

                let expected_size = Size::new(self.viewport_size.width, actual_height);

                // Update size if needed
                if current_size != expected_size {
                    panel.set_size(expected_size);
                    needs_content_refresh = true;
                }

                // Refresh content and re-measure if needed
                if needs_content_refresh {
                    if let (Some(ref data_source), Some(ref item_renderer)) =
                        (&self.data_source, &self.item_renderer)
                    {
                        let item_data = data_source.get_item_data(data_index);
                        item_renderer.update_item(panel, data_index, item_data.as_ref());
                        panel.layout();

                        // Get fresh measurement and update cache
                        let new_size = panel.get_best_size();
                        self.item_size_cache.insert(
                            data_index,
                            Size::new(self.viewport_size.width, new_size.height),
                        );

                        // Update panel with correct final size
                        let final_size = Size::new(self.viewport_size.width, new_size.height);
                        if panel.get_size() != final_size {
                            panel.set_size(final_size);
                        }
                    }
                }

                // Position and show
                panel.move_window(0, item_top - scroll_y);
                panel.show(true);
            }
        }
    }

    /// PHASE 3: Get item context for a panel using owned registry
    fn get_item_context_for_panel(&self, panel: &Panel) -> Option<ItemContext> {
        let panel_id = panel.get_id();
        self.panel_context_registry.get(&panel_id).cloned()
    }

    /// PHASE 3: Get item index for a panel (convenience method)
    fn get_index_for_panel(&self, panel: &Panel) -> Option<usize> {
        self.get_item_context_for_panel(panel).map(|ctx| ctx.index)
    }

    /// PHASE 3: Get typed data for a panel (convenience method)
    fn get_data_for_panel<T>(&self, panel: &Panel) -> Option<T>
    where
        T: Clone + 'static,
    {
        self.get_item_context_for_panel(panel)
            .and_then(|ctx| ctx.data.downcast_ref::<T>().cloned())
    }
}

// TASK 3.1: Add Drop implementation for automatic cleanup
impl Drop for VirtualListState {
    fn drop(&mut self) {
        // Clear the panel context registry to prevent memory leaks
        // This ensures all contexts are cleaned up when the virtual list is destroyed
        self.panel_context_registry.clear();
    }
}

custom_widget!(
    name: VirtualList,
    fields: {
        layout_mode: VirtualListLayoutMode = VirtualListLayoutMode::Vertical,
        scrollbar_size: Option<i32> = None,
        keyboard_scroll_amount: Option<i32> = None,
        mouse_wheel_multiplier: f32 = 1.0,
        state: Rc<RefCell<VirtualListState>> = Rc::new(RefCell::new(VirtualListState::new(
            VirtualListLayoutMode::Vertical,
            None,  // scrollbar_size
            None,  // keyboard_scroll_amount
            1.0,   // mouse_wheel_multiplier
        ))),
    },
    setup_impl: |config, panel| {
        // Set up the panel for virtual list behavior with proper clipping
        panel.set_background_style(BackgroundStyle::System); // Use default system background

        // Enable clipping of child windows to prevent items from appearing outside bounds
        panel.add_style(WindowStyle::ClipChildren);

        // TODO: Add a simple border to define the virtual list area when border API is available

        // VirtualList setup complete

        // Create scrollbars based on layout mode
        let panel_size = panel.get_size();
        let scrollbar_size = config.scrollbar_size.unwrap_or(16); // User preference or system default

        let (content_area, scrollbar) = match config.layout_mode {
            VirtualListLayoutMode::Vertical => {
                // Create vertical scrollbar on the right side
                let scrollbar = crate::widgets::ScrollBar::builder(&panel)
                    .with_style(crate::widgets::ScrollBarStyle::Vertical)
                    .with_pos(Point::new(panel_size.width - scrollbar_size, 0))
                    .with_size(Size::new(scrollbar_size, panel_size.height))
                    .build();

                // Content area is full height, reduced width
                let content_area = Size::new(panel_size.width - scrollbar_size, panel_size.height);
                (content_area, Some((scrollbar, true))) // true = vertical
            },
            VirtualListLayoutMode::Horizontal => {
                // Create horizontal scrollbar at the bottom
                let scrollbar = crate::widgets::ScrollBar::builder(&panel)
                    .with_style(crate::widgets::ScrollBarStyle::Default) // Default = horizontal
                    .with_pos(Point::new(0, panel_size.height - scrollbar_size))
                    .with_size(Size::new(panel_size.width, scrollbar_size))
                    .build();

                // Content area is full width, reduced height
                let content_area = Size::new(panel_size.width, panel_size.height - scrollbar_size);
                (content_area, Some((scrollbar, false))) // false = horizontal
            }
        };

        // Store scrollbar references for event handling
        let (v_scrollbar, h_scrollbar) = if let Some((scrollbar_widget, is_vertical)) = scrollbar {
            if is_vertical {
                // Initialize vertical scrollbar with proper values
                scrollbar_widget.set_scrollbar(0, 20, 100, 20, true);
                scrollbar_widget.enable(true);  // Explicitly enable scrollbar
                scrollbar_widget.show(true);    // Explicitly show scrollbar
                (Some(scrollbar_widget), None)
            } else {
                // Initialize horizontal scrollbar with proper values
                scrollbar_widget.set_scrollbar(0, 20, 100, 20, true);
                scrollbar_widget.enable(true);  // Explicitly enable scrollbar
                scrollbar_widget.show(true);    // Explicitly show scrollbar
                (None, Some(scrollbar_widget))
            }
        } else {
            (None, None)
        };

        // Initialize the state with the actual layout mode and content area
        {
            let mut state = config.state.borrow_mut();
            state.layout_mode = config.layout_mode;
            // Use content area (excluding scrollbar) as viewport
            state.viewport_size = content_area;
        }

        // Function to update scrollbar properties
        let v_scrollbar_update = v_scrollbar.clone();
        let h_scrollbar_update = h_scrollbar.clone();
        let layout_mode_update = config.layout_mode;
        let state_update = config.state.clone();
        let update_scrollbars = move || {
            let state = state_update.borrow();
            match layout_mode_update {
                VirtualListLayoutMode::Vertical => {
                    if let Some(ref vscrollbar) = v_scrollbar_update {
                        let max_scroll = (state.total_content_size.height - state.viewport_size.height).max(1);
                        let current_pos = if max_scroll > 0 {
                            (state.scroll_position.y * 100 / max_scroll).min(100)
                        } else {
                            0
                        };

                        // Set scrollbar: position, thumb_size, range, page_size, refresh
                        // thumb_size represents visible portion, page_size is for page up/down
                        let thumb_size = if state.total_content_size.height > 0 {
                            ((state.viewport_size.height * 100) / state.total_content_size.height).clamp(5, 95)
                        } else {
                            95
                        };

                        vscrollbar.set_scrollbar(current_pos, thumb_size, 100, thumb_size, true);
                    }
                },
                VirtualListLayoutMode::Horizontal => {
                    if let Some(ref hscrollbar) = h_scrollbar_update {
                        let max_scroll = (state.total_content_size.width - state.viewport_size.width).max(1);
                        let current_pos = if max_scroll > 0 {
                            (state.scroll_position.x * 100 / max_scroll).min(100)
                        } else {
                            0
                        };

                        // Set scrollbar: position, thumb_size, range, page_size, refresh
                        let thumb_size = if state.total_content_size.width > 0 {
                            ((state.viewport_size.width * 100) / state.total_content_size.width).clamp(5, 95)
                        } else {
                            95
                        };

                        hscrollbar.set_scrollbar(current_pos, thumb_size, 100, thumb_size, true);
                    }
                }
            }
        };

        // Set up paint event for rendering visible items
        let update_scrollbars_paint = update_scrollbars.clone();
        panel.on_paint(move |event| {
            // Paint events should only draw existing content, not recalculate layout
            update_scrollbars_paint();
            event.skip(true);
        });

        // Set up resize event to update viewport and reposition scrollbars
        let panel_resize = panel.clone();
        let state_resize = config.state.clone();
        let layout_mode_resize = config.layout_mode;
        let vscrollbar_resize = v_scrollbar.clone();
        let hscrollbar_resize = h_scrollbar.clone();
        let update_scrollbars_resize = update_scrollbars.clone();

        panel.on_size(move |event| {
            let new_size = panel_resize.get_size();
            let scrollbar_size = 16;

            // Reposition scrollbars and calculate new content area
            let content_area = match layout_mode_resize {
                VirtualListLayoutMode::Vertical => {
                    if let Some(ref vscrollbar) = vscrollbar_resize {
                        vscrollbar.move_window(new_size.width - scrollbar_size, 0);
                        vscrollbar.set_size(Size::new(scrollbar_size, new_size.height));
                    }
                    Size::new(new_size.width - scrollbar_size, new_size.height)
                },
                VirtualListLayoutMode::Horizontal => {
                    if let Some(ref hscrollbar) = hscrollbar_resize {
                        hscrollbar.move_window(0, new_size.height - scrollbar_size);
                        hscrollbar.set_size(Size::new(new_size.width, scrollbar_size));
                    }
                    Size::new(new_size.width, new_size.height - scrollbar_size)
                }
            };

            let mut state_mut = state_resize.borrow_mut();
            let old_viewport_size = state_mut.viewport_size;
            state_mut.viewport_size = content_area;

            // Auto-reconfigure internal parameters when viewport size changes significantly
            if (old_viewport_size.width - content_area.width).abs() > 50 ||
               (old_viewport_size.height - content_area.height).abs() > 50 {
                state_mut.internal_params.auto_configure(
                    (content_area.width, content_area.height),
                    config.scrollbar_size,
                    config.keyboard_scroll_amount,
                    config.mouse_wheel_multiplier,
                );
            }

            state_mut.update_visible_items(&panel_resize);
            drop(state_mut); // Release borrow before calling update_scrollbars

            // Update scrollbars after viewport size change
            update_scrollbars_resize();
            panel_resize.refresh(false, None);
            event.skip(true);
        });

        // Set up mouse wheel scrolling with proper wheel delta handling
        let panel_scroll = panel.clone();
        let state_scroll = config.state.clone();
        let panel_scroll_wheel = panel_scroll.clone();
        let state_scroll_wheel = state_scroll.clone();
        let update_scrollbars_wheel = update_scrollbars.clone();

        // Mouse wheel with proper bidirectional scrolling using wheel rotation
        panel.on_mouse_wheel(move |event| {
            // Extract wheel rotation from event data for bidirectional scrolling
            let (wheel_rotation, _wheel_delta) = match event {
                crate::event::window_events::WindowEventData::MouseButton(ref mouse_event) => {
                    (mouse_event.event.get_wheel_rotation(), mouse_event.event.get_wheel_delta())
                },
                crate::event::window_events::WindowEventData::General(ref general_event) => {
                    (general_event.get_wheel_rotation(), general_event.get_wheel_delta())
                },
                _ => (0, 120)
            };

            // Calculate scroll amount - negative rotation means scroll down (increase scroll position)
            // Positive rotation means scroll up (decrease scroll position)
            let scroll_amount = -wheel_rotation; // Invert to match expected scroll direction

            if scroll_amount == 0 {
                event.skip(true);
                return;
            }

            let mut state_mut = state_scroll_wheel.borrow_mut();

            match state_mut.layout_mode {
                VirtualListLayoutMode::Vertical => {
                    let max_scroll = (state_mut.total_content_size.height - state_mut.viewport_size.height).max(0);
                    let new_scroll_y = (state_mut.scroll_position.y + scroll_amount)
                        .max(0)
                        .min(max_scroll);

                    if new_scroll_y != state_mut.scroll_position.y {
                        state_mut.scroll_position.y = new_scroll_y;
                        state_mut.update_visible_items(&panel_scroll_wheel);
                        drop(state_mut); // Release borrow before calling update_scrollbars

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_wheel();
                        panel_scroll_wheel.refresh(false, None);
                    }
                }
                VirtualListLayoutMode::Horizontal => {
                    let max_scroll = (state_mut.total_content_size.width - state_mut.viewport_size.width).max(0);
                    let new_scroll_x = (state_mut.scroll_position.x + scroll_amount)
                        .max(0)
                        .min(max_scroll);

                    if new_scroll_x != state_mut.scroll_position.x {
                        state_mut.scroll_position.x = new_scroll_x;
                        state_mut.update_visible_items(&panel_scroll_wheel);
                        drop(state_mut); // Release borrow before calling update_scrollbars

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_wheel();
                        panel_scroll_wheel.refresh(false, None);
                    }
                }
            }

            event.skip(true);
        });

        // Also keep keyboard controls as backup/alternative navigation
        // Arrow Down = scroll forward, Arrow Up = scroll backward
        let update_scrollbars_key = update_scrollbars.clone();
        panel.on_key_down(move |event| {
            let key_code = match event {
                crate::event::window_events::WindowEventData::Keyboard(ref kbd_event) => {
                    kbd_event.get_key_code().unwrap_or(0)
                }
                _ => 0
            };

            let scroll_delta = match key_code {
                314 => Some(300),  // Down arrow - scroll forward
                315 => Some(-300), // Up arrow - scroll backward
                _ => None
            };

            if let Some(delta) = scroll_delta {
                let mut state_mut = state_scroll.borrow_mut();

                match state_mut.layout_mode {
                    VirtualListLayoutMode::Vertical => {
                        let max_scroll = (state_mut.total_content_size.height - state_mut.viewport_size.height).max(0);
                        let new_scroll_y = (state_mut.scroll_position.y + delta)
                            .max(0)
                            .min(max_scroll);

                        if new_scroll_y != state_mut.scroll_position.y {
                            state_mut.scroll_position.y = new_scroll_y;
                            state_mut.update_visible_items(&panel_scroll);
                            drop(state_mut); // Release borrow before calling update_scrollbars

                            // Use central scrollbar update function for consistent behavior
                            update_scrollbars_key();
                            panel_scroll.refresh(false, None);
                        }
                    }
                    VirtualListLayoutMode::Horizontal => {
                        let max_scroll = (state_mut.total_content_size.width - state_mut.viewport_size.width).max(0);
                        let new_scroll_x = (state_mut.scroll_position.x + delta)
                            .max(0)
                            .min(max_scroll);

                    if new_scroll_x != state_mut.scroll_position.x {
                        state_mut.scroll_position.x = new_scroll_x;
                        state_mut.update_visible_items(&panel_scroll);
                            drop(state_mut); // Release borrow before calling update_scrollbars

                            // Use central scrollbar update function for consistent behavior
                            update_scrollbars_key();
                        panel_scroll.refresh(false, None);
                        }
                    }
                }
            }

            event.skip(true);
        });

        // Simplified scrollbar event handling - only thumb track for drag support
        if let Some(ref vscrollbar) = v_scrollbar {
            let panel_vscroll = panel.clone();
            let state_vscroll = config.state.clone();
            let update_scrollbars_vscroll = update_scrollbars.clone();

            // Only handle thumb track for smooth drag scrolling
            vscrollbar.on_thumb_track(move |event| {
                if let Some(position) = event.get_position() {
                    let mut state_mut = state_vscroll.borrow_mut();
                    let max_scroll = (state_mut.total_content_size.height - state_mut.viewport_size.height).max(0);

                    // Convert scrollbar position (0-95) to actual scroll position
                    // wxWidgets scrollbars typically max out at ~95% due to thumb size
                    let effective_max_position = 95.0;
                    let new_scroll_y = if max_scroll > 0 {
                        (((position as f32 * max_scroll as f32) / effective_max_position).round() as i32).max(0).min(max_scroll)
                    } else {
                        0
                    };



                    if new_scroll_y != state_mut.scroll_position.y {
                        state_mut.scroll_position.y = new_scroll_y;
                        state_mut.update_visible_items(&panel_vscroll);

                        // For DynamicSize mode, ensure a final layout pass after rapid scrolling
                        if state_mut.item_sizing_mode == ItemSizingMode::DynamicSize {
                            // Force layout on all visible panels to ensure proper text wrapping
                            for panel in state_mut.item_to_panel.values() {
                                panel.layout();
                            }
                        }

                        drop(state_mut); // Release borrow before calling update_scrollbars

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_vscroll();
                        panel_vscroll.refresh(false, None);
                    }
                }
            });
        }

        if let Some(ref hscrollbar) = h_scrollbar {
            let panel_hscroll = panel.clone();
            let state_hscroll = config.state.clone();
            let update_scrollbars_hscroll = update_scrollbars.clone();

            // Only handle thumb track for horizontal scrollbar drag
            hscrollbar.on_thumb_track(move |event| {
                if let Some(position) = event.get_position() {
                    let mut state_mut = state_hscroll.borrow_mut();
                    let max_scroll = (state_mut.total_content_size.width - state_mut.viewport_size.width).max(0);

                    // Convert scrollbar position (0-100) to actual scroll position
                    let new_scroll_x = if max_scroll > 0 {
                        (position * max_scroll / 100).max(0).min(max_scroll)
                    } else {
                        0
                    };

                    if new_scroll_x != state_mut.scroll_position.x {
                        state_mut.scroll_position.x = new_scroll_x;
                        state_mut.update_visible_items(&panel_hscroll);

                        // For DynamicSize mode, ensure a final layout pass after rapid scrolling
                        if state_mut.item_sizing_mode == ItemSizingMode::DynamicSize {
                            // Force layout on all visible panels to ensure proper text wrapping
                            for panel in state_mut.item_to_panel.values() {
                                panel.layout();
                            }
                        }

                        drop(state_mut); // Release borrow before calling update_scrollbars

                        // Use central scrollbar update function for consistent behavior
                        update_scrollbars_hscroll();
                        panel_hscroll.refresh(false, None);
                    }
                }
            });
        }

        // Set up virtual list event handling
        // TODO: Hit testing will be implemented later without interfering with child widgets

        // VirtualList setup completed
    }
);

impl VirtualList {
    /// Set the data source for the virtual list
    pub fn set_data_source<T: VirtualListDataSource + 'static>(&self, data_source: T) {
        // Setting data source

        {
            let mut state = self.config().state.borrow_mut();
            state.set_data_source(Rc::new(data_source));

            // Total content size will be recalculated on next update
        }

        self.refresh(false, None);

        // Trigger initial update
        self.trigger_initial_update();
    }

    /// Set the item renderer for the virtual list
    pub fn set_item_renderer<T: VirtualListItemRenderer + 'static>(&self, item_renderer: T) {
        // Setting item renderer

        {
            let mut state = self.config().state.borrow_mut();
            state.set_item_renderer(Rc::new(item_renderer));

            // Total content size will be recalculated on next update
        }

        self.refresh(false, None);

        // Trigger initial update
        self.trigger_initial_update();
    }

    /// Set the item sizing mode to control cache invalidation behavior
    ///
    /// - `ItemSizingMode::FixedSize`: Use for items with fixed size (images, fixed text)
    ///   Cache entries are preserved when width changes for better performance
    ///
    /// - `ItemSizingMode::DynamicSize`: Use for items that resize based on container width
    ///   (text wrapping, responsive layouts). Cache entries are invalidated when width changes
    pub fn set_item_sizing_mode(&self, sizing_mode: ItemSizingMode) {
        let mut state = self.config().state.borrow_mut();
        state.item_sizing_mode = sizing_mode;
    }

    /// Manually trigger an initial update to ensure items become visible
    fn trigger_initial_update(&self) {
        // Force item creation without changing the viewport size (it's already set to content area)
        {
            let mut state = self.config().state.borrow_mut();
            state.update_visible_items(self);
        }

        // Force a redraw
        self.refresh(false, None);
    }

    /// PHASE 3: Scroll to show a specific item with error handling
    pub fn scroll_to_item(&self, index: usize) -> VirtualListResult<()> {
        let mut state = self.config().state.borrow_mut();
        if let Some(ref data_source) = &state.data_source {
            let total_items = data_source.get_item_count();

            // Validate index bounds
            if index >= total_items {
                return Err(VirtualListError::invalid_index(index, total_items));
            }

            // Calculate position using mix of actual measurements and estimates
            let estimated_item_height = 80; // Same estimate as in update_visible_items
            let mut y_position = 0;

            for i in 0..index {
                let item_height = if let Some(cached_size) = state.item_size_cache.get(&i) {
                    // Use actual measurement if available
                    match state.layout_mode {
                        VirtualListLayoutMode::Vertical => cached_size.height,
                        VirtualListLayoutMode::Horizontal => cached_size.width,
                    }
                } else {
                    // Use estimate for unmeasured items
                    estimated_item_height
                };

                y_position += item_height;
            }

            let new_scroll_position = match state.layout_mode {
                VirtualListLayoutMode::Vertical => Point::new(0, y_position),
                VirtualListLayoutMode::Horizontal => Point::new(y_position, 0),
            };

            state.scroll_position = new_scroll_position;
            drop(state); // Release the borrow before calling refresh
            self.refresh(false, None);

            Ok(())
        } else {
            Err(VirtualListError::data_source_error("No data source set"))
        }
    }

    /// Refresh the virtual list (recalculate everything)
    pub fn refresh_virtual_list(&self) {
        self.config().state.borrow_mut().invalidate_layout();
        self.refresh(false, None);
    }

    /// Get the currently visible range of items
    pub fn get_visible_range(&self) -> std::ops::Range<usize> {
        self.config().state.borrow().visible_range.clone()
    }

    /// Get pool statistics for debugging/monitoring
    pub fn get_pool_stats(&self) -> super::adaptive_pool::PoolStats {
        self.config().state.borrow().item_pool.get_pool_stats()
    }

    /// PHASE 3: Hit test to find which item is at a given point with validation
    pub fn hit_test(&self, point: Point) -> VirtualListResult<Option<usize>> {
        // Validate point coordinates
        if point.x < 0 || point.y < 0 {
            return Err(VirtualListError::invalid_config(format!(
                "Invalid hit test coordinates: ({}, {})",
                point.x, point.y
            )));
        }

        let state = self.config().state.borrow();
        Ok(state.hit_test(point))
    }

    /// Clear all items and reset the virtual list
    pub fn clear(&self) {
        self.config().state.borrow_mut().clear_all_items();
        self.refresh(false, None);
    }

    /// Get the total content size
    pub fn get_total_content_size(&self) -> Size {
        // Return the cached total content size or a default
        let state = self.config().state.borrow();
        state.total_content_size
    }

    /// PHASE 3: Get item context for a panel with error handling (replaces global registry access)
    pub fn get_item_context_for_panel(&self, panel: &Panel) -> VirtualListResult<ItemContext> {
        self.config()
            .state
            .borrow()
            .get_item_context_for_panel(panel)
            .into_vl_error("get_item_context_for_panel")
    }

    /// PHASE 3: Get item index for a panel with error handling (convenience method)
    pub fn get_index_for_panel(&self, panel: &Panel) -> VirtualListResult<usize> {
        self.config()
            .state
            .borrow()
            .get_index_for_panel(panel)
            .into_vl_error("get_index_for_panel")
    }

    /// PHASE 3: Get typed data for a panel with error handling (convenience method)
    pub fn get_data_for_panel<T>(&self, panel: &Panel) -> VirtualListResult<T>
    where
        T: Clone + 'static,
    {
        self.config()
            .state
            .borrow()
            .get_data_for_panel(panel)
            .into_vl_error("get_data_for_panel")
    }
}
