// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function watermark(filePath: string, id: string): Promise<void>;
  function process(filePath: string): Promise<void>;
}

export async function watermark(filePath: string, id: string): Promise<void> {
  await addon.watermark(filePath, id);
}

export async function process(filePath: string): Promise<void> {
  await addon.process(filePath);
}
