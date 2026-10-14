import { flushSync } from "react-dom";

let running: ViewTransition | null = null;
let pending: (() => void) | null = null;

export const withTransition = (update: () => void) => {
    const doc = document as Document & {
        startViewTransition?: (cb: () => void) => ViewTransition;
    };
    if (!doc.startViewTransition) return update();

    if (running) {
        pending = update;
        return;
    }

    const root = document.documentElement;
    root.classList.add("vt-active");

    const t = doc.startViewTransition(() => flushSync(update));
    running = t;

    t.ready.catch(() => { });
    t.updateCallbackDone.catch(() => { });

    t.finished.finally(() => {
        running = null;
        root.classList.remove("vt-active");

        const next = pending;
        pending = null;
        if (next) withTransition(next);
    });
};
