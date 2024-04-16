import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { db } from "@/server/db";
import { z } from "zod";

export const meRouter = createTRPCRouter({
    deleteAccount: protectedProcedure.mutation(async (opts) => {
        const userId = opts.ctx.session.user.id;

        await db.$transaction([
            db.$executeRaw`DELETE FROM ORG_User WHERE id = ${userId.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Account WHERE userId = ${userId.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Session WHERE userId = ${userId.user.id}`,
        ]);
    }),

    orgs: protectedProcedure.query(async (opts) => {
        const orgs: {
            id: number;
            name: string;
            description: string;
        }[] = await db.$queryRaw`
                SELECT id, name, description 
                FROM ORG_Org 
                WHERE ownerId = ${opts.ctx.session.user.id}
            `;

        return orgs.map((org) => ({
            id: org.id,
            name: org.name,
            description: org.description,
        }));
    }),

    createOrg: protectedProcedure
        .input(
            z.object({
                name: z.string().max(50),
                description: z.string().max(191),
            }),
        )
        .mutation(async (opts) => {
            await db.$executeRaw`
                INSERT INTO ORG_Org (
                    ownerId,
                    name,
                    description
                ) VALUES (
                    ${opts.ctx.session.user.id},
                    ${opts.input.name},
                    ${opts.input.description}
                );
            `;
        }),
});
