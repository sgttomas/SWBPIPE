import * as THREE from "three";
import { describe, expect, it, vi } from "vitest";
import { entityKey } from "../workspace/selectionState";
import {
  createFigureMaterial,
  isFigureMaterial,
  setFigureShadeRatio,
  type FigureMaterialOptions
} from "./viewportFigureMaterial";
import { viewportShadeRatio } from "./viewportPalette";
import { registerInstancedRolePresentation, ViewportOwnershipLedger } from "./viewportResource";

describe("viewport figure material", () => {
  it("is an unlit shader material with a tint, an opacity and a shade ratio, and no texture", () => {
    const material = createFigureMaterial();
    expect(material).toBeInstanceOf(THREE.ShaderMaterial);
    expect(material.lights).toBe(false);
    expect(material.toneMapped).toBe(false);
    expect(material.transparent).toBe(false);
    expect(Object.keys(material.uniforms).sort()).toEqual(["opacity", "shadeRatio", "tint"]);
    expect(material.uniforms.opacity.value).toBe(1);
    expect(material.uniforms.tint.value).toBeInstanceOf(THREE.Color);
    expect(material.uniforms.shadeRatio.value).toBeInstanceOf(THREE.Color);
    const textures = [...Object.values(material), ...Object.values(material.uniforms).map((uniform) => uniform.value)]
      .filter((value) => value instanceof THREE.Texture);
    expect(textures).toEqual([]);
    expect(isFigureMaterial(material)).toBe(true);
    expect(isFigureMaterial(new THREE.ShaderMaterial())).toBe(false);
    expect(isFigureMaterial(new THREE.MeshBasicMaterial())).toBe(false);
    material.dispose();
  });

  it("supports instancing and per-instance colour through three's own chunks", () => {
    const material = createFigureMaterial();
    for (const chunk of [
      "color_pars_vertex", "color_vertex", "beginnormal_vertex", "defaultnormal_vertex", "begin_vertex", "project_vertex"
    ]) {
      expect(material.vertexShader).toContain(`#include <${chunk}>`);
    }
    for (const chunk of ["color_pars_fragment", "colorspace_fragment"]) {
      expect(material.fragmentShader).toContain(`#include <${chunk}>`);
    }
    // Every chunk named is one this version of three ships.
    const includes = [...`${material.vertexShader}\n${material.fragmentShader}`.matchAll(/#include <([a-z_]+)>/g)];
    expect(includes.length).toBeGreaterThanOrEqual(8);
    for (const [, chunk] of includes) expect(THREE.ShaderChunk).toHaveProperty(chunk);
    expect(material.fragmentShader).toContain("smoothstep( 0.0, 0.75, facing )");
    expect(material.fragmentShader).toContain("base * shadeRatio");
    material.dispose();
  });

  it("keeps the deformation overlay's transparency and opacity when asked", () => {
    const material = createFigureMaterial({ opacity: 0.82, transparent: true });
    expect(material.transparent).toBe(true);
    expect(material.uniforms.opacity.value).toBe(0.82);
    material.dispose();
  });

  it("exposes its tint as a colour, so selection and repaint treat it as any coloured material", () => {
    const material = createFigureMaterial();
    expect(material.color).toBe(material.uniforms.tint.value);
    material.color.setHex(0x123456);
    expect((material.uniforms.tint.value as THREE.Color).getHex()).toBe(0x123456);
    // The default tint is white: an instanced figure's drawn colour is its instance colour alone.
    expect(createFigureMaterial().color.toArray()).toEqual([1, 1, 1]);
    material.dispose();
  });

  it("takes a theme's shade ratio in place", () => {
    const material = createFigureMaterial();
    const value = material.uniforms.shadeRatio.value as THREE.Color;
    setFigureShadeRatio(material, viewportShadeRatio("dark"));
    expect(material.uniforms.shadeRatio.value).toBe(value);
    expect(value.toArray()).toEqual([...viewportShadeRatio("dark")]);
    setFigureShadeRatio(material, viewportShadeRatio("light"));
    expect(value.toArray()).toEqual([...viewportShadeRatio("light")]);
    material.dispose();
  });

  it("enters the ownership ledger as one material and no texture", () => {
    const mesh = new THREE.InstancedMesh(new THREE.CylinderGeometry(1, 1, 1, 8), createFigureMaterial(), 2);
    registerInstancedRolePresentation(
      mesh,
      [entityKey({ type: "pipe", id: "p:1" }), entityKey({ type: "pipe", id: "p:2" })],
      "pipe"
    );
    const ledger = new ViewportOwnershipLedger(3);
    ledger.createObjects([mesh]);
    expect(ledger.snapshot().live).toMatchObject({
      pipeMeshes: 1,
      geometries: 1,
      materials: 1,
      textures: 0,
      instanceMatrices: 1,
      instanceColors: 1
    });
    mesh.geometry.dispose();
    (mesh.material as THREE.Material).dispose();
    mesh.dispose();
  });
});

describe("viewport figure material: clone()", () => {
  type Figure = ReturnType<typeof createFigureMaterial>;

  const FORMS: readonly (readonly [string, FigureMaterialOptions])[] = [
    ["a default material", {}],
    ["a transparent 0.82 material", { opacity: 0.82, transparent: true }]
  ];

  // A source with a tint and a shade ratio that are not the defaults, so a copy is told from a rebuild.
  function paintedSource(options: FigureMaterialOptions): Figure {
    const source = createFigureMaterial(options);
    source.color.setHex(0x336699);
    setFigureShadeRatio(source, viewportShadeRatio("dark"));
    return source;
  }

  function expectWholeClone(clone: Figure, source: Figure, options: FigureMaterialOptions): void {
    expect(clone).not.toBe(source);
    expect(isFigureMaterial(clone)).toBe(true);
    // The repaint's own guard, then the identity that makes a repaint reach the shader.
    expect("color" in clone).toBe(true);
    expect(clone.color).toBeInstanceOf(THREE.Color);
    expect(clone.color).toBe(clone.uniforms.tint.value);
    expect(clone.color).not.toBe(source.color);
    expect(clone.color).not.toBe(source.uniforms.tint.value);
    expect(clone.color.getHex()).toBe(0x336699);
    // The shade ratio is the clone's own object and carries the source's numbers.
    expect(clone.uniforms.shadeRatio.value).toBeInstanceOf(THREE.Color);
    expect(clone.uniforms.shadeRatio.value).not.toBe(source.uniforms.shadeRatio.value);
    expect((clone.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio("dark")]);
    // What three's renderer chooses its path from, and what the figure is.
    expect(clone.type).toBe("ShaderMaterial");
    expect(clone.isShaderMaterial).toBe(true);
    expect(clone.lights).toBe(false);
    expect(clone.toneMapped).toBe(false);
    expect(clone.vertexShader).toBe(source.vertexShader);
    expect(clone.fragmentShader).toBe(source.fragmentShader);
    expect(Object.keys(clone.uniforms).sort()).toEqual(["opacity", "shadeRatio", "tint"]);
    expect(clone.transparent).toBe(options.transparent ?? false);
    expect(clone.uniforms.opacity.value).toBe(options.opacity ?? 1);
    // three's clone() is `new this.constructor().copy(this)`: the constructor is the figure's own.
    expect(source.constructor).not.toBe(THREE.ShaderMaterial);
    expect(clone.constructor).toBe(source.constructor);
  }

  it.each(FORMS)("a clone of %s is a whole figure material whose colour is its own live tint", (_name, options) => {
    const source = paintedSource(options);
    const clone = source.clone();
    expectWholeClone(clone, source, options);

    // Setting either colour leaves the other unchanged.
    clone.color.setHex(0x112233);
    expect(source.color.getHex()).toBe(0x336699);
    expect((clone.uniforms.tint.value as THREE.Color).getHex()).toBe(0x112233);
    source.color.setHex(0x445566);
    expect(clone.color.getHex()).toBe(0x112233);
    expect((source.uniforms.tint.value as THREE.Color).getHex()).toBe(0x445566);
    // And either shade ratio.
    setFigureShadeRatio(clone, viewportShadeRatio("light"));
    expect((source.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio("dark")]);
    expect((clone.uniforms.shadeRatio.value as THREE.Color).toArray()).toEqual([...viewportShadeRatio("light")]);
    source.dispose();
    clone.dispose();
  });

  it.each(FORMS)("a clone of a clone of %s still holds all of this", (_name, options) => {
    const source = paintedSource(options);
    const first = source.clone();
    const second = first.clone();
    expectWholeClone(second, first, options);
    expect(second.color).not.toBe(source.color);
    second.color.setHex(0x778899);
    expect(first.color.getHex()).toBe(0x336699);
    expect(source.color.getHex()).toBe(0x336699);
    for (const material of [source, first, second]) material.dispose();
  });

  it("constructs, clones and copies without a console warning or error", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => undefined);
    const error = vi.spyOn(console, "error").mockImplementation(() => undefined);
    try {
      // The spies do see three's own reports: an unknown parameter is reported through console.warn.
      const unknownParameter: Record<string, unknown> = { figureMaterialUnknownParameter: 1 };
      new THREE.ShaderMaterial(unknownParameter).dispose();
      expect(warn).toHaveBeenCalledTimes(1);
      warn.mockClear();

      const plain = createFigureMaterial();
      const overlay = createFigureMaterial({ opacity: 0.82, transparent: true });
      const made = [plain, overlay, plain.clone(), overlay.clone(), overlay.clone().clone()];
      plain.copy(overlay);
      expect(warn).not.toHaveBeenCalled();
      expect(error).not.toHaveBeenCalled();
      for (const material of made) material.dispose();
    } finally {
      warn.mockRestore();
      error.mockRestore();
    }
  });

  it("does not take a plain shader material for a figure material, whatever it copied or carries", () => {
    const figure = paintedSource({ opacity: 0.82, transparent: true });
    // three's Material.copy carries userData across; a marker there must not make a figure material.
    const copied = new THREE.ShaderMaterial().copy(figure);
    expect(isFigureMaterial(copied)).toBe(false);
    const marked = new THREE.ShaderMaterial();
    marked.userData = JSON.parse(JSON.stringify(figure.userData)) as Record<string, unknown>;
    marked.userData.viewportFigureMaterial = true;
    expect(isFigureMaterial(marked)).toBe(false);
    for (const material of [figure, copied, marked]) material.dispose();
  });
});
