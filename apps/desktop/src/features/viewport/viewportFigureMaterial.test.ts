import * as THREE from "three";
import { describe, expect, it } from "vitest";
import { entityKey } from "../workspace/selectionState";
import { createFigureMaterial, isFigureMaterial, setFigureShadeRatio } from "./viewportFigureMaterial";
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
