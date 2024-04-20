import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { db } from "@/server/db";
import { z } from "zod";

export const orgRouter = createTRPCRouter({
    get: protectedProcedure
        .input(
            z.object({
                orgId: z.number(),
            }),
        )
        .query(async (opts) => {
            // TODO perms
            const orgQuery: {
                id: bigint;
                name: string;
                description: string;
            }[] = await db.$queryRaw`
                SELECT id, name, description 
                FROM ORG_Org 
                WHERE id = ${opts.input.orgId};
            `;
            const org = orgQuery[0];
            if (!org) return null;

            return {
                id: org.id,
                name: org.name,
                description: org.description,
            };
        }),
});
