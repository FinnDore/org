"use client";
import { api } from "@/trpc/react";
import { Canvas, useFrame } from "@react-three/fiber";
import { useRef } from "react";
import { Group } from "three";

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
    const scene = api.scene.getSceneByOrg.useQuery({ orgName: "test" });
    const sphereRef = useRef<Group | null>(null);

    useFrame(() => {
        const sphere = sphereRef.current;
        if (sphere) {
            sphere.rotation.y += Math.PI / 500;
        }
    });

    return (
        <>
            <ambientLight intensity={0.5} />
            <directionalLight position={[0, 0, 5]} />
            <group ref={sphereRef}>
                {scene.data?.items.map((item) => {
                    switch (item.meshType) {
                        case "Cube":
                            return (
                                <mesh
                                    key={item.id}
                                    rotation={item.rotation}
                                    position={item.position}
                                >
                                    <boxGeometry args={[1]} />
                                    <meshStandardMaterial color="red" />
                                </mesh>
                            );
                        case "Sphere":
                            return (
                                <mesh
                                    key={item.id}
                                    rotation={item.rotation}
                                    position={item.position}
                                >
                                    <sphereGeometry args={[1]} />
                                    <meshStandardMaterial />
                                </mesh>
                            );
                        case "Cylinder":
                            return (
                                <mesh
                                    key={item.id}
                                    position={item.position}
                                    rotation={item.rotation}
                                >
                                    <cylinderGeometry args={[1]} />
                                    <meshStandardMaterial />
                                </mesh>
                            );
                        case "Plane":
                            return (
                                <mesh
                                    key={item.id}
                                    rotation={item.rotation}
                                    position={item.position}
                                >
                                    <planeGeometry args={[1]} />
                                    <meshStandardMaterial />
                                </mesh>
                            );
                    }
                })}
            </group>
        </>
    );
}
