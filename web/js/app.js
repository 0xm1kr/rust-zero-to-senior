// app.js — composition root for the frontend.
//
// Mirrors what main.rs does on the backend: import each module, wire them
// together, kick off startup. No DOM logic of its own; that lives in the
// per-feature modules.

import { initTheme } from './theme.js';
import { initLessons } from './lessons.js';
import { initPlayground, resetTo as resetPlayground } from './playground.js';
import { initChat, syncToLesson } from './chat.js';

async function main() {
  initTheme();
  initPlayground();
  await initChat();

  await initLessons({
    onChange(lesson) {
      resetPlayground(lesson.code || '');
      syncToLesson(lesson);
    },
  });
}

main();
