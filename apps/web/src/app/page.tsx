"use client";
import { Canvas, useFrame } from "@react-three/fiber";
import { useRef } from "react";
import { Mesh } from "three";

export default function Home() {
  return (
    <main className="relative flex min-h-screen bg-black">
      <Canvas
        style={{
          height: "100vh",
        }}
      >
        <Scene />
      </Canvas>
    </main>
  );
}

function Scene() {
  const sphereRef = useRef<Mesh | null>(null);

  useFrame(() => {
    const sphere = sphereRef.current;
    if (sphere) {
      sphere.rotation.y += Math.PI / 500;
    }
  });

  return (
    <>
      <ambientLight intensity={0.1} />
      <directionalLight position={[0, 0, 5]} />
      <group ref={sphereRef}>
        <mesh position={[-1.5, 0, 1.5]}>
          <sphereGeometry args={[1]} />
          <meshStandardMaterial />
        </mesh>
      </group>
    </>
  );
}
