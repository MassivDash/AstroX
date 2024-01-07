import { defineConfig } from 'astro/config';
import { wasm } from '@rollup/plugin-wasm';

// https://astro.build/config
export default defineConfig({
    integrations: [
        process.platform !== 'freebsd' && wasm()
    ]
});
