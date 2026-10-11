import { flushSync } from "react-dom";

export const withTransition = (update: () => void) => {
    const doc = document as Document & {
        startViewTransition?: (cb: () => void) => unknown;
    };
    if (!doc.startViewTransition) return update();
    doc.startViewTransition(() => flushSync(update));
};
