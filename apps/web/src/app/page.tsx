"use client";
import { type SceneItem } from "@/server/api/routers/scene";
import { api } from "@/trpc/react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import { useEffect, useRef, useState } from "react";
import { damp } from "three/src/math/MathUtils.js";

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
    const { scene: theScene } = useThree();
    const scene = api.scene.getSceneByOrg.useQuery(
        { orgName: "test" },
        {
            refetchOnWindowFocus: false,
            refetchInterval: 0,
        },
    );
    const ref = useRef<SceneItem[] | undefined>(scene.data?.items);

    const [, reRender] = useState<any | null>({});
    useEffect(() => {
        ref.current = scene.data?.items;
        reRender({});
    }, [scene.data?.items]);
    console.log(theScene);
    useWebsocket({
        orgName: "finn",
        onMessage(data) {
            if (ref.current) {
                for (const message of data) {
                    const itemToUpdate = ref.current?.find(
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
        if (ref.current) {
            for (const target of ref.current) {
                const current = theScene.children.find(
                    (i) => i.name === target.id,
                );
                if (!current) {
                    continue;
                }
                const targetPos = target.position;
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
                current.material.color.set(target.color);
            }
        }
    });

    return (
        <>
            <ambientLight intensity={0.5} />
            <directionalLight position={[0, 0, 5]} />
            {ref.current?.map((item) => {
                switch (item.meshType) {
                    case "Cube":
                        return (
                            <mesh
                                key={item.id}
                                name={item.id}
                                rotation={item.rotation}
                                position={item.position}
                            >
                                <boxGeometry args={[1]} />
                                <meshStandardMaterial color={item.color} />
                            </mesh>
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
