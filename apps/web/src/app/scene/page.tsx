"use client";
import { SceneItem } from "@/server/api/routers/scene";
import { api } from "@/trpc/react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import { Bloom, EffectComposer, N8AO } from "@react-three/postprocessing";
import { useEffect, useRef, useState } from "react";
import { damp } from "three/src/math/MathUtils.js";

import { OrbitControls, RoundedBox } from "@react-three/drei";
import { PerspectiveCamera } from "three";

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
    onMessage: (
        data: {
            id: string;
            color?: string;
            position?: [number, number, number];
            rotation?: [number, number, number];
        }[],
    ) => void;
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
    const { scene: theScene, set, viewport } = useThree();
    const cameraRef = useRef<PerspectiveCamera>();
    // This makes sure that size-related calculations are proper
    // Every call to useThree will return this camera instead of the default camera
    useEffect(
        () => cameraRef.current && void set({ camera: cameraRef.current }),
        [],
    );
    const scene = api.scene.getSceneByOrg.useQuery(
        { orgName: "test" },
        {
            refetchOnWindowFocus: false,
            refetchInterval: 0,
        },
    );
    const sceneRef = useRef<SceneItem[] | undefined>(scene.data?.items);

    const [, reRender] = useState<any | null>({});
    useEffect(() => {
        sceneRef.current = scene.data?.items;
        reRender({});
    }, [scene.data?.items]);
    console.log(theScene);
    useWebsocket({
        orgName: "finn",
        onMessage(data) {
            if (sceneRef.current) {
                for (const message of data) {
                    const itemToUpdate = sceneRef.current?.find(
                        (item) => item.id === message.id,
                    );

                    if (!itemToUpdate) {
                        continue;
                    }

                    itemToUpdate.color = message.color ?? itemToUpdate.color;
                    itemToUpdate.position =
                        message.position ?? itemToUpdate.position;
                    itemToUpdate.rotation =
                        message.rotation ?? itemToUpdate.rotation;
                }
            }
        },
    });

    useFrame(() => {
        if (sceneRef.current) {
            for (const target of sceneRef.current) {
                const current = theScene.children.find(
                    (i) => i.name === target.id,
                );
                if (!current) {
                    continue;
                }
                const targetPos = target.position;
                // current.position.set(
                //     targetPos[0] ?? 0,
                //     targetPos[1] ?? 0,
                //     targetPos[2] ?? 0,
                // );
                current.position.set(
                    damp(current.position.x, targetPos[0] ?? 0, 0.45, 0.01),
                    damp(current.position.y, targetPos[1] ?? 0, 0.45, 0.01),
                    damp(current.position.z, targetPos[2] ?? 0, 0.45, 0.01),
                );
                const targetRot = target.rotation;
                current.rotation.set(
                    damp(current.rotation.x, targetRot[0] ?? 0, 0.45, 0.01),
                    damp(current.rotation.y, targetRot[1] ?? 0, 0.45, 0.01),
                    damp(current.rotation.z, targetRot[2] ?? 0, 0.45, 0.01),
                );
                current.rotation.set(targetRot[0], targetRot[1], targetRot[2]);
                (current as any).material.color.set(target.color);
            }
        }
    });

    const config = { fov: 95, position: [0, 0, 10] } as any;
    return (
        <>
            <OrbitControls camera={cameraRef.current} />
            <EffectComposer enableNormalPass={false}>
                <N8AO aoRadius={40} intensity={1} />
                <Bloom
                    luminanceThreshold={1}
                    intensity={0.5}
                    levels={10}
                    mipmapBlur
                />
            </EffectComposer>

            <perspectiveCamera
                scale={(viewport.width / 5) * 1}
                orbi
                fov={40}
                ref={cameraRef}
                position={[0, 0, 2000]}
                rotation={[90, 55, 180]}
                near={0.1}
                far={100000}
            />
            <ambientLight intensity={1} />
            <directionalLight position={[0, 0, 5]} />
            {sceneRef.current?.map((item) => {
                switch (item.meshType) {
                    case "Cube":
                        return (
                            <RoundedBox
                                key={item.id}
                                name={item.id}
                                scale={[100, 30, 100]}
                                radius={0.25}
                                rotation={item.rotation}
                                position={item.position}
                            >
                                <meshStandardMaterial
                                    color={item.color}
                                    emissive={item.color}
                                    emissiveIntensity={10}
                                />
                            </RoundedBox>
                        );
                    case "Sphere":
                        return (
                            <mesh
                                key={item.id}
                                name={item.id}
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
                                name={item.id}
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
                                name={item.id}
                                rotation={item.rotation}
                                position={item.position}
                            >
                                <planeGeometry args={[1]} />
                                <meshStandardMaterial />
                            </mesh>
                        );
                }
            })}
        </>
    );
}
