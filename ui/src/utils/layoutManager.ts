// src/utils/layoutManager.ts
import { Layout } from 'react-grid-layout';

export interface LayoutItem {
  i: string;
  x: number;
  y: number;
  w: number;
  h: number;
}

export class LayoutManager {
  static splitHorizontally(layout: Layout[], itemId: string): Layout[] {
    const item = layout.find(l => l.i === itemId);
    if (!item) return layout;

    const newLayout = layout.map(l => ({...l}));
    const targetItem = newLayout.find(l => l.i === itemId)!;
    
    // Reduce height of original item
    targetItem.h = Math.floor(targetItem.h / 2);
    
    // Create new item below
    const newItem: LayoutItem = {
      i: `${itemId}-split-${Date.now()}`,
      x: targetItem.x,
      y: targetItem.y + targetItem.h,
      w: targetItem.w,
      h: targetItem.h
    };
    
    newLayout.push(newItem);
    return newLayout;
  }

  static splitVertically(layout: Layout[], itemId: string): Layout[] {
    const item = layout.find(l => l.i === itemId);
    if (!item) return layout;

    const newLayout = layout.map(l => ({...l}));
    const targetItem = newLayout.find(l => l.i === itemId)!;
    
    // Reduce width of original item
    targetItem.w = Math.floor(targetItem.w / 2);
    
    // Create new item to the right
    const newItem: LayoutItem = {
      i: `${itemId}-split-${Date.now()}`,
      x: targetItem.x + targetItem.w,
      y: targetItem.y,
      w: targetItem.w,
      h: targetItem.h
    };
    
    newLayout.push(newItem);
    return newLayout;
  }

  static stack(layout: Layout[], sourceId: string, targetId: string): Layout[] {
    // Implementation for stacking panels - this would create tabbed panels
    return layout;
  }

  static remove(layout: Layout[], itemId: string): Layout[] {
    return layout.filter(l => l.i !== itemId);
  }
}