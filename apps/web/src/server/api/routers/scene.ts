import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "@/server/api/trpc";

export type Scene = {
    name: string;
    items: SceneItem[];
};

export type SceneItem = {
    meshType: MeshType;
    id: string;
    position: [number, number, number];
    rotation: [number, number, number];
};

export const MeshType = {
    Cube: "Cube",
    Sphere: "Sphere",
    Cylinder: "Cylinder",
    Plane: "Plane",
} as const;

type MeshType = (typeof MeshType)[keyof typeof MeshType];

export const sceneRouter = createTRPCRouter({
    getSceneByOrg: publicProcedure
        .input(z.object({ orgName: z.string() }))
        .query(({ input }) => {
            return fetch(
                `http://0.0.0.0:3002/scene/${encodeURIComponent(input.orgName)}`,
            ).then((res): Promise<Scene> => res.json());
        }),
});
