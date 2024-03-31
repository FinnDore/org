"use client";
import { type SceneItem } from "@/server/api/routers/scene";
import { api } from "@/trpc/react";
import { Canvas } from "@react-three/fiber";
import { useEffect, useRef, useState } from "react";
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

function useWebsocket(opts: {
    onMessage: (data: {
        objectId: string;
        path: string;
        value: number;
    }) => void;
    orgName: string;
}) {
    useEffect(() => {
        const ws = new WebSocket(
            `ws://localhost:3002/sub/${encodeURIComponent(opts.orgName)}`,
        );
        ws.onopen = () => {
            console.log(
                "%cWebsocket connected",
                "background:#1fde7c;padding:0.5rem",
            );
        };
        ws.onmessage = (event) => {
            const payload = JSON.parse(event.data);
            // console.log(
            //     "%cMessage",
            //     "background:#A001fF;padding:0.5rem",
            //     payload,
            // );
            opts.onMessage?.(payload);
        };
        ws.onclose = () => {
            console.log(
                "%cWebscoket disconnected",
                "background:#de1f2e;padding:0.5rem",
            );
        };
        return () => {
            ws.close();
        };
    }, [opts.orgName]);
}

function Scene() {
    const scene = api.scene.getSceneByOrg.useQuery({ orgName: "test" });
    const [sceneItems, setState] = useState<SceneItem[] | undefined>(
        scene.data?.items,
    );

    useEffect(() => {
        setState(scene.data?.items);
    }, [scene.data?.items]);

    useWebsocket({
        orgName: "finn",
        onMessage(data) {
            setState((prev) => {
                const itemToUpdate = prev?.find(
                    (item) => item.id === data.objectId,
                );
                if (!itemToUpdate || !prev) {
                    return prev;
                }
                itemToUpdate[data.path as keyof SceneItem] = data.value as any;
                return [...prev];
            });
        },
    });
    const sphereRef = useRef<Group | null>(null);

    return (
        <>
            <ambientLight intensity={0.5} />
            <directionalLight position={[0, 0, 5]} />
            <group ref={sphereRef}>
                {sceneItems?.map((item) => {
                    switch (item.meshType) {
                        case "Cube":
                            return (
                                <mesh
                                    key={item.id}
                                    rotation={item.rotation}
                                    position={item.position}
                                >
                                    <boxGeometry args={[1]} />
                                    <meshStandardMaterial color="#F11FF1" />
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
