import commonjs from '@rollup/plugin-commonjs';
import copy from 'rollup-plugin-copy'
import livereload from 'rollup-plugin-livereload';
import resolve from '@rollup/plugin-node-resolve';
import svelte from 'rollup-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import { terser } from 'rollup-plugin-terser';
const smelte = require("smelte/rollup-plugin-smelte");
import rust from "@wasm-tool/rollup-plugin-rust";
require('fast-text-encoding');

const staticDir = 'static';
const production = !process.env.ROLLUP_WATCH;

let conf = {
  distDir: './public',
  inlineDynamicImports: true,
  output: {
    sourcemap: true,
    format: 'iife',
    file: './public/build/bundle.js',
  }
}

export default {
  inlineDynamicImports: conf.inlineDynamicImports,
  input: 'src/main.ts',
  output: conf.output,
  plugins: [
    svelte({
      dev: !production,
      emitCss: true,
      preprocess: sveltePreprocess(),
    }),
    copy({
      targets: [
        { src: [`${staticDir}/*`, "!*/(__index.html)"], dest: conf.distDir },
        { src: [`${staticDir}/__index.html`], dest: conf.distDir, rename: '__app.html', transform },
      ],
      copyOnce: true,
      flatten: false
    }),
    resolve({
      browser: true,
      dedupe: ['svelte'],
    }),
    commonjs(),
    rust({
      serverPath: "/build/",
      verbose: true,
    }),
    smelte({
      purge: production,
      output: `${conf.distDir}/global.css`,
      colors: {
        primary: "#b027b0",
        secondary: "#009688",
      }
    }),
    typescript({ sourceMap: !production }),
    !production && serve(),
    !production && livereload(conf.distDir),
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
};

function transform(contents) {
  return contents.toString().replace('__SCRIPT__', conf.inlineDynamicImports
    ? '<script type="module" defer src="/build/main.js"></script>'
    : '<script defer src="/build/bundle.js"></script>')
}


function serve() {
  let server;

  function toExit() {
    if (server) server.kill(0);
  }

  return {
    writeBundle() {
      if (server) return;
      server = require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
        stdio: ['ignore', 'inherit', 'inherit'],
        shell: true,
      });

      process.on('SIGTERM', toExit);
      process.on('exit', toExit);
    },
  };
}
