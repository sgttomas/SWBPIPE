// Minimal ambient declaration for the three.js addon controls used by the
// viewport. This three build ships the addon JS but not its .d.ts, and the
// desktop build runs `tsc -b`, so we declare only the surface we consume.
// Display/interaction only — no runtime behavior is defined here.
declare module "three/addons/controls/OrbitControls.js" {
  import type { Camera, Vector3 } from "three";
  export class OrbitControls {
    constructor(object: Camera, domElement?: HTMLElement);
    target: Vector3;
    enabled: boolean;
    enableDamping: boolean;
    dampingFactor: number;
    enableZoom: boolean;
    enablePan: boolean;
    enableRotate: boolean;
    update(): boolean;
    dispose(): void;
    addEventListener(type: string, listener: () => void): void;
    removeEventListener(type: string, listener: () => void): void;
  }
}
