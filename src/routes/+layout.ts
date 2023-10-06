export const prerender = true
export const ssr = false

// TailwindCSS
import '../app.postcss';

// Floating UI for Popups
import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
import { storePopup } from '@skeletonlabs/skeleton';
storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
