import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { db } from "@/server/db";

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

export const meRouter = createTRPCRouter({
    deleteAccount: protectedProcedure.mutation(async (opts) => {
        const session = opts.ctx.session;

        console.log("session", session);
        await db.$transaction([
            db.$executeRaw`DELETE FROM ORG_User WHERE id = ${session.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Account WHERE userId = ${session.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Session WHERE userId = ${session.user.id}`,
        ]);
    }),
});
