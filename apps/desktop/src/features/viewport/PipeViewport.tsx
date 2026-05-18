import { useEffect, useRef } from "react";
import * as THREE from "three";
import type { EntityRef, PreviewModel, Vec3 } from "../../types";

type Props = {
  model: PreviewModel;
  selection: EntityRef;
};

export function PipeViewport({ model, selection }: Props) {
  const hostRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    if (!hasWebGL()) {
      host.replaceChildren();
      const fallback = document.createElement("div");
      fallback.className = "viewport-fallback";
      fallback.textContent = "3D viewport requires WebGL; model data is still loaded in the tree.";
      host.appendChild(fallback);
      return;
    }

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0xf6f7f4);
    const camera = new THREE.PerspectiveCamera(42, host.clientWidth / host.clientHeight, 0.1, 1000);
    camera.position.set(7.6, 7, 8);
    camera.lookAt(3.8, 1.2, 0.7);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.setSize(host.clientWidth, host.clientHeight);
    host.replaceChildren(renderer.domElement);

    scene.add(new THREE.AmbientLight(0xffffff, 0.72));
    const key = new THREE.DirectionalLight(0xffffff, 1.2);
    key.position.set(4, 9, 7);
    scene.add(key);
    scene.add(grid());

    const nodeMap = new Map(model.nodes.map((node) => [node.id, node.position]));
    for (const segment of model.pipe_segments) {
      const from = nodeMap.get(segment.from);
      const to = nodeMap.get(segment.to);
      if (!from || !to) continue;
      scene.add(pipeMesh(from, to, selection.id === segment.id));
    }
    for (const node of model.nodes) {
      scene.add(marker(node.position, selection.id === node.id ? 0xf08c22 : 0x2f6f73, 0.095));
    }
    for (const support of model.supports) {
      const node = nodeMap.get(support.node);
      if (!node) continue;
      scene.add(supportMesh(node, selection.id === support.id));
    }
    for (const component of model.components) {
      const node = nodeMap.get(component.node);
      if (!node) continue;
      scene.add(componentMesh(node, selection.id === component.id));
    }

    const resize = () => {
      const { clientWidth, clientHeight } = host;
      camera.aspect = clientWidth / clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(clientWidth, clientHeight);
      renderer.render(scene, camera);
    };
    resize();
    window.addEventListener("resize", resize);

    let frame = 0;
    const animate = () => {
      frame = requestAnimationFrame(animate);
      scene.rotation.y = Math.sin(Date.now() / 6000) * 0.05;
      renderer.render(scene, camera);
    };
    animate();

    return () => {
      cancelAnimationFrame(frame);
      window.removeEventListener("resize", resize);
      renderer.dispose();
      host.replaceChildren();
    };
  }, [model, selection]);

  return (
    <div className="viewport-shell">
      <div className="viewport-toolbar">
        <span>3D Centerline</span>
        <span>{selection.id}</span>
      </div>
      <div className="viewport-canvas" ref={hostRef} aria-label="Three.js pipe centerline viewport" />
    </div>
  );
}

function pipeMesh(from: Vec3, to: Vec3, active: boolean) {
  const start = toVector(from);
  const end = toVector(to);
  const direction = end.clone().sub(start);
  const length = direction.length();
  const geometry = new THREE.CylinderGeometry(active ? 0.07 : 0.052, active ? 0.07 : 0.052, length, 18);
  const material = new THREE.MeshStandardMaterial({
    color: active ? 0xf08c22 : 0x4f6f73,
    metalness: 0.2,
    roughness: 0.58
  });
  const mesh = new THREE.Mesh(geometry, material);
  mesh.position.copy(start.clone().add(end).multiplyScalar(0.5));
  mesh.quaternion.setFromUnitVectors(new THREE.Vector3(0, 1, 0), direction.normalize());
  return mesh;
}

function marker(position: Vec3, color: number, radius: number) {
  return new THREE.Mesh(
    new THREE.SphereGeometry(radius, 24, 16),
    new THREE.MeshStandardMaterial({ color, roughness: 0.48 })
  ).translateX(position.x).translateY(position.y).translateZ(position.z);
}

function supportMesh(position: Vec3, active: boolean) {
  const group = new THREE.Group();
  const cone = new THREE.Mesh(
    new THREE.ConeGeometry(0.18, 0.34, 4),
    new THREE.MeshStandardMaterial({ color: active ? 0xf08c22 : 0x6b7d49, roughness: 0.7 })
  );
  cone.position.set(position.x, position.y - 0.26, position.z);
  cone.rotation.y = Math.PI / 4;
  group.add(cone);
  return group;
}

function componentMesh(position: Vec3, active: boolean) {
  const box = new THREE.Mesh(
    new THREE.BoxGeometry(0.24, 0.24, 0.24),
    new THREE.MeshStandardMaterial({ color: active ? 0xf08c22 : 0x874c62, roughness: 0.52 })
  );
  box.position.set(position.x, position.y + 0.2, position.z);
  return box;
}

function grid() {
  const helper = new THREE.GridHelper(10, 10, 0x8b9490, 0xd6dbd4);
  helper.position.set(3.8, -0.02, 1.1);
  helper.rotation.x = Math.PI / 2;
  return helper;
}

function toVector(position: Vec3) {
  return new THREE.Vector3(position.x, position.y, position.z);
}

function hasWebGL() {
  if (navigator.userAgent.toLowerCase().includes("jsdom")) {
    return false;
  }
  try {
    const canvas = document.createElement("canvas");
    return Boolean(canvas.getContext("webgl") || canvas.getContext("experimental-webgl"));
  } catch {
    return false;
  }
}
