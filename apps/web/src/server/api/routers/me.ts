import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { db } from "@/server/db";

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
