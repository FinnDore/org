import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { db } from "@/server/db";

export const meRouter = createTRPCRouter({
    deleteAccount: protectedProcedure.mutation(async (opts) => {
        const userId = opts.ctx.session.user.id;

        await db.$transaction([
            db.$executeRaw`DELETE FROM ORG_User WHERE id = ${userId.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Account WHERE userId = ${userId.user.id}`,
            db.$executeRaw`DELETE FROM ORG_Session WHERE userId = ${userId.user.id}`,
        ]);
    }),
});
