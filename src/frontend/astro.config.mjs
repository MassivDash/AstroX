import { defineConfig } from 'astro/config';
import { wasm } from '@rollup/wasm-node';

// https://astro.build/config
export default defineConfig({
    ...(process.platform === 'freebsd' && { vite: {
        plugins: [wasm()]
      }})
});
