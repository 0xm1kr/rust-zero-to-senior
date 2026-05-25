// Playground: code editor + Run button + output panel.
//
// The editor is a CodeMirror 5 instance mounted on the original
// <textarea id="code">. CodeMirror is loaded from /vendor/codemirror/ via
// plain <script> tags in index.html, so the global `window.CodeMirror` is
// guaranteed to exist by the time initPlayground() runs.
//
// Public API:
//   initPlayground()      — mount the editor and wire buttons (call once)
//   resetTo(code)         — load new starter code (called when lesson changes)
//   getCurrentCode()      — read the current editor buffer (used by chat.js
//                           so context-aware AI replies see live edits)

import { $ } from './dom.js';
import { runCode } from './api.js';

let editor = null;
let originalCode = '';

const DARK_THEME = 'material-darker';
const LIGHT_THEME = 'default';

// getCurrentCode returns whatever the user currently has in the editor,
// or the original textarea value if CodeMirror failed to load.
export function getCurrentCode() {
  if (editor) return editor.getValue();
  const ta = $('#code');
  return ta ? ta.value : '';
}

// initPlayground mounts CodeMirror on #code and wires up the Run/Reset
// buttons. Safe to call exactly once at startup.
export function initPlayground() {
  const textarea = $('#code');

  if (!window.CodeMirror) {
    console.warn('CodeMirror not loaded; falling back to plain textarea.');
    wirePlainTextarea(textarea);
    return;
  }

  editor = window.CodeMirror.fromTextArea(textarea, {
    mode: 'rust',
    theme: currentCmTheme(),
    lineNumbers: true,
    indentUnit: 4,
    tabSize: 4,
    indentWithTabs: false, // rustfmt uses spaces
    smartIndent: true,
    matchBrackets: true,
    autoCloseBrackets: true,
    lineWrapping: false,
    viewportMargin: Infinity, // grow with content; size capped via CSS
    extraKeys: {
      // ⌘/Ctrl + Enter to run.
      'Cmd-Enter':  () => run(),
      'Ctrl-Enter': () => run(),
      Tab: (cm) => {
        if (cm.somethingSelected()) {
          cm.indentSelection('add');
        } else {
          cm.replaceSelection('    ', 'end', '+input');
        }
      },
      'Shift-Tab': (cm) => cm.indentSelection('subtract'),
    },
  });

  watchAppTheme((theme) => {
    editor.setOption('theme', theme === 'light' ? LIGHT_THEME : DARK_THEME);
  });

  $('#run-code').addEventListener('click', run);
  $('#reset-code').addEventListener('click', () => {
    editor.setValue(originalCode);
    editor.focus();
  });
}

// resetTo loads new starter code into the editor and remembers it as the
// "original" so the Reset button can restore it later.
export function resetTo(code) {
  const next = code || '';
  originalCode = next;
  if (editor) {
    editor.setValue(next);
    editor.clearHistory();
    editor.scrollTo(0, 0);
  } else {
    const ta = $('#code');
    if (ta) {
      ta.value = next;
      ta.dataset.original = next;
    }
  }
  $('#output').innerHTML = 'Click <b>Run</b> to execute the code above. <span class="meta">(⌘/Ctrl + Enter)</span>';
  $('#run-status').textContent = '';
  $('#run-status').className = '';
}

async function run() {
  const btn = $('#run-code');
  const status = $('#run-status');
  const output = $('#output');

  btn.disabled = true;
  status.textContent = 'compiling…';
  status.className = 'running';
  output.textContent = '';

  try {
    const data = await runCode(getCurrentCode());
    output.innerHTML = '';

    if (data.stdout) appendSpan(output, 'stdout', data.stdout);
    if (data.stderr) appendSpan(output, 'stderr', data.stderr);
    if (data.error) {
      const prefix = (data.stdout || data.stderr) ? '\n' : '';
      appendSpan(output, 'error', prefix + 'error: ' + data.error);
    }
    if (!data.stdout && !data.stderr && !data.error) {
      output.innerHTML = '<span class="meta">(no output)</span>';
    }

    const meta = document.createElement('div');
    meta.className = 'meta';
    meta.style.marginTop = '8px';
    meta.textContent = `— exited in ${data.duration}`;
    output.appendChild(meta);

    status.textContent = data.error ? 'failed' : 'succeeded';
    status.className = data.error ? 'error' : 'success';
  } catch (err) {
    output.innerHTML = '';
    appendSpan(output, 'error', 'request failed: ' + err.message);
    status.textContent = 'failed';
    status.className = 'error';
  } finally {
    btn.disabled = false;
  }
}

function appendSpan(parent, className, text) {
  const span = document.createElement('span');
  span.className = className;
  span.textContent = text;
  parent.appendChild(span);
}

function currentCmTheme() {
  const t = document.documentElement.getAttribute('data-theme');
  return t === 'light' ? LIGHT_THEME : DARK_THEME;
}

function watchAppTheme(cb) {
  const obs = new MutationObserver(() => {
    cb(document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark');
  });
  obs.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}

function wirePlainTextarea(ta) {
  $('#run-code').addEventListener('click', run);
  $('#reset-code').addEventListener('click', () => {
    ta.value = ta.dataset.original || '';
  });
  ta.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
      e.preventDefault();
      const { selectionStart: s, selectionEnd: e2 } = ta;
      ta.value = ta.value.substring(0, s) + '    ' + ta.value.substring(e2);
      ta.selectionStart = ta.selectionEnd = s + 4;
      return;
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      run();
    }
  });
}
